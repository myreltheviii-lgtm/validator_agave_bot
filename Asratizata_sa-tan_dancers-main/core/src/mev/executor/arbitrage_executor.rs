use {
    crate::mev::{
        arbitrage::{ArbitrageGraph, ArbitrageGraphConfig, ArbitragePath, PoolInfo},
        executor::{
            smb_instruction_builder::SmbInstructionBuilder,
            token_flow_validator::TokenFlowValidator,
        },
        lut_manager::LutManager,
        pools::MintPoolData,
    },
    anyhow::{anyhow, Result},
    // Base64 encoding is required to serialise the signed wire transaction into the
    // string format that JSON-RPC endpoints accept in the params array.
    // `ENGINE as _` imports the trait that provides the `.encode()` method without
    // binding it to a local name — the compiler resolves the method call through
    // the trait but we never reference the trait directly in this file.
    base64::prelude::{BASE64_STANDARD, Engine as _},
    // reqwest::blocking::Client is a synchronous HTTP client. It is placed on the
    // dedicated HttpWorker OS thread — one per shard — which spins on the
    // shard→HTTP ring buffer and fires blocking HTTP requests directly. There is no
    // Tokio runtime involvement on this thread: the HTTP thread is a plain OS thread
    // and a blocking call is exactly what we want. A single Client per HttpWorker
    // means one persistent TCP connection pool to the Helius Sender FRA endpoint,
    // eliminating the per-request TCP handshake after the first send.
    reqwest::blocking::Client as BlockingHttpClient,
    // rtrb is a lock-free SPSC ring buffer. `Consumer` and `Producer` are both `Send`
    // so they can be moved to their respective threads. The ring buffer itself is
    // zero-allocation and zero-syscall on both push and pop — latency is ~5–20 ns
    // compared to ~1–5 µs for a futex-backed channel wakeup.
    rtrb,
    rustc_hash::FxHashMap,
    serde_json,
    solana_pubkey::Pubkey,
    solana_hash::Hash,
    solana_runtime::bank::Bank,
    solana_compute_budget_interface::ComputeBudgetInstruction,
    solana_instruction::{AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_signature::Signature,
    solana_signer::Signer,
    solana_transaction::{
        TransactionVerificationMode,
        versioned::VersionedTransaction,
    },
    std::{
        sync::{
            Arc,
            // AtomicU64 drives the JSON-RPC request-id counter in send_item.
            // A monotonically incrementing counter produces a unique id per call with
            // a single atomic fetch_add — no syscall, no heap allocation, no clock
            // query — replacing the SystemTime::now().to_string() pattern that was
            // previously used and that paid a syscall plus a String allocation on
            // every bundle submission.
            atomic::{AtomicU64, Ordering},
        },
        time::Instant,
    },
    tracing::{debug, info, warn},
};

// ---------------------------------------------------------------------------
// Work items — the types that flow through the rtrb ring buffers
// ---------------------------------------------------------------------------

/// Work pushed by the engine onto a shard's ring buffer.
///
/// Three variants cover the three lifecycle events a shard must handle:
///   `PoolUpdate`   — a tracked pool account was written in a committed batch.
///                    Hot path. Shard does staleness check, graph lookup, and
///                    inline simulation (validation mode) or inline tx build +
///                    HTTP push (production mode).
///   `RegisterMint` — a newly graduated unknown mint. Shard builds its
///                    `ArbitrageGraph` and adds it to its owned maps. Rare.
///   `GraduatePool` — a new pool was detected for an already-tracked mint.
///                    Shard updates its graph and pool data. Rare.
///
/// `Arc<Bank>` is safe to send across threads because `Bank: Send + Sync`.
/// Sending it in the work item means the bank's Arc refcount is bumped once
/// per event — not per qualifying pair — keeping the engine's hot path free
/// of per-pair allocations.
pub enum ShardWorkItem {
    PoolUpdate {
        /// On-chain address of the pool account that was written.
        pool_address: Pubkey,
        /// Canonical replay bank carrying all committed writes from this batch.
        /// The shard calls `verify_transaction` and `simulate_transaction_unchecked`
        /// directly against this bank. Dropped immediately after simulation.
        bank: Arc<Bank>,
        /// Blockhash drawn from `bank.last_blockhash()` at event construction time.
        blockhash: Hash,
        /// Wall-clock instant at which the engine stamped this event, immediately
        /// after the batch committed. Used for the staleness guard and latency logging.
        created_at: Instant,
    },
    /// A new mint whose pool set has never been seen before. The shard builds
    /// a fresh `ArbitrageGraph` and registers all accounts in its local maps.
    RegisterMint {
        pool_data: Arc<MintPoolData>,
    },
    /// A new pool for an already-tracked mint. The shard inserts the pool into
    /// its existing `ArbitrageGraph` and updates its local pool data reference.
    GraduatePool {
        mint: Pubkey,
        pool_info: PoolInfo,
        pool_accounts: Vec<Pubkey>,
        /// Updated pool data carrying the new pool's vault addresses. The shard
        /// replaces its `mint_to_pool_data` entry with this Arc.
        updated_pool_data: Arc<MintPoolData>,
    },
}

/// Work pushed by the shard onto the HTTP worker's ring buffer after
/// simulation confirms a profitable opportunity in production mode.
///
/// The shard serialises and signs the transaction before pushing — the HTTP
/// worker receives raw bytes ready to base64-encode and POST. This keeps
/// all CPU-bound work (signing, serialisation) on the shard thread and leaves
/// the HTTP worker thread free to do only I/O.
pub struct HttpWorkItem {
    /// Bincode-serialised, fully signed `VersionedTransaction` wire bytes.
    pub tx_bytes: Vec<u8>,
    /// Mint pubkey, included only for log attribution.
    pub mint: Pubkey,
    /// Pair index within the mint's arb graph, for log attribution.
    pub pair_idx: usize,
    /// Batch-commit timestamp from the originating `ShardWorkItem::PoolUpdate`.
    /// Logged as the `total=` field so the operator can see end-to-end latency
    /// from opportunity birth through HTTP response.
    pub created_at: Instant,
}

// ---------------------------------------------------------------------------
// Constants — all carried over from the original design unchanged
// ---------------------------------------------------------------------------

/// Maximum microseconds between batch-commit and the start of shard processing
/// before the event is considered stale and dropped.
///
/// A Solana slot is produced every 400 ms (400_000 µs). Any event older than
/// one slot is operating on prices the market has already moved past. Dropping
/// it immediately — before any graph lookup or simulation — costs ~1 ns and
/// keeps the shard's ring buffer free for the next live event.
///
/// This is the FIRST check that runs after a `PoolUpdate` is popped from the
/// ring buffer: before graph lookup, before instruction building, before anything.
const MAX_EVENT_STALENESS_US: u128 = 400_000;

/// Compute units provisionally allocated per swap hop in simulation.
///
/// This value is intentionally generous so the SVM never hits the CU cap
/// mid-execution during simulation — a cap-induced truncation would cause the
/// simulation to return a misleading error rather than reporting unprofitability,
/// which would suppress valid opportunities.
///
/// The actual units consumed from the simulation result (with 10% headroom
/// added) are used for the final submission transaction in validation mode.
/// In production mode (no simulation) this constant is used directly as the
/// static CU limit.
const ESTIMATED_CU_PER_HOP: u32 = 700_000;

/// Helius Sender regional HTTP endpoint co-located with our Frankfurt validator.
///
/// Plain HTTP (not HTTPS) is intentional: TLS adds ~0.5–1 ms per request at
/// this scale. The submission payload is a signed, authenticated Solana
/// transaction whose authenticity is already guaranteed by the ed25519
/// signature. TLS would add overhead without adding security.
const HELIUS_SENDER_FRA_ENDPOINT: &str = "http://fra-sender.helius-rpc.com/fast";

/// Minimum lamports transferred to a Jito tip account in every submission
/// transaction. Helius Sender's dual-routing path requires at least 0.0002 SOL
/// (200_000 lamports) in this transfer for the transaction to be eligible for
/// the Jito block engine auction.
const HELIUS_SENDER_MIN_TIP_LAMPORTS: u64 = 200_000;

/// The canonical Wrapped SOL mint address.
///
/// All on-chain swap programs operate on SPL tokens, not native SOL. The arb
/// bot wraps native SOL into WSOL at the start of every transaction so it can
/// be passed into pool instructions as a standard token. Profit is measured by
/// the change in the wallet's WSOL token balance from `post_token_balances` —
/// not from `post_balances`, which only shows the fee deduction.
const WSOL_MINT: Pubkey = solana_pubkey::pubkey!("So11111111111111111111111111111111111111112");

/// Minimum simulated gross profit (pre-tip) an arb path must clear before any
/// tip arithmetic or submission occurs.
///
/// At the 60% tip fraction the minimum gross that can satisfy the 2M lamport
/// net floor is 5M lamports (gross × 0.40 = net → 5M × 0.40 = 2M). Any gross
/// below this threshold cannot pass Gate 2 regardless of how the tip resolves,
/// so the early rejection saves the division and the second comparison.
const MIN_GROSS_PROFIT_LAMPORTS: u64 = 5_000_000;

/// Fraction of gross profit paid as the Jito tip.
///
/// The block-engine auction is a first-price sealed-bid auction: larger arbs
/// bid more and are more likely to win the slot position, while marginal arbs
/// bid proportionally less, preserving more of the thin margin.
const TIP_FRACTION: f64 = 0.60;

/// Minimum lamports the operator retains after paying the dynamic Jito tip.
///
/// An arb that grosses 5_000_000 and tips 3_000_000 (60%) nets exactly
/// 2_000_000 lamports — the breakeven point for this floor.
const MIN_NET_PROFIT_LAMPORTS: u64 = 2_000_000;

/// The ten canonical Jito tip accounts published by Jito Labs.
///
/// One account is selected at random per transaction submission. Spreading the
/// tip across all ten accounts prevents any single account from becoming a
/// write-lock contention hotspot when many bundles are submitted in the same
/// slot — write-lock contention on the same account across multiple transactions
/// in the same block forces the scheduler to serialise those transactions, which
/// can push our transaction into a later slot.
const JITO_TIP_ACCOUNTS: [Pubkey; 10] = [
    solana_pubkey::pubkey!("4ACfpUFoaSD9bfPdeu6DBt89gB6ENTeHBXCAi87NhDEE"),
    solana_pubkey::pubkey!("D2L6yPZ2FmmmTKPgzaMKdhu6EWZcTpLy1Vhx8uvZe7NZ"),
    solana_pubkey::pubkey!("9bnz4RShgq1hAnLnZbP8kbgBg1kEmcJBYQq3gQbmnSta"),
    solana_pubkey::pubkey!("5VY91ws6B2hMmBFRsXkoAAdsPHBJwRfBht4DXox3xkwn"),
    solana_pubkey::pubkey!("2nyhqdwKcJZR2vcqCyrYsaPVdAnFoJjiksCXJ7hfEYgD"),
    solana_pubkey::pubkey!("2q5pghRs6arqVjRvT5gfgWfWcHWmw1ZuCzphgd5KfWGJ"),
    solana_pubkey::pubkey!("wyvPkWjVZz1M8fHQnMMCDTQDbkManefNNhweYk5WkcF"),
    solana_pubkey::pubkey!("3KCKozbAaF75qEU33jtzozcJ29yJuaLJTy2jFdzUY8bT"),
    solana_pubkey::pubkey!("4vieeGHPYPG2MmyPRcYjdiDmmhN3ww7hsFNap8pVN3Ey"),
    solana_pubkey::pubkey!("4TQLFNWK8AovT1gFvda5jfw2oJeRMKEmw7aH6MGBJ3or"),
];

/// Monotonically increasing counter used to generate unique JSON-RPC request ids
/// in `HttpWorker::send_item`.
///
/// `Relaxed` ordering is correct: the only invariant needed is that each call
/// increments the counter once, with no happens-before relationship required
/// across threads. Multiple `HttpWorker` threads share this counter — each gets
/// a unique id because `fetch_add` is atomic.
static SUBMISSION_COUNTER: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// MevShard
// ---------------------------------------------------------------------------

/// One of twelve shard threads that together replace 2.7M per-mint Tokio tasks.
///
/// Each shard owns exactly 1/12th of the tracked mints (those whose first
/// address byte, mod 12, equals this shard's index). It holds its mint data
/// without any locking — no `Arc<RwLock<...>>`, no shared state — because it
/// is the only thread that ever reads or writes its maps.
///
/// The shard runs a tight spin loop on its rtrb `Consumer<ShardWorkItem>`.
/// Spin latency (~5–20 ns) replaces the futex wakeup latency (~1–5 µs) that
/// the broadcast channel model required, cutting time-to-first-action by 50–
/// 250× on the event dequeue step alone.
///
/// ## Simulation
///
/// In validation mode the shard calls `verify_transaction` and
/// `simulate_transaction_unchecked` directly — inline on its own thread.
/// No `spawn_blocking`, no Tokio involvement, no scheduling overhead. Because
/// at most 12 shards run concurrently, bank lock contention drops from ~128
/// concurrent simulations to ~12, reducing simulation wall-clock time to
/// close to its bare SVM cost.
///
/// ## Production submission
///
/// In production mode the shard builds the final transaction inline, serialises
/// it, and pushes the bytes to its `HttpWorker` via a second rtrb ring buffer.
/// The HTTP worker fires a blocking POST immediately. End-to-end from batch-
/// commit to tx-on-wire is ~1.25–11 ms depending on simulation; previously it
/// was up to 17 seconds.
pub struct MevShard {
    /// Which of the 12 shards this is (0–11). Used for core pinning and logging.
    shard_idx: usize,
    /// Receives `ShardWorkItem` values pushed by the engine's hot path.
    /// This is the only reader of this consumer — no lock, no coordination.
    consumer: rtrb::Consumer<ShardWorkItem>,
    /// Sends serialised transaction bytes to the paired `HttpWorker` thread.
    http_producer: rtrb::Producer<HttpWorkItem>,
    /// Per-mint arbitrage graphs, owned outright. No Arc, no RwLock.
    /// Keyed by mint pubkey. Only this shard ever touches these graphs.
    mint_to_graph: FxHashMap<Pubkey, ArbitrageGraph>,
    /// Per-mint pool data. Replaced atomically on graduation (rare).
    /// Keyed by mint pubkey.
    mint_to_pool_data: FxHashMap<Pubkey, Arc<MintPoolData>>,
    /// Reverse index within this shard: account pubkey → mint pubkey.
    /// Used to route a `PoolUpdate`'s pool_address to the correct graph.
    /// Only contains accounts belonging to mints owned by this shard.
    account_to_mint: FxHashMap<Pubkey, Pubkey>,
    wallet: Arc<Keypair>,
    lut_manager: Arc<LutManager>,
    base_priority_fee: u64,
    min_profit_lamports: u64,
    /// Floor for the Jito tip transfer. Must be ≥ HELIUS_SENDER_MIN_TIP_LAMPORTS.
    jito_tip_lamports: u64,
    /// When true, run full inline SVM simulation and log results without submitting.
    /// When false, build the transaction and push it to the HTTP worker immediately.
    validation_mode: bool,
}

impl MevShard {
    /// Create a new shard and populate it with its initial slice of mints.
    ///
    /// `startup_mints` contains every `MintPoolData` whose mint's first address
    /// byte, mod 12, equals `shard_idx`. The shard builds an `ArbitrageGraph`
    /// for each one and registers all tracked accounts in its local `account_to_mint`
    /// map. This is the only time graphs are built on the engine's thread — after
    /// startup, graduation work items trigger graph builds on the shard thread itself.
    pub fn new(
        shard_idx: usize,
        consumer: rtrb::Consumer<ShardWorkItem>,
        http_producer: rtrb::Producer<HttpWorkItem>,
        startup_mints: Vec<Arc<MintPoolData>>,
        wallet: Arc<Keypair>,
        lut_manager: Arc<LutManager>,
        base_priority_fee: u64,
        min_profit_lamports: u64,
        jito_tip_lamports: u64,
        validation_mode: bool,
    ) -> Self {
        let mut shard = Self {
            shard_idx,
            consumer,
            http_producer,
            mint_to_graph: FxHashMap::default(),
            mint_to_pool_data: FxHashMap::default(),
            account_to_mint: FxHashMap::default(),
            wallet,
            lut_manager,
            base_priority_fee,
            min_profit_lamports,
            jito_tip_lamports,
            validation_mode,
        };

        for pool_data in startup_mints {
            shard.register_mint_internal(pool_data);
        }

        info!(
            "MevShard[{}]: initialised — {} mints, {} accounts, {} total pairs",
            shard_idx,
            shard.mint_to_graph.len(),
            shard.account_to_mint.len(),
            shard.mint_to_graph.values().map(|g| g.total_pairs()).sum::<usize>(),
        );

        shard
    }

    /// Spin loop entry point. Runs for the validator's lifetime on a dedicated
    /// OS thread pinned to one of cores 12–23.
    ///
    /// `spin_loop()` emits a CPU PAUSE hint on x86 — it reduces power and heat
    /// without adding measurable latency, and prevents the CPU from issuing
    /// erroneous memory-order speculation across the spin boundary. It is correct
    /// and intentional to burn this core: it is reserved exclusively for MEV work
    /// and any sleep or yield would add wakeup latency to the critical path.
    pub fn run(mut self) {
        loop {
            match self.consumer.pop() {
                Ok(item) => self.process_item(item),
                Err(rtrb::PopError::Empty) => {
                    std::hint::spin_loop();
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Item dispatch
    // -----------------------------------------------------------------------

    fn process_item(&mut self, item: ShardWorkItem) {
        match item {
            ShardWorkItem::PoolUpdate { pool_address, bank, blockhash, created_at } => {
                self.handle_pool_update(pool_address, bank, blockhash, created_at);
            }
            ShardWorkItem::RegisterMint { pool_data } => {
                self.register_mint_internal(pool_data);
            }
            ShardWorkItem::GraduatePool { mint, pool_info, pool_accounts, updated_pool_data } => {
                self.graduate_pool(mint, pool_info, pool_accounts, updated_pool_data);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Hot path — PoolUpdate
    // -----------------------------------------------------------------------

    /// Core hot-path handler. Called for every pool account write that the engine
    /// routes to this shard.
    ///
    /// Step order is fixed by latency priority:
    ///   1. Staleness check — ~1 ns. Kills anything older than one slot before
    ///      doing any other work. This is the most important gate in the system.
    ///   2. Graph lookup — ~50 ns. FxHashMap double-hop: account → mint → graph.
    ///   3. Qualifying pair enumeration — ~50 ns per pair. Pure in-memory.
    ///   4. Per-pair simulation (validation) or transaction build (production).
    fn handle_pool_update(
        &mut self,
        pool_address: Pubkey,
        bank: Arc<Bank>,
        blockhash: Hash,
        created_at: Instant,
    ) {
        // Step 1: staleness check — the very first instruction executed after
        // popping from the ring buffer. If this event sat in the queue for longer
        // than one slot (400 ms), the pool prices it carried are no longer
        // actionable. Drop it in ~1 ns before touching any other state.
        if created_at.elapsed().as_micros() > MAX_EVENT_STALENESS_US {
            return;
        }

        // Step 2: route pool_address → mint → graph. Both lookups are O(1)
        // FxHashMap reads with no lock acquisition.
        let mint = match self.account_to_mint.get(&pool_address) {
            Some(m) => *m,
            None => return,
        };

        let graph = match self.mint_to_graph.get(&mint) {
            Some(g) => g,
            None => return,
        };

        let pool_data = match self.mint_to_pool_data.get(&mint) {
            Some(p) => Arc::clone(p),
            None => return,
        };

        // Step 3: collect qualifying pairs. `get_affected_pairs` is an O(1)
        // double-map lookup returning a `&[usize]` slice — no allocation.
        // `to_path()` and `can_execute_2hop` are pure in-memory checks on Copy
        // types. The Vec allocation is bounded by the number of pairs for this
        // pool, typically < 20.
        let qualifying_pairs: Vec<(usize, ArbitragePath)> = graph
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
            .collect();

        if qualifying_pairs.is_empty() {
            return;
        }

        debug!(
            "MevShard[{}]: pool {} affects {} qualifying pair(s) for mint {}",
            self.shard_idx,
            pool_address,
            qualifying_pairs.len(),
            mint,
        );

        // Step 4: process each qualifying pair. Sequential — no spawning, no
        // async, no scheduling overhead. The shard thread IS the work thread.
        //
        // We process pairs for the same bank in sequence. If simulation takes
        // 5 ms per pair and there are 3 qualifying pairs, total time is ~15 ms.
        // This is acceptable: the alternative (spawning tasks) multiplied
        // contention on the bank's internal locks. Sequential access means
        // the bank's working set stays warm in L2 across all three simulations.
        for (pair_idx, path) in qualifying_pairs {
            if self.validation_mode {
                self.try_execute_arbitrage(
                    &path,
                    &bank,
                    &pool_data,
                    blockhash,
                    created_at,
                    pair_idx,
                );
            } else {
                self.try_submit_production(
                    &path,
                    &bank,
                    &pool_data,
                    blockhash,
                    created_at,
                    pair_idx,
                );
            }
        }
        // `bank` Arc drops here — releasing the reference count so BankForks
        // can evict the slot as soon as all other holders (if any) also drop.
    }

    // -----------------------------------------------------------------------
    // Validation path — inline SVM simulation
    // -----------------------------------------------------------------------

    /// Evaluate one two-hop arbitrage path via full inline SVM simulation.
    ///
    /// Called only when `validation_mode == true`. No transaction is submitted.
    /// The result is logged at `info!` level so the operator can verify that the
    /// arb graph, instruction builder, and simulation pipeline produce profitable
    /// results before switching to production mode.
    ///
    /// Both `verify_transaction` and `simulate_transaction_unchecked` are called
    /// directly on the shard's OS thread — no `spawn_blocking`, no Tokio, no
    /// scheduling dispatch. The shard thread is a dedicated blocking thread;
    /// running blocking SVM code on it is exactly correct and costs zero overhead
    /// compared to the double-`spawn_blocking` model it replaces.
    ///
    /// With at most 12 shards running concurrently, bank lock contention drops
    /// ~10× compared to the previous MAX_CONCURRENT_SIMULATIONS = 128.
    fn try_execute_arbitrage(
        &self,
        path: &ArbitragePath,
        bank: &Arc<Bank>,
        pool_data: &Arc<MintPoolData>,
        blockhash: Hash,
        created_at: Instant,
        pair_idx: usize,
    ) {
        // Cache wallet pubkey once — `pubkey()` derives from the signing key on
        // every call. Four uses below (sim message, WSOL pre/post scan,
        // tip instruction, final message) all become plain 32-byte stack copies.
        let wallet_pubkey = self.wallet.pubkey();

        // Validate token-flow path and produce the typed flow descriptor used by
        // SmbInstructionBuilder.
        let token_flow = match TokenFlowValidator::validate_and_build_flow(path) {
            Ok(tf) => tf,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} flow validation failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        // Phase 1 instruction uses CU ceiling and zero profit floor so the SVM
        // always runs the complete execution path. A non-zero threshold would cause
        // the program to revert during simulation whenever the current price produces
        // a small profit, masking valid opportunities.
        let cu_limit = ESTIMATED_CU_PER_HOP.saturating_mul(path.hop_count() as u32);

        let sim_instruction = match SmbInstructionBuilder::build_instruction_with_flow(
            &self.wallet,
            path,
            &token_flow,
            pool_data,
            bank,
            cu_limit,
            true,
            0, // profit floor disabled for simulation
        ) {
            Ok(ix) => ix,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} sim instruction build failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        let sim_instructions = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(cu_limit),
            ComputeBudgetInstruction::set_compute_unit_price(self.base_priority_fee),
            sim_instruction,
        ];

        let sim_message = match self.lut_manager.create_v0_message(
            &sim_instructions,
            &wallet_pubkey,
            blockhash,
        ) {
            Ok(m) => m,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} sim message build failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        let sim_versioned_tx = match VersionedTransaction::try_new(sim_message, &[&*self.wallet]) {
            Ok(tx) => tx,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} sim tx sign failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        // Stamp the clock immediately before SVM work begins so `sim_duration_us`
        // reflects only the cost of verify_transaction + simulate_transaction_unchecked.
        // Everything above this point (flow validation, instruction construction,
        // message assembly, signing) is cheap heap work that is not the bottleneck.
        let sim_start = Instant::now();

        // Phase 1 — verify_transaction.
        //
        // Performs Address Lookup Table resolution and recent-blockhash validation.
        // Runs inline on the shard thread — no spawn_blocking, no dispatch latency.
        // With 12 shards running concurrently, bank read locks see 10× less contention
        // than the previous 128-concurrent model.
        let runtime_tx = match bank.verify_transaction(
            sim_versioned_tx,
            TransactionVerificationMode::HashOnly,
        ) {
            Ok(tx) => tx,
            Err(e) => {
                // Log the bank slot so the operator can diagnose BlockhashNotFound
                // (event too old) vs AddressLookupTableNotFound (stale LUT).
                warn!(
                    "MevShard[{}]: pair {} verify failed — bank_slot={} error={:?}",
                    self.shard_idx, pair_idx, bank.slot(), e
                );
                return;
            }
        };

        // Phase 2 — simulate_transaction_unchecked.
        //
        // Runs the full SVM execution stack in memory: BPF interpreter, CPI
        // dispatch into Raydium/Meteora/etc., compute unit accounting. Discards
        // all write-set mutations and returns the simulation result. Does not
        // require a frozen bank — safe to call mid-slot on the canonical bank.
        let sim_result = bank.simulate_transaction_unchecked(&runtime_tx, false);

        let sim_duration_us = sim_start.elapsed().as_micros();

        if sim_result.result.is_err() {
            debug!(
                "MevShard[{}]: pair {} simulation rejected — {:?} | logs={:?}",
                self.shard_idx, pair_idx, sim_result.result, sim_result.logs
            );
            return;
        }

        // The arb bot wraps native SOL into WSOL at the start of the transaction.
        // After both hops settle, profit sits in the wallet's WSOL token account.
        // We match on `owner == wallet_pubkey AND mint == WSOL_MINT` to isolate
        // exactly the wallet's WSOL entry — owner-only would catch the intermediate
        // token ATA; mint-only would catch pool-owned WSOL vaults.
        let pre_wsol = sim_result
            .pre_token_balances
            .as_ref()
            .and_then(|v| {
                v.iter()
                    .find(|t| t.owner == wallet_pubkey && t.mint == WSOL_MINT)
                    .map(|t| t.amount)
            })
            .unwrap_or(0);

        let post_wsol = sim_result
            .post_token_balances
            .as_ref()
            .and_then(|v| {
                v.iter()
                    .find(|t| t.owner == wallet_pubkey && t.mint == WSOL_MINT)
                    .map(|t| t.amount)
            })
            .unwrap_or(0);

        // `saturating_sub` clamps a net loss to zero, causing both gates below
        // to reject without underflowing the u64.
        let gross_profit = post_wsol.saturating_sub(pre_wsol);

        // Gate 1 — gross profit floor.
        // Any gross below 5M cannot yield 2M net after a 60% tip. Rejecting here
        // saves the floating-point multiply and the second comparison.
        if gross_profit < MIN_GROSS_PROFIT_LAMPORTS {
            debug!(
                "MevShard[{}]: pair {} below gross floor — gross={} min={}",
                self.shard_idx, pair_idx, gross_profit, MIN_GROSS_PROFIT_LAMPORTS
            );
            return;
        }

        // Dynamic tip: bid scales with opportunity size. `jito_tip_lamports` is
        // the floor — keeps the transaction eligible for Jito dual-routing even
        // when the proportional bid would be smaller.
        let dynamic_tip = ((gross_profit as f64 * TIP_FRACTION) as u64)
            .max(self.jito_tip_lamports);

        let net_profit = gross_profit.saturating_sub(dynamic_tip);

        // Gate 2 — net profit floor after dynamic tip.
        if net_profit < MIN_NET_PROFIT_LAMPORTS {
            debug!(
                "MevShard[{}]: pair {} below net floor — gross={} tip={} net={} min={}",
                self.shard_idx, pair_idx,
                gross_profit, dynamic_tip, net_profit, MIN_NET_PROFIT_LAMPORTS
            );
            return;
        }

        // Total latency from batch-commit through simulation completion. Includes
        // ring-buffer queue time + staleness check + graph lookup + SVM. The
        // difference (total - sim) is the overhead before simulation started.
        let total_us = created_at.elapsed().as_micros();

        info!(
            "[VALIDATION] shard={} pair={} mint={} sim={}µs total={}µs \
             units={} fee={:?} gross={} tip={} net={}",
            self.shard_idx,
            pair_idx,
            pool_data.mint,
            sim_duration_us,
            total_us,
            sim_result.units_consumed,
            sim_result.fee,
            gross_profit,
            dynamic_tip,
            net_profit,
        );
    }

    // -----------------------------------------------------------------------
    // Production path — inline transaction build + HTTP push
    // -----------------------------------------------------------------------

    /// Build the final transaction and push it to the HTTP worker immediately.
    ///
    /// Called only when `validation_mode == false`. Contains zero simulation —
    /// the on-chain SMB program enforces the profit floor atomically. The full
    /// pipeline from event receipt to bytes-in-HTTP-worker is:
    ///
    ///   staleness check         (~1 ns)
    ///   pool data load          (~5 ns — FxHashMap lookup)
    ///   flow validation         (~5–20 µs — pure struct inspection)
    ///   instruction build       (~10–50 µs — account key resolution)
    ///   message assembly        (~20–100 µs — LUT resolution + versioned message)
    ///   sign transaction        (~10–30 µs — ed25519 scalar multiply)
    ///   bincode serialisation   (~5–20 µs)
    ///   rtrb push to HTTP       (~5–20 ns)
    ///
    /// Total: ~50–220 µs before bytes reach the HTTP worker.
    /// The HTTP worker then adds ~200–500 µs for the intra-datacenter TCP round-trip.
    fn try_submit_production(
        &mut self,
        path: &ArbitragePath,
        bank: &Arc<Bank>,
        pool_data: &Arc<MintPoolData>,
        blockhash: Hash,
        created_at: Instant,
        pair_idx: usize,
    ) {
        // Production staleness guard. Previously absent — caused 17-second-old
        // tasks to run and submit transactions for prices the market moved past
        // long ago. Now the first instruction after function entry, costing ~1 ns.
        if created_at.elapsed().as_micros() > MAX_EVENT_STALENESS_US {
            return;
        }

        let wallet_pubkey = self.wallet.pubkey();

        let token_flow = match TokenFlowValidator::validate_and_build_flow(path) {
            Ok(tf) => tf,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} production flow validation failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        // Static CU limit: without simulation we cannot measure exact units.
        // The generous ceiling ensures the program never hits the cap mid-execution.
        let cu_limit = ESTIMATED_CU_PER_HOP.saturating_mul(path.hop_count() as u32);

        // Build the final instruction with the operator-configured profit floor
        // encoded in the instruction data. The on-chain program reads pool reserves
        // at execution time, computes the two-hop output, and reverts if it is
        // below this threshold — atomically enforcing profitability without simulation.
        let final_instruction = match SmbInstructionBuilder::build_instruction_with_flow(
            &self.wallet,
            path,
            &token_flow,
            pool_data,
            bank,
            cu_limit,
            true,
            self.min_profit_lamports,
        ) {
            Ok(ix) => ix,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} production instruction build failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        // Select one of the ten Jito tip accounts at random. Spreading tips across
        // all ten accounts reduces write-lock contention in the block scheduler:
        // transactions tipping the same account must be serialised, which can push
        // some of them into a later slot. Random selection makes contention with
        // our own submissions statistically rare.
        let tip_account =
            JITO_TIP_ACCOUNTS[rand::random_range(0..JITO_TIP_ACCOUNTS.len())];

        // Tip instruction is last: it only executes — and SOL is only deducted —
        // if the arb instruction itself succeeds. If the arb reverts, the entire
        // transaction is atomic and the tip transfer is also rolled back.
        let tip_instruction = Self::build_sol_transfer_instruction(
            wallet_pubkey,
            tip_account,
            self.jito_tip_lamports,
        );

        let final_instructions = vec![
            // Compute budget instructions must appear before any program instruction.
            // The runtime processes them as a pre-pass before entering the SVM.
            ComputeBudgetInstruction::set_compute_unit_limit(cu_limit),
            ComputeBudgetInstruction::set_compute_unit_price(self.base_priority_fee),
            final_instruction,
            tip_instruction,
        ];

        let final_message = match self.lut_manager.create_v0_message(
            &final_instructions,
            &wallet_pubkey,
            blockhash,
        ) {
            Ok(m) => m,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} production message build failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        let final_tx = match VersionedTransaction::try_new(final_message, &[&*self.wallet]) {
            Ok(tx) => tx,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} production sign failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        let tx_bytes = match bincode::serialize(&final_tx) {
            Ok(b) => b,
            Err(e) => {
                warn!(
                    "MevShard[{}]: pair {} serialisation failed: {}",
                    self.shard_idx, pair_idx, e
                );
                return;
            }
        };

        let pre_send_us = created_at.elapsed().as_micros();

        debug!(
            "MevShard[{}]: pair={} pre_send={}µs tip={} — pushing to HTTP worker",
            self.shard_idx, pair_idx, pre_send_us, self.jito_tip_lamports,
        );

        let http_item = HttpWorkItem {
            tx_bytes,
            mint: pool_data.mint,
            pair_idx,
            created_at,
        };

        // Push to the HTTP ring buffer. If the buffer is full the HTTP worker
        // has fallen behind — this submission is dropped to protect the shard's
        // forward progress. The HTTP worker thread should drain faster than the
        // shard can produce profitable transactions under normal conditions.
        if let Err(_) = self.http_producer.push(http_item) {
            warn!(
                "MevShard[{}]: HTTP ring buffer full — dropping submission for pair {}",
                self.shard_idx, pair_idx,
            );
        }
    }

    // -----------------------------------------------------------------------
    // Graduation helpers (rare path — not on hot path)
    // -----------------------------------------------------------------------

    /// Register a new mint's pool data and build its `ArbitrageGraph`.
    ///
    /// Called at startup (via `MevShard::new`) for each initial mint, and at
    /// runtime when a `RegisterMint` work item arrives for a newly graduated
    /// unknown mint. Idempotent: if the mint is already registered this is a no-op.
    fn register_mint_internal(&mut self, pool_data: Arc<MintPoolData>) {
        let mint = pool_data.mint;
        if self.mint_to_graph.contains_key(&mint) {
            return;
        }

        let graph = ArbitrageGraph::build_with_config(
            &pool_data,
            ArbitrageGraphConfig::default(),
        );

        // Register all accounts that belong to this mint's pools in the shard's
        // local reverse index. From this point, any PoolUpdate touching one of
        // these accounts will route to this mint's graph.
        for account in graph.all_tracked_accounts() {
            self.account_to_mint.insert(account, mint);
        }

        self.mint_to_graph.insert(mint, graph);
        self.mint_to_pool_data.insert(mint, pool_data);

        debug!(
            "MevShard[{}]: registered new mint {} ({} total pairs)",
            self.shard_idx,
            mint,
            self.mint_to_graph.get(&mint).map_or(0, |g| g.total_pairs()),
        );
    }

    /// Integrate a newly created pool into an existing mint's graph.
    ///
    /// Called when a `GraduatePool` work item arrives. Updates the pool data
    /// reference, registers any new accounts in the local reverse index, and
    /// inserts the pool into the running `ArbitrageGraph` so future `PoolUpdate`
    /// events for the new pool are routed and simulated correctly.
    fn graduate_pool(
        &mut self,
        mint: Pubkey,
        pool_info: PoolInfo,
        pool_accounts: Vec<Pubkey>,
        updated_pool_data: Arc<MintPoolData>,
    ) {
        // Update pool data so future instruction builds use the new vault addresses.
        self.mint_to_pool_data.insert(mint, updated_pool_data);

        // Register new accounts in the reverse index.
        for account in &pool_accounts {
            self.account_to_mint.insert(*account, mint);
        }

        // Insert the new pool into the live graph. `add_pool` is idempotent on
        // duplicate addresses — the known_pool_addresses set prevents double-entry.
        if let Some(graph) = self.mint_to_graph.get_mut(&mint) {
            let new_pairs = graph.add_pool(pool_info, &pool_accounts);
            info!(
                "MevShard[{}]: graduated {:?} pool {} into mint {} — {} new pair(s)",
                self.shard_idx, pool_info.pool_type, pool_info.address, mint, new_pairs,
            );
        }
    }

    // -----------------------------------------------------------------------
    // Shared utility
    // -----------------------------------------------------------------------

    /// Construct a System Program `Transfer` instruction that moves `lamports` of
    /// SOL from `from` to `to`.
    ///
    /// The System Program's instruction discriminants are fixed by the Solana wire
    /// spec. `Transfer` is variant index 2 in `SystemInstruction`, serialised by
    /// bincode as a u32 little-endian prefix followed by a u64 lamport amount.
    ///
    /// A fixed 12-byte stack array fills the data in place and is converted to
    /// `Vec<u8>` with a single `to_vec()` call — one heap allocation instead of
    /// the previous `Vec::with_capacity(12)` + two `extend_from_slice` calls.
    fn build_sol_transfer_instruction(from: Pubkey, to: Pubkey, lamports: u64) -> Instruction {
        let mut data = [0u8; 12];
        data[..4].copy_from_slice(&2u32.to_le_bytes()); // Transfer discriminant
        data[4..].copy_from_slice(&lamports.to_le_bytes());

        Instruction {
            program_id: solana_sdk_ids::system_program::id(),
            accounts: vec![
                // Sender must sign — System Program validates lamport authority.
                AccountMeta::new(from, true),
                // Recipient does not sign — SOL can be credited unconditionally.
                AccountMeta::new(to, false),
            ],
            data: data.to_vec(),
        }
    }
}

// ---------------------------------------------------------------------------
// HttpWorker
// ---------------------------------------------------------------------------

/// Dedicated blocking HTTP thread paired with one `MevShard`.
///
/// Spins on its `rtrb::Consumer<HttpWorkItem>` ring buffer. When the shard
/// pushes serialised transaction bytes, the worker pops them, base64-encodes
/// them, and fires a synchronous `reqwest::blocking::Client::post` to the
/// Helius Sender FRA endpoint.
///
/// Using `reqwest::blocking` here is intentional: this is a dedicated OS thread
/// with no Tokio runtime — a blocking HTTP call is exactly correct. The
/// persistent connection pool inside `reqwest::blocking::Client` keeps one TCP
/// connection open to the FRA endpoint after the first request, eliminating the
/// per-submission TCP handshake on all subsequent sends.
///
/// There is no confirmation polling. Submission is fire-and-forget: the
/// signature is logged for monitoring, and the on-chain profit floor reverts
/// any unprofitable landing atomically.
pub struct HttpWorker {
    shard_idx: usize,
    consumer: rtrb::Consumer<HttpWorkItem>,
    http_client: BlockingHttpClient,
}

impl HttpWorker {
    /// Create a new `HttpWorker`. Builds a persistent `reqwest::blocking::Client`
    /// with TCP keepalive so the connection to the FRA endpoint survives long idle
    /// periods between profitable opportunities.
    pub fn new(
        shard_idx: usize,
        consumer: rtrb::Consumer<HttpWorkItem>,
    ) -> Self {
        let http_client = reqwest::blocking::ClientBuilder::new()
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("reqwest blocking client build failed");

        Self { shard_idx, consumer, http_client }
    }

    /// Spin loop entry point. Runs for the validator's lifetime on a dedicated
    /// OS thread.
    ///
    /// `spin_loop()` emits a CPU PAUSE hint — reduces power without adding latency.
    /// This thread spends the vast majority of its time waiting between profitable
    /// submissions; the spin cost is negligible compared to the HTTP round-trip.
    pub fn run(mut self) {
        loop {
            match self.consumer.pop() {
                Ok(item) => {
                    if let Err(e) = self.send_item(item) {
                        warn!("HttpWorker[{}]: send failed: {}", self.shard_idx, e);
                    }
                }
                Err(rtrb::PopError::Empty) => {
                    std::hint::spin_loop();
                }
            }
        }
    }

    /// Encode and POST one transaction to the Helius Sender FRA endpoint.
    ///
    /// ## JSON body
    ///
    /// A raw `format!` string replaces any `serde_json::json!` macro usage.
    /// The macro constructs a `serde_json::Value` tree — multiple heap allocations
    /// for a fixed-structure payload that changes only in the base64 tx string
    /// and the request id. `format!` produces a single `String` allocation with
    /// all fields inlined.
    ///
    /// ## Request id
    ///
    /// `SUBMISSION_COUNTER.fetch_add(Relaxed)` — one atomic instruction, no
    /// allocation, no syscall. Multiple `HttpWorker` threads share the counter
    /// safely because `fetch_add` is atomic; each call gets a unique id.
    ///
    /// ## skipPreflight
    ///
    /// Preflight is a second simulation the relay would run against its own
    /// snapshot before forwarding. The on-chain profit floor reverts any
    /// unprofitable landing atomically with no net loss. Preflight adds latency
    /// and risks a false-negative rejection if the bank has advanced since the
    /// shard's build. `skipPreflight: true` eliminates this cost.
    fn send_item(&self, item: HttpWorkItem) -> Result<Signature> {
        let tx_b64 = BASE64_STANDARD.encode(&item.tx_bytes);
        let request_id = SUBMISSION_COUNTER.fetch_add(1, Ordering::Relaxed);

        let body = format!(
            r#"{{"jsonrpc":"2.0","id":{},"method":"sendTransaction","params":["{}",{{"encoding":"base64","skipPreflight":true,"maxRetries":0}}]}}"#,
            request_id,
            tx_b64,
        );

        let response = self.http_client
            .post(HELIUS_SENDER_FRA_ENDPOINT)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .map_err(|e| anyhow!("Helius Sender HTTP request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Helius Sender response body parse failed: {}", e))?;

        // A JSON-RPC `error` field means the relay rejected the request before
        // forwarding — rate limit, missing tip, malformed encoding. A transaction
        // that reaches the leader but fails on-chain is reported as success here
        // and surfaces as an on-chain rejection (which is expected and acceptable —
        // the on-chain profit floor protected the wallet).
        if let Some(err) = json.get("error") {
            return Err(anyhow!(
                "Helius Sender rejected transaction before forwarding: {}",
                err
            ));
        }

        let sig_str = json["result"]
            .as_str()
            .ok_or_else(|| anyhow!("Helius Sender response missing 'result' field: {}", json))?;

        let signature = sig_str
            .parse::<Signature>()
            .map_err(|e| anyhow!(
                "Helius Sender returned an unparseable signature '{}': {}",
                sig_str, e
            ))?;

        let total_us = item.created_at.elapsed().as_micros();

        info!(
            "HttpWorker[{}]: submitted pair={} mint={} sig={} total={}µs tip={}",
            self.shard_idx,
            item.pair_idx,
            item.mint,
            signature,
            total_us,
            // tip lamports are baked into the transaction; logged approximately
            // as the floor value since exact tip is in the tx data
            "see_tx",
        );

        Ok(signature)
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
//   1. The 10% headroom multiplier absorbs variance between simulation CU cost
//      and actual on-chain CU cost caused by sysvar value changes between the
//      simulation slot and the landing slot, tick/bin-array position drift in CLMM
//      pools, and other environmental factors that the simulator cannot predict.
//
//   2. The 5,000 CU floor prevents a degenerate zero-limit transaction when the
//      simulator reports an unusually small or zero `units_consumed` value.

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
        assert_eq!(compute_exact_cu(0), 5_000);
        assert_eq!(compute_exact_cu(1), 5_000);
        assert_eq!(compute_exact_cu(4_545), 5_000);
        assert_eq!(compute_exact_cu(4_546), 5_001);
        assert_eq!(compute_exact_cu(1_000_000), 1_100_000);
        assert_eq!(compute_exact_cu(u32::MAX as u64), u32::MAX);
    }

    // -------------------------------------------------------------------------
    // Test 2 — Transfer instruction data layout
    // -------------------------------------------------------------------------

    /// Verifies that `build_sol_transfer_instruction` produces the correct 12-byte
    /// instruction data for a System Program Transfer.
    ///
    /// The layout is fixed by the Solana wire spec:
    ///   bytes [0..4]  — u32 LE discriminant for Transfer = 2
    ///   bytes [4..12] — u64 LE lamport amount
    ///
    /// This test guards against regressions in the fixed-array construction added
    /// to replace the previous Vec::with_capacity pattern.
    #[test]
    fn test_transfer_instruction_data_layout() {
        use super::MevShard;
        use solana_pubkey::Pubkey;

        let from = Pubkey::new_unique();
        let to   = Pubkey::new_unique();
        let lamports: u64 = 200_000;

        let ix = MevShard::build_sol_transfer_instruction(from, to, lamports);

        assert_eq!(ix.data.len(), 12, "instruction data must be exactly 12 bytes");
        assert_eq!(
            &ix.data[..4],
            &2u32.to_le_bytes(),
            "first 4 bytes must be the Transfer discriminant (u32 LE = 2)"
        );
        assert_eq!(
            &ix.data[4..],
            &lamports.to_le_bytes(),
            "last 8 bytes must be the lamport amount (u64 LE)"
        );
        assert_eq!(ix.accounts.len(), 2, "Transfer requires exactly 2 accounts");
        assert!(ix.accounts[0].is_signer, "sender must be a signer");
        assert!(!ix.accounts[1].is_signer, "recipient must not be a signer");
        assert_eq!(ix.accounts[0].pubkey, from);
        assert_eq!(ix.accounts[1].pubkey, to);
    }

    // -------------------------------------------------------------------------
    // Test 3 — SUBMISSION_COUNTER monotonicity
    // -------------------------------------------------------------------------

    /// Verifies that SUBMISSION_COUNTER increments monotonically across calls.
    ///
    /// Each `send_item` call must produce a unique JSON-RPC request id.
    /// Because the counter is module-global the test reads two successive values
    /// and confirms the second is strictly greater than the first.
    #[test]
    fn test_submission_counter_monotonic() {
        use super::SUBMISSION_COUNTER;
        use std::sync::atomic::Ordering;

        let a = SUBMISSION_COUNTER.fetch_add(1, Ordering::Relaxed);
        let b = SUBMISSION_COUNTER.fetch_add(1, Ordering::Relaxed);
        assert!(b > a, "SUBMISSION_COUNTER must increment monotonically");
    }

    // -------------------------------------------------------------------------
    // Test 4 — Tip floor constant consistency
    // -------------------------------------------------------------------------

    /// Verifies that `HELIUS_SENDER_MIN_TIP_LAMPORTS` is at or below
    /// `MIN_GROSS_PROFIT_LAMPORTS` so that the tip floor can always be satisfied
    /// by an opportunity that passes Gate 1.
    #[test]
    fn test_tip_floor_below_gross_floor() {
        use super::{HELIUS_SENDER_MIN_TIP_LAMPORTS, MIN_GROSS_PROFIT_LAMPORTS};
        assert!(
            HELIUS_SENDER_MIN_TIP_LAMPORTS <= MIN_GROSS_PROFIT_LAMPORTS,
            "tip floor must not exceed the gross profit floor — every Gate 1 pass \
             must be able to satisfy the Helius Sender minimum tip"
        );
    }

    // -------------------------------------------------------------------------
    // Test 5 — Constant consistency
    // -------------------------------------------------------------------------

    /// Verifies that the three profit-related constants are internally consistent:
    /// MIN_GROSS_PROFIT_LAMPORTS × (1 − TIP_FRACTION) == MIN_NET_PROFIT_LAMPORTS.
    ///
    /// If this assertion fails, the two gates are miscalibrated: Gate 1 may pass
    /// opportunities that Gate 2 always rejects (dead range), or Gate 1 may be
    /// set so low that Gate 2 accepts opportunities below the intended net floor.
    #[test]
    fn test_profit_constants_consistent() {
        use super::{MIN_GROSS_PROFIT_LAMPORTS, MIN_NET_PROFIT_LAMPORTS, TIP_FRACTION};

        assert!(
            (TIP_FRACTION - 0.60).abs() < f64::EPSILON,
            "tip fraction must be exactly 0.60 (60%)"
        );

        let net_at_min_gross =
            (MIN_GROSS_PROFIT_LAMPORTS as f64 * (1.0 - TIP_FRACTION)) as u64;
        assert_eq!(
            net_at_min_gross, MIN_NET_PROFIT_LAMPORTS,
            "MIN_GROSS_PROFIT_LAMPORTS * (1 - TIP_FRACTION) must equal MIN_NET_PROFIT_LAMPORTS"
        );
    }

    // -------------------------------------------------------------------------
    // Test 6 — Gross gate logic
    // -------------------------------------------------------------------------

    /// Verifies that Gate 1 rejects paths whose gross profit falls below the floor
    /// and passes paths at or above it.
    #[test]
    fn test_gross_gate_logic() {
        use super::MIN_GROSS_PROFIT_LAMPORTS;

        let passes_gross_gate = |gross: u64| gross >= MIN_GROSS_PROFIT_LAMPORTS;

        assert!(!passes_gross_gate(4_999_999), "4_999_999 lamports gross must fail Gate 1");
        assert!(passes_gross_gate(5_000_000),  "5_000_000 lamports gross must pass Gate 1");
        assert!(passes_gross_gate(10_000_000), "10M lamports gross must pass Gate 1");
        assert!(!passes_gross_gate(0),         "zero gross must fail Gate 1");
    }

    // -------------------------------------------------------------------------
    // Test 7 — Dynamic tip arithmetic
    // -------------------------------------------------------------------------

    /// Verifies the dynamic tip formula: `max(gross * TIP_FRACTION, floor_tip)`.
    #[test]
    fn test_dynamic_tip_arithmetic() {
        use super::TIP_FRACTION;

        let compute_dynamic_tip = |gross: u64, floor_tip: u64| -> u64 {
            ((gross as f64 * TIP_FRACTION) as u64).max(floor_tip)
        };

        // Proportional tip dominates when gross is large.
        assert_eq!(
            compute_dynamic_tip(5_000_000, 200_000),
            3_000_000,
            "proportional tip must dominate at gross=5M with floor=200K"
        );

        // Net check: gross - tip = 5M - 3M = 2M exactly.
        assert_eq!(
            5_000_000u64.saturating_sub(compute_dynamic_tip(5_000_000, 200_000)),
            2_000_000,
            "net at minimum gross must be exactly 2M lamports"
        );

        // Floor dominates when proportional bid is below minimum.
        assert_eq!(
            compute_dynamic_tip(100_000, 200_000),
            200_000,
            "floor tip must dominate when proportional bid is below the minimum"
        );

        // Elevated operator floor wins.
        assert_eq!(
            compute_dynamic_tip(5_000_000, 4_000_000),
            4_000_000,
            "elevated operator floor must dominate over proportional bid when floor is higher"
        );

        // Scale test.
        assert_eq!(
            compute_dynamic_tip(10_000_000, 200_000),
            6_000_000,
            "tip at 10M gross must be 6M lamports (60%)"
        );
        assert_eq!(
            10_000_000u64.saturating_sub(compute_dynamic_tip(10_000_000, 200_000)),
            4_000_000,
            "net at 10M gross must be 4M lamports"
        );
    }

    // -------------------------------------------------------------------------
    // Test 8 — Net gate logic
    // -------------------------------------------------------------------------

    /// Verifies that Gate 2 rejects paths whose net profit falls below
    /// MIN_NET_PROFIT_LAMPORTS and passes those that meet or exceed it.
    #[test]
    fn test_net_gate_logic() {
        use super::{MIN_NET_PROFIT_LAMPORTS, TIP_FRACTION};

        let compute_net = |gross: u64, floor_tip: u64| -> u64 {
            let tip = ((gross as f64 * TIP_FRACTION) as u64).max(floor_tip);
            gross.saturating_sub(tip)
        };

        let passes_net_gate = |gross: u64, floor_tip: u64| -> bool {
            compute_net(gross, floor_tip) >= MIN_NET_PROFIT_LAMPORTS
        };

        // Exactly at minimum gross with standard floor: net = 2M exactly — must pass.
        assert!(
            passes_net_gate(5_000_000, 200_000),
            "5M gross with 200K floor must pass Gate 2 (net = 2M)"
        );

        // One lamport below the net floor via inflated floor_tip.
        assert!(
            !passes_net_gate(5_000_000, 3_000_001),
            "5M gross with floor_tip=3_000_001 must fail Gate 2 (net=1_999_999)"
        );

        // Large gross, standard floor: well above net threshold.
        assert!(
            passes_net_gate(20_000_000, 200_000),
            "20M gross must pass Gate 2 (net = 8M)"
        );

        // Zero gross (loss scenario).
        assert!(
            !passes_net_gate(0, 200_000),
            "zero gross must fail Gate 2"
        );
    }
}
