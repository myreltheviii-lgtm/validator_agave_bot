use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use arc_swap::ArcSwap;
use crossbeam_channel::Receiver;
use solana_account::{AccountSharedData, ReadableAccount};
use solana_client::rpc_client::RpcClient;
use solana_clock::Slot;
use solana_ledger::devil_mode_jito__::{SpeculativeAccountUpdate, SpeculativeSlotExecutor};
use solana_runtime::bank::Bank;
use solana_runtime::bank_forks::BankForks;
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use tokio::sync::{broadcast, Semaphore};
use tracing::{error, info, warn};

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
    /// ever de-registered (not yet implemented, but the structure is ready).
    /// Updated on graduation to include newly discovered vault accounts.
    tracked_accounts: Vec<Pubkey>,
}

/// Timing record for a single actively-speculated slot.
///
/// Created the moment the first shredstream batch for a slot is dispatched
/// through `handle_speculative_update`. Consumed and dropped when `handle_frozen_bank`
/// fires for the same slot — at that point the lead time is computed and logged.
///
/// The lead time is the wall-clock gap between when speculative execution first
/// produced account state for this slot and when canonical replay froze the slot.
/// A positive lead time means speculative execution ran ahead of canonical — the
/// normal operating condition. A near-zero or negative lead time means canonical
/// replay is catching up to or overtaking shredstream delivery, indicating the
/// validator is under load or shredstream is delayed.
struct SlotTiming {
    /// Wall-clock time when the first shredstream batch for this slot arrived
    /// at `handle_speculative_update`. This is measured inside the engine's
    /// select loop, not inside `execute()` in the bridge — so it includes the
    /// channel transit time from the bridge task to the engine, which is
    /// typically sub-microsecond for an in-process crossbeam channel.
    first_speculative_at: Instant,

    /// Number of shredstream entry batches processed for this slot. Each batch
    /// is one gRPC message from the shredstream proxy. Multiple batches per slot
    /// are normal — the leader produces entries progressively across the slot's
    /// 400ms window and shredstream delivers each group as it is produced. A
    /// slot with many transactions will have more batches than a sparse slot.
    batch_count: u32,

    /// Number of pool-update events successfully broadcast to executors for
    /// this slot. Zero means no tracked pools were touched by any transaction
    /// in this slot's speculative batches — the slot was irrelevant for arb.
    /// A positive count means at least one executor was woken to simulate.
    events_broadcast: u32,
}

pub struct MevEngine {
    update_rx: Receiver<SpeculativeAccountUpdate>,
    /// Sender half of the speculative account-update channel.  Cloned into
    /// `run_speculative_executor` so the shredstream bridge task can push
    /// `SpeculativeAccountUpdate` values into this engine's `update_rx` without
    /// needing a direct reference to the engine itself.
    speculative_update_tx: crossbeam_channel::Sender<SpeculativeAccountUpdate>,
    /// Receives frozen canonical banks directly from `ReplayStage` the moment
    /// `bank.freeze()` completes.  Bypassing `OptimisticallyConfirmedBankTracker`
    /// eliminates three sources of latency and correctness risk:
    ///
    /// 1. The no-RPC path vulnerability: when `config.rpc_addrs` is `None` the
    ///    tracker is never constructed and `bank_notification_sender` events are
    ///    never forwarded — the engine would run blind.  With a direct channel
    ///    the connection is independent of whether RPC is enabled.
    ///
    /// 2. The dependency-tracker coupling: the tracker thread waits for
    ///    transaction-status processing to complete before forwarding the Frozen
    ///    event.  That wait can take many milliseconds on a busy validator.  A
    ///    direct channel has zero additional latency.
    ///
    /// 3. The BankForks lookup: the tracker strips the `Arc<Bank>` and sends only
    ///    `(slot, parent_slot)`, forcing the engine to re-acquire the BankForks
    ///    read lock to recover the bank.  The direct channel carries the already-
    ///    cloned `Arc<Bank>` so no lock acquisition is needed in the engine.
    bank_rx: Receiver<Arc<Bank>>,
    /// Receives dead-slot numbers from `ReplayStage::mark_dead_slot`.
    ///
    /// A slot is declared dead when the canonical replay pipeline rejects it due
    /// to an invalid PoH hash chain, failed Ed25519 batch verification, SVM
    /// execution error, or chained block-ID mismatch.  Dead slots travel a
    /// completely separate path from frozen banks — they emit `SlotUpdate::Dead`
    /// to RPC WebSocket subscribers but never flow through `bank_notification_sender`.
    /// This dedicated channel is therefore the only reliable way for the engine to
    /// learn that a slot is dead.
    ///
    /// On receipt, `discard_slot` atomically removes the dead slot and all of its
    /// speculative descendants from the internal `slot_banks` map.  Without this
    /// eviction, speculative banks built on invalid entries would persist
    /// indefinitely and the engine would simulate arbitrage against account state
    /// the network has permanently rejected.
    dead_slot_rx: Receiver<Slot>,
    /// Sender half of the engine→bridge dead-slot forwarding channel.
    ///
    /// When the engine's select loop receives a dead slot from `dead_slot_rx`, it
    /// immediately forwards a copy to the bridge task via this sender.  The bridge
    /// calls `detector.clear_dead_slot(slot)` to sweep its per-DEX pending maps,
    /// preventing stale entries from accumulating and crowding out genuine new-pool
    /// detections.
    ///
    /// This explicit forwarding channel replaces the earlier pattern of calling
    /// `self.dead_slot_rx.clone()` before spawning the bridge.  In crossbeam, cloning
    /// a `Receiver` creates a SECOND INDEPENDENT consumer — both the engine's select
    /// loop and the bridge's try_recv() loop drain from the SAME underlying queue.
    /// Dead-slot messages are therefore split non-deterministically: one consumer sees
    /// some subset and the other sees the rest.  The engine misses slots the bridge
    /// consumed; the bridge misses slots the engine consumed.  Using a dedicated channel
    /// means each side always sees every event.
    bridge_dead_slot_tx: crossbeam_channel::Sender<Slot>,
    /// Receiver half of the engine→bridge forwarding channel.  Wrapped in `Option`
    /// so `run_async` can move it out via `take()` into the bridge spawn closure
    /// exactly once without requiring the whole `MevEngine` to be wrapped in an `Arc`.
    bridge_dead_slot_rx: Option<crossbeam_channel::Receiver<Slot>>,
    /// Receives `DetectedPool` values from the shredstream bridge whenever it
    /// observes a pool-creation instruction in the raw entry stream.
    ///
    /// The bridge performs Phase 1 of the two-phase graduation pipeline: scanning
    /// raw instruction bytes for DEX-specific pool-creation discriminators before
    /// any bank execution has occurred.  The engine performs Phase 2: waiting for
    /// the `SpeculativeAccountUpdate` that confirms the creation transaction
    /// succeeded, then integrating the new pool into the arb graph.
    ///
    /// Using a crossbeam channel (not tokio) lets this arm participate in the same
    /// `crossbeam_channel::select!` as the other engine inputs without mixing async
    /// and synchronous polling.
    graduation_rx: Receiver<DetectedPool>,
    /// Sender half of the graduation channel, stored here so it can be moved into
    /// the bridge spawn inside `run_async`.  Wrapped in `Option` so `take()` can
    /// transfer ownership exactly once without cloning.
    graduation_tx: Option<crossbeam_channel::Sender<DetectedPool>>,
    bank_forks: Arc<RwLock<BankForks>>,
    speculative_executor: Arc<SpeculativeSlotExecutor>,
    wallet: Arc<Keypair>,
    lut_manager: Arc<LutManager>,
    rpc_client: Arc<RpcClient>,
    base_priority_fee: u64,
    min_profit_lamports: u64,
    /// When true, the executor runs the full simulation pipeline but does not
    /// submit any transactions. Used to verify that the arb graph, instruction
    /// builder, and simulation path produce valid results before deploying live
    /// capital. Independent of `speculative_accuracy_check`.
    validation_mode: bool,
    shredstream_url: String,
    simulation_semaphore: Arc<Semaphore>,
    /// Most recently frozen canonical bank, shared as a fallback with every
    /// `ArbitrageExecutor`.  Updated on every `bank_rx` message — i.e. every
    /// time a canonical slot is frozen.  The inner `Option` is `None` only
    /// during the startup window before the first block has been replayed.
    canonical_bank: Arc<RwLock<Option<Arc<Bank>>>>,
    mint_states: HashMap<Pubkey, MintState>,
    /// Reverse index: account pubkey → mint.  Routes each incoming pool-account
    /// address from a `SpeculativeAccountUpdate` to the correct per-mint broadcast
    /// channel in O(1) time without iterating over all registered mints.
    account_to_mint: HashMap<Pubkey, Pubkey>,
    /// Executor fan-out tasks queued up during `new()` and `register_mint()`.
    /// They cannot be spawned at construction time because `MevEngine::new` is
    /// called from the synchronous `Validator::new` context where no Tokio runtime
    /// is active — calling `tokio::spawn` outside a runtime panics.  The pairs are
    /// drained inside `run_async` after the MEV runtime has been created.
    pending_executor_starts: Vec<(Arc<ArbitrageExecutor>, broadcast::Receiver<MevPoolUpdateEvent>)>,
    /// Zero-allocation per-batch account list shared with the shredstream bridge.
    ///
    /// Replaced `Arc<RwLock<HashSet<Pubkey>>>`. The bridge calls
    /// `ArcSwap::load()` on every entry batch — one atomic pointer read returning
    /// a `Guard<Arc<Vec<Pubkey>>>`, with no heap allocation and no lock.  The
    /// engine writes a new Vec only at graduation events (rare): it builds the Vec
    /// off the hot path and calls `store(Arc::new(new_vec))`, which atomically
    /// replaces the pointer.  Guards held by in-flight bridge iterations see a
    /// consistent snapshot until they drop, at which point the old Arc is freed.
    ///
    /// Deduplication is preserved: before inserting into the new Vec the engine
    /// checks `account_to_mint` so no pubkey appears twice.  A plain `HashSet`
    /// is used internally in `register_mint` to ensure that, then a sorted unique
    /// Vec is stored.
    cached_accounts_to_watch: Arc<ArcSwap<Vec<Pubkey>>>,
    /// Per-slot timing records keyed by slot number. An entry is created when the
    /// first speculative batch for a slot arrives at `handle_speculative_update`
    /// and is removed when `handle_frozen_bank` fires for that slot. The elapsed
    /// time between creation and removal is the speculative lead time — how far
    /// ahead of canonical replay the engine was operating for that slot.
    ///
    /// Entries for slots that go dead (via `dead_slot_rx`) are also removed so
    /// the map does not accumulate stale entries for abandoned forks. The map
    /// stays small: at steady state it contains only the slots that are
    /// currently between first speculative delivery and canonical freeze,
    /// typically 1–3 slots on mainnet.
    slot_timing: HashMap<Slot, SlotTiming>,
    /// When true, the engine stores the speculative account state it observes for
    /// each slot and compares it byte-for-byte against the canonical account state
    /// once `bank_rx` delivers the frozen bank for that slot.
    ///
    /// This flag is completely independent from `validation_mode`. `validation_mode`
    /// controls whether transactions are submitted. This flag controls whether the
    /// engine audits its own speculative predictions against canonical ground truth.
    /// Both can be active simultaneously, or either can be active alone, or neither.
    ///
    /// The accuracy check answers one question: does speculative execution reliably
    /// produce the same account state that canonical replay produces? A consistently
    /// high match rate means the engine's predictions are trustworthy and full
    /// production deployment is appropriate. A low match rate means something in the
    /// speculative path — entry ordering, parent bank selection, fee collector identity,
    /// or status cache state — is diverging from canonical behavior and needs investigation
    /// before real capital is deployed.
    ///
    /// Zero overhead when false: the snapshot map is never written, read, or allocated.
    speculative_accuracy_check: bool,
    /// Per-slot accumulation of the latest speculative account values observed
    /// across all shredstream batch deliveries for that slot.
    ///
    /// Keyed by slot number → (account pubkey → last speculative value). When
    /// multiple batches arrive for the same slot and touch the same account, the
    /// last batch's value overwrites the earlier one. This gives a running picture
    /// of what speculative execution believes the final state of each account will
    /// be by the time the slot closes.
    ///
    /// The outer map is only populated when `speculative_accuracy_check` is true
    /// AND the batch contains at least one changed account — tick-only batches with
    /// empty account maps do not allocate a snapshot entry.
    ///
    /// When a correction arrives for a child slot via `handle_correction_update`, the
    /// entire snapshot for that child slot is REPLACED (not merged) with the correction's
    /// account map. A correction holds the TOTAL re-executed result against the canonical
    /// parent — any pre-rebase speculative values stored by earlier batches are superseded
    /// and must be discarded to avoid producing false accuracy mismatches at freeze time.
    ///
    /// Entries are removed at canonical freeze (comparison completed) or at dead
    /// slot receipt (slot invalid, no meaningful comparison possible).
    ///
    /// `bank.get_account()` (not `get_account_with_fixed_root()`) is used when
    /// reading from the frozen canonical bank inside `handle_frozen_bank`. The
    /// `_with_fixed_root` variant panics when called from off-chain threads because
    /// the AccountsDb root may not be fixed at the moment this engine thread reads.
    /// `get_account()` uses `LoadHint::Unspecified` which is safe from any thread.
    speculative_snapshot: HashMap<Slot, HashMap<Pubkey, AccountSharedData>>,
    /// Pool addresses detected by the bridge's graduation detector (Phase 1) that
    /// are waiting for their creation transaction to be confirmed by a
    /// `SpeculativeAccountUpdate` (Phase 2).
    ///
    /// When the bridge sees a pool-creation instruction, it sends a `DetectedPool`
    /// to the engine before the execute() call for the same batch.  The engine
    /// stores it here.  When a `SpeculativeAccountUpdate` later arrives with that
    /// pool address in its accounts map, it means execute() applied the creation
    /// transaction to the speculative bank — the pool now exists in the bank's write
    /// cache.  The engine then calls `handle_pool_graduation` to integrate the pool
    /// into the appropriate arb graph.
    ///
    /// Entries for failed transactions are cleared by the dead-slot handler: when
    /// canonical replay rejects a slot, all `DetectedPool` entries whose `slot`
    /// field matches the dead slot are swept out.  This bounds the map to at most
    /// one slot's worth of unconfirmed pool creations and prevents indefinite
    /// accumulation of garbage from transactions that will never land.
    ///
    /// Bounded by `MAX_PENDING_READY` to prevent adversarial pool-creation spam
    /// from growing the map unboundedly.
    pending_ready: HashMap<Pubkey, DetectedPool>,
}

impl MevEngine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        update_rx: Receiver<SpeculativeAccountUpdate>,
        speculative_update_tx: crossbeam_channel::Sender<SpeculativeAccountUpdate>,
        bank_rx: Receiver<Arc<Bank>>,
        dead_slot_rx: Receiver<Slot>,
        bank_forks: Arc<RwLock<BankForks>>,
        speculative_executor: Arc<SpeculativeSlotExecutor>,
        wallet: Arc<Keypair>,
        lut_manager: Arc<LutManager>,
        rpc_client: Arc<RpcClient>,
        base_priority_fee: u64,
        min_profit_lamports: u64,
        validation_mode: bool,
        speculative_accuracy_check: bool,
        shredstream_url: String,
        mint_pool_data: Vec<Arc<MintPoolData>>,
    ) -> Self {
        let canonical_bank: Arc<RwLock<Option<Arc<Bank>>>> = Arc::new(RwLock::new(None));
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
            update_rx,
            speculative_update_tx,
            bank_rx,
            dead_slot_rx,
            bridge_dead_slot_tx,
            bridge_dead_slot_rx: Some(bridge_dead_slot_rx),
            graduation_rx,
            graduation_tx: Some(graduation_tx),
            bank_forks,
            speculative_executor,
            wallet,
            lut_manager,
            rpc_client,
            base_priority_fee,
            min_profit_lamports,
            validation_mode,
            speculative_accuracy_check,
            shredstream_url,
            simulation_semaphore,
            canonical_bank,
            mint_states: HashMap::new(),
            account_to_mint: HashMap::new(),
            pending_executor_starts: Vec::new(),
            // Initialised as an empty Vec. The shredstream bridge does not start
            // until run_async, so nothing reads this field while the registration
            // loop below executes. All tracked accounts are written into this
            // ArcSwap in a single O(n) pass after every mint has been registered
            // via register_mint_startup — see the comment after the loop.
            cached_accounts_to_watch: Arc::new(ArcSwap::from(Arc::new(Vec::new()))),
            slot_timing: HashMap::new(),
            // The snapshot map starts empty. Entries are added only when
            // speculative_accuracy_check is true and a batch carries at least one
            // changed account. Tick-only batches never cause an allocation here.
            speculative_snapshot: HashMap::new(),
            pending_ready: HashMap::new(),
        };

        for pool_data in mint_pool_data {
            engine.register_mint_startup(pool_data);
        }

        // account_to_mint is now fully populated — every tracked account across
        // every registered mint maps to exactly one mint pubkey, making it the
        // correct, deduplicated source of truth for the complete account watch list.
        // A single O(n) pass over its keys builds the Vec the shredstream bridge
        // reads on every entry batch to decide which account writes to forward as
        // SpeculativeAccountUpdates. Building the Vec once here — after all
        // registrations are complete — avoids the O(n²) cost of cloning and
        // rebuilding the growing Vec inside each register_mint_startup call.
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
            let g = arb_graph.read().unwrap();
            g.all_tracked_accounts()
        };

        let (pool_update_tx, pool_update_rx) = broadcast::channel(1024);

        // Wrap pool_data in an ArcSwap so the engine can atomically update it
        // when a graduation event brings a new pool for this mint. Both MintState
        // and ArbitrageExecutor hold Arc<ArcSwap<...>> clones pointing to the same
        // ArcSwap. The engine calls store() on graduation (rare write); the executor
        // calls load() on every simulation (frequent lock-free read).
        let pool_data_swap = Arc::new(ArcSwap::from(pool_data));

        let executor = Arc::new(ArbitrageExecutor::new(
            Arc::clone(&arb_graph),
            Arc::clone(&pool_data_swap),
            Arc::clone(&self.canonical_bank),
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
        // The previously used `new_vec.contains(account)` is O(n) per call. With
        // hundreds of mints each with dozens of accounts, the total cost at startup
        // is O(n_mints × n_accounts²) — quadratic in the account count. Instead,
        // a HashSet is built once from the current Vec for O(1) membership testing,
        // reducing the total insertion cost to O(n_new_accounts) per mint.
        {
            let current = self.cached_accounts_to_watch.load();
            // Clone the current Vec once to produce new_vec.  A single clone is
            // necessary because ArcSwap::load() returns a Guard (a shared borrow)
            // and we need an owned Vec to extend.  Building `existing` from
            // new_vec.iter() (rather than from a second (**current).iter() call)
            // eliminates the second read of the Guard and keeps the allocation
            // count at one per register_mint call.
            let mut new_vec: Vec<Pubkey> = (**current).clone();
            // Build the dedup set from the already-owned new_vec, not by cloning
            // current a second time.  Both reads refer to the same data but the
            // previous version cloned the underlying Vec twice.
            let existing: std::collections::HashSet<Pubkey> =
                new_vec.iter().copied().collect();
            for account in &tracked_accounts {
                // O(1) HashSet lookup — no linear scan of the growing Vec.
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
    ///
    /// To understand why the incremental approach is O(n²): each call to the
    /// equivalent block in `register_mint` loads the current Vec (which grows by ~3
    /// entries per call), clones the entire thing, builds a HashSet from the clone
    /// to detect duplicates, appends the new accounts, and stores a fresh Arc.
    /// By the time the 2.14 millionth mint is registered the Vec holds roughly
    /// 6.4 million entries and each iteration copies ~200 MB of pubkey data. The
    /// cumulative memory traffic across all registrations reaches into the hundreds
    /// of terabytes, turning what should be a linear initialisation into an
    /// 18-hour blocking operation on the validator's startup thread.
    ///
    /// [`register_mint`] is preserved unchanged for runtime graduation calls, where
    /// the bridge is already active and `cached_accounts_to_watch` must reflect
    /// newly discovered vault accounts within the same slot they are detected.
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
            let g = arb_graph.read().unwrap();
            g.all_tracked_accounts()
        };

        let (pool_update_tx, pool_update_rx) = broadcast::channel(1024);

        let pool_data_swap = Arc::new(ArcSwap::from(pool_data));

        let executor = Arc::new(ArbitrageExecutor::new(
            Arc::clone(&arb_graph),
            Arc::clone(&pool_data_swap),
            Arc::clone(&self.canonical_bank),
            Arc::clone(&self.wallet),
            Arc::clone(&self.lut_manager),
            Arc::clone(&self.rpc_client),
            self.base_priority_fee,
            self.min_profit_lamports,
            self.validation_mode,
        ));

        self.pending_executor_starts.push((executor, pool_update_rx));

        // account_to_mint is the reverse index the engine queries on every
        // SpeculativeAccountUpdate to route pool-account writes to the correct
        // executor. Populating it here is the only mutation that matters during
        // startup — it is the source of truth from which cached_accounts_to_watch
        // is built once in MevEngine::new after all mints are registered.
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
        // draining bank_rx and dead_slot_rx but receives no further speculative
        // updates. Without this log the operator has no way to distinguish a silent
        // bridge failure from a period where no tracked pools happened to be touched.
        {
            let speculative_executor = Arc::clone(&self.speculative_executor);
            let bank_forks = Arc::clone(&self.bank_forks);
            let cached_accounts_to_watch = Arc::clone(&self.cached_accounts_to_watch);
            let speculative_update_tx = self.speculative_update_tx.clone();
            let shredstream_url = self.shredstream_url.clone();
            // Take the dedicated bridge dead-slot receiver. This receiver is the
            // engine-to-bridge forwarding channel: every dead slot the engine receives
            // from replay_stage via dead_slot_rx is echoed here, giving the bridge a
            // complete copy of every event without sharing the underlying queue with the
            // engine. Sharing a queue (the old clone() approach) caused both endpoints
            // to consume from the same channel, splitting events non-deterministically.
            let dead_slot_rx_for_bridge = self
                .bridge_dead_slot_rx
                .take()
                .expect("bridge_dead_slot_rx taken twice — run_async called more than once");
            tokio::spawn(async move {
                crate::mev::shredstream_bridge::run_speculative_executor(
                    speculative_executor,
                    bank_forks,
                    cached_accounts_to_watch,
                    speculative_update_tx,
                    graduation_tx,
                    dead_slot_rx_for_bridge,
                    shredstream_url,
                )
                .await;
                error!(
                    "MevEngine: shredstream bridge task exited — speculative execution \
                     has stopped. No further MEV opportunities will be detected until \
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
                recv(self.update_rx) -> msg => {
                    match msg {
                        Ok(update) => {
                            if update.is_correction {
                                // Correction updates arrive from confirm_slot() after the
                                // canonical replay pipeline has verified a parent slot and
                                // re-executed all stored child-slot batches against it.
                                // The accounts map in a correction holds the TOTAL effect
                                // of every batch for that child slot, measured from the
                                // canonical parent — it replaces all prior speculative
                                // state for that slot, it does not accumulate on top of it.
                                self.handle_correction_update(update);
                            } else {
                                // Incremental updates arrive from execute() — one per
                                // shredstream batch delivery. Each carries only the accounts
                                // that changed during that specific delivery. The caller
                                // accumulates these as the slot progresses.
                                self.handle_speculative_update(update);
                            }
                        }
                        Err(_) => {
                            info!("MevEngine: speculative update channel closed — shutting down");
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
                    // speculative banks built on top of that slot's entries become
                    // permanently invalid.  `discard_slot` atomically evicts the dead
                    // slot and every speculative descendant from the internal `slot_banks`
                    // map.  Without eviction those invalid banks would persist and the engine
                    // would simulate arb against account state the network has rejected.
                    if let Ok(dead_slot) = msg {
                        // Remove the timing record. A dead slot will never receive a
                        // canonical freeze signal, so without removal the entry leaks.
                        if let Some(timing) = self.slot_timing.remove(&dead_slot) {
                            warn!(
                                "MevEngine: slot {} declared dead after {} speculative \
                                 batch(es) and {} event(s) broadcast — all speculative \
                                 state condemned",
                                dead_slot,
                                timing.batch_count,
                                timing.events_broadcast,
                            );
                        }

                        // Remove the accuracy snapshot for this slot. The slot was
                        // rejected by canonical replay — there is no frozen canonical
                        // bank for it and therefore no ground truth to compare against.
                        if self.speculative_accuracy_check {
                            self.speculative_snapshot.remove(&dead_slot);
                        }

                        // Sweep pending_ready entries that belong to this dead slot.
                        // Pool creation transactions that were part of a dead slot will
                        // never be confirmed on-chain. Keeping their DetectedPool entries
                        // would cause them to match against SpeculativeAccountUpdates from
                        // future slots that happen to touch the same address — a false
                        // positive that would attempt to integrate a pool that does not
                        // actually exist in canonical state.
                        self.pending_ready.retain(|_, v| v.slot != dead_slot);

                        let condemned = self.speculative_executor.discard_slot(dead_slot);
                        if !condemned.is_empty() {
                            info!(
                                "MevEngine: evicted dead slot {} and {} speculative \
                                 descendant(s)",
                                dead_slot,
                                condemned.len().saturating_sub(1),
                            );
                        }

                        // Forward the dead slot to the bridge task so it can sweep its
                        // per-DEX pending maps (pending_clmm, pending_whirlpool,
                        // pending_dlmm). The bridge calls detector.clear_dead_slot()
                        // to prevent stale entries from accumulating and crowding out
                        // genuine new-pool detections. The send is best-effort: if the
                        // bridge has exited (channel disconnected) the error is silently
                        // dropped — a disconnected bridge has already stopped processing.
                        let _ = self.bridge_dead_slot_tx.send(dead_slot);
                    }
                }

                recv(self.graduation_rx) -> msg => {
                    // Phase 1 of the two-phase graduation pipeline completed in the bridge.
                    // The bridge detected a pool-creation instruction in the raw entry stream
                    // and sent the pool address, mints, and DEX type here. The engine stores
                    // this in pending_ready. When a SpeculativeAccountUpdate arrives that
                    // contains this pool address, it means execute() applied the creation
                    // transaction to the speculative bank — Phase 2 fires to integrate the
                    // pool into the arb graph.
                    if let Ok(detected) = msg {
                        // Skip if this pool is already tracked — a second graduation event
                        // for an already-registered pool address is redundant. The bridge
                        // may send duplicates when a pool's accounts appear in multiple
                        // consecutive entry batches. Using `continue` here stays inside the
                        // select! loop; `return` would exit run_async() entirely and shut
                        // down the whole engine.
                        if self.account_to_mint.contains_key(&detected.pool_address) {
                            continue;
                        }

                        // Enforce the cap to prevent adversarial pool-creation spam from
                        // growing the map without bound. Dropped events are recoverable:
                        // the pool will be discovered by the startup scan on the next restart.
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

    /// Route an incremental speculative account update batch to the correct
    /// per-mint executor.
    ///
    /// Every `MevPoolUpdateEvent` produced here carries `from_speculative_execution:
    /// true` and `is_correction: false`. The downstream `ArbitrageExecutor` treats
    /// these events as incremental deltas — each one reports only what changed
    /// during this specific shredstream batch delivery. The executor accumulates
    /// them as the slot progresses.
    fn handle_speculative_update(&mut self, update: SpeculativeAccountUpdate) {
        let slot = update.slot;

        // First timing borrow window: create-or-update the slot's timing record and
        // increment the batch counter, then immediately release the borrow.
        //
        // This MUST be a separate scope from the account-processing loop below.
        // `entry().or_insert_with()` returns a `&mut SlotTiming` that mutably borrows
        // `self.slot_timing`. Later in this function, `self.handle_pool_graduation()`
        // is called, which takes `&mut self` — requiring exclusive access to all of
        // self including self.slot_timing. If `timing` were still live at that point,
        // the two borrows would overlap, producing a compile error. The explicit scope
        // ensures the borrow ends here, before any `&mut self` method is invoked.
        {
            let timing = self.slot_timing.entry(slot).or_insert_with(|| {
                info!(
                    "MevEngine: first speculative batch for slot {} ({} changed account(s))",
                    slot,
                    update.accounts.len(),
                );
                SlotTiming {
                    first_speculative_at: Instant::now(),
                    batch_count: 0,
                    events_broadcast: 0,
                }
            });
            timing.batch_count += 1;
        } // timing borrow released — self is now fully available for &mut self calls

        // When accuracy checking is enabled, record the latest speculative value
        // for every account this batch touched. The guard on accounts.is_empty()
        // prevents allocating an inner HashMap for tick-only batches that carry
        // no account changes — a tick advances the slot's PoH chain but does not
        // modify any account state, so there is nothing meaningful to snapshot.
        if self.speculative_accuracy_check && !update.accounts.is_empty() {
            let slot_snapshot = self
                .speculative_snapshot
                .entry(slot)
                .or_insert_with(HashMap::new);
            for (pubkey, account_data) in &update.accounts {
                slot_snapshot.insert(*pubkey, account_data.clone());
            }
        }

        let speculative_bank = match self.speculative_executor.get_slot_bank(slot) {
            Some(bank) => bank,
            None => {
                warn!(
                    "MevEngine: no speculative bank for slot {} \
                     (already confirmed or discarded) — dropping {} update(s)",
                    slot,
                    update.accounts.len()
                );
                return;
            }
        };

        let blockhash = speculative_bank.last_blockhash();
        let mut events_sent: u32 = 0;

        for (pool_address, account_data) in &update.accounts {
            match self.account_to_mint.get(pool_address) {
                Some(mint) => {
                    let state = match self.mint_states.get(mint) {
                        Some(s) => s,
                        None => continue,
                    };

                    let event = MevPoolUpdateEvent {
                        pool_address: *pool_address,
                        speculative_bank: Some(Arc::clone(&speculative_bank)),
                        blockhash,
                        from_speculative_execution: true,
                        is_correction: false,
                    };

                    match state.pool_update_tx.send(event) {
                        Ok(_) => {
                            events_sent += 1;
                        }
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
                    // This account is not part of any currently registered mint.
                    // Before checking pending_ready, drain any graduation events that
                    // are already in graduation_rx but have not yet been processed by
                    // the select! loop.
                    //
                    // The bridge sends graduation_tx BEFORE update_tx for the same
                    // batch, so any DetectedPool for this batch is already in the
                    // graduation_rx channel by the time this update arrives. However,
                    // crossbeam_channel::select! is non-deterministic among ready arms
                    // and may have picked update_rx first. Draining here ensures that
                    // graduation events in-flight for this batch are absorbed into
                    // pending_ready before we check it, so no new pool is ever missed
                    // due to select! scheduling order.
                    while let Ok(g) = self.graduation_rx.try_recv() {
                        if !self.account_to_mint.contains_key(&g.pool_address) {
                            if self.pending_ready.len() < MAX_PENDING_READY {
                                self.pending_ready.insert(g.pool_address, g);
                            }
                        }
                    }

                    // If the account is now in pending_ready (either from the drain
                    // above or from a prior graduation_rx select! arm dispatch),
                    // Phase 2 fires: the pool is parsed and integrated into the graph.
                    if let Some(detected) = self.pending_ready.remove(pool_address) {
                        self.handle_pool_graduation(detected, &speculative_bank);
                    }
                    // Accounts that are neither tracked nor newly graduated belong to
                    // programs, sysvars, token accounts, or other on-chain state that
                    // the engine has no interest in. They are silently skipped.
                }
            }
        }

        // Second timing borrow window: record how many pool-update events were
        // broadcast to executors for this batch. A new get_mut is needed because
        // the first timing borrow was explicitly dropped above. The engine is a
        // single-threaded select loop, so no other code path removes this entry
        // between the two windows — if let Some is used defensively rather than
        // unwrap, but the None arm is unreachable in practice.
        if let Some(timing) = self.slot_timing.get_mut(&slot) {
            timing.events_broadcast += events_sent;
        }
    }

    /// Route a canonical rebase correction to the correct per-mint executor.
    ///
    /// Correction updates differ fundamentally from incremental speculative updates:
    /// the accounts map holds the TOTAL accumulated effect of every batch for the
    /// child slot, re-executed against the now-verified canonical parent.  The
    /// downstream `ArbitrageExecutor` must REPLACE its prior cached state for this
    /// slot with exactly what this correction contains, rather than accumulating it
    /// on top of previously delivered incremental deltas.
    ///
    /// Every `MevPoolUpdateEvent` produced here carries `is_correction: true` so
    /// the executor can identify the replacement semantic without inspecting any
    /// other field.
    fn handle_correction_update(&mut self, update: SpeculativeAccountUpdate) {
        let slot = update.slot;

        let speculative_bank = match self.speculative_executor.get_slot_bank(slot) {
            Some(bank) => bank,
            None => {
                // The slot was evicted between the correction being produced and
                // this dispatch. Nothing to correct — the executor has no state for
                // this slot anyway.
                return;
            }
        };

        let blockhash = speculative_bank.last_blockhash();

        // When a correction arrives the accuracy snapshot for this slot must be
        // REPLACED entirely, not merged with prior incremental deltas.
        //
        // Case A — non-empty accounts map: the correction contains the total
        //   re-executed state delta from the canonical parent.  Replace whatever
        //   pre-rebase snapshot was stored with exactly this new map.
        //
        // Case B — empty accounts map: re-execution produced no changes to any
        //   watched account.  The correct ground-truth for this slot is therefore
        //   "no watched-account changes" — the pre-rebase incremental deltas are
        //   wrong and must be removed.  If they are left in place, handle_frozen_bank
        //   will compare them against the canonical bank and record false mismatches
        //   in the accuracy log, making the metric unreliable.
        if self.speculative_accuracy_check {
            if update.accounts.is_empty() {
                // Empty correction: the slot re-executed cleanly but touched no
                // watched accounts.  Remove any stale snapshot so the accuracy
                // comparison at freeze time finds nothing to compare against.
                self.speculative_snapshot.remove(&slot);
            } else {
                let mut new_snapshot = HashMap::with_capacity(update.accounts.len());
                for (pubkey, account_data) in &update.accounts {
                    new_snapshot.insert(*pubkey, account_data.clone());
                }
                self.speculative_snapshot.insert(slot, new_snapshot);
            }
        }

        for (pool_address, _account_data) in &update.accounts {
            let Some(mint) = self.account_to_mint.get(pool_address) else {
                continue;
            };
            let Some(state) = self.mint_states.get(mint) else {
                continue;
            };

            // is_correction: true — the executor must replace all prior speculative
            // state it holds for this slot. from_speculative_execution: false —
            // the re-execution used a canonically verified parent bank, so this
            // result carries only the child slot's own unverified uncertainty.
            let event = MevPoolUpdateEvent {
                pool_address: *pool_address,
                speculative_bank: Some(Arc::clone(&speculative_bank)),
                blockhash,
                from_speculative_execution: false,
                is_correction: true,
            };

            if let Err(e) = state.pool_update_tx.send(event) {
                warn!(
                    "MevEngine: correction pool_update_tx send error for mint {}: {}",
                    mint, e
                );
            }
        }
    }

    /// Handle a frozen canonical bank delivered directly by `ReplayStage`.
    ///
    /// `ReplayStage` calls `bank.freeze()` and then sends the `Arc<Bank>` directly
    /// through this channel before doing anything else.  The bank is already
    /// immutable by the time it arrives here — `freeze()` sets an internal atomic
    /// flag that prevents further writes and commits the bank hash to the
    /// `bank.hash` field.
    ///
    /// The engine performs two operations with the frozen bank:
    ///
    /// 1. Stores it as the canonical fallback in `canonical_bank` so that
    ///    `ArbitrageExecutor` instances have a valid bank to simulate against
    ///    when no speculative bank is available.
    ///
    /// 2. Calls `confirm_slot` on the `SpeculativeSlotExecutor`.  This function
    ///    rebases all speculative child banks that were built on speculative state
    ///    for this slot onto the now-verified canonical parent, producing
    ///    "correction updates" — deltas between what the speculative execution
    ///    predicted and what the canonical bank actually contains.
    fn handle_frozen_bank(&mut self, bank: Arc<Bank>) {
        let slot = bank.slot();

        // Measure the speculative lead time: how far ahead of canonical replay
        // the engine ran for this slot. Removing the entry here (rather than
        // just reading it) is intentional — each slot is measured exactly once,
        // at canonical freeze, and the map entry is not needed after that.
        let (lead_ms, batch_count, events_broadcast) =
            if let Some(timing) = self.slot_timing.remove(&slot) {
                let lead_us = timing.first_speculative_at.elapsed().as_micros();
                (lead_us / 1000, timing.batch_count, timing.events_broadcast)
            } else {
                (0u128, 0u32, 0u32)
            };

        // When accuracy checking is enabled, compare the accumulated speculative
        // predictions for this slot against the canonical ground truth now held in
        // the frozen bank.
        //
        // `bank.get_account()` uses `load_without_fixed_root` internally
        // (LoadHint::Unspecified), which is safe to call from this off-chain MEV
        // engine thread. The alternative `get_account_with_fixed_root()` must NOT
        // be used here — it asserts that the AccountsDb root is fixed at call time,
        // which is only guaranteed when called from BankingStage or ReplayStage
        // threads.
        //
        // A zero-lamport account in the speculative snapshot means a transaction
        // closed that account. AccountsDb does not store zero-lamport accounts after
        // freeze — bank.get_account() returns None for them. Both representations
        // (speculative lamports=0 and canonical None) mean the account does not
        // exist — treating this as a mismatch would produce false positives.
        let (accuracy_matched, accuracy_total) = if self.speculative_accuracy_check {
            if let Some(speculative_accounts) = self.speculative_snapshot.remove(&slot) {
                let mut matched: u32 = 0;
                let mut mismatched: u32 = 0;

                for (pubkey, speculative_value) in &speculative_accounts {
                    match bank.get_account(pubkey) {
                        Some(ref canonical) if canonical == speculative_value => {
                            matched += 1;
                        }
                        Some(canonical) => {
                            mismatched += 1;
                            info!(
                                "MevEngine: ACCURACY slot={} account={} MISMATCH \
                                 speculative_lamports={} canonical_lamports={}",
                                slot,
                                pubkey,
                                speculative_value.lamports(),
                                canonical.lamports(),
                            );
                        }
                        None => {
                            if speculative_value.lamports() == 0 {
                                matched += 1;
                            } else {
                                mismatched += 1;
                                info!(
                                    "MevEngine: ACCURACY slot={} account={} MISMATCH \
                                     speculative_lamports={} canonical=account does not exist",
                                    slot,
                                    pubkey,
                                    speculative_value.lamports(),
                                );
                            }
                        }
                    }
                }

                (matched, matched + mismatched)
            } else {
                (0u32, 0u32)
            }
        } else {
            (0u32, 0u32)
        };

        if lead_ms > 0 {
            if self.speculative_accuracy_check && accuracy_total > 0 {
                info!(
                    "MevEngine: slot {} canonical freeze — lead {}ms ({} batch(es), \
                     {} event(s) broadcast) accuracy={}/{} accounts matched canonical",
                    slot, lead_ms, batch_count, events_broadcast,
                    accuracy_matched, accuracy_total,
                );
            } else {
                info!(
                    "MevEngine: slot {} canonical freeze — lead {}ms ({} batch(es), \
                     {} event(s) broadcast to executors)",
                    slot, lead_ms, batch_count, events_broadcast,
                );
            }
        } else {
            info!(
                "MevEngine: slot {} canonical freeze — no prior speculative execution \
                 (leader slot, shredstream gap, or canonical beat shredstream)",
                slot,
            );
        }

        // Update the canonical fallback shared with all ArbitrageExecutor instances.
        // The write lock is held for exactly one assignment then dropped.
        {
            let mut guard = match self.canonical_bank.write() {
                Ok(g) => g,
                Err(e) => {
                    error!(
                        "MevEngine: canonical_bank RwLock poisoned on slot {}: {}",
                        slot, e
                    );
                    return;
                }
            };
            *guard = Some(Arc::clone(&bank));
        }

        // Load the current account list with a single atomic pointer read.
        // ArcSwap::load() returns a Guard that dereferences to &Vec<Pubkey> with no
        // allocation.  The guard keeps the Vec alive for the duration of confirm_slot.
        let accounts_guard = self.cached_accounts_to_watch.load();
        let accounts_snapshot: &[Pubkey] = &*accounts_guard;

        match self.speculative_executor.confirm_slot(
            slot,
            bank,
            &*self.bank_forks,
            &accounts_snapshot,
        ) {
            Ok(correction_updates) => {
                if !correction_updates.is_empty() {
                    info!(
                        "MevEngine: slot {} confirmed — {} child slot(s) rebased onto \
                         canonical parent (corrections dispatched to executors)",
                        slot,
                        correction_updates.len()
                    );
                }
                for update in correction_updates {
                    self.handle_correction_update(update);
                }
            }
            Err(e) => {
                warn!(
                    "MevEngine: confirm_slot {} failed: {:?} — child slot state indeterminate",
                    slot, e
                );
            }
        }
    }

    /// Phase 2 of the graduation pipeline.
    ///
    /// Called when a `SpeculativeAccountUpdate` arrives for a pool address that
    /// was previously placed in `pending_ready` by the graduation detector's Phase 1
    /// scan.  At this point `execute()` has already applied the creation transaction
    /// to the speculative bank — `bank.get_account(pool_address)` will return the
    /// freshly created pool state account.
    ///
    /// ## Known mint path
    ///
    /// If the non-quote token of the new pool is already tracked by a running
    /// `ArbitrageExecutor`, the pool is fully integrated:
    ///
    /// 1. `initialize_mint_from_discovered` reads the new pool's vault addresses,
    ///    tick arrays, and oracle accounts directly from the speculative bank's write
    ///    cache — every account the creation transaction wrote is already there.
    ///
    /// 2. The current `MintPoolData` is atomically replaced via `ArcSwap::store`.
    ///    The executor's next `load()` call returns the new version. Any simulation
    ///    task holding a Guard from the previous generation completes safely against
    ///    the old data — the previous Arc is freed only after all such Guards drop.
    ///
    /// 3. All new accounts (pool state, vaults, tick arrays, etc.) are registered
    ///    in `account_to_mint`, `cached_accounts_to_watch`, and `arb_graph` so that
    ///    vault reserve changes trigger re-evaluation of arb pairs through this pool.
    ///
    /// ## Unknown mint path
    ///
    /// If the non-quote token has not been seen before, `initialize_mint_from_discovered`
    /// is called synchronously with the speculative bank.  This reads all sub-accounts
    /// (vaults, tick arrays, oracles) directly from the bank's write cache — the
    /// creation transaction's writes are already there.  On success, `register_mint`
    /// is called and the executor is spawned immediately.
    fn handle_pool_graduation(
        &mut self,
        detected: DetectedPool,
        speculative_bank: &Arc<Bank>,
    ) {
        // Confirm the pool account exists in the speculative bank. execute() applied
        // the creation transaction's writes to the bank before producing the
        // SpeculativeAccountUpdate, so if the pool address is absent here, the
        // creation transaction failed within this batch.
        if speculative_bank.get_account(&detected.pool_address).is_none() {
            return;
        }

        // Identify which token is the speculative (non-quote) side of this pool.
        // That token's mint is the key that determines which existing graph receives
        // the new pool, or whether a brand new mint entry must be created.
        let is_quote = |m: &Pubkey| -> bool {
            *m == SOL_MINT || *m == USDC_MINT || *m == USDT_MINT || *m == USD1_MINT
        };

        let mint = if !is_quote(&detected.mint0) {
            detected.mint0
        } else {
            detected.mint1
        };

        // Both-quote pools (SOL/USDC, USDC/USDT, SOL/USD1, etc.) pass the
        // has_quote_token filter in the graduation detector because at least one
        // side is a quote token. But BOTH sides are quote tokens, so the selection
        // above yields a quote token as the "speculative mint". Quote tokens are
        // the denominators of the arb model — they are never the intermediate
        // speculative token. Registering one as a speculative mint would corrupt
        // the graph. Skip this pool entirely.
        if is_quote(&mint) {
            return;
        }

        // Map the graduation source to the pool type used in the arb graph.
        // This pairing is one-to-one: each GraduationSource corresponds to exactly
        // one PoolType, and the PoolType drives which instruction builder path fires
        // when a simulation or trade is attempted through this pool.
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
            // Known mint — fully integrate the new pool by:
            //   a) Discovering vault / sub-account addresses from the speculative bank.
            //   b) Atomically updating pool_data so the executor can build valid
            //      swap instructions through the new pool immediately.
            //   c) Registering all new accounts in routing and the bridge's watch set
            //      so vault mutations trigger arb re-evaluation.

            let discovered = build_single_pool_discovered(&detected);

            // Collect the new pool's account list. A temporary single-pool graph
            // is built from the init result — this reuses the existing account
            // extraction logic that runs at startup and avoids duplicating it here.
            let new_accounts: Vec<Pubkey> = match initialize_mint_from_discovered(
                &mint,
                discovered,
                &self.wallet.pubkey(),
                speculative_bank,
            ) {
                Ok(init) => {
                    let temp_graph = ArbitrageGraph::build_with_config(
                        &init.pool_data,
                        ArbitrageGraphConfig::default(),
                    );
                    let accounts = temp_graph.all_tracked_accounts();

                    // Atomically replace pool_data with an updated version that
                    // includes the new pool's vault addresses, tick arrays, and
                    // oracle accounts. The executor's current Guard holders continue
                    // reading the previous version safely until they drop their Guard.
                    // The borrow on self.mint_states is scoped here so account_to_mint
                    // mutations below can proceed without overlapping borrows.
                    {
                        let state = self.mint_states.get(&mint).unwrap();
                        let current: Arc<MintPoolData> = state.pool_data.load_full();
                        // Deep-clone the existing pool data to build the updated version.
                        // This clone is O(n_pools) and happens once per graduation event.
                        let mut updated: MintPoolData = (*current).clone();
                        updated.merge_pools_from(init.pool_data);
                        state.pool_data.store(Arc::new(updated));
                    }

                    accounts
                }
                Err(e) => {
                    // Vault data extraction failed — pool is grafted into the arb
                    // graph for pair detection but simulation will fail until the
                    // vault addresses are available. This is a degraded but bounded
                    // state: the pool is not lost and will be fully available on the
                    // next validator restart.
                    warn!(
                        "MevEngine: known-mint graduation vault extraction failed for \
                         {:?} pool {} mint {}: {} — pairs wired, simulation degraded",
                        detected.source, detected.pool_address, mint, e
                    );
                    vec![detected.pool_address]
                }
            };

            // Wire all new accounts into the arb graph. Passing the full account
            // list (pool state + vaults + tick arrays + oracle) ensures that any
            // reserve change in any sub-account triggers pair re-evaluation.
            let new_pairs = {
                let state = self.mint_states.get(&mint).unwrap();
                let mut graph = state.arb_graph.write().unwrap();
                graph.add_pool(pool_info, &new_accounts)
            };

            for account in &new_accounts {
                self.account_to_mint.insert(*account, mint);
            }
            // Atomically extend the watched account Vec.  Load the current Vec,
            // append only genuinely new accounts (dedup via account_to_mint which
            // was just updated above), then store the new Vec.  This runs only on
            // graduation events (rare) so the clone+store overhead is negligible.
            //
            // A HashSet is built from the existing Vec to provide O(1) membership
            // testing — Vec::contains is O(n) and becomes O(n²) when called for
            // each account over a large existing list.
            {
                let current = self.cached_accounts_to_watch.load();
                // Single clone — build new_vec first, then build the dedup set
                // from it rather than cloning current a second time.
                let mut new_vec: Vec<Pubkey> = (**current).clone();
                let existing: std::collections::HashSet<Pubkey> =
                    new_vec.iter().copied().collect();
                for account in &new_accounts {
                    if !existing.contains(account) {
                        new_vec.push(*account);
                    }
                }
                self.cached_accounts_to_watch.store(Arc::new(new_vec));
            }

            // Append new accounts to tracked_accounts so a future de-registration
            // sweep can bulk-remove them from account_to_mint without leaking entries.
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
            // Unknown mint — run full discovery synchronously using the speculative bank.
            // The bank's write cache already holds all accounts created in the same
            // transaction as the pool (vaults, LUTs, etc.), so the parser can read
            // them without any additional RPC calls.
            let discovered = build_single_pool_discovered(&detected);

            match initialize_mint_from_discovered(
                &mint,
                discovered,
                &self.wallet.pubkey(),
                speculative_bank,
            ) {
                Ok(init) => {
                    info!(
                        "MevEngine: graduated new mint {} from {:?} pool {}",
                        mint, detected.source, detected.pool_address,
                    );
                    self.register_mint(Arc::new(init.pool_data));

                    // Spawn the executor for the newly discovered mint.
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
// These tests exercise two properties of MevEngine that are critical for
// correctness but invisible to the compiler:
//
//   1. Channel type contracts: the crossbeam channels that carry `Arc<Bank>` and
//      `Slot` between ReplayStage and MevEngine must have exactly the right element
//      type. The channels are created in `validator.rs` with concrete type parameters
//      and passed through `tvu.rs` → `ReplaySenders` → `replay_stage.rs`. A type
//      mismatch anywhere in that chain is a compile error, but these tests make the
//      contract explicit and verify the values survive the round-trip without loss.
//
//   2. canonical_bank update pattern: `handle_frozen_bank` writes the incoming
//      `Arc<Bank>` into `canonical_bank: Arc<RwLock<Option<Arc<Bank>>>>` under a
//      write lock held for one assignment. `ArbitrageExecutor::try_execute_arbitrage`
//      reads it under a read lock. These tests verify that the write pattern leaves
//      the bank readable immediately after the lock is released and that the startup
//      `None` state is correctly detected by the executor.

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

    // -------------------------------------------------------------------------
    // Test 3 — canonical_bank RwLock update pattern
    // -------------------------------------------------------------------------

    /// Verifies the write pattern that `handle_frozen_bank` uses to update the
    /// `canonical_bank: Arc<RwLock<Option<Arc<Bank>>>>` shared with all
    /// `ArbitrageExecutor` instances.
    #[test]
    fn test_canonical_bank_rwlock_write_then_read_pattern() {
        let canonical_bank: Arc<RwLock<Option<Arc<Bank>>>> =
            Arc::new(RwLock::new(None));

        assert!(
            canonical_bank.read().unwrap().is_none(),
            "canonical_bank must start as None before the first frozen bank arrives"
        );

        let GenesisConfigInfo { genesis_config: gc1, .. } = create_genesis_config(500_000);
        let bank1 = Arc::new(Bank::new_for_tests(&gc1));
        bank1.freeze();
        let slot1 = bank1.slot();

        {
            let mut guard = canonical_bank.write().unwrap();
            *guard = Some(Arc::clone(&bank1));
        }

        {
            let read = canonical_bank.read().unwrap();
            let stored = read.as_ref().expect("canonical_bank must be Some after first write");
            assert_eq!(stored.slot(), slot1);
            assert!(stored.is_frozen());
        }

        let GenesisConfigInfo { genesis_config: gc2, .. } = create_genesis_config(500_000);
        let bank2 = Arc::new(Bank::new_for_tests(&gc2));
        bank2.freeze();

        {
            let mut guard = canonical_bank.write().unwrap();
            *guard = Some(Arc::clone(&bank2));
        }

        {
            let read = canonical_bank.read().unwrap();
            let stored = read.as_ref().expect("canonical_bank must be Some after second write");
            assert_eq!(stored.slot(), bank2.slot());
            assert!(stored.is_frozen());
        }
    }
}
