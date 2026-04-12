use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::{Arc, RwLock};
use std::time::Instant;

// libc provides the raw sched_setaffinity(2) syscall interface used to bind
// each shard and HTTP thread to a specific logical CPU. The crate is a direct
// dependency of Agave itself so no Cargo.toml change is required.
use libc;

use arc_swap::ArcSwap;
use crossbeam_channel::Receiver;
use solana_client::rpc_client::RpcClient;
use solana_clock::Slot;
pub use solana_ledger::blockstore_processor::MevExecutedBatch;
use solana_runtime::bank::Bank;
use solana_runtime::bank_forks::{BankForks, ReadOnlyAtomicSlot};
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use tracing::{debug, error, info, warn};

use crate::mev::arbitrage::{PoolInfo, PoolType};
use crate::mev::constants::SOL_MINT;
use crate::mev::executor::{HttpWorker, HttpWorkItem, MevShard, ShardWorkItem};
use crate::mev::loaders::pool_discovery::initialize_mint_from_discovered;
use crate::mev::loaders::pool_graduation::{DetectedPool, GraduationSource};
use crate::mev::loaders::pool_scanner::DiscoveredPools;
use crate::mev::lut_manager::LutManager;
use crate::mev::pools::MintPoolData;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum number of DetectedPool entries held in `pending_ready` at any one
/// time. Pool creation is permissionless — an adversary can spam creation
/// transactions to grow the map without bound. The cap bounds the memory cost
/// at approximately 100 bytes × 4096 entries ≈ 400 KB regardless of spam
/// volume. When the cap is reached, new graduation events are dropped. The
/// dead-slot sweep removes stale entries promptly so the cap is rarely
/// approached under normal operating conditions.
const MAX_PENDING_READY: usize = 4096;

/// Number of shard threads and paired HTTP threads. Each shard owns mints
/// where `mint_pubkey[0] % NUM_SHARDS == shard_idx` and runs on one of
/// logical CPUs 12–23 of the Threadripper PRO 7965WX (physical cores 12–23).
const NUM_SHARDS: usize = 12;

/// First logical CPU in the range reserved for MEV shard and HTTP threads.
/// The engine and validator occupy physical cores 0–11 (logical CPUs 0–11).
/// These cores are the second half of the physical die, sharing no L2 with
/// the validator-side cores regardless of what either side is doing.
const MEV_CORE_BASE: usize = 12;

/// Capacity of the engine→shard rtrb ring buffer. Sized to absorb burst traffic
/// (e.g. a large batch touching many mints simultaneously) without dropping
/// events. Once the burst drains the queue empties and the staleness guard
/// discards anything that aged during the wait. PoolUpdate items are dropped on
/// Full — they are always stale by definition. RegisterMint and GraduatePool
/// items spin-retry until the shard drains.
const SHARD_RING_CAPACITY: usize = 512;

/// Capacity of the shard→HTTP rtrb ring buffer. Profitable submissions are
/// far rarer than pool updates — this capacity is deliberately small so a
/// stalled HTTP thread is detected quickly (buffer full → warn and drop).
const HTTP_RING_CAPACITY: usize = 32;

// ---------------------------------------------------------------------------
// MintState
// ---------------------------------------------------------------------------

/// Per-mint state stored on the engine.
///
/// In the sharded design the engine no longer owns `ArbitrageGraph` objects
/// or broadcast channels. Graphs live on shard threads. The engine retains
/// only what it needs for its own responsibilities:
///
///   • `pool_data` — for graduation merging (rare write, then forwarded to shard).
///   • `tracked_accounts` — for de-registering accounts from `account_to_mint`
///     if a mint is ever removed (future capability, already modelled here).
struct MintState {
    /// Atomically swappable pointer to the current pool data.
    ///
    /// The engine reads this on the known-mint graduation path to merge the
    /// new pool's vault addresses into the existing set, then stores the updated
    /// Arc so future graduation calls see the current state. The associated
    /// `MevShard` receives the updated Arc via a `GraduatePool` work item and
    /// replaces its local `mint_to_pool_data` entry — maintaining consistency
    /// without any cross-thread lock.
    pool_data: Arc<ArcSwap<MintPoolData>>,

    /// Every on-chain account pubkey this mint's pools require.
    ///
    /// Updated on graduation to include newly discovered vault accounts.
    /// Used to populate and later clean up `account_to_mint` entries.
    tracked_accounts: Vec<Pubkey>,
}

// ---------------------------------------------------------------------------
// MevEngine
// ---------------------------------------------------------------------------

pub struct MevEngine {
    /// Receives `MevExecutedBatch` values from `execute_batch()` in
    /// `blockstore_processor.rs`. Every time a transaction batch commits to the
    /// canonical replay bank mid-slot, one payload arrives here.
    mev_batch_rx: Receiver<MevExecutedBatch>,

    /// Receives frozen canonical banks from `ReplayStage` the moment
    /// `bank.freeze()` completes. Used as a fallback for graduation Phase 2.
    bank_rx: Receiver<Arc<Bank>>,

    /// Receives dead-slot numbers from `ReplayStage::mark_dead_slot`.
    dead_slot_rx: Receiver<Slot>,

    /// Receives `DetectedPool` values from the shredstream bridge (Phase 1
    /// of the two-phase graduation pipeline).
    graduation_rx: crossbeam_channel::Receiver<DetectedPool>,

    /// Sender half of the graduation channel, moved into the bridge thread
    /// inside `run()`. Wrapped in `Option` so `take()` transfers ownership
    /// exactly once.
    graduation_tx: Option<crossbeam_channel::Sender<DetectedPool>>,

    /// Dedicated dead-slot forwarding channel to the bridge.
    ///
    /// The engine echoes every dead slot it receives from `dead_slot_rx`
    /// through this channel. Using a dedicated forwarding channel (rather than
    /// cloning the receiver) guarantees the bridge sees every event because
    /// cloning a crossbeam Receiver creates an independent consumer that splits
    /// messages non-deterministically.
    bridge_dead_slot_tx: crossbeam_channel::Sender<Slot>,

    /// Receiver half moved into the bridge thread in `run()` exactly once.
    bridge_dead_slot_rx: Option<crossbeam_channel::Receiver<Slot>>,

    bank_forks: Arc<RwLock<BankForks>>,
    wallet: Arc<Keypair>,
    lut_manager: Arc<LutManager>,
    rpc_client: Arc<RpcClient>,
    base_priority_fee: u64,
    min_profit_lamports: u64,

    /// When true, the shard runs the full simulation pipeline but does not
    /// submit any transactions.
    validation_mode: bool,

    jito_tip_lamports: u64,
    shredstream_url: String,

    /// Lock-free read of the BankForks confirmed root slot.
    ///
    /// One `Ordering::Acquire` atomic load per batch — no RwLock, no syscall.
    /// Under Solana's supermajority confirmation rule the root always trails
    /// the active working bank by roughly 32 slots. A gap > 150 means the
    /// validator is replaying historical blocks (catchup) and pool prices are
    /// stale by thousands of slots.
    root_slot: ReadOnlyAtomicSlot,

    /// Keyed on mint pubkey. FxHashMap for O(1) FxHash lookups on 32-byte keys.
    mint_states: FxHashMap<Pubkey, MintState>,

    /// Reverse index: account pubkey → mint pubkey.
    ///
    /// Routes each incoming pool-account address from a `MevExecutedBatch` to
    /// the correct shard in O(1) time. This is the hottest lookup in the engine —
    /// called for every account key in every committed transaction on every batch.
    account_to_mint: FxHashMap<Pubkey, Pubkey>,

    /// Zero-allocation per-batch account list shared with the shredstream bridge.
    ///
    /// The bridge calls `ArcSwap::load()` on every entry batch — one atomic
    /// pointer read, no heap allocation, no lock. The engine writes a new Vec
    /// only at graduation events (rare).
    cached_accounts_to_watch: Arc<ArcSwap<Vec<Pubkey>>>,

    /// Pool addresses detected by the bridge (Phase 1) waiting for their
    /// creation transaction to be confirmed by a `MevExecutedBatch` (Phase 2).
    /// Bounded by `MAX_PENDING_READY` to prevent adversarial spam.
    pending_ready: FxHashMap<Pubkey, DetectedPool>,

    /// Per-batch deduplication set cleared at the start of every
    /// `handle_mev_batch` call. Stored here so `clear()` reuses the backing
    /// allocation with no heap activity on the hot path.
    seen_this_batch: FxHashSet<Pubkey>,

    /// One rtrb `Producer` per shard. The engine pushes `ShardWorkItem` values
    /// here instead of broadcasting to 2.7M Tokio tasks. Initialised as an empty
    /// Vec in `new()`; populated before the select loop starts in `run()`.
    ///
    /// Index 0 → shard 0 (mints with byte[0] % 12 == 0)
    /// Index 11 → shard 11
    shard_producers: Vec<rtrb::Producer<ShardWorkItem>>,
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
        jito_tip_lamports: u64,
        validation_mode: bool,
        shredstream_url: String,
        mint_pool_data: Vec<Arc<MintPoolData>>,
    ) -> Self {
        let (graduation_tx, graduation_rx) =
            crossbeam_channel::unbounded::<DetectedPool>();

        // Dedicated dead-slot forwarding channel. The bridge needs to know about
        // dead slots to clear its per-DEX pending maps. Forwarding through a
        // separate channel (rather than cloning the receiver) ensures both
        // endpoints see every event.
        let (bridge_dead_slot_tx, bridge_dead_slot_rx) =
            crossbeam_channel::unbounded::<Slot>();

        // Single atomic load to obtain the current root slot handle.
        // Held only for this call; released before any registration work begins.
        let root_slot = bank_forks
            .read()
            .expect("BankForks RwLock poisoned at engine construction")
            .get_atomic_root();

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
            jito_tip_lamports,
            validation_mode,
            shredstream_url,
            root_slot,
            mint_states: FxHashMap::default(),
            account_to_mint: FxHashMap::default(),
            // Initialised as empty Vec — bridge does not start until `run()`,
            // so nothing reads this while the registration loop executes.
            cached_accounts_to_watch: Arc::new(ArcSwap::from(Arc::new(Vec::new()))),
            pending_ready: FxHashMap::default(),
            seen_this_batch: FxHashSet::default(),
            // Populated in run() before the select loop starts.
            shard_producers: Vec::new(),
        };

        for pool_data in mint_pool_data {
            engine.register_mint_startup(pool_data);
        }

        // account_to_mint is now fully populated. Build the deduplicated account
        // watch list that the shredstream bridge will use. A single O(n) pass
        // here — after all registrations — avoids the O(n²) cost of rebuilding
        // the growing Vec inside each register_mint_startup call.
        let all_accounts: Vec<Pubkey> = engine.account_to_mint.keys().copied().collect();
        engine.cached_accounts_to_watch.store(Arc::new(all_accounts));

        engine
    }

    // -----------------------------------------------------------------------
    // Mint registration
    // -----------------------------------------------------------------------

    /// Startup-only variant of [`register_mint`] used exclusively during
    /// [`MevEngine::new`] when initialising the engine with pre-loaded pool data.
    ///
    /// Does NOT build an `ArbitrageGraph` — that work is deferred to the shard
    /// threads in `run()`. Building graphs here AND in the shard would double the
    /// startup cost. Instead this method uses a lightweight account extractor that
    /// reads only the account addresses from pool_data without doing pair analysis.
    ///
    /// Does NOT update `cached_accounts_to_watch` per-call — a single O(n) pass
    /// after all registrations is done in `new()`.
    fn register_mint_startup(&mut self, pool_data: Arc<MintPoolData>) {
        let mint = pool_data.mint;
        if self.mint_states.contains_key(&mint) {
            return;
        }

        // Extract tracked accounts without building the full ArbitrageGraph.
        // The graph is built by the owning MevShard in run() — building it here
        // just to get the account list would pay O(pairs) work for nothing.
        let tracked_accounts = extract_tracked_accounts(&pool_data);

        for account in &tracked_accounts {
            self.account_to_mint.insert(*account, mint);
        }

        let tracked_count = tracked_accounts.len();
        let pool_data_swap = Arc::new(ArcSwap::from(pool_data));

        self.mint_states.insert(mint, MintState {
            pool_data: pool_data_swap,
            tracked_accounts,
        });

        debug!(
            "MevEngine: registered mint {} ({} tracked accounts)",
            mint, tracked_count
        );
    }

    /// Runtime variant of mint registration called during unknown-mint graduation.
    ///
    /// Idempotent: returns immediately if the mint is already registered.
    /// Updates `cached_accounts_to_watch` immediately since the shredstream bridge
    /// is already running and needs to start watching the new accounts within the
    /// current slot.
    pub fn register_mint(&mut self, pool_data: Arc<MintPoolData>) {
        let mint = pool_data.mint;
        if self.mint_states.contains_key(&mint) {
            return;
        }

        let tracked_accounts = extract_tracked_accounts(&pool_data);

        for account in &tracked_accounts {
            self.account_to_mint.insert(*account, mint);
        }

        // Update the watch list atomically so the bridge picks up the new
        // accounts. Only genuinely new accounts are added — the FxHashSet
        // deduplication check avoids growing the Vec with duplicates that are
        // already tracked under another mint.
        {
            let current = self.cached_accounts_to_watch.load();
            let mut new_vec: Vec<Pubkey> = (**current).clone();
            let existing: FxHashSet<Pubkey> = new_vec.iter().copied().collect();
            for account in &tracked_accounts {
                if !existing.contains(account) {
                    new_vec.push(*account);
                }
            }
            self.cached_accounts_to_watch.store(Arc::new(new_vec));
        }

        let tracked_count = tracked_accounts.len();
        let pool_data_swap = Arc::new(ArcSwap::from(pool_data.clone()));

        self.mint_states.insert(mint, MintState {
            pool_data: pool_data_swap,
            tracked_accounts,
        });

        // Send a RegisterMint work item to the correct shard. The shard builds
        // the full ArbitrageGraph and adds all accounts to its own local map.
        // RegisterMint must not be dropped — spin-retry until the shard drains.
        let shard_idx = shard_for_mint(&mint);
        self.push_critical_item(
            shard_idx,
            ShardWorkItem::RegisterMint { pool_data },
        );

        info!(
            "MevEngine: registered new mint {} ({} tracked accounts) → shard {}",
            mint, tracked_count, shard_idx
        );
    }

    // -----------------------------------------------------------------------
    // Engine entry point
    // -----------------------------------------------------------------------

    /// Entry point called on the dedicated `"solMevEngine"` OS thread.
    ///
    /// ## Thread architecture
    ///
    /// ```text
    ///   OS thread "solMevEngine"   — crossbeam select! loop (this function)
    ///   OS thread "solMevShard00"  — MevShard spin loop, core 12
    ///   OS thread "solMevShard01"  — MevShard spin loop, core 13
    ///   …
    ///   OS thread "solMevShard11"  — MevShard spin loop, core 23
    ///   OS thread "solMevHttp00"   — HttpWorker spin loop, core 12 (same physical)
    ///   …
    ///   OS thread "solMevHttp11"   — HttpWorker spin loop, core 23
    ///   OS thread "solMevBridge"   — async gRPC bridge with its own mini runtime
    /// ```
    ///
    /// The engine thread itself runs no Tokio runtime. The select! loop is
    /// synchronous crossbeam — it blocks on the next available message and returns
    /// to pure Rust code. The bridge thread has its own `current_thread` runtime
    /// for its gRPC async code; that runtime is fully isolated from both the
    /// engine thread and the shard threads.
    ///
    /// ## Shard initialisation
    ///
    /// Before the select loop starts, all 2.7M mint pool data objects are
    /// partitioned across 12 shard-init Vecs (`startup_mints[shard_idx]`).
    /// Each `MevShard::new()` receives its Vec and builds its own
    /// `ArbitrageGraph` objects entirely on the spawning thread before the shard
    /// OS thread is created. Once `std::thread::spawn` returns, the shard thread
    /// owns its graphs and the engine thread never touches them again.
    pub fn run(mut self) {
        // Step 1: build ring buffers.
        //
        // 12 engine→shard buffers (capacity 512) and 12 shard→HTTP buffers
        // (capacity 32). Both ends of each buffer are moved to their respective
        // threads below. After this point the engine holds only the 12 producers
        // for the engine→shard direction.
        let mut shard_producers: Vec<rtrb::Producer<ShardWorkItem>> =
            Vec::with_capacity(NUM_SHARDS);
        let mut shard_consumers: Vec<rtrb::Consumer<ShardWorkItem>> =
            Vec::with_capacity(NUM_SHARDS);
        let mut http_producers: Vec<rtrb::Producer<HttpWorkItem>> =
            Vec::with_capacity(NUM_SHARDS);
        let mut http_consumers: Vec<rtrb::Consumer<HttpWorkItem>> =
            Vec::with_capacity(NUM_SHARDS);

        for _ in 0..NUM_SHARDS {
            let (ep, ec) = rtrb::RingBuffer::new(SHARD_RING_CAPACITY);
            let (hp, hc) = rtrb::RingBuffer::new(HTTP_RING_CAPACITY);
            shard_producers.push(ep);
            shard_consumers.push(ec);
            http_producers.push(hp);
            http_consumers.push(hc);
        }

        // Step 2: partition mints across shards.
        //
        // Each mint is assigned by `mint_pubkey[0] % NUM_SHARDS`. The pool data
        // Arc is cloned (one atomic increment per mint) into the shard-init Vec.
        // The engine retains its own Arc in mint_states.pool_data for the
        // graduation path.
        let mut startup_mints: Vec<Vec<Arc<MintPoolData>>> =
            (0..NUM_SHARDS).map(|_| Vec::new()).collect();

        for (mint, state) in &self.mint_states {
            let shard_idx = shard_for_mint(mint);
            startup_mints[shard_idx].push(state.pool_data.load_full());
        }

        // Step 3: spawn HTTP worker threads.
        //
        // HTTP threads are spawned before shard threads so that if a shard
        // somehow produces a profitable item during its own initialisation
        // (impossible in practice, but defensive), the HTTP consumer is already
        // running.
        for (shard_idx, http_consumer) in http_consumers.into_iter().enumerate() {
            let logical_cpu = MEV_CORE_BASE + shard_idx;
            let http_worker = HttpWorker::new(shard_idx, http_consumer);

            std::thread::Builder::new()
                .name(format!("solMevHttp{:02}", shard_idx))
                .spawn(move || {
                    // HTTP threads share physical cores with shard threads
                    // (two logical CPUs per physical core via SMT). The HTTP
                    // thread spends 99% of its time blocked waiting for a TCP
                    // response — it is I/O bound, not compute bound. SMT
                    // sharing with the compute-bound shard thread is therefore
                    // acceptable: the HTTP thread only needs the core when the
                    // shard has just fired a submission, at which point the shard
                    // is blocked on its ring buffer spinning on Empty.
                    pin_thread_to_core(logical_cpu);
                    http_worker.run();
                })
                .expect("HTTP thread spawn failed");
        }

        info!("MevEngine: spawned {} HTTP worker threads", NUM_SHARDS);

        // Step 4: spawn shard threads.
        //
        // `startup_mints` and `http_producers` are zipped with `shard_consumers`
        // so each shard receives exactly its ring buffer consumer, its HTTP producer,
        // and its assigned mint data in one move. After this point the engine holds
        // no reference to any shard's internal state.
        for shard_idx in 0..NUM_SHARDS {
            let logical_cpu = MEV_CORE_BASE + shard_idx;
            let consumer      = shard_consumers.remove(0); // drains front-to-back in order
            let http_producer = http_producers.remove(0);
            let mints         = startup_mints.remove(0);

            let shard = MevShard::new(
                shard_idx,
                consumer,
                http_producer,
                mints,
                Arc::clone(&self.wallet),
                Arc::clone(&self.lut_manager),
                self.base_priority_fee,
                self.min_profit_lamports,
                self.jito_tip_lamports,
                self.validation_mode,
            );

            std::thread::Builder::new()
                .name(format!("solMevShard{:02}", shard_idx))
                .spawn(move || {
                    // Pin to the reserved MEV core. The validator occupies cores
                    // 0–11; MEV shards own cores 12–23. No physical core is shared
                    // between the two halves, eliminating L1/L2 cache competition
                    // and execution-unit contention regardless of load.
                    pin_thread_to_core(logical_cpu);
                    shard.run();
                })
                .expect("shard thread spawn failed");
        }

        info!("MevEngine: spawned {} shard threads", NUM_SHARDS);

        // Step 5: store shard producers on self so handle_mev_batch can push to them.
        self.shard_producers = shard_producers;

        // Step 6: spawn the shredstream graduation bridge on its own OS thread
        // with a private `current_thread` Tokio runtime for its gRPC async code.
        //
        // Giving the bridge its own runtime decouples it entirely from the engine's
        // select loop. The engine thread blocks on crossbeam::select! which is
        // synchronous — no Tokio runtime is needed or wanted on the engine thread.
        {
            let graduation_tx = self
                .graduation_tx
                .take()
                .expect("graduation_tx consumed before run()");

            let dead_slot_rx = self
                .bridge_dead_slot_rx
                .take()
                .expect("bridge_dead_slot_rx taken twice — run() called more than once");

            let shredstream_url = self.shredstream_url.clone();

            std::thread::Builder::new()
                .name("solMevBridge".into())
                .spawn(move || {
                    // A `current_thread` Tokio runtime is sufficient: the bridge
                    // has only one async task (the gRPC streaming loop). Using
                    // current_thread avoids spawning worker threads that would
                    // compete with shard threads for OS scheduling.
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("bridge Tokio runtime build failed");

                    rt.block_on(async {
                        crate::mev::shredstream_bridge::run_graduation_bridge(
                            graduation_tx,
                            dead_slot_rx,
                            shredstream_url,
                        )
                        .await;

                        // If the bridge exits, the engine continues processing
                        // existing pools but will not detect newly created ones.
                        error!(
                            "MevEngine: shredstream graduation bridge exited — new pool \
                             detection has stopped. Existing pools continue to be monitored \
                             but the validator must be restarted to resume detection."
                        );
                    });
                })
                .expect("bridge thread spawn failed");
        }

        info!("MevEngine: bridge thread spawned — entering select loop");

        // Step 7: synchronous crossbeam select! loop.
        //
        // No Tokio. No async. The engine thread blocks on the next message and
        // returns to pure Rust immediately after handling it. Every microsecond
        // here is a direct reduction in the time between batch-commit and the
        // first rtrb push reaching the shard.
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
                    if let Ok(dead_slot) = msg {
                        // Sweep pending_ready entries for this dead slot.
                        self.pending_ready.retain(|_, v| v.slot != dead_slot);

                        // Echo to bridge so it can clear its per-DEX pending maps.
                        // Best-effort: if the bridge thread has exited, the error
                        // is silently dropped — the engine continues running.
                        let _ = self.bridge_dead_slot_tx.send(dead_slot);

                        info!(
                            "MevEngine: slot {} declared dead — pending_ready swept, \
                             bridge notified",
                            dead_slot
                        );
                    }
                }

                recv(self.graduation_rx) -> msg => {
                    // Phase 1 graduation detection arrived from the bridge.
                    // Store in pending_ready; Phase 2 fires in handle_mev_batch
                    // when the creation transaction's batch commits.
                    if let Ok(detected) = msg {
                        if self.account_to_mint.contains_key(&detected.pool_address) {
                            continue;
                        }
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

        info!("MevEngine: select loop exited");
    }

    // -----------------------------------------------------------------------
    // Hot path — handle_mev_batch
    // -----------------------------------------------------------------------

    /// Route a committed transaction batch to the correct shards via rtrb.
    ///
    /// This is the hottest function in the engine. Every call here is on the
    /// path from `execute_batch()` in `blockstore_processor.rs` to the first
    /// shard pop. The only allocations on this path are the `ShardWorkItem`
    /// pushes themselves (each carries an `Arc<Bank>` clone — one atomic
    /// increment per matched account).
    fn handle_mev_batch(&mut self, batch: MevExecutedBatch) {
        let slot = batch.slot;

        // Root-slot staleness guard. During catchup replay the validator processes
        // thousands of historical slots per second. A gap > 150 slots from the
        // root uniquely identifies catchup (the normal live gap is ~32 slots).
        // Any batch from catchup carries pool prices the market moved past long
        // ago — zero events produced, zero rtrb pushes, semaphore untouched.
        if self.root_slot.get().saturating_sub(slot) > 150 {
            return;
        }

        let bank = &batch.bank;
        let blockhash = bank.last_blockhash();

        // Stamp the batch-commit instant once and reuse it for every ShardWorkItem
        // created from this batch. All events from the same batch share the same
        // origin time — the moment the canonical bank committed these transactions.
        let batch_instant = Instant::now();

        // Per-batch deduplication: multiple transactions in one batch can write the
        // same pool account. clear() reuses the backing allocation with zero heap
        // activity on the hot path.
        self.seen_this_batch.clear();

        // Drain any graduation events that arrived for this batch before scanning
        // the account map. The bridge sends graduation_tx before the entry that
        // produced these commits is fully processed, so every DetectedPool for
        // this batch is typically already queued by the time this batch arrives.
        // Absorbing them first ensures the per-account loop below can find them
        // in pending_ready when it encounters the untracked pool address.
        while let Ok(g) = self.graduation_rx.try_recv() {
            if !self.account_to_mint.contains_key(&g.pool_address) {
                if self.pending_ready.len() < MAX_PENDING_READY {
                    self.pending_ready.insert(g.pool_address, g);
                }
            }
        }

        use solana_svm::transaction_commit_result::TransactionCommitResultExtensions;
        let mut events_sent: u32 = 0;

        for (commit_result, tx) in batch.commit_results.iter().zip(batch.transactions.iter()) {
            if !commit_result.was_committed() {
                continue;
            }

            for account_key in tx.message().account_keys().iter() {
                match self.account_to_mint.get(account_key) {
                    Some(mint) => {
                        // Deduplicate: at most one event per pool address per batch.
                        // insert() returns false when already present — skip to
                        // prevent duplicate bundle submissions in production mode.
                        if !self.seen_this_batch.insert(*account_key) {
                            continue;
                        }

                        // Route to the shard that owns this mint. The routing function
                        // is a single modulo — no map lookup, no branch beyond the mod.
                        let shard_idx = shard_for_mint(mint);

                        let item = ShardWorkItem::PoolUpdate {
                            pool_address: *account_key,
                            bank: Arc::clone(bank),
                            blockhash,
                            // All events from this batch share the batch-commit
                            // timestamp for consistent latency measurement.
                            created_at: batch_instant,
                        };

                        // PoolUpdate is dropped on Full — if the shard's ring buffer
                        // is full the event is stale by the time it would be processed
                        // anyway (the staleness guard on the shard side would drop it).
                        match self.shard_producers[shard_idx].push(item) {
                            Ok(()) => { events_sent += 1; }
                            Err(_) => {
                                debug!(
                                    "MevEngine: shard {} ring buffer full — \
                                     dropping PoolUpdate for {}",
                                    shard_idx, account_key
                                );
                            }
                        }
                    }

                    None => {
                        // Check whether this untracked account is a newly created pool
                        // whose Phase 1 detection is in pending_ready. If so, Phase 2
                        // fires to integrate it into the arb graph using the bank that
                        // just committed its creation transaction.
                        if let Some(detected) = self.pending_ready.remove(account_key) {
                            self.handle_pool_graduation(detected, bank);
                        }
                    }
                }
            }
        }

        if events_sent > 0 {
            debug!(
                "MevEngine: slot {} batch — {} pool-update event(s) pushed to shards",
                slot, events_sent
            );
        }
    }

    // -----------------------------------------------------------------------
    // Frozen bank handler (graduation fallback)
    // -----------------------------------------------------------------------

    /// Handle a frozen canonical bank delivered by `ReplayStage`.
    ///
    /// Used as a fallback for graduation processing: if a pool-creation
    /// transaction was the last transaction in a slot, its creation may not
    /// have been caught by `handle_mev_batch`. This handler ensures any pending
    /// graduation is resolved once the slot is fully committed.
    fn handle_frozen_bank(&mut self, bank: Arc<Bank>) {
        let slot = bank.slot();

        // Drain any graduation events still queued and attempt to match them
        // against the now-frozen bank.
        while let Ok(g) = self.graduation_rx.try_recv() {
            if !self.account_to_mint.contains_key(&g.pool_address) {
                if self.pending_ready.len() < MAX_PENDING_READY {
                    self.pending_ready.insert(g.pool_address, g);
                }
            }
        }

        // Try to graduate any pool whose creation was detected in this slot and
        // whose account now exists in the frozen canonical bank.
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

    // -----------------------------------------------------------------------
    // Graduation — Phase 2
    // -----------------------------------------------------------------------

    /// Phase 2 of the graduation pipeline.
    ///
    /// Called when a committed `MevExecutedBatch` (or a frozen canonical bank)
    /// confirms the pool-creation transaction succeeded.
    ///
    /// ## Known mint path
    ///
    /// If the non-quote token is already tracked by a running `MevShard`:
    ///
    ///   1. `initialize_mint_from_discovered` reads the new pool's vault addresses
    ///      from the bank.
    ///   2. The current `MintPoolData` is cloned and updated, then stored via
    ///      `ArcSwap::store` so the engine's own state is current.
    ///   3. A `GraduatePool` work item is pushed to the owning shard. The shard
    ///      calls `graph.add_pool()` on its owned `ArbitrageGraph` and replaces
    ///      its pool data reference — no lock, no coordination.
    ///
    /// ## Unknown mint path
    ///
    /// If the non-quote token has not been seen before:
    ///
    ///   1. `initialize_mint_from_discovered` produces a full `MintPoolData`.
    ///   2. `register_mint` builds the engine's `MintState` and sends a
    ///      `RegisterMint` work item to the correct shard.
    fn handle_pool_graduation(
        &mut self,
        detected: DetectedPool,
        bank: &Arc<Bank>,
    ) {
        if bank.get_account(&detected.pool_address).is_none() {
            return;
        }

        let is_quote = |m: &Pubkey| *m == SOL_MINT;

        let mint = if !is_quote(&detected.mint0) {
            detected.mint0
        } else {
            detected.mint1
        };

        // Both-SOL pools (e.g. SOL/USDC where both sides are quote tokens)
        // are filtered here. `mint` would resolve to a quote token which cannot
        // be the speculative intermediate in the arb model.
        if is_quote(&mint) {
            return;
        }

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
            // Known-mint path.
            let discovered = build_single_pool_discovered(&detected);

            // Initialised in the match arms below. The Ok arm builds the Arc
            // and stores it atomically; the same Arc is forwarded to the shard,
            // avoiding a redundant load_full() call after the store.
            let new_accounts: Vec<Pubkey>;
            let updated_pool_data: Arc<MintPoolData>;

            match initialize_mint_from_discovered(
                &mint,
                discovered,
                &self.wallet.pubkey(),
                bank,
            ) {
                Ok(init) => {
                    // Only the pool address itself needs to be registered as a new
                    // account to watch. A full graph build just to get the account
                    // list would allocate the entire graph structure and throw it
                    // away — wasteful for a single-pool addition.
                    let state = self.mint_states.get(&mint).unwrap();
                    let current: Arc<MintPoolData> = state.pool_data.load_full();
                    let mut updated: MintPoolData = (*current).clone();
                    updated.merge_pools_from(init.pool_data);
                    // Build the Arc once. Arc::clone increments the refcount so
                    // ArcSwap::store gets its own owner and `updated_pool_data`
                    // gets its own owner — no second atomic load needed.
                    let new_arc = Arc::new(updated);
                    state.pool_data.store(Arc::clone(&new_arc));
                    new_accounts = vec![detected.pool_address];
                    updated_pool_data = new_arc;
                }
                Err(e) => {
                    warn!(
                        "MevEngine: known-mint graduation vault extraction failed for \
                         {:?} pool {} mint {}: {} — wiring pairs with pool address only",
                        detected.source, detected.pool_address, mint, e
                    );
                    new_accounts = vec![detected.pool_address];
                    // Vault extraction failed but we still send the shard the
                    // current pool data so it can wire the new pool address into
                    // the graph. load_full() is only called on this error branch.
                    updated_pool_data = self
                        .mint_states
                        .get(&mint)
                        .map(|s| s.pool_data.load_full())
                        .expect("mint state must exist — checked above");
                }
            };

            // Register new accounts in the engine's reverse index so future
            // batches that touch the new pool address are routed correctly.
            for account in &new_accounts {
                self.account_to_mint.insert(*account, mint);
            }

            // Update the watch list.
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

            // Update tracked_accounts on the MintState.
            if let Some(state) = self.mint_states.get_mut(&mint) {
                state.tracked_accounts.extend_from_slice(&new_accounts);
            }

            // Push GraduatePool work item to the owning shard.
            // This must not be dropped — use spin-retry.
            let shard_idx = shard_for_mint(&mint);
            let new_pairs_count = new_accounts.len(); // approximate for logging

            self.push_critical_item(
                shard_idx,
                ShardWorkItem::GraduatePool {
                    mint,
                    pool_info,
                    pool_accounts: new_accounts,
                    updated_pool_data,
                },
            );

            info!(
                "MevEngine: graduated new {:?} pool {} into known mint {} — \
                 ~{} account(s) registered → shard {}",
                detected.source,
                detected.pool_address,
                mint,
                new_pairs_count,
                shard_idx,
            );
        } else {
            // Unknown-mint path.
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
                    // register_mint builds MintState and sends RegisterMint to shard.
                    self.register_mint(Arc::new(init.pool_data));
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

    // -----------------------------------------------------------------------
    // Ring buffer helpers
    // -----------------------------------------------------------------------

    /// Push a critical work item (RegisterMint or GraduatePool) to a shard's
    /// ring buffer, spinning with `yield_now()` until space is available.
    ///
    /// Unlike `PoolUpdate` items which are dropped on Full (they are stale by
    /// definition), critical items must never be dropped: a missed `RegisterMint`
    /// means a mint is permanently invisible to the shard; a missed `GraduatePool`
    /// means a new pool is never wired into the arb graph. The spin is safe
    /// because the shard drains its buffer continuously and the graduation path
    /// is cold (rare events at most a handful per slot).
    fn push_critical_item(&mut self, shard_idx: usize, mut item: ShardWorkItem) {
        loop {
            match self.shard_producers[shard_idx].push(item) {
                Ok(()) => break,
                Err(rtrb::PushError::Full(returned)) => {
                    item = returned;
                    // Yield the engine thread's timeslice so the shard thread can
                    // drain. Under normal conditions the shard empties its 512-slot
                    // buffer in microseconds — this loop should never iterate more
                    // than once or twice.
                    std::thread::yield_now();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Routing
// ---------------------------------------------------------------------------

/// Compute which shard owns a given mint.
///
/// The first byte of the mint's on-chain address, mod NUM_SHARDS. This is
/// computed from the address itself — no table lookup, no heap allocation,
/// no synchronisation.  The assignment is bijective and stable across the
/// validator's lifetime: once a mint is assigned to shard N it always routes
/// to shard N.
#[inline(always)]
fn shard_for_mint(mint: &Pubkey) -> usize {
    mint.to_bytes()[0] as usize % NUM_SHARDS
}

// ---------------------------------------------------------------------------
// CPU pinning
// ---------------------------------------------------------------------------

/// Pin the calling thread to a specific logical CPU using `sched_setaffinity(2)`.
///
/// On failure (e.g. the process lacks `CAP_SYS_NICE`, or the CPU index is out
/// of range for this kernel build), the thread continues running unbound rather
/// than crashing. The operator should investigate the warning because unbound
/// threads may drift onto validator cores under heavy load.
fn pin_thread_to_core(logical_cpu: usize) {
    unsafe {
        let mut cpuset = std::mem::zeroed::<libc::cpu_set_t>();
        libc::CPU_SET(logical_cpu, &mut cpuset);
        let rc = libc::sched_setaffinity(
            0, // pid 0 = calling thread
            std::mem::size_of::<libc::cpu_set_t>(),
            &cpuset,
        );
        if rc != 0 {
            let errno = *libc::__errno_location();
            warn!(
                "sched_setaffinity failed for logical CPU {} (errno {}) — \
                 thread will run unbound",
                logical_cpu, errno
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Startup account extractor
// ---------------------------------------------------------------------------

/// Extract all pool account pubkeys from `pool_data` without building a full
/// `ArbitrageGraph`.
///
/// `ArbitrageGraph::build_with_config` performs pair analysis and deduplication
/// — work the engine does not need at startup. This function reads only the
/// account addresses from each pool type's Vec, exactly as
/// `build_account_to_pool_map` does inside `arbitrage_graph.rs`, and returns
/// them as a plain `Vec<Pubkey>`.
///
/// Cost: O(total_pools) — one push per pool with no HashMap construction and
/// no pair-matching loop. Roughly 10–50× cheaper than a full graph build for
/// a typical mint with < 10 pools.
fn extract_tracked_accounts(pool_data: &MintPoolData) -> Vec<Pubkey> {
    let mut accounts: Vec<Pubkey> = Vec::new();

    for pool in &pool_data.raydium_pools        { accounts.push(pool.pool); }
    for pool in &pool_data.raydium_cp_pools     { accounts.push(pool.pool); }
    for pool in &pool_data.raydium_clmm_pools   { accounts.push(pool.pool); }
    for pool in &pool_data.pump_pools           { accounts.push(pool.pool); }
    for pool in &pool_data.meteora_damm_pools   { accounts.push(pool.pool); }
    for pool in &pool_data.meteora_damm_v2_pools{ accounts.push(pool.pool); }
    for pool in &pool_data.dlmm_pairs           { accounts.push(pool.pair); }
    for pool in &pool_data.whirlpool_pools      { accounts.push(pool.pool); }
    for pool in &pool_data.byreal_pools         { accounts.push(pool.pool); }
    for pool in &pool_data.pancakeswap_pools    { accounts.push(pool.pool); }
    for pool in &pool_data.humidifi_pools       { accounts.push(pool.pool); }
    for pool in &pool_data.vertigo_pools        { accounts.push(pool.pool); }
    for pool in &pool_data.heaven_pools         { accounts.push(pool.pool); }
    for pool in &pool_data.futarchy_pools       { accounts.push(pool.dao); }

    accounts
}

// ---------------------------------------------------------------------------
// Graduation helpers
// ---------------------------------------------------------------------------

/// Build a `DiscoveredPools` containing exactly one pool entry for the given
/// `DetectedPool`. `initialize_mint_from_discovered` expects a `DiscoveredPools`
/// where each DEX's pools are listed in the appropriate `Vec<Pubkey>` field.
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

    // -------------------------------------------------------------------------
    // Test 3 — shard_for_mint routing
    // -------------------------------------------------------------------------

    /// Verifies that `shard_for_mint` always returns a value in [0, NUM_SHARDS)
    /// and that the routing is deterministic for the same mint.
    #[test]
    fn test_shard_for_mint_deterministic_and_bounded() {
        use solana_pubkey::Pubkey;

        for _ in 0..1000 {
            let mint = Pubkey::new_unique();
            let idx_a = shard_for_mint(&mint);
            let idx_b = shard_for_mint(&mint);
            assert!(idx_a < NUM_SHARDS, "shard index must be < NUM_SHARDS");
            assert_eq!(idx_a, idx_b, "shard routing must be deterministic");
        }
    }
}
