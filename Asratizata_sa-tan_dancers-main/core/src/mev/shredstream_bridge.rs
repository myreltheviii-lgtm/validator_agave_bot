use crossbeam_channel::{Receiver, Sender};
use jito_protos::proto::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient,
    SubscribeEntriesRequest,
};
use solana_clock::Slot;
use solana_entry::entry::Entry;
use solana_pubkey::Pubkey;
use tokio::time::{Duration, sleep};

use crate::mev::loaders::pool_graduation::{
    DetectedPool, GraduationDetector, is_graduation_program,
};

/// Connect to the Jito shredstream proxy, subscribe to the slot-entry stream,
/// and drive Phase 1 graduation detection for every arriving entry batch.
///
/// # Sole responsibility
///
/// This function's only job is to detect pool-creation instructions in the raw
/// shredstream entry data and forward `DetectedPool` values to `MevEngine` via
/// `graduation_tx`.  All account-state processing — pool parsing, arb-graph
/// updates, and simulation — belongs to the canonical pipeline:
/// `execute_batch()` in `blockstore_processor.rs` fires `MevExecutedBatch`
/// events which the engine routes through `handle_mev_batch`, and
/// `bank.freeze()` fires `bank_rx` events which the engine routes through
/// `handle_frozen_bank`.  The bridge never reads account state or executes
/// transactions.
///
/// # Graduation detection (Phase 1)
///
/// Before any bank has processed the entry batch, the bridge deserialises entry
/// bytes and scans every instruction for pool-creation discriminators.  When a
/// supported DEX creates a tradeable pool, a `DetectedPool` is sent over
/// `graduation_tx` to `MevEngine`.  The engine stores it in `pending_ready`
/// keyed by pool address.  When the canonical pipeline later commits the
/// pool's creation transaction, Phase 2 fires to integrate it into the arb
/// graph using the bank that just confirmed the creation.
///
/// Sending the graduation event *before* the corresponding `MevExecutedBatch`
/// can arrive guarantees that `pending_ready` is populated by the time the
/// engine processes the batch — the engine drains `graduation_rx` at the top
/// of `handle_mev_batch` to close the remaining race window caused by
/// `crossbeam::select!` non-determinism.
///
/// # Dead slot pruning
///
/// `dead_slot_rx` receives `Slot` values forwarded from the engine whenever
/// canonical replay permanently rejects a slot.  The bridge calls
/// `detector.clear_dead_slot(slot)` to sweep its three per-DEX pending maps
/// (pending_clmm, pending_whirlpool, pending_dlmm).  Without this sweep the
/// maps fill to their per-DEX caps over time and genuine new-pool detections
/// begin to be silently dropped.  `try_recv()` is non-blocking — the bridge
/// never stalls waiting for a dead-slot notification.
///
/// # URL clone
///
/// `shredstream_url` is moved (not cloned) into this function.  Each reconnect
/// iteration calls `url.clone()` once for `ShredstreamProxyClient::connect`;
/// reconnects are rare (network events) so this String clone is irrelevant to
/// steady-state latency.
///
/// # Scratch buffer
///
/// A `Vec<Pubkey>` is pre-allocated once and reused across all instructions in
/// all entry batches.  Clearing and refilling the same allocation instead of
/// allocating a new Vec<Pubkey> per qualifying instruction eliminates thousands
/// of heap allocations per second on a busy mainnet slot where many instructions
/// target watched DEX programs.
pub async fn run_graduation_bridge(
    graduation_tx: Sender<DetectedPool>,
    // Forwarded from MevEngine via a dedicated channel so the bridge receives a
    // complete copy of every dead-slot event without splitting the engine's own
    // dead_slot_rx queue.
    dead_slot_rx: Receiver<Slot>,
    shredstream_url: String,
) {
    // GraduationDetector is owned exclusively by this async task.  It holds the
    // three per-DEX pending maps for concentrated-liquidity two-step graduation.
    // No locks — all mutations happen serially within this single task.
    let mut detector = GraduationDetector::new();

    // Pre-allocate once.  Cleared and refilled on each instruction that passes
    // the is_graduation_program pre-filter.  No allocation inside the hot loop.
    let mut resolved_accounts_scratch: Vec<Pubkey> = Vec::with_capacity(32);

    // shredstream_url is moved here.  Each reconnect iteration clones it once
    // for ShredstreamProxyClient::connect — a String clone on a reconnect path
    // is negligible.
    let url = shredstream_url;

    loop {
        // Drain dead-slot notifications that arrived during the previous
        // connection's lifetime before attempting a new connection.  This
        // prevents stale entries from accumulating across reconnects.
        while let Ok(dead_slot) = dead_slot_rx.try_recv() {
            detector.clear_dead_slot(dead_slot);
        }

        // Attempt to open a gRPC channel to the shredstream proxy.
        let mut client: ShredstreamProxyClient<tonic::transport::Channel> =
            match ShredstreamProxyClient::connect(url.clone()).await {
                Ok(c) => {
                    log::info!(
                        "shredstream_bridge: connected to shredstream proxy at {}",
                        url
                    );
                    c
                }
                Err(e) => {
                    log::warn!(
                        "shredstream_bridge: failed to connect to {} — {}, retrying in 2s",
                        url, e
                    );
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };

        // Open the server-streaming RPC.
        let stream = match client.subscribe_entries(SubscribeEntriesRequest {}).await {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    "shredstream_bridge: subscribe_entries failed — {}, retrying in 2s",
                    e
                );
                sleep(Duration::from_secs(2)).await;
                continue;
            }
        };

        let mut stream = stream.into_inner();

        log::info!("shredstream_bridge: entry stream active");

        while let Ok(Some(entry)) = stream.message().await {
            // Drain dead-slot notifications that arrived since the last batch.
            // try_recv() never blocks — it returns immediately if the channel is
            // empty.  Pruning here keeps pending map sizes bounded even when
            // dead slots are infrequent.
            while let Ok(dead_slot) = dead_slot_rx.try_recv() {
                detector.clear_dead_slot(dead_slot);
            }

            // Phase 1: scan this entry batch for pool-creation instructions.
            //
            // Agave's validator replaced the serde/bincode serialization contract
            // for Entry with wincode — a purpose-built codec that owns the
            // canonical wire format for validator-internal types.  Entry only
            // implements wincode::Deserialize, not serde::Deserialize, so bincode
            // cannot be used here.  The shredstream proxy reassembles shreds into
            // entry bytes using the same wincode encoding that the blockstore uses
            // internally, making wincode::deserialize the correct and only valid
            // deserialization path.
            //
            // Deserialization is best-effort — a failure silently skips graduation
            // scanning for this batch and lets the canonical pipeline handle
            // confirmation normally.  No entry bytes are cloned: the reference
            // `&entry.entries` is valid for the duration of this scope.
            if let Ok(parsed_entries) = wincode::deserialize::<Vec<Entry>>(&entry.entries) {
                for parsed_entry in &parsed_entries {
                    for tx in &parsed_entry.transactions {
                        let message = &tx.message;
                        // Pool-creation instructions always use static account keys —
                        // the pool address, program ID, and mint pubkeys are resolved
                        // at transaction build time and are never hidden behind a LUT.
                        let static_keys = message.static_account_keys();

                        for ix in message.instructions() {
                            let program_id = match static_keys.get(ix.program_id_index as usize) {
                                Some(pk) => pk,
                                None => continue,
                            };

                            // Pre-filter: only the seven DEX programs whose creation
                            // instructions the graduation detector recognises.  The
                            // vast majority of instructions on mainnet — swaps, token
                            // transfers, compute budget, system program — are not on
                            // this list and are skipped in nanoseconds, eliminating
                            // the resolved-accounts allocation cost for all of them.
                            if !is_graduation_program(program_id) {
                                continue;
                            }

                            // Resolve instruction account indices into concrete Pubkeys.
                            // The scratch buffer is cleared and refilled on each
                            // qualifying instruction — one reused allocation across all
                            // instructions and all batches.  Out-of-bounds indices are
                            // silently skipped so a malformed instruction cannot panic
                            // the validator.
                            resolved_accounts_scratch.clear();
                            resolved_accounts_scratch.extend(
                                ix.accounts
                                    .iter()
                                    .filter_map(|&idx| static_keys.get(idx as usize).copied()),
                            );

                            if let Some(detected) = detector.detect_instruction(
                                entry.slot,
                                program_id,
                                &ix.data,
                                &resolved_accounts_scratch,
                            ) {
                                // A new tradeable pool was detected.  Send it to the
                                // engine before the corresponding MevExecutedBatch
                                // arrives so pending_ready is populated when the engine
                                // processes the canonical commit for this batch.
                                if graduation_tx.send(detected).is_err() {
                                    log::warn!(
                                        "shredstream_bridge: MevEngine graduation \
                                         receiver dropped — shutting down bridge task"
                                    );
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Both Ok(None) (clean server close) and stream errors fall through here.
        log::warn!(
            "shredstream_bridge: entry stream ended — reconnecting in 2s"
        );
        sleep(Duration::from_secs(2)).await;
    }
}
