use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::{Arc, RwLock};

use arc_swap::ArcSwap;
use crossbeam_channel::Receiver;
use solana_client::rpc_client::RpcClient;
use solana_clock::Slot;
pub use solana_ledger::blockstore_processor::MevExecutedBatch;
use solana_runtime::bank::Bank;
use solana_runtime::bank_forks::BankForks;
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use tokio::sync::{broadcast, Semaphore};
use tracing::{debug, error, info, warn};

use crate::mev::arbitrage::{ArbitrageGraph, ArbitrageGraphConfig, MevPoolUpdateEvent, PoolInfo, PoolType};
use crate::mev::constants::{SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT};
use crate::mev::executor::ArbitrageExecutor;
use crate::mev::loaders::pool_discovery::initialize_mint_from_discovered;
use crate::mev::loaders::pool_graduation::{DetectedPool, GraduationSource};
use crate::mev::loaders::pool_scanner::DiscoveredPools;
use crate::mev::lut_manager::LutManager;
use crate::mev::pools::MintPoolData;

/// Maximum number of simulation tasks that may execute concurrently across ALL
/// tracked mints.  A single `Arc<Semaphore>` with this many permits is shared
/// by every `ArbitrageExecutor` in the engine.  When all permits are held,
/// additional simulation tasks yield rather than spawn, capping the total MEV
/// simulation load on the validator's CPU regardless of how many pool-update
/// events arrive simultaneously.
const MAX_CONCURRENT_SIMULATIONS: usize = 64;

/// Maximum number of DetectedPool entries the engine holds in pending_ready at
/// any one time.  Pool creation is permissionless — an adversary can spam
/// creation transactions to grow the map without bound.  The cap bounds the
/// memory cost at approximately 100 bytes × 4096 entries ≈ 400 KB regardless
/// of spam volume.  When the cap is reached, new graduation events are dropped
/// and the pools they describe are missed for this run of the engine.  The dead-
/// slot sweep removes stale entries promptly so the cap is rarely approached
/// under normal operating conditions.
const MAX_PENDING_READY: usize = 4096;

struct MintState {
    /// Shared, atomically swappable pointer to the current pool data for this mint.
    ///
    /// `ArcSwap<MintPoolData>` provides lock-free reads via epoch-based RCU. The
    /// executor calls `load()` on the hot path — one atomic pointer read with no
    /// mutex, no compare-and-swap under contention, and no blocking. The engine
    /// calls `store()` only during graduation events (rare), atomically replacing
    /// the Arc. Readers holding a Guard from the previous generation see consistent
    /// data until they drop the Guard; the old Arc is freed only after all such
    /// Guards are released.
    pool_data: Arc<ArcSwap<MintPoolData>>,
    /// The arb graph is behind a `RwLock` so the engine can insert newly
    /// graduated pools into it without stopping or restarting the associated
    /// `ArbitrageExecutor`.  The executor acquires the read lock exactly once
    /// per event in a single window covering: address → pair-index lookup,
    /// pair → path clone, and structural pre-filter.  The lock is released
    /// before any `tokio::spawn` or `await` point, ensuring that a graduation
    /// write never waits for a mid-await simulation task.
    arb_graph: Arc<RwLock<ArbitrageGraph>>,
    /// Broadcast sender for this mint's pool-update events.  Capacity is 1024
    /// events.  The broadcast semantics evict the oldest event when a receiver
    /// falls behind — the engine's select loop is never blocked by a slow executor.
    pool_update_tx: broadcast::Sender<MevPoolUpdateEvent>,
    /// Every on-chain account pubkey this mint's pools require, stored here so
    /// they can be removed from `account_to_mint` in bulk if the mint is
    /// ever de-registered.  Updated on graduation to include newly discovered
    /// vault accounts.
    tracked_accounts: Vec<Pubkey>,
}

pub struct MevEngine {
    /// Receives `MevExecutedBatch` values from `execute_batch()` in
    /// `blockstore_processor.rs`.  Every time a transaction batch commits to the
    /// canonical replay bank mid-slot, one payload arrives here carrying the
    /// committed bank, the sanitized transactions, and their commit results.
    /// This is the earliest possible signal: the hook fires before
    /// `bank.freeze()`, before `TransactionStatusService`, and before any Geyser
    /// plugin notification.
    mev_batch_rx: Receiver<MevExecutedBatch>,
    /// Receives frozen canonical banks directly from `ReplayStage` the moment
    /// `bank.freeze()` completes.  Used to drive pool-graduation Phase 2 when a
    /// new pool's creation transaction lands just before slot completion.
    bank_rx: Receiver<Arc<Bank>>,
    /// Receives dead-slot numbers from `ReplayStage::mark_dead_slot`.
    ///
    /// A slot is declared dead when the canonical replay pipeline rejects it due
    /// to invalid PoH, failed signature batch verification, SVM execution error,
    /// or chained block-ID mismatch.  On receipt the engine sweeps `pending_ready`
    /// entries for that slot and forwards the slot to the graduation detector in
    /// the bridge so it can clear its per-DEX pending maps.
    dead_slot_rx: Receiver<Slot>,
    /// Receives `DetectedPool` values from the shredstream bridge whenever it
    /// observes a pool-creation instruction in the raw entry stream.
    ///
    /// The bridge performs Phase 1 of the two-phase graduation pipeline: scanning
    /// raw instruction bytes for DEX-specific pool-creation discriminators before
    /// any bank execution has occurred.  The engine performs Phase 2: when a
    /// `MevExecutedBatch` arrives whose committed transactions include the detected
    /// pool address, the creation is confirmed and the pool is integrated into the
    /// arb graph using the bank that just committed those transactions.
    graduation_rx: Receiver<DetectedPool>,
    /// Sender half of the graduation channel, stored here so it can be moved into
    /// the bridge spawn inside `run_async`.  Wrapped in `Option` so `take()` can
    /// transfer ownership exactly once without cloning.
    graduation_tx: Option<crossbeam_channel::Sender<DetectedPool>>,
    /// Sender half of the engine→bridge dead-slot forwarding channel.
    ///
    /// When the engine receives a dead slot from `dead_slot_rx` it immediately
    /// echoes it through this channel.  The bridge calls
    /// `detector.clear_dead_slot(slot)` to sweep its per-DEX pending maps,
    /// preventing stale entries from accumulating and crowding out genuine
    /// new-pool detections.  Using a dedicated forwarding channel (rather than
    /// cloning the receiver) guarantees both endpoints see every event because
    /// cloning a crossbeam Receiver creates an independent consumer that splits
    /// messages non-deterministically.
    bridge_dead_slot_tx: crossbeam_channel::Sender<Slot>,
    /// Receiver half moved into the bridge spawn in `run_async` exactly once.
    /// Wrapped in `Option` so `take()` transfers ownership without cloning.
    bridge_dead_slot_rx: Option<crossbeam_channel::Receiver<Slot>>,
    bank_forks: Arc<RwLock<BankForks>>,
    wallet: Arc<Keypair>,
    lut_manager: Arc<LutManager>,
    rpc_client: Arc<RpcClient>,
    base_priority_fee: u64,
    min_profit_lamports: u64,
    /// When true, the executor runs the full simulation pipeline but does not
    /// submit any transactions.  Used to verify that the arb graph, instruction
    /// builder, and simulation path produce valid results before deploying live
    /// capital.
    validation_mode: bool,
    shredstream_url: String,
    simulation_semaphore: Arc<Semaphore>,
    /// Keyed on mint pubkey.  FxHashMap is used throughout because all keys are
    /// 32-byte Pubkeys, for which SipHash's collision resistance adds no security
    /// value while costing 4–6× the CPU time of FxHash's identity-style mixing.
    mint_states: FxHashMap<Pubkey, MintState>,
    /// Reverse index: account pubkey → mint.  Routes each incoming pool-account
    /// address from a `MevExecutedBatch` to the correct per-mint broadcast channel
    /// in O(1) time without iterating over all registered mints.
    ///
    /// FxHashMap is used because this map is queried for every account key in every
    /// committed transaction on every batch — the hottest lookup in the engine.
    /// SipHash's 4–6× overhead over FxHash for 32-byte keys is a measurable penalty
    /// on this path; FxHash's non-cryptographic mixing is safe because Pubkeys are
    /// not attacker-controlled hash inputs inside a network service.
    account_to_mint: FxHashMap<Pubkey, Pubkey>,
    /// Executor fan-out tasks queued up during `new()` and `register_mint()`.
    /// They cannot be spawned at construction time because `MevEngine::new` is
    /// called from the synchronous `Validator::new` context where no Tokio runtime
    /// is active — calling `tokio::spawn` outside a runtime panics.  The pairs are
    /// drained inside `run_async` after the MEV runtime has been created.
    pending_executor_starts: Vec<(Arc<ArbitrageExecutor>, broadcast::Receiver<MevPoolUpdateEvent>)>,
    /// Zero-allocation per-batch account list shared with the shredstream bridge.
    ///
    /// The bridge calls `ArcSwap::load()` on every entry batch — one atomic pointer
    /// read returning a `Guard<Arc<Vec<Pubkey>>>`, with no heap allocation and no
    /// lock.  The engine writes a new Vec only at graduation events (rare): it builds
    /// the Vec off the hot path and calls `store(Arc::new(new_vec))`, which atomically
    /// replaces the pointer.  Guards held by in-flight bridge iterations see a
    /// consistent snapshot until they drop, at which point the old Arc is freed.
    cached_accounts_to_watch: Arc<ArcSwap<Vec<Pubkey>>>,
    /// Pool addresses detected by the bridge's graduation detector (Phase 1) that
    /// are waiting for their creation transaction to be confirmed by a committed
    /// `MevExecutedBatch` (Phase 2).
    ///
    /// When the bridge sees a pool-creation instruction, it sends a `DetectedPool`
    /// here before the shredstream entry is processed further.  The engine stores it
    /// in this map.  When a `MevExecutedBatch` arrives that includes the pool address
    /// among the committed accounts, Phase 2 fires: the pool is parsed from the bank
    /// and integrated into the appropriate arb graph.
    ///
    /// Entries for failed or dead-slot transactions are swept on receipt of the
    /// corresponding `dead_slot_rx` message.  Bounded by `MAX_PENDING_READY` to
    /// prevent adversarial pool-creation spam from growing the map unboundedly.
    ///
    /// FxHashMap is used for consistency with `account_to_mint` and `mint_states`;
    /// the map is probed on every committed account key lookup.
    pending_ready: FxHashMap<Pubkey, DetectedPool>,
}

impl MevEngine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mev_batch_rx: Receiver<MevExecutedBatch>,
        bank_rx: Receiver<Arc<Bank>>,
        dead_slot_rx: Receiver<Slot>,
        bank_forks: Arc<RwLock<BankForks>>,
        wallet: Arc<Keypair>,
        lut_manager: Arc<LutManager>,
        rpc_client: Arc<RpcClient>,
        base_priority_fee: u64,
        min_profit_lamports: u64,
        validation_mode: bool,
        shredstream_url: String,
        mint_pool_data: Vec<Arc<MintPoolData>>,
    ) -> Self {
        let simulation_semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_SIMULATIONS));

        // Both halves of the graduation channel are stored on the engine. In
        // run_async, graduation_tx is taken out (via Option::take) and moved into
        // the bridge spawn. graduation_rx stays on the engine and participates in
        // the select! loop. The channel is unbounded so Phase 1 detections never
        // block the bridge regardless of how fast the engine processes them.
        let (graduation_tx, graduation_rx) =
            crossbeam_channel::unbounded::<DetectedPool>();

        // The bridge needs to know about dead slots so it can clear stale entries
        // from its per-DEX pending maps. Rather than cloning the engine's dead_slot_rx
        // (which creates an independent consumer and splits messages between the two
        // endpoints), the engine forwards each received dead slot through this
        // dedicated channel. The bridge receives a complete copy of every event.
        let (bridge_dead_slot_tx, bridge_dead_slot_rx) =
            crossbeam_channel::unbounded::<Slot>();

        let mut engine = Self {
            mev_batch_rx,
            bank_rx,
            dead_slot_rx,
            bridge_dead_slot_tx,
            bridge_dead_slot_rx: Some(bridge_dead_slot_rx),
            graduation_rx,
            graduation_tx: Some(graduation_tx),
            bank_forks,
            wallet,
            lut_manager,
            rpc_client,
            base_priority_fee,
            min_profit_lamports,
            validation_mode,
            shredstream_url,
            simulation_semaphore,
            mint_states: FxHashMap::default(),
            account_to_mint: FxHashMap::default(),
            pending_executor_starts: Vec::new(),
            // Initialised as an empty Vec. The shredstream bridge does not start
            // until run_async, so nothing reads this field while the registration
            // loop below executes. All tracked accounts are written into this
            // ArcSwap in a single O(n) pass after every mint has been registered
            // via register_mint_startup — see the comment after the loop.
            cached_accounts_to_watch: Arc::new(ArcSwap::from(Arc::new(Vec::new()))),
            pending_ready: FxHashMap::default(),
        };

        for pool_data in mint_pool_data {
            engine.register_mint_startup(pool_data);
        }

        // account_to_mint is now fully populated — every tracked account across
        // every registered mint maps to exactly one mint pubkey, making it the
        // correct, deduplicated source of truth for the complete account watch list.
        // A single O(n) pass over its keys builds the Vec the shredstream bridge
        // reads on every entry batch to decide which account writes to forward.
        // Building the Vec once here — after all registrations are complete —
        // avoids the O(n²) cost of cloning and rebuilding the growing Vec inside
        // each register_mint_startup call.
        let all_accounts: Vec<Pubkey> = engine.account_to_mint.keys().copied().collect();
        engine.cached_accounts_to_watch.store(Arc::new(all_accounts));

        engine
    }

    /// Register a mint and queue its `ArbitrageExecutor` fan-out task for launch
    /// when the MEV Tokio runtime becomes active inside `run_async`.
    ///
    /// Idempotent: if the mint is already registered the function returns
    /// immediately without rebuilding the arb graph or creating a second executor.
    ///
    /// Every tracked account pubkey for the newly registered mint is inserted into
    /// `cached_accounts_to_watch` under a brief write lock.  Because the set
    /// deduplicates automatically, accounts shared with already-registered mints
    /// are silently skipped — the set stays minimal.  The shredstream bridge holds
    /// an `Arc` clone and snapshots the set once per entry batch — new accounts are
    /// therefore covered by speculative execution within one slot of this call.
    pub fn register_mint(&mut self, pool_data: Arc<MintPoolData>) {
        let mint = pool_data.mint;
        if self.mint_states.contains_key(&mint) {
            return;
        }

        let config = ArbitrageGraphConfig::default();
        // The arb graph is wrapped in RwLock so new pools detected by the
        // graduation pipeline can be inserted at runtime without rebuilding the
        // graph or restarting the executor. The executor acquires the read lock
        // briefly on each event; the engine acquires the write lock only when a
        // new pool graduates — a rare event that does not contend with the hot path.
        let arb_graph = Arc::new(RwLock::new(
            ArbitrageGraph::build_with_config(&pool_data, config)
        ));

        let tracked_accounts = {
            // Recover from a poisoned lock: if a previous holder panicked while
            // the write lock was held, the guard is still valid and the data is
            // still usable — the panic already unwound that writer's stack.
            let g = arb_graph.read().unwrap_or_else(|p| p.into_inner());
            g.all_tracked_accounts()
        };
        // when a graduation event brings a new pool for this mint. Both MintState
        // and ArbitrageExecutor hold Arc<ArcSwap<...>> clones pointing to the same
        // ArcSwap. The engine calls store() on graduation (rare write); the executor
        // calls load() on every simulation (frequent lock-free read).
        let pool_data_swap = Arc::new(ArcSwap::from(pool_data));

        // Each registered mint owns a dedicated broadcast channel. Capacity 1024
        // matches the startup path: the oldest event is silently evicted when a
        // lagging receiver falls behind rather than blocking the engine. The sender
        // half is stored in MintState; the receiver half is handed to the executor
        // so it can be driven by the pending_executor_starts mechanism in run_async.
        let (pool_update_tx, pool_update_rx) = broadcast::channel(1024);

        let executor = Arc::new(ArbitrageExecutor::new(
            Arc::clone(&arb_graph),
            Arc::clone(&pool_data_swap),
            Arc::clone(&self.wallet),
            Arc::clone(&self.lut_manager),
            Arc::clone(&self.rpc_client),
            self.base_priority_fee,
            self.min_profit_lamports,
            self.validation_mode,
        ));

        self.pending_executor_starts.push((executor, pool_update_rx));

        for account in &tracked_accounts {
            self.account_to_mint.insert(*account, mint);
        }

        // Build a new Vec that merges all previously tracked accounts with the
        // newly registered ones.  account_to_mint is already the authoritative
        // deduplication source: any account already present in the map is owned by
        // a previously registered mint and must not appear twice in the Vec.
        // The existing Vec is loaded atomically, extended with only genuinely new
        // accounts, and then atomically stored back.  This is done off the critical
        // path (register_mint runs at startup, not per slot) so the Vec rebuild cost
        // is irrelevant to steady-state latency.
        //
        // A HashSet is built from the existing Vec to provide O(1) membership
        // testing — Vec::contains is O(n) and becomes O(n²) when called for
        // each account over a large existing list.
        {
            let current = self.cached_accounts_to_watch.load();
            let mut new_vec: Vec<Pubkey> = (**current).clone();
            // FxHashSet provides O(1) membership tests for the deduplication scan.
            // The set is built from the existing Vec once per register_mint call —
            // this is off the hot path (startup / rare graduation) so the
            // allocation is not a concern.
            let existing: FxHashSet<Pubkey> = new_vec.iter().copied().collect();
            for account in &tracked_accounts {
                if !existing.contains(account) {
                    new_vec.push(*account);
                }
            }
            self.cached_accounts_to_watch.store(Arc::new(new_vec));
        }

        let tracked_count = tracked_accounts.len();

        self.mint_states.insert(
            mint,
            MintState {
                pool_data: pool_data_swap,
                arb_graph,
                pool_update_tx,
                tracked_accounts,
            },
        );

        info!(
            "MevEngine: registered mint {} ({} tracked accounts)",
            mint, tracked_count
        );
    }

    /// Startup-only variant of [`register_mint`] used exclusively during [`MevEngine::new`]
    /// when initialising the engine with the full batch of pre-loaded pool data.
    ///
    /// The shredstream bridge does not start until [`run_async`] is called. Because
    /// nothing reads `cached_accounts_to_watch` while this method executes, updating
    /// it on every call would be pure waste. This method omits that update entirely.
    /// After the registration loop in [`new`] completes, a single O(n) pass over
    /// `account_to_mint` builds the full deduplicated account Vec and stores it
    /// atomically into `cached_accounts_to_watch` — one allocation for all mints
    /// combined, rather than one allocation per mint.
    fn register_mint_startup(&mut self, pool_data: Arc<MintPoolData>) {
        let mint = pool_data.mint;
        if self.mint_states.contains_key(&mint) {
            return;
        }

        let config = ArbitrageGraphConfig::default();
        let arb_graph = Arc::new(RwLock::new(
            ArbitrageGraph::build_with_config(&pool_data, config)
        ));

        let tracked_accounts = {
            // Recover from a poisoned lock — see register_mint for the rationale.
            let g = arb_graph.read().unwrap_or_else(|p| p.into_inner());
            g.all_tracked_accounts()
        };

        let (pool_update_tx, pool_update_rx) = broadcast::channel(1024);

        let pool_data_swap = Arc::new(ArcSwap::from(pool_data));

        let executor = Arc::new(ArbitrageExecutor::new(
            Arc::clone(&arb_graph),
            Arc::clone(&pool_data_swap),
            Arc::clone(&self.wallet),
            Arc::clone(&self.lut_manager),
            Arc::clone(&self.rpc_client),
            self.base_priority_fee,
            self.min_profit_lamports,
            self.validation_mode,
        ));

        self.pending_executor_starts.push((executor, pool_update_rx));

        // account_to_mint is the reverse index the engine queries on every
        // MevExecutedBatch to route pool-account writes to the correct executor.
        // Populating it here is the only mutation that matters during startup —
        // it is the source of truth from which cached_accounts_to_watch is built
        // once in MevEngine::new after all mints are registered.
        for account in &tracked_accounts {
            self.account_to_mint.insert(*account, mint);
        }

        let tracked_count = tracked_accounts.len();

        self.mint_states.insert(
            mint,
            MintState {
                pool_data: pool_data_swap,
                arb_graph,
                pool_update_tx,
                tracked_accounts,
            },
        );

        info!(
            "MevEngine: registered mint {} ({} tracked accounts)",
            mint, tracked_count
        );
    }

    /// Entry point called on the dedicated `"solMevEngine"` OS thread.
    ///
    /// Creates a private 4-thread Tokio runtime for all MEV work.  Using a
    /// separate runtime decouples MEV simulation scheduling from the validator's
    /// main Tokio runtime so a saturated simulation queue cannot starve consensus,
    /// replay, or banking tasks.
    pub fn run(self) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name_fn(|| {
                static ID: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                format!(
                    "solMevArb{:02}",
                    ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                )
            })
            .enable_all()
            .build()
            .expect("failed to build MEV Tokio runtime");

        info!("MevEngine: Tokio runtime created (4 worker threads)");
        rt.block_on(self.run_async());
        info!("MevEngine: Tokio runtime shut down");
    }

    async fn run_async(mut self) {
        info!(
            "MevEngine: starting {} executor fan-out task(s) and background tasks",
            self.pending_executor_starts.len()
        );

        let graduation_tx = self
            .graduation_tx
            .take()
            .expect("graduation_tx consumed before run_async");

        // The shredstream bridge is wrapped so that its exit is always logged.
        // When the bridge exits — whether due to a permanent gRPC disconnect, a
        // network partition, or a panic — the engine's select loop continues
        // draining bank_rx and dead_slot_rx but receives no further graduation
        // detections.  Without this log the operator has no way to distinguish a
        // silent bridge failure from a period where no tracked pools were touched.
        {
            let shredstream_url = self.shredstream_url.clone();
            // Take the dedicated bridge dead-slot receiver. The engine echoes every
            // dead slot it receives from replay_stage through this channel, giving the
            // bridge a complete copy of every event without splitting the underlying
            // queue (the old clone() approach caused non-deterministic message splitting
            // because both endpoints consumed from the same channel).
            let dead_slot_rx_for_bridge = self
                .bridge_dead_slot_rx
                .take()
                .expect("bridge_dead_slot_rx taken twice — run_async called more than once");
            tokio::spawn(async move {
                crate::mev::shredstream_bridge::run_graduation_bridge(
                    graduation_tx,
                    dead_slot_rx_for_bridge,
                    shredstream_url,
                )
                .await;
                error!(
                    "MevEngine: shredstream graduation bridge task exited — new pool \
                     detection has stopped. The engine continues running against \
                     existing pools but will not detect newly created ones until \
                     the validator is restarted."
                );
            });
        }

        for (executor, rx) in self.pending_executor_starts.drain(..) {
            let sem = Arc::clone(&self.simulation_semaphore);
            tokio::spawn(async move {
                let mint = executor.pool_data_mint();
                if let Err(e) = executor.start(rx, sem).await {
                    error!(
                        "ArbitrageExecutor for mint {} terminated with error: {}",
                        mint, e
                    );
                }
            });
        }

        info!("MevEngine: event loop started");

        loop {
            crossbeam_channel::select! {
                recv(self.mev_batch_rx) -> msg => {
                    match msg {
                        Ok(batch) => self.handle_mev_batch(batch),
                        Err(_) => {
                            info!("MevEngine: mev_batch channel closed — shutting down");
                            break;
                        }
                    }
                }

                recv(self.bank_rx) -> msg => {
                    match msg {
                        Ok(bank) => self.handle_frozen_bank(bank),
                        Err(_) => {
                            info!("MevEngine: bank channel closed — shutting down");
                            break;
                        }
                    }
                }

                recv(self.dead_slot_rx) -> msg => {
                    // When the canonical replay pipeline marks a slot as dead, any
                    // pool-creation transactions that were part of that slot will never
                    // land on-chain.  Sweep pending_ready for all entries that belong
                    // to the dead slot so they cannot produce false-positive graduation
                    // events when future batches touch the same pool address.
                    if let Ok(dead_slot) = msg {
                        self.pending_ready.retain(|_, v| v.slot != dead_slot);

                        // Forward the dead slot to the bridge task so it can sweep its
                        // per-DEX pending maps (pending_clmm, pending_whirlpool,
                        // pending_dlmm). The send is best-effort: if the bridge has
                        // exited (channel disconnected) the error is silently dropped.
                        let _ = self.bridge_dead_slot_tx.send(dead_slot);

                        info!(
                            "MevEngine: slot {} declared dead — pending_ready swept, \
                             bridge notified",
                            dead_slot
                        );
                    }
                }

                recv(self.graduation_rx) -> msg => {
                    // Phase 1 of the two-phase graduation pipeline completed in the bridge.
                    // The bridge detected a pool-creation instruction in the raw entry stream
                    // and sent the pool address, mints, and DEX type here. The engine stores
                    // this in pending_ready. When a MevExecutedBatch arrives that contains
                    // this pool address among committed accounts, Phase 2 fires to integrate
                    // the pool into the arb graph using the bank from that batch.
                    if let Ok(detected) = msg {
                        // Skip if this pool is already tracked — a second graduation event
                        // for an already-registered pool address is redundant.
                        if self.account_to_mint.contains_key(&detected.pool_address) {
                            continue;
                        }

                        // Enforce the cap to prevent adversarial pool-creation spam from
                        // growing the map without bound.
                        if self.pending_ready.len() < MAX_PENDING_READY {
                            self.pending_ready.insert(detected.pool_address, detected);
                        } else {
                            warn!(
                                "MevEngine: pending_ready at cap {MAX_PENDING_READY}, \
                                 dropping graduated pool {}",
                                detected.pool_address
                            );
                        }
                    }
                }
            }
        }

        info!("MevEngine: event loop exited");
    }

    /// Route a committed transaction batch to the correct per-mint executors.
    ///
    /// Called every time `execute_batch()` in `blockstore_processor.rs` commits a
    /// group of transactions to the canonical replay bank mid-slot.  The bank
    /// carried in `batch` reflects ALL writes from every `Ok` commit result in this
    /// batch and is immediately usable for simulation — no further waiting for slot
    /// completion or `bank.freeze()`.
    ///
    /// For every account that was touched by a committed transaction and that maps
    /// to a tracked mint, a `MevPoolUpdateEvent` is broadcast to that mint's
    /// `ArbitrageExecutor`.  The event carries the exact bank reference so the
    /// executor can call `simulate_transaction_unchecked` directly against it.
    fn handle_mev_batch(&mut self, batch: MevExecutedBatch) {
        let slot = batch.slot;
        let bank = &batch.bank;
        let blockhash = bank.last_blockhash();

        // Drain any graduation events that arrived for this batch before scanning
        // the account map. The bridge sends graduation_tx before the entry that
        // produced these commits is fully processed, so every DetectedPool for
        // this batch is typically already in graduation_rx by the time this batch
        // arrives here. Absorbing them first ensures that when the per-account loop
        // below encounters an untracked address that is in pending_ready, the
        // pending_ready entry was inserted BEFORE the loop rather than being missed
        // because select! dispatched mev_batch_rx before graduation_rx for this pair.
        while let Ok(g) = self.graduation_rx.try_recv() {
            if !self.account_to_mint.contains_key(&g.pool_address) {
                if self.pending_ready.len() < MAX_PENDING_READY {
                    self.pending_ready.insert(g.pool_address, g);
                }
            }
        }

        // Walk the committed transactions and collect every account address that
        // a successfully committed transaction wrote.  `TransactionCommitResult`
        // does not carry the write-set directly — but the bank has already applied
        // all writes, so we use the transaction's static account keys as the
        // routing signal: if a tracked pool address appears as a writable account
        // in any successfully committed transaction, the pool's state has changed.
        //
        // This is a routing heuristic, not a precise write-set: a transaction that
        // touches a pool account but produces no actual state change (e.g. a failed
        // inner instruction that reverts) will still trigger a simulation.  That is
        // acceptable — the simulation itself will observe no profitable price
        // discrepancy and return without submitting.  A false negative (missing a
        // real state change) would be worse than a false positive.
        use solana_svm::transaction_commit_result::TransactionCommitResultExtensions;
        let mut events_sent: u32 = 0;

        for (commit_result, tx) in batch.commit_results.iter().zip(batch.transactions.iter()) {
            if !commit_result.was_committed() {
                continue;
            }

            for account_key in tx.message().account_keys().iter() {
                match self.account_to_mint.get(account_key) {
                    Some(mint) => {
                        let state = match self.mint_states.get(mint) {
                            Some(s) => s,
                            None => continue,
                        };

                        let event = MevPoolUpdateEvent {
                            pool_address: *account_key,
                            bank: Arc::clone(bank),
                            blockhash,
                        };

                        match state.pool_update_tx.send(event) {
                            Ok(_) => { events_sent += 1; }
                            Err(e) => {
                                // A send error on a broadcast channel means there are no
                                // active receivers — all ArbitrageExecutor tasks for this
                                // mint have exited. This is unusual and worth logging.
                                warn!(
                                    "MevEngine: pool_update_tx send error for mint {}: {}",
                                    mint, e
                                );
                            }
                        }
                    }

                    None => {
                        // Check whether this untracked account is a newly created pool
                        // address that Phase 1 registered in pending_ready. If so, Phase
                        // 2 fires to integrate it into the arb graph using the bank that
                        // just committed its creation transaction — the freshest possible
                        // state, available immediately after the creation succeeded.
                        if let Some(detected) = self.pending_ready.remove(account_key) {
                            self.handle_pool_graduation(detected, bank);
                        }
                    }
                }
            }
        }

        if events_sent > 0 {
            debug!(
                "MevEngine: slot {} batch committed — {} pool-update event(s) broadcast",
                slot, events_sent
            );
        }
    }

    /// Handle a frozen canonical bank delivered directly by `ReplayStage`.
    ///
    /// At this point the slot is complete and the bank hash is finalized.  The
    /// engine uses the frozen bank only as a fallback for graduation processing:
    /// if a pool-creation transaction was the last transaction in a slot, its
    /// creation may have arrived via `handle_mev_batch` already.  This handler
    /// ensures that any pending graduation that was not triggered mid-slot is
    /// resolved once the slot is fully committed.
    fn handle_frozen_bank(&mut self, bank: Arc<Bank>) {
        let slot = bank.slot();

        // Drain any graduation events that are still queued and attempt to match
        // them against the now-frozen bank. This covers the edge case where the
        // bridge detected a pool creation in a batch that also happened to be the
        // final batch of the slot — the graduation event may have arrived in the
        // channel slightly after the mev_batch for that same creation, causing
        // pending_ready to be populated after handle_mev_batch already processed
        // the relevant accounts.
        while let Ok(g) = self.graduation_rx.try_recv() {
            if !self.account_to_mint.contains_key(&g.pool_address) {
                if self.pending_ready.len() < MAX_PENDING_READY {
                    self.pending_ready.insert(g.pool_address, g);
                }
            }
        }

        // Attempt graduation for any pool whose creation was detected in this slot
        // and whose account now exists in the frozen canonical bank.
        let detected_in_slot: Vec<Pubkey> = self
            .pending_ready
            .iter()
            .filter(|(_, v)| v.slot == slot)
            .map(|(k, _)| *k)
            .collect();

        for pool_address in detected_in_slot {
            if bank.get_account(&pool_address).is_some() {
                if let Some(detected) = self.pending_ready.remove(&pool_address) {
                    self.handle_pool_graduation(detected, &bank);
                }
            }
        }

        debug!("MevEngine: slot {} canonical freeze processed", slot);
    }

    /// Phase 2 of the graduation pipeline.
    ///
    /// Called when a committed `MevExecutedBatch` (or a frozen canonical bank at
    /// slot completion) confirms that the pool-creation transaction succeeded.
    /// At this point `bank.get_account(pool_address)` returns the freshly written
    /// pool state account — all sub-accounts (vaults, tick arrays, oracles) created
    /// in the same transaction are also present in the bank.
    ///
    /// ## Known mint path
    ///
    /// If the non-quote token of the new pool is already tracked by a running
    /// `ArbitrageExecutor`, the pool is fully integrated:
    ///
    /// 1. `initialize_mint_from_discovered` reads the new pool's vault addresses,
    ///    tick arrays, and oracle accounts directly from the bank's write cache.
    ///
    /// 2. The current `MintPoolData` is atomically replaced via `ArcSwap::store`.
    ///    The executor's next `load()` call returns the new version.
    ///
    /// 3. All new accounts are registered in `account_to_mint`,
    ///    `cached_accounts_to_watch`, and `arb_graph` so that vault reserve changes
    ///    trigger re-evaluation of arb pairs through this pool.
    ///
    /// ## Unknown mint path
    ///
    /// If the non-quote token has not been seen before, `initialize_mint_from_discovered`
    /// is called synchronously with the bank.  On success, `register_mint` is called
    /// and the executor is spawned immediately.
    fn handle_pool_graduation(
        &mut self,
        detected: DetectedPool,
        bank: &Arc<Bank>,
    ) {
        // Confirm the pool account exists in the bank. If the creation transaction
        // failed within this batch, the pool address is absent — skip silently.
        if bank.get_account(&detected.pool_address).is_none() {
            return;
        }

        // Identify which token is the speculative (non-quote) side of this pool.
        let is_quote = |m: &Pubkey| -> bool {
            *m == SOL_MINT || *m == USDC_MINT || *m == USDT_MINT || *m == USD1_MINT
        };

        let mint = if !is_quote(&detected.mint0) {
            detected.mint0
        } else {
            detected.mint1
        };

        // Both-quote pools (SOL/USDC, USDC/USDT, etc.) pass the has_quote_token
        // filter in the graduation detector because at least one side is a quote
        // token. But BOTH sides are quote tokens, so the selection above yields a
        // quote token as the "speculative mint". Quote tokens are the denominators
        // of the arb model — they are never the intermediate speculative token.
        // Registering one as a speculative mint would corrupt the graph.
        if is_quote(&mint) {
            return;
        }

        // Map the graduation source to the pool type used in the arb graph.
        let pool_type = match detected.source {
            GraduationSource::PumpSwap      => PoolType::PumpSwap,
            GraduationSource::RaydiumClmm   => PoolType::RaydiumClmm,
            GraduationSource::RaydiumCpmm   => PoolType::RaydiumCpmm,
            GraduationSource::RaydiumAmmV4  => PoolType::RaydiumV4,
            GraduationSource::MeteoraDammV2 => PoolType::MeteoraDammV2,
            GraduationSource::MeteoraDlmm   => PoolType::MeteoraDlmm,
            GraduationSource::OrcaWhirlpool => PoolType::OrcaWhirlpool,
        };

        let pool_info = PoolInfo {
            address: detected.pool_address,
            pool_type,
            token_x: detected.mint0,
            token_y: detected.mint1,
        };

        if self.mint_states.contains_key(&mint) {
            let discovered = build_single_pool_discovered(&detected);

            let new_accounts: Vec<Pubkey> = match initialize_mint_from_discovered(
                &mint,
                discovered,
                &self.wallet.pubkey(),
                bank,
            ) {
                Ok(init) => {
                    let temp_graph = ArbitrageGraph::build_with_config(
                        &init.pool_data,
                        ArbitrageGraphConfig::default(),
                    );
                    let accounts = temp_graph.all_tracked_accounts();

                    {
                        let state = self.mint_states.get(&mint).unwrap();
                        let current: Arc<MintPoolData> = state.pool_data.load_full();
                        let mut updated: MintPoolData = (*current).clone();
                        updated.merge_pools_from(init.pool_data);
                        state.pool_data.store(Arc::new(updated));
                    }

                    accounts
                }
                Err(e) => {
                    warn!(
                        "MevEngine: known-mint graduation vault extraction failed for \
                         {:?} pool {} mint {}: {} — pairs wired, simulation degraded",
                        detected.source, detected.pool_address, mint, e
                    );
                    vec![detected.pool_address]
                }
            };

            let new_pairs = {
                let state = self.mint_states.get(&mint).unwrap();
                // Acquire the write lock to insert the new pool into the live arb
                // graph. Recover from a poisoned lock: a panic inside add_pool on a
                // previous call would have unwound cleanly; the graph remains
                // structurally consistent and the new pool can still be inserted.
                let mut graph = state.arb_graph.write().unwrap_or_else(|p| p.into_inner());
                graph.add_pool(pool_info, &new_accounts)
            };

            for account in &new_accounts {
                self.account_to_mint.insert(*account, mint);
            }

            {
                let current = self.cached_accounts_to_watch.load();
                let mut new_vec: Vec<Pubkey> = (**current).clone();
                let existing: FxHashSet<Pubkey> = new_vec.iter().copied().collect();
                for account in &new_accounts {
                    if !existing.contains(account) {
                        new_vec.push(*account);
                    }
                }
                self.cached_accounts_to_watch.store(Arc::new(new_vec));
            }

            if let Some(state) = self.mint_states.get_mut(&mint) {
                state.tracked_accounts.extend_from_slice(&new_accounts);
            }

            info!(
                "MevEngine: graduated new {:?} pool {} into known mint {} — {} new pair(s), \
                 {} account(s) registered",
                detected.source,
                detected.pool_address,
                mint,
                new_pairs,
                new_accounts.len(),
            );
        } else {
            let discovered = build_single_pool_discovered(&detected);

            match initialize_mint_from_discovered(
                &mint,
                discovered,
                &self.wallet.pubkey(),
                bank,
            ) {
                Ok(init) => {
                    info!(
                        "MevEngine: graduated new mint {} from {:?} pool {}",
                        mint, detected.source, detected.pool_address,
                    );
                    self.register_mint(Arc::new(init.pool_data));

                    let idx = self
                        .pending_executor_starts
                        .iter()
                        .position(|(exec, _)| exec.pool_data_mint() == mint);

                    if let Some(idx) = idx {
                        let (executor, rx) = self.pending_executor_starts.remove(idx);
                        let sem = Arc::clone(&self.simulation_semaphore);
                        tokio::spawn(async move {
                            if let Err(e) = executor.start(rx, sem).await {
                                error!(
                                    "ArbitrageExecutor for graduated mint {} terminated: {}",
                                    mint, e
                                );
                            }
                        });
                        info!(
                            "MevEngine: executor spawned for graduated mint {}",
                            mint
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "MevEngine: graduation parse failed for mint {} pool {}: {}",
                        mint, detected.pool_address, e
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Graduation helpers
// ---------------------------------------------------------------------------

/// Build a `DiscoveredPools` containing exactly one pool entry for the given
/// `DetectedPool`.
///
/// `initialize_mint_from_discovered` expects a `DiscoveredPools` struct where each
/// DEX's pools are listed in the appropriate `Vec<Pubkey>` field.  When graduating
/// a single newly created pool, only one field is populated — the rest remain empty
/// Vecs that the parser skips with an early `Ok(Vec::new())` return.
fn build_single_pool_discovered(detected: &DetectedPool) -> DiscoveredPools {
    let mut d = DiscoveredPools::new();
    match detected.source {
        GraduationSource::PumpSwap      => d.pump.push(detected.pool_address),
        GraduationSource::RaydiumClmm   => d.raydium_clmm.push(detected.pool_address),
        GraduationSource::RaydiumCpmm   => d.raydium_cpmm.push(detected.pool_address),
        GraduationSource::RaydiumAmmV4  => d.raydium_v4.push(detected.pool_address),
        GraduationSource::MeteoraDammV2 => d.meteora_dammv2.push(detected.pool_address),
        GraduationSource::MeteoraDlmm   => d.meteora_dlmm.push(detected.pool_address),
        GraduationSource::OrcaWhirlpool => d.whirlpool.push(detected.pool_address),
    }
    d
}

// =============================================================================
// Tests
// =============================================================================
//
// These tests cover the channel contracts that connect ReplayStage to MevEngine.
// The channels carry `Arc<Bank>` (frozen canonical banks) and `Slot` (dead slots).
// Verifying round-trip fidelity here makes the type contracts explicit and confirms
// that values survive the crossbeam channel without loss or mutation.

#[cfg(test)]
mod tests {
    use super::*;

    use {
        solana_runtime::{
            bank::Bank,
            genesis_utils::{GenesisConfigInfo, create_genesis_config},
        },
        std::sync::Arc,
    };

    // -------------------------------------------------------------------------
    // Test 1 — Frozen bank channel round-trip
    // -------------------------------------------------------------------------

    /// Verifies that an `Arc<Bank>` frozen by `bank.freeze()` can be sent through
    /// a `crossbeam_channel::unbounded::<Arc<Bank>>()` channel and arrive at the
    /// receiver with its `is_frozen()` and `slot()` properties intact.
    #[test]
    fn test_frozen_bank_channel_preserves_slot_and_frozen_state() {
        let (tx, rx) = crossbeam_channel::unbounded::<Arc<Bank>>();

        let GenesisConfigInfo { genesis_config, .. } = create_genesis_config(500_000);
        let bank = Arc::new(Bank::new_for_tests(&genesis_config));
        let expected_slot = bank.slot();

        bank.freeze();

        assert!(bank.is_frozen(), "bank must be frozen before entering the channel");

        tx.send(Arc::clone(&bank)).unwrap();

        let received = rx.recv().unwrap();

        assert!(
            received.is_frozen(),
            "bank must still be frozen after the channel round-trip"
        );
        assert_eq!(
            received.slot(),
            expected_slot,
            "bank slot number must be preserved through the channel"
        );
        assert!(rx.is_empty(), "channel must be empty after the single recv");
    }

    // -------------------------------------------------------------------------
    // Test 2 — Dead slot channel round-trip
    // -------------------------------------------------------------------------

    /// Verifies that a `Slot` (u64) value sent through the dead-slot channel
    /// arrives at the receiver unchanged.
    #[test]
    fn test_dead_slot_channel_preserves_slot_value() {
        let (tx, rx) = crossbeam_channel::unbounded::<solana_clock::Slot>();

        let dead_slot: solana_clock::Slot = 320_000_042;
        tx.send(dead_slot).unwrap();

        let received = rx.recv().unwrap();
        assert_eq!(
            received, dead_slot,
            "dead slot value must be preserved exactly through the channel"
        );
    }
}
