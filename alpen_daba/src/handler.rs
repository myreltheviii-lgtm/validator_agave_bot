// src/handler.rs

use std::io::Write;
use std::os::unix::net::UnixStream;

use tracing::{info, warn};

use crate::account_map::AccountMap;
use crate::router;
// decode_frame, encode_frame, TwoHopSimRequest, and TwoHopSimResponse all live
// in the sim-client crate. The framing functions implement the length-prefixed
// bincode protocol that both sides of the Unix socket must agree on exactly.
// Importing them from sim-client rather than a local wire module guarantees
// that the server's encode/decode logic is always byte-for-byte identical to
// the validator's — they are the same compiled code, not two copies that could
// silently drift apart.
use sim_client::{decode_frame, encode_frame, TwoHopSimRequest, TwoHopSimResponse};

/// Drive a single shard connection until the socket closes or an error occurs.
/// Called from a dedicated std::thread — blocking IO is intentional.
pub fn run(mut stream: UnixStream) {
    let peer = stream.peer_addr()
        .map(|a| format!("{:?}", a))
        .unwrap_or_else(|_| "unknown".to_string());

    info!("sim_server: connection from {}", peer);

    loop {
        // ── read one length-prefixed request ────────────────────────────────

        let req: TwoHopSimRequest = match decode_frame(&mut stream) {
            Ok(r)  => r,
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("connection closed") || msg.contains("UnexpectedEof") {
                    info!("sim_server: connection closed ({})", peer);
                } else {
                    warn!("sim_server: read error ({}): {}", peer, e);
                }
                break;
            }
        };

        // ── build AccountMap from wire account list ──────────────────────────

        let accounts = AccountMap::from_request(&req);
        info!(
            "sim_server: request slot={} accounts={} hop1={:?} hop2={:?}",
            req.slot, accounts.len(), req.hop1_dex, req.hop2_dex
        );

        // ── hop 1 ────────────────────────────────────────────────────────────

        let hop1_out = router::dispatch(
            req.hop1_dex,
            &accounts,
            &req.hop1_pool,
            req.slot,
            req.unix_timestamp,
            req.initial_amount_in,
            &req.hop1_token_in,
        );

        // ── hop 2 — only runs if hop 1 produced non-zero output ──────────────

        let final_out = if hop1_out > 0 {
            router::dispatch(
                req.hop2_dex,
                &accounts,
                &req.hop2_pool,
                req.slot,
                req.unix_timestamp,
                hop1_out,
                &req.hop2_token_in,
            )
        } else {
            0
        };

        info!(
            "sim_server: result hop1_out={} final_out={} initial={}",
            hop1_out, final_out, req.initial_amount_in
        );

        // ── write response ───────────────────────────────────────────────────

        let response = TwoHopSimResponse { hop1_out, final_out };

        let frame = match encode_frame(&response) {
            Ok(f)  => f,
            Err(e) => { warn!("sim_server: encode error ({}): {}", peer, e); break; }
        };

        if let Err(e) = stream.write_all(&frame) {
            warn!("sim_server: write error ({}): {}", peer, e);
            break;
        }
    }

    info!("sim_server: handler exiting ({})", peer);
}
