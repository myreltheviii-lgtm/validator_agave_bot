use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_channel::{Receiver, Sender};
use jito_protos::proto::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient,
    SubscribeEntriesRequest,
};
use solana_clock::Slot;
use solana_entry::entry::Entry;
use solana_ledger::devil_mode_jito__::{
    SpeculativeAccountUpdate, SpeculativeExecutorError, SpeculativeSlotExecutor,
};
use solana_pubkey::Pubkey;
use solana_runtime::bank_forks::BankForks;
use std::sync::RwLock;
use tokio::time::{Duration, sleep};
// wincode is the schema-validated serialization library used by solana_entry::entry::Entry.
// Entry derives SchemaRead/SchemaWrite from wincode, which generates a wincode::Deserialize
// implementation that enforces a compile-time length bound (MaxDataShredsLen) on the
// transactions field. Using wincode::deserialize here — the same decoder the speculative
// executor uses — ensures that any entry payload that deserializes successfully for
// graduation detection is also one that the executor will accept, eliminating the class of
// false-positive detections where graduation fires but the executor rejects the bytes.
// Using bincode (the earlier approach) skips the MaxDataShredsLen check and may accept
// payloads that the executor later rejects, causing Phase 1 to record a pending pool that
// Phase 2 never confirms — a stale entry that occupies a slot in the pending map until
// the dead-slot sweeper removes it.
use wincode;

use crate::mev::loaders::pool_graduation::{
    DetectedPool, GraduationDetector, is_graduation_program,
};

/// Connect to the Jito shredstream proxy, subscribe to the slot-entry stream,
/// and drive `SpeculativeSlotExecutor::execute` for every arriving entry batch.
///
/// # Two responsibilities per batch
///
/// 1. **Graduation detection (Phase 1).** Before `execute`, deserializes entry
///    bytes and scans every instruction for pool-creation discriminators.  When
///    a supported DEX creates a tradeable pool, a `DetectedPool` is sent over
///    `graduation_tx` to `MevEngine` BEFORE the corresponding
///    `SpeculativeAccountUpdate`, guaranteeing that `pending_ready` is populated
///    by the time the update arrives.
///
/// 2. **Speculative execution.** Calls `execute`, which applies the batch to a
///    speculative child bank and returns the resulting account deltas.
///
/// # Dead slot pruning
///
/// `dead_slot_rx` receives Slot numbers forwarded from the engine whenever
/// canonical replay permanently rejects a slot.  The bridge calls
/// `detector.clear_dead_slot(slot)` to remove any pending concentrated-liquidity
/// pool creation from that slot.  Without this sweep the per-DEX pending maps
/// fill to their caps (1024 each) over time and genuine new pool detections
/// begin to be silently dropped.
///
/// # Account list — zero allocation per batch
///
/// `accounts_to_watch` is an `Arc<ArcSwap<Vec<Pubkey>>>` rather than
/// `Arc<RwLock<HashSet<Pubkey>>>`.  On every entry batch the bridge calls
/// `accounts_to_watch.load()` — one atomic pointer read returning a `Guard`
/// that dereferences to a `&[Pubkey]` with no allocation at all.  The engine
/// writes a new `Vec` only at graduation events (rare): it builds the new Vec
/// off the hot path and then calls `accounts_to_watch.store(Arc::new(new_vec))`,
/// which atomically replaces the pointer.  Guards held by in-flight bridge
/// iterations continue referencing the old Vec until they drop naturally.
///
/// # URL clone
///
/// `shredstream_url` is cloned once before the reconnect loop so every iteration
/// uses the same pre-cloned `String` rather than re-cloning from the original.
///
/// # Rebasing retry
///
/// `execute` returns `Err(Rebasing(slot))` when `confirm_slot` is mid-rebase.
/// The bridge sleeps 5 ms and retries; the rebase window is microseconds so
/// one retry is always sufficient.
///
/// # Blocking-in-async
///
/// `SpeculativeSlotExecutor::execute` calls `execute_entries_speculatively` which
/// drives the full SVM pipeline — 1–10 ms of CPU-bound work per batch.  Running
/// this directly in an async context would block the Tokio worker thread for that
/// duration and starve every other task scheduled on the same worker.  Each call is
/// therefore wrapped in `tokio::task::spawn_blocking`, which moves the work onto a
/// dedicated blocking thread from Tokio's blocking pool and lets the async worker
/// continue executing I/O tasks and other simulation tasks immediately.
pub async fn run_speculative_executor(
    executor: Arc<SpeculativeSlotExecutor>,
    bank_forks: Arc<RwLock<BankForks>>,
    // Zero-allocation per-batch account list.  The engine builds a new Vec and
    // stores it atomically via ArcSwap::store() when new pools are registered.
    // The bridge reads via ArcSwap::load() — one atomic pointer read, no lock.
    accounts_to_watch: Arc<ArcSwap<Vec<Pubkey>>>,
    update_tx: Sender<SpeculativeAccountUpdate>,
    graduation_tx: Sender<DetectedPool>,
    // Dead-slot receiver forwarded from the engine.  The bridge uses try_recv()
    // (non-blocking) inside the entry-processing loop so it never stalls waiting
    // for a dead-slot notification.
    dead_slot_rx: Receiver<Slot>,
    shredstream_url: String,
) {
    // GraduationDetector is owned exclusively by this async task.
    // No locks needed — all state transitions happen serially within this task.
    let mut detector = GraduationDetector::new();

    // The URL is an owned String moved in from the caller. Each reconnect
    // attempt clones it once to pass ownership to tonic's connect function,
    // which requires a String or value that can be converted to an Endpoint.
    // Reconnects are rare — they occur only when the gRPC stream to the
    // shredstream proxy fails, typically seconds apart. Cloning a short URL
    // string on a path that fires at most once every few seconds is negligible.
    // This is NOT the hot path; the hot path is the entry-processing loop below.
    let url = shredstream_url;

    // Pre-allocate a scratch buffer for resolving instruction account indices
    // into concrete Pubkeys.  This buffer is cleared and refilled on each
    // instruction that passes the is_graduation_program pre-filter.  Reusing
    // a single allocation across all instructions and all batches eliminates
    // one Vec<Pubkey> heap allocation per graduation-candidate instruction —
    // on a busy mainnet slot with many CLMM swaps this prevents thousands of
    // allocations per second.
    let mut resolved_accounts_scratch: Vec<Pubkey> = Vec::with_capacity(32);

    loop {
        // Drain any dead-slot notifications that arrived during the previous
        // connection's lifetime.  This prevents stale entries from accumulating
        // in the pending maps across reconnects.
        while let Ok(dead_slot) = dead_slot_rx.try_recv() {
            detector.clear_dead_slot(dead_slot);
        }

        // Attempt to open a gRPC channel to the shredstream proxy.
        // url.clone() allocates once per reconnect — acceptable given reconnects
        // are rare (seconds apart) and this is not the entry-processing hot path.
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
            // Drain any dead-slot notifications that arrived since the last
            // batch.  try_recv() is non-blocking — it never stalls the task.
            // Pruning here keeps pending map sizes bounded even when dead slots
            // are infrequent (the typical case) and prevents gradual memory growth.
            while let Ok(dead_slot) = dead_slot_rx.try_recv() {
                detector.clear_dead_slot(dead_slot);
            }

            // Load the current account list with a single atomic pointer read.
            // ArcSwap::load() increments an epoch counter and returns a Guard
            // that dereferences to &Vec<Pubkey>.  No RwLock, no allocation, no
            // copy — the bridge reads the exact same Vec the engine last stored.
            // The Guard is valid for the lifetime of this scope and ensures the
            // Vec stays alive even if the engine stores a new one concurrently.
            let accounts_guard = accounts_to_watch.load();

            // Phase 1: scan this entry batch for pool-creation instructions
            // before running execute(). Sending graduation events first ensures
            // that when the engine processes the resulting SpeculativeAccountUpdate
            // on the same batch, pending_ready already contains the DetectedPool
            // for the newly created pool address.
            //
            // wincode::deserialize is used here rather than bincode::deserialize.
            // Entry derives SchemaRead from wincode, which enforces the
            // MaxDataShredsLen bound at decode time. Using the same codec as the
            // speculative executor guarantees that any entry that deserializes here
            // is one the executor will also accept — preventing graduation detection
            // from firing for payloads the executor will silently reject.
            // Detection is best-effort: a deserialization failure silently skips
            // graduation scanning and lets execute() proceed normally.
            if let Ok(parsed_entries) = wincode::deserialize::<Vec<Entry>>(&entry.entries) {
                for parsed_entry in &parsed_entries {
                    for tx in &parsed_entry.transactions {
                        let message = &tx.message;
                        // Pool creation instructions always use static account keys —
                        // the pool address, program ID, and mint pubkeys are resolved
                        // at transaction build time and are never hidden behind a LUT.
                        let static_keys = message.static_account_keys();

                        for ix in message.instructions() {
                            let program_id = match static_keys.get(ix.program_id_index as usize) {
                                Some(pk) => pk,
                                None => continue,
                            };

                            // Pre-filter: only the 7 DEX programs whose creation
                            // instructions the graduation detector can recognize are
                            // worth the cost of resolving account indices into Pubkeys.
                            // The vast majority of instructions (swaps, token transfers,
                            // compute budget, system program) are not on this list and
                            // are skipped in nanoseconds, eliminating millions of Vec
                            // allocations per slot.
                            if !is_graduation_program(program_id) {
                                continue;
                            }

                            // Resolve instruction account indices into concrete Pubkeys.
                            // The scratch buffer is cleared and reused on each qualifying
                            // instruction so that no heap allocation occurs here. Out-of-
                            // bounds indices are silently skipped — a malformed instruction
                            // cannot panic the validator.
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
                                // A new tradeable pool was detected. Send it to the engine
                                // before the execute() call so the engine's pending_ready
                                // map is populated when the SpeculativeAccountUpdate for
                                // this batch arrives.
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

            // Retry loop: re-attempt the same entry batch if the executor signals
            // that it is mid-rebase.  Any other error is non-retryable.
            //
            // execute() calls execute_entries_speculatively which drives full SVM
            // execution — 1–10 ms of CPU-bound work per batch.  Running this
            // directly in the async task would block the Tokio worker thread for
            // that duration and starve every other task on that worker.
            // spawn_blocking moves each attempt onto a dedicated blocking thread
            // from Tokio's blocking pool, keeping async worker threads free.
            //
            // entry_bytes is Arc<Vec<u8>> rather than Vec<u8>. Wrapping in Arc
            // means each retry-loop iteration pays one atomic refcount increment
            // (Arc::clone) rather than a full Vec<u8> heap copy. On the happy path
            // (one iteration, no rebasing) this saves one Vec<u8> clone per batch.
            // Entry bytes can be many kilobytes; the saving is real on the hot path.
            //
            // entry.entries is MOVED directly into Arc::new rather than cloned.
            // entry is the owned value from `while let Ok(Some(entry)) = stream.message()`.
            // The only borrow of entry.entries was inside the graduation-detection block
            // above (lines 199-258); that borrow ended at the closing brace of that block.
            // By this point entry.entries is available for an unconditional move.
            // entry.slot and entry.parent_slot are u64 (Copy), so extracting them first
            // does not partially move the struct and the subsequent field move compiles.
            let entry_slot = entry.slot;
            let entry_parent_slot = entry.parent_slot;
            let entry_bytes: Arc<Vec<u8>> = Arc::new(entry.entries);
            // Arc clone of the accounts Vec — one atomic increment, no heap allocation.
            let accounts_vec: Arc<Vec<Pubkey>> = accounts_guard.clone();

            loop {
                let executor_clone = Arc::clone(&executor);
                let bank_forks_clone = Arc::clone(&bank_forks);
                // Arc clone here, not Vec<u8> clone — all retry iterations share the
                // same underlying bytes without copying them.
                let bytes_arc = Arc::clone(&entry_bytes);
                let accounts_clone = Arc::clone(&accounts_vec);

                let result = tokio::task::spawn_blocking(move || {
                    executor_clone.execute(
                        entry_slot,
                        entry_parent_slot,
                        bytes_arc.as_slice(),
                        &bank_forks_clone,
                        &accounts_clone,
                    )
                })
                .await;

                match result {
                    Err(join_err) => {
                        // The blocking task panicked. Log and break — retrying a
                        // panicking task would likely panic again.
                        log::warn!(
                            "shredstream_bridge: execute task panicked for slot {}: {}",
                            entry_slot, join_err
                        );
                        break;
                    }
                    Ok(Ok(update)) => {
                        if update_tx.send(update).is_err() {
                            log::warn!(
                                "shredstream_bridge: MevEngine update receiver dropped — \
                                 shutting down bridge task"
                            );
                            return;
                        }
                        break;
                    }

                    Ok(Err(SpeculativeExecutorError::Rebasing(slot))) => {
                        // confirm_slot holds the internal write lock for microseconds.
                        // Sleeping 5 ms is always sufficient for the rebase to complete.
                        log::trace!(
                            "shredstream_bridge: slot {} rebasing, retrying in 5ms",
                            slot
                        );
                        sleep(Duration::from_millis(5)).await;
                    }

                    Ok(Err(SpeculativeExecutorError::SlotCompleted(slot))) => {
                        // The slot was confirmed or condemned before this late batch
                        // arrived.  Discard the batch — canonical pipeline already
                        // handled this slot definitively.
                        log::trace!(
                            "shredstream_bridge: slot {} already completed, discarding late batch",
                            slot
                        );
                        break;
                    }

                    Ok(Err(e)) => {
                        // Non-retryable: the same bytes produce the same error on retry.
                        log::warn!(
                            "shredstream_bridge: speculative execute error: {}",
                            e
                        );
                        break;
                    }
                }
            }
        }

        // Both Ok(None) (clean close) and stream errors fall through here.
        log::warn!(
            "shredstream_bridge: entry stream ended — reconnecting in 2s"
        );
        sleep(Duration::from_secs(2)).await;
    }
}
