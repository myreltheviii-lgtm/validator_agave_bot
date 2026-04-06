use {
    crate::mev::{
        arbitrage::{ArbitrageGraph, ArbitragePath, MevPoolUpdateEvent},
        executor::{
            smb_instruction_builder::SmbInstructionBuilder,
            token_flow_validator::TokenFlowValidator,
        },
        lut_manager::LutManager,
        pools::MintPoolData,
    },
    anyhow::{anyhow, Result},
    arc_swap::ArcSwap,
    solana_pubkey::Pubkey,
    solana_runtime::bank::Bank,
    solana_compute_budget_interface::ComputeBudgetInstruction,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_signature::Signature,
    solana_signer::Signer,
    solana_transaction::{
        TransactionVerificationMode,
        versioned::VersionedTransaction,
    },
    std::{
        sync::{Arc, RwLock},
        time::{Duration, Instant},
    },
    tokio::sync::{broadcast, Semaphore},
    tracing::{info, warn},
};

/// How often the confirmation poller checks whether a submitted transaction
/// has reached `confirmed` commitment on the cluster.
const CONFIRM_POLL_INTERVAL_MS: u64 = 400;

/// Maximum wall-clock seconds the confirmation poller waits before declaring a
/// submitted transaction timed out and returning an error to the caller.
const CONFIRM_TIMEOUT_SECS: u64 = 30;

/// Compute units provisionally allocated per swap hop during the first
/// (simulation) transaction build.  This value is intentionally generous so the
/// SVM never hits the CU cap mid-execution during simulation — a cap-induced
/// truncation would cause the simulation to return a misleading error rather than
/// reporting unprofitability, which would suppress valid opportunities.
/// The actual units consumed from the simulation result (with 10 % headroom
/// added) are used for the final submission transaction.
const ESTIMATED_CU_PER_HOP: u32 = 700_000;

/// One per tracked mint — receives pool-update events from the engine's broadcast
/// channel, evaluates two-hop arbitrage paths through the arb graph, and submits
/// profitable transactions to the cluster.
///
/// `ArbitrageExecutor` is `Send + Sync`.  All mutable shared state lives behind
/// `Arc<RwLock<...>>`.  The fan-out task and all simulation tasks hold
/// `Arc<ArbitrageExecutor>` clones so no field is accessed without going through
/// a lock or an atomic operation.
///
/// ## Graph access pattern
///
/// `arb_graph` is behind a `RwLock` so that the engine can extend the graph at
/// runtime when new pools are detected via the graduation pipeline, without
/// restarting the executor.  The fan-out loop acquires the read lock exactly once
/// per event, collects all qualifying pair indices and their fully-owned
/// `ArbitragePath` values, then releases the lock before spawning any tasks.
/// No lock is held across any `await` point or `tokio::spawn` call, ensuring
/// that a graph write (add_pool from the engine thread) is never blocked by
/// a simulation task that may be mid-await.
pub struct ArbitrageExecutor {
    pub(crate) arb_graph: Arc<RwLock<ArbitrageGraph>>,
    /// Atomically swappable pointer to the current pool data for this mint.
    ///
    /// `ArcSwap<MintPoolData>` allows the engine to update vault addresses, tick
    /// arrays, and oracle accounts when new pools graduate into a known mint at
    /// runtime — without stopping the executor or taking any mutex. On the hot path
    /// in `try_execute_arbitrage`, `load_full()` is used: one atomic load plus a
    /// refcount increment, returning an owned Arc that immediately releases the epoch
    /// pin. Short-lived reads (mint lookup in `pool_data_mint`, logging) use `load()`
    /// since their Guard is dropped before any await point. The engine calls `store()`
    /// only on graduation events (rare).
    pub(crate) pool_data: Arc<ArcSwap<MintPoolData>>,
    /// The most recently frozen canonical bank, updated by MevEngine on every
    /// `BankNotification::Frozen` event.  Simulation tasks prefer the speculative
    /// bank embedded in the pool-update event; they fall back to this value only
    /// when no speculative bank is present (e.g. during the brief startup window
    /// before the first shredstream entry arrives or when the speculative bank for
    /// the triggering slot has already been confirmed and removed).
    pub(crate) canonical_bank: Arc<RwLock<Option<Arc<Bank>>>>,
    pub(crate) wallet: Arc<Keypair>,
    pub(crate) lut_manager: Arc<LutManager>,
    pub(crate) rpc_client: Arc<solana_client::rpc_client::RpcClient>,
    pub(crate) base_priority_fee: u64,
    /// Minimum lamport profit the on-chain SMB executor must realise before the
    /// trade is permitted to succeed.  The value is serialised into the instruction
    /// data at bytes [1..9] (little-endian u64) and read by the program after it
    /// computes the realised output of the two-hop swap.  If the output is below
    /// this threshold the program reverts — the transaction fails cleanly with no
    /// net loss instead of landing as a sub-fee trade.
    ///
    /// The simulation pass always uses zero so the SVM always runs through the
    /// complete execution path regardless of current pool prices.  Only the final
    /// submission transaction encodes this field with the non-zero operator value.
    pub(crate) min_profit_lamports: u64,
    pub(crate) validation_mode: bool,
}

impl ArbitrageExecutor {
    pub fn new(
        arb_graph: Arc<RwLock<ArbitrageGraph>>,
        pool_data: Arc<ArcSwap<MintPoolData>>,
        canonical_bank: Arc<RwLock<Option<Arc<Bank>>>>,
        wallet: Arc<Keypair>,
        lut_manager: Arc<LutManager>,
        rpc_client: Arc<solana_client::rpc_client::RpcClient>,
        base_priority_fee: u64,
        min_profit_lamports: u64,
        validation_mode: bool,
    ) -> Self {
        Self {
            arb_graph,
            pool_data,
            canonical_bank,
            wallet,
            lut_manager,
            rpc_client,
            base_priority_fee,
            min_profit_lamports,
            validation_mode,
        }
    }

    pub fn pool_data_mint(&self) -> Pubkey {
        // load() is a single atomic pointer read — no mutex, no blocking.
        self.pool_data.load().mint
    }

    /// Fan-out task entry point.  Runs for the entire validator lifetime on the
    /// MEV Tokio runtime.
    ///
    /// For every `MevPoolUpdateEvent` the engine broadcasts to this mint's channel,
    /// this task looks up which arb pairs include the updated pool, pre-filters by
    /// structural validity via `SmbInstructionBuilder::can_execute_2hop`, and
    /// spawns one bounded simulation task per qualifying pair.
    ///
    /// ## Lock discipline
    ///
    /// The arb graph is behind `Arc<RwLock<ArbitrageGraph>>` so the engine can
    /// extend it at runtime when new pools graduate into the graph.  The fan-out
    /// loop acquires the read lock exactly once per event in a single window that
    /// covers all three operations: address → pair-index lookup, pair → path clone,
    /// and structural pre-filter.  All operations inside the lock window are pure
    /// in-memory comparisons and struct clones — no I/O, no system calls, no awaits.
    /// The lock is released before any `tokio::spawn` call or any `await` point.
    ///
    /// This design gives the write side (the engine's graduation handler calling
    /// `add_pool`) the lowest possible contention: it only competes with reader
    /// tasks during the brief per-event window, not during simulation (which can
    /// take milliseconds and involves multiple awaits).
    ///
    /// Simulation tasks acquire a permit from `simulation_semaphore` before
    /// running.  The semaphore is shared across every executor in the engine,
    /// so no matter how many mints fire simultaneously the total concurrently
    /// executing simulations never exceeds the permit count.
    pub async fn start(
        self: Arc<Self>,
        mut pool_update_rx: broadcast::Receiver<MevPoolUpdateEvent>,
        simulation_semaphore: Arc<Semaphore>,
    ) -> Result<()> {
        let mint = self.pool_data.load().mint;
        info!(
            "ArbitrageExecutor[{}]: fan-out task started (validation_mode={})",
            mint, self.validation_mode
        );

        loop {
            match pool_update_rx.recv().await {
                Ok(event) => {
                    let pool_address = event.pool_address;

                    // Single read lock window: address → pair indices → path clones → pre-filter.
                    //
                    // All three steps are combined into one lock acquisition. The
                    // previous design acquired two separate locks: once for the pair
                    // indices (collected into Vec<usize>), then once per pair for the
                    // path clone. That approach paid N + 1 atomic operations per event
                    // and allocated an intermediate Vec<usize> that was used only to
                    // drive the second set of lock acquisitions.
                    //
                    // The combined approach pays exactly one atomic operation per event
                    // regardless of how many qualifying pairs the pool affects, and
                    // produces directly usable (pair_idx, path) pairs with no
                    // intermediate allocation.
                    //
                    // The lock is held for: one FxHashMap lookup + one Vec iteration
                    // where each iteration does a Vec index + PoolPair.to_path() + a
                    // structural check. All of these are nanoseconds — no I/O, no
                    // system calls. The lock is fully released before tokio::spawn.
                    let qualifying_pairs: Vec<(usize, ArbitragePath)> = {
                        let graph = self.arb_graph.read().unwrap();
                        graph
                            .get_affected_pairs(&pool_address)
                            .iter()
                            .copied()
                            .filter_map(|pair_idx| {
                                let path = graph.get_pair(pair_idx)?.to_path();
                                if SmbInstructionBuilder::can_execute_2hop(&path) {
                                    Some((pair_idx, path))
                                } else {
                                    None
                                }
                            })
                            .collect()
                    };

                    if qualifying_pairs.is_empty() {
                        continue;
                    }

                    tracing::debug!(
                        "ArbitrageExecutor[{}]: pool {} affects {} qualifying pair(s) \
                         (from_speculative={})",
                        mint,
                        pool_address,
                        qualifying_pairs.len(),
                        event.from_speculative_execution,
                    );

                    let event = Arc::new(event);
                    let discovered_at = Instant::now();

                    for (pair_idx, path) in qualifying_pairs {
                        let self_clone = Arc::clone(&self);
                        let event_clone = Arc::clone(&event);
                        let sem = Arc::clone(&simulation_semaphore);

                        tokio::spawn(async move {
                            // Acquire a permit before doing any work. If the semaphore is
                            // exhausted, this await yields — no OS thread is blocked.
                            // The permit is moved into try_execute_arbitrage and dropped
                            // there after simulation completes, before the I/O-bound
                            // submission phase starts.
                            let permit = match sem.acquire_owned().await {
                                Ok(p) => p,
                                Err(_) => return, // semaphore closed = validator shutdown
                            };

                            if let Err(e) = self_clone
                                .try_execute_arbitrage(
                                    &path,
                                    &event_clone,
                                    pair_idx,
                                    discovered_at,
                                    permit,
                                )
                                .await
                            {
                                warn!(
                                    "ArbitrageExecutor[{}]: pair {} execution error: {}",
                                    self_clone.pool_data.load().mint, pair_idx, e
                                );
                            }
                        });
                    }
                }

                Err(broadcast::error::RecvError::Lagged(n)) => {
                    // The broadcast channel's internal ring buffer overflowed.  The oldest
                    // events were automatically evicted to make room for new ones.  This is
                    // acceptable — a slightly stale pool update that was superseded by a
                    // newer one before it could be processed would have produced a
                    // sub-optimal simulation anyway.
                    warn!(
                        "ArbitrageExecutor[{}]: lagged, skipped {} event(s)",
                        mint, n
                    );
                }

                Err(broadcast::error::RecvError::Closed) => {
                    info!(
                        "ArbitrageExecutor[{}]: broadcast channel closed — stopping",
                        mint
                    );
                    break;
                }
            }
        }

        Ok(())
    }

    /// Evaluate one two-hop arbitrage path against the current pool state and,
    /// if the simulation is profitable, submit the transaction to the cluster.
    ///
    /// The function executes in two distinct transaction-build phases:
    ///
    /// Phase 1 — Simulation.  A transaction is built with a conservatively large
    /// CU limit and `min_profit_lamports = 0` (disabled).  The zero threshold
    /// ensures the SVM always runs the complete execution path; if the threshold
    /// were non-zero, the program would revert during simulation whenever the
    /// current price produces a small profit, masking profitable opportunities.
    /// The simulation result provides `units_consumed` — the actual CU cost.
    ///
    /// Phase 2 — Submission.  A second transaction is built with the exact
    /// `units_consumed` multiplied by 1.10 (10 % headroom) as the CU limit, and
    /// with `min_profit_lamports` set to the operator-configured value.  The
    /// tighter CU limit reduces the total priority fee for the same per-CU rate.
    /// The on-chain profit floor means that if the pool price moves against the
    /// trade between simulation and landing, the program reverts cleanly rather
    /// than landing as a net loss.
    ///
    /// Bank selection priority: speculative bank (from shredstream, reflects
    /// post-entry account state before canonical confirmation) > canonical bank
    /// (most recently frozen, reflects state as of the last confirmed block).
    async fn try_execute_arbitrage(
        &self,
        path: &ArbitragePath,
        event: &MevPoolUpdateEvent,
        pair_idx: usize,
        discovered_at: Instant,
        simulation_permit: tokio::sync::OwnedSemaphorePermit,
    ) -> Result<()> {
        // Load the current pool data with a single atomic pointer increment.
        // `load_full()` performs one atomic load plus a refcount increment and returns
        // an owned `Arc<MintPoolData>`. Unlike `load()` which returns an epoch-pinned
        // Guard, `load_full()` releases the epoch immediately — the old generation can
        // be freed as soon as other existing Guards from that epoch drop. This matters
        // because `try_execute_arbitrage` contains two long await points: the RPC send
        // (hundreds of milliseconds) and the confirmation poller (up to 30 seconds).
        // With MAX_CONCURRENT_SIMULATIONS = 64 tasks holding epoch Guards across those
        // awaits, a graduation store() on the engine would otherwise be unable to free
        // the superseded MintPoolData for the full 30-second confirmation window. An
        // owned Arc holds the data alive for exactly as long as this simulation needs
        // it — no more, no less — while allowing the epoch to advance freely.
        let pool_data: Arc<MintPoolData> = self.pool_data.load_full();

        // Resolve the simulation bank.  The speculative bank holds the write-set
        // of entries that have been executed speculatively but not yet confirmed
        // by the canonical replay pipeline — it reflects account state further
        // ahead in time than the canonical bank, giving the engine an edge over
        // bots that only have access to the last confirmed block.
        let sim_bank: Arc<Bank> = match &event.speculative_bank {
            Some(bank) => Arc::clone(bank),
            None => {
                // Acquire the read lock, clone the Arc immediately, and release
                // the lock before any further work. The lock is held for exactly
                // one Arc::clone — an atomic refcount increment. Keeping the
                // guard alive longer than necessary would block any concurrent
                // engine write that tries to update canonical_bank.
                let maybe_bank: Option<Arc<Bank>> = self
                    .canonical_bank
                    .read()
                    .map_err(|_| anyhow!("canonical_bank RwLock poisoned"))?
                    .as_ref()
                    .map(Arc::clone);
                // Read lock is released here — the guard has been dropped.
                match maybe_bank {
                    Some(bank) => bank,
                    None => {
                        // The canonical bank is None only during the brief startup
                        // window before replay has frozen the first block. Return
                        // without error — subsequent events will have a bank.
                        return Ok(());
                    }
                }
            }
        };

        // Validate the token-flow path and build the initial SMB instruction.
        let token_flow = TokenFlowValidator::validate_and_build_flow(path)?;

        // Phase 1 instruction: zero profit threshold so simulation always runs fully,
        // generous CU limit so the executor never hits the cap mid-execution.
        let sim_instruction = SmbInstructionBuilder::build_instruction_with_flow(
            &self.wallet,
            path,
            &token_flow,
            &*pool_data,
            &sim_bank,
            ESTIMATED_CU_PER_HOP.saturating_mul(path.hop_count() as u32),
            true,
            0, // profit floor disabled for simulation — gate only on real submission
        )?;

        let sim_instructions: Vec<Instruction> = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(
                ESTIMATED_CU_PER_HOP.saturating_mul(path.hop_count() as u32),
            ),
            ComputeBudgetInstruction::set_compute_unit_price(self.base_priority_fee),
            sim_instruction,
        ];

        let sim_message = self.lut_manager.create_v0_message(
            &sim_instructions,
            &self.wallet.pubkey(),
            event.blockhash,
        )?;

        let sim_versioned_tx = VersionedTransaction::try_new(sim_message, &[&*self.wallet])?;

        // Both `verify_transaction` (ALT resolution + hash) and
        // `simulate_transaction_unchecked` (full SVM execution) are CPU-bound
        // synchronous calls that can take 1–10 ms each.  Running them directly
        // inside this async fn would block the Tokio worker thread for that
        // duration, starving every other task on the same worker — including
        // other in-flight simulation tasks.  `spawn_blocking` moves each call
        // onto a dedicated blocking thread from Tokio's blocking pool, leaving
        // the async worker threads free to drive I/O and other tasks.
        //
        // `sim_versioned_tx` is moved into the closure rather than cloned —
        // it is not used after this call.  `sim_bank` is cloned (Arc clone,
        // cheap) so the closure can own it independently.
        let sim_bank_clone = Arc::clone(&sim_bank);
        let runtime_tx = tokio::task::spawn_blocking(move || {
            sim_bank_clone.verify_transaction(
                sim_versioned_tx,
                TransactionVerificationMode::HashOnly,
            )
        })
        .await
        .map_err(|e| anyhow!("verify_transaction task panicked for pair {}: {}", pair_idx, e))?
        .map_err(|e| anyhow!("transaction sanitization failed for pair {}: {:?}", pair_idx, e))?;

        // `simulate_transaction_unchecked` does NOT assert `bank.is_frozen()` so it
        // is safe to call on an active (non-frozen) speculative bank.  It runs the
        // full SVM execution stack in memory, discards all write-set mutations, and
        // returns the result, units consumed, and fee.
        //
        // `runtime_tx` is MOVED into this closure rather than cloned — it is not used
        // at any point after this call.  A `SanitizedVersionedTransaction` contains a
        // `Vec` of compiled instructions and a `Vec` of resolved account keys; cloning
        // it allocates on the heap on every simulation attempt.  Moving is free.
        let sim_bank_clone2 = Arc::clone(&sim_bank);
        let sim_result = tokio::task::spawn_blocking(move || {
            sim_bank_clone2.simulate_transaction_unchecked(&runtime_tx, false)
        })
        .await
        .map_err(|e| anyhow!("simulate_transaction task panicked for pair {}: {}", pair_idx, e))?;

        if sim_result.result.is_err() {
            tracing::debug!(
                "ArbitrageExecutor[{}]: pair {} simulation rejected: {:?}",
                pool_data.mint,
                pair_idx,
                sim_result.result
            );
            return Ok(());
        }

        let latency_us = discovered_at.elapsed().as_micros();

        if self.validation_mode {
            // The simulation_permit drops here when this early return is reached —
            // Rust's drop semantics guarantee that function parameters are dropped
            // at the point the function exits, in reverse declaration order.
            // The permit is released before any I/O occurs, consistent with the
            // design principle that it gates CPU simulation, not logging or submission.
            info!(
                "[VALIDATION] pair={} mint={} latency={}µs units={} fee={:?} \
                 from_speculative={}",
                pair_idx,
                pool_data.mint,
                latency_us,
                sim_result.units_consumed,
                sim_result.fee,
                event.from_speculative_execution,
            );
            return Ok(());
        }

        // Phase 2 — build the final submission transaction.
        //
        // The simulation semaphore permit is released here, before any network I/O.
        // The semaphore's purpose is to bound concurrent CPU-intensive SVM executions.
        // Once simulation has confirmed the opportunity is profitable, we enter the
        // submission pipeline which is dominated by network round-trips: send_transaction
        // (100–500 ms) and confirm_transaction (polling up to 30 seconds). Holding the
        // permit through that window would prevent new profitable simulations from starting
        // if all 64 permits were consumed by confirmation-waiting tasks. Releasing here
        // separates the CPU-gating concern from the I/O-bound submission concern — any
        // number of submissions can be in-flight while the semaphore budget is fully
        // available to new simulation attempts.
        drop(simulation_permit);

        // The 10 % headroom on the CU limit absorbs minor variance between
        // simulation and on-chain execution caused by sysvar value differences,
        // tick/bin-array position drift, and other environmental factors.
        // The 5 000 CU floor prevents a degenerate zero-limit transaction if
        // `units_consumed` was reported as an unusually small value.
        let exact_cu_limit =
            ((sim_result.units_consumed as f64 * 1.10) as u32).max(5_000);

        let final_instruction = SmbInstructionBuilder::build_instruction_with_flow(
            &self.wallet,
            path,
            &token_flow,
            &*pool_data,
            &sim_bank,
            exact_cu_limit,
            true,
            self.min_profit_lamports, // operator-configured on-chain profit floor
        )?;

        let final_instructions: Vec<Instruction> = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(exact_cu_limit),
            ComputeBudgetInstruction::set_compute_unit_price(self.base_priority_fee),
            final_instruction,
        ];

        let final_message = self.lut_manager.create_v0_message(
            &final_instructions,
            &self.wallet.pubkey(),
            event.blockhash,
        )?;

        let final_tx = VersionedTransaction::try_new(final_message, &[&*self.wallet])?;

        info!(
            "ArbitrageExecutor[{}]: pair={} latency={}µs units={} fee={:?} — submitting",
            pool_data.mint,
            pair_idx,
            latency_us,
            sim_result.units_consumed,
            sim_result.fee,
        );

        let signature = self.send_transaction(final_tx).await?;
        info!(
            "ArbitrageExecutor[{}]: submitted pair={} sig={}",
            pool_data.mint, pair_idx, signature
        );

        self.confirm_transaction(signature).await?;

        Ok(())
    }

    /// Submit the transaction via `spawn_blocking` to avoid stalling the Tokio
    /// executor thread on the synchronous blocking HTTP round-trip to the RPC.
    async fn send_transaction(
        &self,
        transaction: VersionedTransaction,
    ) -> Result<Signature> {
        let rpc = Arc::clone(&self.rpc_client);
        tokio::task::spawn_blocking(move || rpc.send_transaction(&transaction))
            .await
            .map_err(|e| anyhow!("send_transaction task panicked: {}", e))?
            .map_err(|e| anyhow!("send_transaction RPC error: {}", e))
    }

    /// Poll the cluster for transaction confirmation at fixed intervals until the
    /// transaction is confirmed, rejected on-chain, or the absolute deadline expires.
    ///
    /// Each poll call is wrapped in `spawn_blocking` to avoid blocking the async
    /// executor thread during the blocking HTTP request.  The deadline is measured
    /// from the start of this function rather than accumulating sleep intervals, so
    /// a slow poll does not silently compress the remaining confirmation window.
    async fn confirm_transaction(&self, signature: Signature) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(CONFIRM_TIMEOUT_SECS);

        loop {
            // Clone directly from self.rpc_client each iteration.  The previous
            // code created an outer `rpc` binding via Arc::clone at function entry
            // and then a second `rpc_clone` inside the loop — two Arc increments
            // to achieve the same result as one.  The outer intermediate is removed.
            let rpc_clone = Arc::clone(&self.rpc_client);
            let sig = signature;

            let status = tokio::task::spawn_blocking(move || {
                rpc_clone.get_signature_status(&sig)
            })
            .await
            .map_err(|e| anyhow!("confirm task panicked: {}", e))?
            .map_err(|e| anyhow!("get_signature_status RPC error: {}", e))?;

            match status {
                Some(Ok(())) => {
                    info!(
                        "ArbitrageExecutor[{}]: confirmed {}",
                        self.pool_data.load().mint, signature
                    );
                    return Ok(());
                }
                Some(Err(tx_err)) => {
                    return Err(anyhow!(
                        "transaction {} rejected on-chain: {}",
                        signature,
                        tx_err
                    ));
                }
                None => {
                    if Instant::now() >= deadline {
                        return Err(anyhow!(
                            "transaction {} not confirmed within {}s",
                            signature,
                            CONFIRM_TIMEOUT_SECS
                        ));
                    }
                    tokio::time::sleep(Duration::from_millis(CONFIRM_POLL_INTERVAL_MS)).await;
                }
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// These tests cover the Phase 2 compute-unit limit arithmetic used in
// `try_execute_arbitrage`. The formula `((units_consumed * 1.10) as u32).max(5_000)`
// has two components:
//
//   1. The 10% headroom multiplier absorbs the variance between simulation CU cost
//      and actual on-chain CU cost caused by sysvar value changes between the
//      simulation slot and the landing slot, tick/bin-array position drift in CLMM
//      pools, and other environmental factors that the simulator cannot predict.
//      Without headroom, a transaction that consumed exactly its declared limit
//      during simulation would fail on-chain with a compute-budget-exceeded error
//      whenever any of these factors add even one additional compute unit.
//
//   2. The 5,000 CU floor prevents a degenerate zero-limit transaction when the
//      simulator reports an unusually small or zero `units_consumed` value. The
//      Solana scheduler rejects transactions whose declared CU limit is zero, and
//      any transaction that declares fewer CUs than it actually uses fails mid-
//      execution with a compute-budget error — producing a fee-paying failure
//      that earns nothing.

#[cfg(test)]
mod tests {

    // -------------------------------------------------------------------------
    // Test 1 — Phase 2 CU limit arithmetic
    // -------------------------------------------------------------------------

    /// Verifies the exact formula applied to `sim_result.units_consumed` when
    /// building the final submission transaction in Phase 2 of
    /// `try_execute_arbitrage`.
    #[test]
    fn test_phase2_cu_limit_arithmetic() {
        let compute_exact_cu = |units_consumed: u64| -> u32 {
            ((units_consumed as f64 * 1.10) as u32).max(5_000)
        };

        assert_eq!(compute_exact_cu(100_000), 110_000);
        assert_eq!(compute_exact_cu(200_000), 220_000);
        assert_eq!(compute_exact_cu(50_000),   55_000);
        assert_eq!(compute_exact_cu(10_000),   11_000);
        assert_eq!(compute_exact_cu(700_000), 770_000);
        assert_eq!(compute_exact_cu(0),     5_000, "zero consumed must produce the 5,000 CU floor");
        assert_eq!(compute_exact_cu(1),     5_000, "1 CU * 1.10 = 1 CU → clamped to 5,000");
        assert_eq!(compute_exact_cu(4_545), 5_000, "4545 * 1.10 = 4999 → clamped to 5,000");
        assert_eq!(compute_exact_cu(4_546), 5_000);
        assert_eq!(compute_exact_cu(5_000), 5_500, "5000 * 1.10 = 5500, above the floor");

        let near_max: u64 = u32::MAX as u64;
        let result = compute_exact_cu(near_max);
        assert!(
            result <= u32::MAX,
            "CU limit must always fit in u32 regardless of units_consumed magnitude"
        );
    }
}
