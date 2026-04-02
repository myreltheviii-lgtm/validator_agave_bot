use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
    time::Duration,
};

use crossbeam_channel::Receiver;
use jito_protos::shredstream::{
    shredstream_proxy_server::{ShredstreamProxy, ShredstreamProxyServer},
    Entry as PbEntry, SubscribeEntriesRequest,
};
use log::{debug, info, warn};
use tokio::sync::broadcast::{
    error::RecvError as BroadcastRecvError, Receiver as BroadcastReceiver, Sender,
};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

#[derive(Debug)]
pub struct ShredstreamProxyService {
    entry_sender: Arc<Sender<PbEntry>>,
}

pub fn start_server_thread(
    addr: SocketAddr,
    entry_sender: Arc<Sender<PbEntry>>,
    exit: Arc<AtomicBool>,
    shutdown_receiver: Receiver<()>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let server_handle = runtime.spawn(async move {
            info!("starting server on {:?}", addr);
            if let Err(e) = tonic::transport::Server::builder()
                .add_service(ShredstreamProxyServer::new(ShredstreamProxyService {
                    entry_sender,
                }))
                .serve(addr)
                .await
            {
                // Log the error rather than panicking; the OS thread will continue
                // and will exit cleanly when the shutdown signal arrives.
                warn!("gRPC server exited with error: {e}");
            }
        });

        while !exit.load(std::sync::atomic::Ordering::Relaxed) {
            if shutdown_receiver
                .recv_timeout(Duration::from_secs(1))
                .is_ok()
            {
                server_handle.abort();
                info!("shutting down entries server");
                break;
            }
        }
    })
}

#[tonic::async_trait]
impl ShredstreamProxy for ShredstreamProxyService {
    type SubscribeEntriesStream = ReceiverStream<Result<PbEntry, tonic::Status>>;

    async fn subscribe_entries(
        &self,
        _request: tonic::Request<SubscribeEntriesRequest>,
    ) -> Result<tonic::Response<Self::SubscribeEntriesStream>, tonic::Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let mut entry_receiver: BroadcastReceiver<PbEntry> = self.entry_sender.subscribe();

        tokio::spawn(async move {
            loop {
                match entry_receiver.recv().await {
                    Ok(entry) => {
                        if tx.send(Ok(entry)).await.is_err() {
                            // The mpsc receiver was dropped — the gRPC client disconnected.
                            debug!("client disconnected");
                            break;
                        }
                    }
                    // The broadcast channel is a fixed-size ring buffer. When a slow subscriber
                    // falls behind, old entries are overwritten and the next recv() returns Lagged.
                    // We log the miss count and continue — recv() automatically advances the
                    // subscriber's position past the dropped entries to the current head of the
                    // ring buffer. Treating Lagged as a fatal error and breaking would cause the
                    // gRPC stream to terminate silently, forcing the client to reconnect with no
                    // indication of what happened.
                    Err(BroadcastRecvError::Lagged(n)) => {
                        warn!(
                            "gRPC subscriber lagged by {n} entries; those entries were dropped. \
                             Consider increasing the broadcast channel capacity or reducing \
                             subscriber processing time."
                        );
                        // Continue — the next recv() will start from the current channel head.
                    }
                    // The broadcast sender was dropped, meaning the entry pipeline has shut down.
                    Err(BroadcastRecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        Ok(tonic::Response::new(ReceiverStream::new(rx)))
    }
}
