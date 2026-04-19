// src/server.rs
//
// Binds a Unix domain socket and dispatches each incoming client connection
// to a dedicated OS thread running handler::run(stream).
//
// The sim server is a long-lived companion process to the validator. It starts
// once at engine boot and remains alive for the validator's lifetime. Client
// connections (one per MevShard) are persistent — each shard establishes its
// connection on first query and reuses it for the entire session.
//
// Unix domain sockets are chosen over TCP loopback because they bypass the
// kernel's TCP/IP stack entirely. The round-trip for a length-prefixed bincode
// frame over a Unix socket is ~5–20 µs versus ~50–200 µs over TCP loopback
// on the same machine — a 10× latency improvement on the hot path.

use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::thread;

use anyhow::{Context, Result};
use tracing::{info, warn};

use crate::handler;

pub struct SimServer;

impl SimServer {
    /// Bind the Unix socket and start accepting connections.
    ///
    /// Spawns one listener thread that loops forever. Each accepted connection
    /// gets its own dedicated blocking handler thread (one per MevShard).
    /// Must be called before the Agave validator process starts so the socket
    /// is ready when shards call SimClient::connect().
    pub fn start(socket_path: &str) -> Result<()> {
        // Remove any stale socket file from a previous run. A stale file is a
        // dead filesystem entry left behind when the previous process exited —
        // the OS does not clean up Unix socket files automatically. Without
        // removing it, bind() fails with EADDRINUSE even though nothing is
        // actually listening, making every restart require manual cleanup.
        let path = Path::new(socket_path);
        if path.exists() {
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove stale socket: {}", socket_path))?;
        }

        let listener = UnixListener::bind(socket_path)
            .with_context(|| format!("Failed to bind Unix socket: {}", socket_path))?;

        info!("sim_server: listening on {}", socket_path);

        let socket_path_owned = socket_path.to_string();

        thread::Builder::new()
            .name("sim_server_listener".to_string())
            .spawn(move || {
                info!("sim_server: listener thread started ({})", socket_path_owned);
                accept_loop(listener);
            })
            .context("Failed to spawn sim_server listener thread")?;

        Ok(())
    }
}

fn accept_loop(listener: UnixListener) {
    let mut conn_id: u32 = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                conn_id = conn_id.wrapping_add(1);
                let id = conn_id;
                info!("sim_server: accepted connection #{}", id);

                // Each connection gets its own OS thread running a blocking
                // request/response loop for its entire lifetime. One thread per
                // connection matches the one-SimClient-per-MevShard model on
                // the validator side and avoids any event-loop or async overhead
                // on paths that are fundamentally sequential per shard.
                if let Err(e) = thread::Builder::new()
                    .name(format!("sim_handler_{}", id))
                    .spawn(move || handler::run(stream))
                {
                    warn!("sim_server: failed to spawn handler #{}: {}", id, e);
                }
            }
            Err(e) => {
                // A single accept error is not fatal — log and keep looping.
                warn!("sim_server: accept error: {}", e);
            }
        }
    }

    info!("sim_server: accept loop exited");
}
