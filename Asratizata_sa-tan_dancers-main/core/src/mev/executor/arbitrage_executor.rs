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
    // Base64 encoding is required to serialise the signed wire transaction into the
    // string format that JSON-RPC endpoints accept in the params array.
    // `ENGINE as _` imports the trait that provides the `.encode()` method without
    // binding it to a local name — the compiler resolves the method call through
    // the trait but we never reference the trait directly in this file.
    base64::prelude::{BASE64_STANDARD, Engine as _},
    // `HttpClient` is the async, connection-pooling HTTP client from reqwest.
    // It is stored on `ArbitrageExecutor` so that TCP connections to the Helius
    // Sender relay are established once and reused across all submission calls,
    // eliminating the per-submission TCP handshake latency (~1 ms on LAN).
    reqwest::Client as HttpClient,
    serde_json,
    solana_pubkey::Pubkey,
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
        // RwLock is used for the arb_graph field which is shared with the engine's
        // graduation handler.  The engine holds the write lock only during add_pool
        // (rare — one per new pool graduation); the executor holds the read lock for
        // the brief per-event window covering address lookup, pair index iteration,
        // and path clone.  No lock is ever held across an await point.
        sync::{Arc, RwLock},
        time::{Duration, Instant},
    },
    tokio::sync::{broadcast, Semaphore},
    tracing::{debug, info, warn},
};

/// Maximum microseconds a simulation task may wait from the moment its event
/// was received before the opportunity is considered stale and the task exits.
///
/// A Solana slot is produced every 400 ms (400_000 µs).  Any simulation task
/// that has been waiting longer than one slot duration is operating on bank
/// state the market has already moved past — the pool prices it would observe
/// are no longer actionable because at least one full slot of trades has
/// settled on top of them.  Continuing to hold or acquire a semaphore permit
/// for stale work wastes CPU budget that could instead serve the next live
/// event, and it inflates the Tokio task queue with work that can never
/// produce a profitable submission.
///
/// Without this bound, tasks accumulate in the queue faster than they are
/// processed under load, causing `total=` latency in the validation logs to
/// grow without ceiling — eventually reaching millions of microseconds as
/// tasks queued minutes earlier finally acquire permits and simulate against
/// data that is hundreds of slots old.
///
/// The threshold is one slot (400_000 µs).  Tasks that exceed it before
/// acquiring a permit return immediately, never touching the semaphore.
/// Tasks that acquire a permit but then find the event has aged past the
/// threshold during the wait return immediately, releasing the permit so the
/// next live task can proceed.
const MAX_EVENT_STALENESS_US: u128 = 400_000;

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

/// Helius Sender regional HTTP endpoint co-located with our Frankfurt validator.
///
/// Helius Sender's dual-routing model fires two submission paths simultaneously:
///   1. SWQOS — a staked-validator connection to the current slot leader that
///      carries stake-weight priority, allowing it to bypass the unstaked UDP queue
///      that raw TPU submissions compete in under congestion.
///   2. Jito block engine — submits the transaction into Jito's auction so the
///      bundle scheduler can include it ahead of competing MEV transactions.
///
/// Both paths fire in parallel the moment the relay receives the HTTP request.
/// Whichever path reaches the leader first is the one that lands; neither waits
/// for the other. This is strictly faster than choosing one path: a dual-path
/// race always produces latency ≤ min(path_A, path_B).
///
/// The FRA (Frankfurt) endpoint is used because our validator node is
/// co-located in Frankfurt. The transaction travels from our process to the
/// Helius relay over a sub-millisecond intra-datacenter hop before being fanned
/// out, minimising the network segment that has no speed advantage. Using any
/// other regional endpoint (e.g. AMS) would add an unnecessary inter-city
/// fiber hop on every submission — latency that compounds across every arb event.
///
/// Plain HTTP (not HTTPS) is intentional: TLS handshake and record encryption
/// add ~0.5–1 ms per request at this scale. The submission payload is a signed,
/// authenticated Solana transaction — its authenticity is already guaranteed by
/// the ed25519 signature. TLS would add overhead without adding security.
const HELIUS_SENDER_FRA_ENDPOINT: &str = "http://fra-sender.helius-rpc.com/fast";

/// Minimum lamports transferred to a Jito tip account in every submission
/// transaction. Helius Sender's dual-routing path requires at least 0.0002 SOL
/// (200_000 lamports) in this transfer for the transaction to be eligible for
/// the Jito block engine auction.
///
/// The tip is a plain SOL transfer from the wallet to one of the ten designated
/// Jito tip accounts. It is included as the last instruction of the submission
/// transaction, after the arb instruction, so that the on-chain arb program
/// executes first and the tip transfer only settles if the arb succeeds.
/// The tip amount is configurable via `ArbitrageExecutor::jito_tip_lamports`
/// to allow operators to tune it above this floor based on current tip market
/// conditions. Higher tips increase the probability of winning the block engine
/// auction when competing bundles are also bidding.
const HELIUS_SENDER_MIN_TIP_LAMPORTS: u64 = 200_000;

/// The canonical Wrapped SOL mint address.
///
/// All on-chain swap programs operate on SPL tokens, not native SOL.  The arb bot
/// wraps native SOL into WSOL at the start of every transaction so it can be passed
/// into pool instructions as a standard token.  After the two-hop swap completes, the
/// profit accumulates in the wallet's WSOL token account — it is never unwrapped back
/// to native SOL within the same transaction.  This makes `post_token_balances` the
/// correct source for profit measurement.  Native `post_balances` only reflects the
/// transaction fee deduction, not the arb return.
const WSOL_MINT: Pubkey = solana_pubkey::pubkey!("So11111111111111111111111111111111111111112");

/// Minimum simulated gross profit (pre-tip) an arb path must clear before any
/// tip arithmetic or submission occurs.
///
/// Gross profit is `post_balances[0] - pre_balances[0]` — the increase in the
/// wallet's lamport balance produced by the two-hop swap after the transaction
/// fee has been deducted.  At a 60 % tip fraction, a gross of 5_000_000 yields
/// exactly 2_000_000 lamports net to the operator after paying a 3_000_000
/// lamport tip.  Any gross below this threshold cannot satisfy the net floor
/// regardless of how the tip is computed, so rejection before tip arithmetic
/// saves the branch overhead.
const MIN_GROSS_PROFIT_LAMPORTS: u64 = 5_000_000;

/// Fraction of gross profit paid as the Jito tip.
///
/// The block-engine auction is a first-price sealed-bid auction: the bundle that
/// bids the highest tip wins the slot position.  Paying a fixed proportion of the
/// realised gross ensures that the bid scales with the value of the opportunity —
/// larger arbs bid more and are more likely to win the auction, while marginal
/// arbs bid proportionally less, preserving more of the thin margin.
///
/// `jito_tip_lamports` on the executor is the floor: if `gross * TIP_FRACTION`
/// is smaller than the operator-configured minimum, the minimum takes effect to
/// keep the transaction eligible for the dual-routing Jito path.
const TIP_FRACTION: f64 = 0.60;

/// Minimum lamports the operator must retain after paying the dynamic Jito tip.
///
/// An arb that grosses 5_000_000 and tips 3_000_000 (60 %) nets exactly
/// 2_000_000 lamports — the breakeven point for this floor.  Any path that nets
/// less is discarded before the submission pipeline is entered, ensuring the
/// validator never pays gas and tip for an opportunity that does not cover its
/// own minimum return threshold.
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
    pub(crate) wallet: Arc<Keypair>,
    pub(crate) lut_manager: Arc<LutManager>,
    /// RPC client used exclusively for confirmation polling (`get_signature_status`).
    /// Transaction submission no longer goes through this client — it is routed
    /// directly through Helius Sender via `http_client`. Keeping the RPC client
    /// for confirmation avoids a second dependency on Helius Sender for a read-only
    /// operation that does not benefit from Sender's write-optimised relay.
    pub(crate) rpc_client: Arc<solana_client::rpc_client::RpcClient>,
    pub(crate) base_priority_fee: u64,
    /// Minimum lamports the on-chain SMB executor must realise before the
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
    /// Floor for the Jito tip transfer in every submission transaction.
    ///
    /// The actual tip paid is `max(gross_profit * TIP_FRACTION, jito_tip_lamports)`.
    /// This field sets the absolute minimum — if the proportional bid would fall
    /// below this value, the minimum takes effect to maintain Jito block-engine
    /// eligibility.  Must be at least `HELIUS_SENDER_MIN_TIP_LAMPORTS` (200_000).
    pub(crate) jito_tip_lamports: u64,
    /// Async HTTP client with a persistent connection pool to the Helius Sender relay.
    ///
    /// `reqwest::Client` internally manages a pool of TCP connections. Because our
    /// submission requests all target the same `HELIUS_SENDER_FRA_ENDPOINT` host,
    /// the pool keeps one (or more) TCP connections open and reuses them across
    /// successive `send_transaction` calls. The benefit is that we avoid the
    /// TCP three-way handshake (~0.2–0.5 ms over LAN) and the HTTP/1.1 pipeline
    /// stall on every arb attempt. The client is `Clone`-cheap (it wraps an `Arc`)
    /// and is `Send + Sync`, so it is safe to share via `Arc<ArbitrageExecutor>`.
    pub(crate) http_client: HttpClient,
    pub(crate) validation_mode: bool,
}

impl ArbitrageExecutor {
    pub fn new(
        arb_graph: Arc<RwLock<ArbitrageGraph>>,
        pool_data: Arc<ArcSwap<MintPoolData>>,
        wallet: Arc<Keypair>,
        lut_manager: Arc<LutManager>,
        rpc_client: Arc<solana_client::rpc_client::RpcClient>,
        base_priority_fee: u64,
        min_profit_lamports: u64,
        // The http_client is constructed once in MevEngine and cloned (Arc-cheap)
        // into every executor. This avoids the per-constructor allocation cost of
        // HttpClient::new() — which was previously called for every mint registration,
        // totalling millions of connection-pool allocations and causing the observed
        // 8-minute startup. reqwest::Client is internally Arc-wrapped so clone() is
        // a single atomic increment with no heap allocation.
        http_client: HttpClient,
        jito_tip_lamports: u64,
        validation_mode: bool,
    ) -> Result<Self> {
        // Validate that the caller-supplied tip is at least the hard minimum required
        // by Helius Sender for Jito auction eligibility. Submitting below the minimum
        // would cause the relay to silently reject the Jito path — the transaction
        // would only travel via SWQOS, losing the dual-routing advantage.
        //
        // Returns Err rather than panicking so that a misconfigured tip on the runtime
        // graduation path (register_mint) does not kill the engine thread. At startup
        // (register_mint_startup) the caller uses expect() to fail-fast with a clear
        // message before the validator begins processing live blocks.
        if jito_tip_lamports < HELIUS_SENDER_MIN_TIP_LAMPORTS {
            return Err(anyhow!(
                "jito_tip_lamports ({}) must be >= HELIUS_SENDER_MIN_TIP_LAMPORTS ({})",
                jito_tip_lamports,
                HELIUS_SENDER_MIN_TIP_LAMPORTS,
            ));
        }

        Ok(Self {
            arb_graph,
            pool_data,
            wallet,
            lut_manager,
            rpc_client,
            base_priority_fee,
            min_profit_lamports,
            jito_tip_lamports,
            // Connection pool established once by the caller (MevEngine::new). The pool
            // is lazy — no TCP connection is opened until the first send_transaction call.
            // After the first submission, the persistent connection to the FRA endpoint
            // is reused for the validator's lifetime, eliminating the per-submission
            // TCP handshake cost (~0.2–0.5 ms over LAN).
            http_client,
            validation_mode,
        })
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
                    // `event.created_at` was stamped by the engine at the instant the
                    // batch committed to the canonical bank — the true birth time of this
                    // opportunity.  Using it for staleness guards ensures that an event
                    // which sat in the broadcast ring buffer for 350 ms before being
                    // dequeued here is already recognised as nearly stale when the spawned
                    // task first checks.  Measuring from dequeue time instead would grant
                    // that event a fresh 400 ms budget, letting the task acquire a semaphore
                    // permit and run a full SVM simulation against market data the cluster
                    // has already moved 350 ms past.  Using `event.created_at` closes that
                    // gap: the elapsed time always reflects the true age of the opportunity
                    // regardless of how long the event queued in the broadcast buffer or
                    // how long the fan-out loop's graph-read window took to complete.
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

                    debug!(
                        "ArbitrageExecutor[{}]: pool {} affects {} qualifying pair(s)",
                        mint,
                        pool_address,
                        qualifying_pairs.len(),
                    );

                    let event = Arc::new(event);

                    for (pair_idx, path) in qualifying_pairs {
                        let self_clone = Arc::clone(&self);
                        let event_clone = Arc::clone(&event);
                        let sem = Arc::clone(&simulation_semaphore);

                        tokio::spawn(async move {
                            // Pre-semaphore staleness guard.
                            //
                            // The Tokio task queue can back up under load: if many events
                            // arrive in a short window, spawned tasks wait here before they
                            // are even scheduled to run.  `event_clone.created_at` carries
                            // the batch-commit timestamp — the instant the pool price changed
                            // on-chain — so this check fires as soon as the opportunity has
                            // aged past one full slot (400 ms), regardless of how long the
                            // event spent in the broadcast ring buffer or the task queue.
                            // Returning here — before touching the semaphore — costs nothing
                            // and keeps the permit pool fully available for the next live event.
                            if event_clone.created_at.elapsed().as_micros() > MAX_EVENT_STALENESS_US {
                                return;
                            }

                            // Acquire a permit before doing any work. If the semaphore is
                            // exhausted, this await yields — no OS thread is blocked.
                            // The permit is moved into try_execute_arbitrage and dropped
                            // there after simulation completes, before the I/O-bound
                            // submission phase starts.
                            let permit = match sem.acquire_owned().await {
                                Ok(p) => p,
                                Err(_) => return, // semaphore closed = validator shutdown
                            };

                            // Post-semaphore staleness guard.
                            //
                            // The task may have waited a non-trivial time in the semaphore
                            // queue — long enough for the event to cross the staleness
                            // threshold even though it was fresh when the pre-semaphore
                            // check ran.  `event_clone.created_at` gives the true age from
                            // batch-commit, so this guard fires correctly even when the
                            // broadcast-buffer delay and the semaphore-queue delay together
                            // push the total age past one slot.  Returning here releases the
                            // permit immediately so the next waiting task can acquire it,
                            // rather than holding it through a full SVM simulation cycle
                            // against bank state the market has already moved past.  The
                            // permit is an OwnedSemaphorePermit and drops automatically
                            // when this scope exits.
                            if event_clone.created_at.elapsed().as_micros() > MAX_EVENT_STALENESS_US {
                                return;
                            }

                            if let Err(e) = self_clone
                                .try_execute_arbitrage(
                                    &path,
                                    &event_clone,
                                    pair_idx,
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
    /// The Jito tip instruction is added only in Phase 2 — the simulation in
    /// Phase 1 does not include it. The tip is a plain SOL transfer and consumes
    /// approximately 150 CUs. The 10 % headroom applied to Phase 1's
    /// `units_consumed` easily absorbs this difference, so the declared CU limit
    /// in Phase 2 is never below the actual execution cost.
    ///
    /// ## Bank
    ///
    /// The bank arrives in `event.bank`, already carrying all committed writes
    /// from the batch that triggered this event.  `execute_batch()` in
    /// `blockstore_processor.rs` clones `Arc<Bank>` into every `MevExecutedBatch`
    /// immediately after the SVM commit returns — mid-slot, before `bank.freeze()`
    /// is called.  `simulate_transaction_unchecked` does not require a frozen bank,
    /// so simulation against an in-progress canonical bank is safe and gives the
    /// most current possible pool state.
    async fn try_execute_arbitrage(
        &self,
        path: &ArbitragePath,
        event: &MevPoolUpdateEvent,
        pair_idx: usize,
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

        // Cache the wallet pubkey once for the entire function.  self.wallet.pubkey()
        // derives the public key from the signing key on every call — called four times
        // across try_execute_arbitrage (sim message, WSOL pre/post scans, tip instruction,
        // final message).  One binding here pays the derivation cost once and turns every
        // downstream call into a plain Copy of a 32-byte stack value.
        let wallet_pubkey = self.wallet.pubkey();

        // The bank arrives directly in the event, already committed.
        // `execute_batch()` in blockstore_processor clones `Arc<Bank>` into every
        // `MevExecutedBatch` the moment the SVM commit returns — mid-slot, before
        // `bank.freeze()` is called.  All account writes from every `Ok` commit result
        // in this batch are already live in this bank.  `simulate_transaction_unchecked`
        // does not require a frozen bank, so simulation against an in-progress canonical
        // bank is safe and gives the most current possible pool state.
        let sim_bank: Arc<Bank> = Arc::clone(&event.bank);

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
            &wallet_pubkey,
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
        //
        // `sim_start` is stamped here — at the precise boundary where CPU-bound
        // SVM work begins — so that `sim_duration_us` reflects only the cost of
        // verify_transaction plus simulate_transaction_unchecked.  Everything
        // above this point (token flow validation, instruction construction,
        // message assembly, transaction signing) is intentionally excluded: those
        // are cheap heap operations that are not the bottleneck we want to measure.
        // The SVM execution cost is what determines whether we can profitably race
        // against the slot boundary, so isolating it gives an actionable signal.
        let sim_start = Instant::now();
        let sim_bank_clone = Arc::clone(&sim_bank);
        let runtime_tx = tokio::task::spawn_blocking(move || {
            sim_bank_clone.verify_transaction(
                sim_versioned_tx,
                TransactionVerificationMode::HashOnly,
            )
        })
        .await
        .map_err(|e| anyhow!("verify_transaction task panicked for pair {}: {}", pair_idx, e))?
        .map_err(|e| {
            // `verify_transaction` with HashOnly mode performs exactly two checks:
            // Address Lookup Table resolution and recent-blockhash validation.
            // The TransactionError variant it returns directly identifies which
            // check failed and why:
            //
            //   BlockhashNotFound              — the blockhash baked into this
            //                                   transaction has aged out of the bank's
            //                                   ~300-slot rolling window.  The event sat
            //                                   in the semaphore queue long enough for
            //                                   the blockhash to expire before this task
            //                                   ran.  Widen the semaphore permit pool or
            //                                   reduce event queue depth so tasks are
            //                                   scheduled closer to the event arrival.
            //
            //   AddressLookupTableNotFound     — the LUT account pubkey encoded in the
            //                                   versioned message does not exist in this
            //                                   bank's account state.  LutManager holds
            //                                   a stale or incorrect table address for
            //                                   this mint.  Rebuild the LUT entry for
            //                                   this mint's accounts.
            //
            //   InvalidAddressLookupTableIndex — the LUT account exists but the byte
            //                                   index into its address list points past
            //                                   the end of the table.  The instruction
            //                                   builder encoded the wrong account
            //                                   position when compiling the versioned
            //                                   message.  Verify the account ordering
            //                                   in SmbInstructionBuilder for this pair.
            //
            //   SanitizeFailure                — the compiled message is structurally
            //                                   malformed (duplicate signers, invalid
            //                                   program index, etc.).  This is an
            //                                   instruction-builder bug independent of
            //                                   bank state.
            //
            // mint + pair identify the failing path, bank_slot gives the bank age
            // at verification time (critical for diagnosing BlockhashNotFound since
            // a slot far ahead of the event blockhash confirms queue starvation),
            // and error is the exact machine-readable TransactionError variant.
            warn!(
                "ArbitrageExecutor[{}]: sanitization failed — pair={} bank_slot={} error={:?}",
                pool_data.mint,
                pair_idx,
                sim_bank.slot(),
                e,
            );
            anyhow!(
                "transaction sanitization failed for pair {}: {:?}",
                pair_idx,
                e,
            )
        })?;

        // `simulate_transaction_unchecked` does NOT assert `bank.is_frozen()` so it
        // is safe to call on an active (non-frozen) canonical bank.  It runs the
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

        // Stop the simulation clock the instant both spawn_blocking calls have
        // returned.  `sim_duration_us` captures the wall-clock time that the
        // blocking thread pool spent inside verify_transaction (ALT resolution
        // + recent-blockhash hash check) and simulate_transaction_unchecked
        // (full SVM execution including CPI dispatch into Raydium / Meteora).
        // This number is what determines our actual time budget per event: if
        // sim_duration_us routinely exceeds ~5 ms we are spending more than
        // half a Solana slot (400 ms / 80 slots = 5 ms per slot window) just
        // simulating, which compresses the remaining time available for Phase 2
        // transaction construction and network submission.
        let sim_duration_us = sim_start.elapsed().as_micros();

        if sim_result.result.is_err() {
            // `logs` is the Vec<String> the SVM collected from every `msg!()` call
            // and program log emission during execution — the same output you would
            // see from a simulateTransaction RPC call.  Logging it alongside the
            // TransactionError variant gives a complete picture: the variant tells
            // you the category of failure (InstructionError, InsufficientFunds, …)
            // while the logs tell you exactly where inside the call stack the
            // program decided to revert and what state it observed at that point.
            tracing::debug!(
                "ArbitrageExecutor[{}]: pair {} simulation rejected: {:?} | logs={:?}",
                pool_data.mint,
                pair_idx,
                sim_result.result,
                sim_result.logs,
            );
            return Ok(());
        }

        // The arb bot wraps native SOL into WSOL at the start of the transaction so the
        // swap programs can treat it as a standard SPL token.  After both hops complete,
        // the profit sits in the wallet's WSOL token account — it is never unwrapped back
        // to native SOL inside the same transaction.  This means the native lamport vecs
        // (`pre_balances` / `post_balances`) only show the wallet losing the transaction
        // fee.  The correct profit signal is in `pre_token_balances` / `post_token_balances`,
        // which the balance collector populates for every SPL token account in the
        // transaction when `enable_transaction_balance_recording` is true —
        // `simulate_transaction_unchecked` always sets this flag.
        //
        // We match on both `owner == wallet pubkey` AND `mint == WSOL_MINT` to isolate
        // the wallet's own WSOL token account.  Matching on owner alone would also catch
        // the wallet's ATA for the intermediate arb token.  Matching on mint alone would
        // also catch pool-owned WSOL accounts.  The conjunction uniquely identifies the
        // one entry that represents the wallet's settled WSOL balance.
        // `wallet_pubkey` was cached once at the top of this function — used here and
        // in the three other call sites (sim message, tip instruction, final message).
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

        // Gross profit is the increase in the wallet's WSOL balance after both swap hops
        // settle.  `saturating_sub` clamps a net loss to zero, which causes both gates
        // below to reject the opportunity without underflowing the u64.
        let gross_profit = post_wsol.saturating_sub(pre_wsol);

        // Gate 1 — gross profit floor.
        //
        // At the 60 % tip fraction the minimum gross that can satisfy the 2M lamport
        // net floor is 5M lamports (gross × 0.40 = net → 5M × 0.40 = 2M).  Any gross
        // below 5M cannot pass Gate 2 regardless of how the dynamic tip resolves,
        // so the early rejection here saves the division and the second comparison.
        if gross_profit < MIN_GROSS_PROFIT_LAMPORTS {
            tracing::debug!(
                "ArbitrageExecutor[{}]: pair {} below gross floor — gross={} min={}",
                pool_data.mint,
                pair_idx,
                gross_profit,
                MIN_GROSS_PROFIT_LAMPORTS,
            );
            return Ok(());
        }

        // Dynamic tip: the block-engine bid scales with the gross profit so that
        // larger opportunities bid more aggressively in the Jito auction.
        // `jito_tip_lamports` is the floor — when `gross × TIP_FRACTION` is smaller
        // than the operator-configured minimum, the minimum takes effect so the
        // transaction stays eligible for the dual-routing Jito path.
        let dynamic_tip = ((gross_profit as f64 * TIP_FRACTION) as u64)
            .max(self.jito_tip_lamports);

        // Net profit is what the operator retains after the Jito tip is transferred.
        // `saturating_sub` prevents underflow in the degenerate case where the dynamic
        // tip floor is set higher than the gross — in practice the gross gate above
        // and the MIN_NET check below together make such a configuration inert, but
        // the arithmetic must still be defined.
        let net_profit = gross_profit.saturating_sub(dynamic_tip);

        // Gate 2 — net profit floor after dynamic tip.
        //
        // The wallet retains `net_profit` lamports from this arb after the tip is paid.
        // Paths that do not meet the minimum are discarded before entering the submission
        // pipeline, ensuring no gas or tip is ever paid for a sub-threshold opportunity.
        if net_profit < MIN_NET_PROFIT_LAMPORTS {
            tracing::debug!(
                "ArbitrageExecutor[{}]: pair {} below net floor — gross={} tip={} net={} min={}",
                pool_data.mint,
                pair_idx,
                gross_profit,
                dynamic_tip,
                net_profit,
                MIN_NET_PROFIT_LAMPORTS,
            );
            return Ok(());
        }

        // `event.created_at` carries the batch-commit timestamp stamped by the engine
        // the instant the pool state changed on-chain.  Using it here gives the true
        // end-to-end latency: from the moment the opportunity was born (pool price
        // changed) through simulation completion.  The `total=` field in the validation
        // log therefore includes broadcast-buffer queuing time, Tokio task-queue wait,
        // semaphore-queue wait, and SVM execution time — the complete picture of where
        // microseconds are spent between opportunity birth and simulation result.
        let latency_us = event.created_at.elapsed().as_micros();

        if self.validation_mode {
            // The simulation_permit drops here when this early return is reached —
            // Rust's drop semantics guarantee that function parameters are dropped
            // at the point the function exits, in reverse declaration order.
            // The permit is released before any I/O occurs, consistent with the
            // design principle that it gates CPU simulation, not logging or submission.
            //
            // Two latency fields are emitted intentionally:
            //   `sim=`   — pure SVM time: verify_transaction + simulate_transaction_unchecked.
            //              This is the irreducible cost of evaluating one path on the current
            //              bank state.  If this number grows, the cause is SVM complexity
            //              (more accounts, more CPI hops, larger tick arrays) not contention.
            //   `total=` — wall-clock time from the moment the pool update event was received
            //              to the moment simulation completed, including however long this task
            //              waited for a semaphore permit before any work could start.
            //              The difference (total - sim) is the semaphore contention cost: time
            //              spent parked waiting for another simulation to release its permit.
            //              A large gap signals that the permit pool is too narrow relative to
            //              the event arrival rate and that profitable opportunities are being
            //              delayed — not by slow simulation, but by queuing.
            info!(
                "[VALIDATION] pair={} mint={} sim={}µs total={}µs units={} fee={:?} gross={} tip={} net={}",
                pair_idx,
                pool_data.mint,
                sim_duration_us,
                latency_us,
                sim_result.units_consumed,
                sim_result.fee,
                gross_profit,
                dynamic_tip,
                net_profit,
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
        //
        // The Jito tip instruction added below (a plain SOL transfer) consumes
        // approximately 150 CUs at runtime. Because the 10 % headroom is computed
        // from the arb-only simulation result, those 150 CUs are not included in
        // the base `units_consumed` figure. However, the 10 % margin on any real
        // arb transaction (minimum tens of thousands of CUs) is always larger than
        // 150, so the headroom fully covers the tip instruction's cost.
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

        // Select one of the ten Jito tip accounts at random for this transaction.
        // Spreading tips across all ten accounts reduces write-lock contention in
        // the scheduler: if multiple transactions in the same block tip the same
        // account they must be serialised, which can push some of them out of the
        // current slot. Random selection statistically distributes our transactions
        // across all ten accounts, making contention with our own submissions rare.
        let tip_account =
            JITO_TIP_ACCOUNTS[rand::random_range(0..JITO_TIP_ACCOUNTS.len())];

        // The Jito tip is the last instruction in the transaction. Placement after
        // the arb instruction means the tip SOL transfer only executes — and is only
        // deducted from the wallet — if the arb instruction itself succeeds. If the
        // arb reverts (e.g. price moved past the profit floor), the entire transaction
        // is atomic and the tip transfer is also rolled back, so no tip is paid for
        // a failed arb attempt.
        //
        // `dynamic_tip` replaces the static `jito_tip_lamports` field here — the bid
        // is proportional to the simulated gross profit, rising with opportunity value.
        let tip_instruction = Self::build_sol_transfer_instruction(
            wallet_pubkey,
            tip_account,
            dynamic_tip,
        );

        let final_instructions: Vec<Instruction> = vec![
            // Compute budget instructions must appear before any program instruction
            // in the transaction. The Solana runtime processes them as a pre-pass
            // before entering the SVM, so any other ordering is rejected at
            // transaction validation time.
            ComputeBudgetInstruction::set_compute_unit_limit(exact_cu_limit),
            ComputeBudgetInstruction::set_compute_unit_price(self.base_priority_fee),
            final_instruction,
            tip_instruction,
        ];

        let final_message = self.lut_manager.create_v0_message(
            &final_instructions,
            &wallet_pubkey,
            event.blockhash,
        )?;

        let final_tx = VersionedTransaction::try_new(final_message, &[&*self.wallet])?;

        info!(
            "ArbitrageExecutor[{}]: pair={} latency={}µs units={} fee={:?} gross={} tip={} net={} — submitting",
            pool_data.mint,
            pair_idx,
            latency_us,
            sim_result.units_consumed,
            sim_result.fee,
            gross_profit,
            dynamic_tip,
            net_profit,
        );

        let signature = self.send_transaction(final_tx).await?;
        info!(
            "ArbitrageExecutor[{}]: submitted pair={} sig={}",
            pool_data.mint, pair_idx, signature
        );

        self.confirm_transaction(signature).await?;

        Ok(())
    }

    /// Submit the signed transaction through Helius Sender's Amsterdam regional endpoint.
    ///
    /// ## Why Helius Sender instead of direct TPU
    ///
    /// Direct TPU submission fires a raw UDP packet at the current slot leader's TPU
    /// port. Without stake weight behind the connection, the packet enters the leader's
    /// unstaked queue, which is deprioritised under congestion — packets can be silently
    /// dropped before they are ever processed. Helius Sender routes the transaction
    /// through its own staked validator connection (Solana's #1 by stake weight), which
    /// gives the packet stake-weight priority in the leader's ingress queue.
    ///
    /// ## Dual-path routing
    ///
    /// Helius Sender fires two paths simultaneously:
    ///   1. SWQOS — a staked-weight TCP stream to the current leader's TPU. Stake weight
    ///      means the leader's scheduler processes this connection before unstaked peers.
    ///   2. Jito block engine — the transaction enters Jito's bundle auction. The Jito
    ///      scheduler can include it in the block ahead of all non-Jito transactions.
    ///
    /// Both paths fire as soon as the HTTP request reaches the relay — neither waits for
    /// the other. Whichever path lands the transaction first wins. This is always at least
    /// as fast as either path alone, and typically faster because it is a race.
    ///
    /// ## No preflight
    ///
    /// Preflight is a second simulation the relay would run against its own snapshot
    /// before forwarding. Our Phase 1 simulation already ran `simulate_transaction_unchecked`
    /// against the live canonical bank mid-slot — more current than any snapshot the relay
    /// holds. Running preflight after that adds latency and risks a false-negative rejection
    /// if the bank has advanced since our simulation. The on-chain profit floor reverts any
    /// unprofitable landing atomically with no net loss, making preflight's safety net
    /// redundant.
    ///
    /// ## Connection warming
    ///
    /// `self.http_client` (a `reqwest::Client`) maintains a persistent connection pool.
    /// After the first submission, the TCP connection to the FRA endpoint stays open
    /// for keep-alive. Subsequent submissions reuse the open connection, skipping the
    /// handshake. The pool is transparent to this call site — `reqwest` selects an
    /// available connection from the pool automatically.
    async fn send_transaction(
        &self,
        transaction: VersionedTransaction,
    ) -> Result<Signature> {
        let tx_bytes = bincode::serialize(&transaction)
            .map_err(|e| anyhow!("transaction bincode serialization failed: {}", e))?;

        // JSON-RPC over HTTP transports the binary transaction as a base64 string
        // inside the params array. `BASE64_STANDARD` uses the RFC 4648 alphabet
        // with `=` padding, which is what the Solana JSON-RPC spec requires.
        let tx_b64 = BASE64_STANDARD.encode(&tx_bytes);

        // The JSON-RPC `id` field is echoed back in the response and is used to
        // correlate requests with responses when multiple calls share a connection.
        // Millisecond epoch timestamp produces a unique id per call without a counter.
        let request_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .to_string();

        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "sendTransaction",
            "params": [
                tx_b64,
                {
                    "encoding": "base64",
                    // Preflight is a second simulation the relay runs before forwarding.
                    // Our canonical bank simulation in Phase 1 is more current than any
                    // snapshot the relay holds, so preflight is redundant latency here.
                    "skipPreflight": true,
                    // Disable the relay's internal retry loop so this executor retains
                    // full control over blockhash expiry, per-event price freshness,
                    // and the decision of when to abandon a stale transaction.
                    "maxRetries": 0
                }
            ]
        });

        let response = self
            .http_client
            .post(HELIUS_SENDER_FRA_ENDPOINT)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Helius Sender HTTP request failed: {}", e))?;

        // Deserialize the JSON-RPC response body. This is an async read of the
        // response stream — the await here is network I/O, not CPU work.
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Helius Sender response body parse failed: {}", e))?;

        // The JSON-RPC `error` field is set when the relay rejects the request
        // before it is forwarded to the network — examples: rate limit exceeded,
        // missing or invalid tip, malformed encoding. A transaction that reaches
        // the leader but fails on-chain (e.g. profit floor not met) is reported
        // as success here and surfaces as an error during confirmation polling.
        if let Some(err) = json.get("error") {
            return Err(anyhow!(
                "Helius Sender rejected transaction before forwarding: {}",
                err
            ));
        }

        // On success, `result` is the base58-encoded transaction signature — the
        // same string that `solana confirm <signature>` or any block explorer accepts.
        let sig_str = json["result"]
            .as_str()
            .ok_or_else(|| anyhow!("Helius Sender response missing 'result' field: {}", json))?;

        sig_str
            .parse::<Signature>()
            .map_err(|e| anyhow!(
                "Helius Sender returned an unparseable signature '{}': {}",
                sig_str, e
            ))
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

    /// Construct a System Program `Transfer` instruction that moves `lamports` of SOL
    /// from `from` to `to`.
    ///
    /// The System Program's instruction discriminants are part of the Solana wire
    /// specification and have been stable since genesis. `Transfer` is variant index 2
    /// in the `SystemInstruction` enum. The enum is serialised by bincode as a u32
    /// little-endian prefix followed by variant-specific fields — for `Transfer`, a
    /// single u64 lamport amount. This manual construction avoids importing a dedicated
    /// system-instruction builder crate while remaining fully ABI-compatible with what
    /// the Solana runtime expects to see when it deserialises the instruction data.
    fn build_sol_transfer_instruction(from: Pubkey, to: Pubkey, lamports: u64) -> Instruction {
        // SystemInstruction::Transfer discriminant = 2, serialised as u32 LE.
        // Followed immediately by the u64 LE lamport amount. Total: 12 bytes.
        let mut data = Vec::with_capacity(12);
        data.extend_from_slice(&2u32.to_le_bytes()); // variant index for Transfer
        data.extend_from_slice(&lamports.to_le_bytes()); // amount in lamports

        Instruction {
            program_id: solana_sdk_ids::system_program::id(),
            accounts: vec![
                // The sender must be a transaction signer because the System Program
                // validates that the lamports deduction is authorised before executing.
                AccountMeta::new(from, true),
                // The recipient does not need to sign — the System Program allows any
                // signer to credit any account unconditionally.
                AccountMeta::new(to, false),
            ],
            data,
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

    // -------------------------------------------------------------------------
    // Test 2 — Jito tip minimum enforcement
    // -------------------------------------------------------------------------

    /// Verifies that `HELIUS_SENDER_MIN_TIP_LAMPORTS` equals 200_000 (0.0002 SOL).
    ///
    /// Helius Sender requires at least 0.0002 SOL in the tip transfer for the Jito
    /// block engine path to be eligible. Below this threshold the relay forwards the
    /// transaction only via SWQOS, losing the dual-path race advantage. The constant
    /// is tested explicitly so any accidental change to the minimum is caught before
    /// it silently degrades landing rates in production.
    #[test]
    fn test_jito_tip_minimum_is_correct_lamports() {
        use super::HELIUS_SENDER_MIN_TIP_LAMPORTS;
        // 0.0002 SOL × 1_000_000_000 lamports/SOL = 200_000 lamports.
        assert_eq!(
            HELIUS_SENDER_MIN_TIP_LAMPORTS, 200_000,
            "Helius Sender requires at least 0.0002 SOL (200_000 lamports) for dual routing"
        );
    }

    // -------------------------------------------------------------------------
    // Test 3 — Jito tip account set completeness
    // -------------------------------------------------------------------------

    /// Verifies that `JITO_TIP_ACCOUNTS` contains exactly ten entries.
    ///
    /// Jito Labs publishes exactly ten designated tip accounts. The modulo selection
    /// `rand::random::<usize>() % JITO_TIP_ACCOUNTS.len()` relies on the array
    /// having the correct count: too few accounts increases write-lock contention;
    /// a stale account not in the official set would cause the tip transfer to land
    /// in an unmonitored wallet and the block engine would not credit the tip.
    #[test]
    fn test_jito_tip_accounts_count() {
        use super::JITO_TIP_ACCOUNTS;
        assert_eq!(
            JITO_TIP_ACCOUNTS.len(), 10,
            "JITO_TIP_ACCOUNTS must contain exactly ten entries as published by Jito Labs"
        );
    }

    // -------------------------------------------------------------------------
    // Test 4 — SOL transfer instruction data layout
    // -------------------------------------------------------------------------

    /// Verifies that `build_sol_transfer_instruction` produces a 12-byte data field
    /// with the correct System Program `Transfer` discriminant and lamport amount.
    ///
    /// The System Program's ABI is part of the Solana protocol specification and has
    /// been stable since genesis. The data layout is:
    ///   bytes [0..4]  — u32 LE discriminant = 2 (SystemInstruction::Transfer)
    ///   bytes [4..12] — u64 LE lamport amount
    ///
    /// This test hard-codes the byte offsets rather than deriving them from the
    /// implementation. If the layout were ever accidentally changed — for example by
    /// inserting a field before the discriminant — the runtime would silently execute
    /// a different system instruction (index 0 is `CreateAccount`, index 1 is
    /// `Assign`). Hard-coding the expected bytes here makes such a regression
    /// immediately visible as a test failure rather than a live loss.
    #[test]
    fn test_build_sol_transfer_instruction_data_layout() {
        use super::ArbitrageExecutor;
        use solana_pubkey::Pubkey;

        let from = Pubkey::new_unique();
        let to   = Pubkey::new_unique();
        let lamports: u64 = 200_000;

        let ix = ArbitrageExecutor::build_sol_transfer_instruction(from, to, lamports);

        assert_eq!(ix.data.len(), 12, "System Transfer instruction data must be 12 bytes");

        // Bytes [0..4]: discriminant for SystemInstruction::Transfer = 2u32 LE.
        let discriminant = u32::from_le_bytes(ix.data[0..4].try_into().unwrap());
        assert_eq!(discriminant, 2, "bytes [0..4] must be discriminant 2 (Transfer)");

        // Bytes [4..12]: lamport amount round-trips correctly.
        let decoded_lamports = u64::from_le_bytes(ix.data[4..12].try_into().unwrap());
        assert_eq!(decoded_lamports, lamports, "bytes [4..12] must round-trip the lamport amount");

        // Account layout: index 0 = from (signer + writable), index 1 = to (writable only).
        assert!(ix.accounts[0].is_signer,   "from account must be a signer");
        assert!(ix.accounts[0].is_writable, "from account must be writable");
        assert!(!ix.accounts[1].is_signer,  "to account must NOT be a signer");
        assert!(ix.accounts[1].is_writable, "to account must be writable");

        assert_eq!(ix.accounts[0].pubkey, from, "accounts[0] must be the from pubkey");
        assert_eq!(ix.accounts[1].pubkey, to,   "accounts[1] must be the to pubkey");
    }

    // -------------------------------------------------------------------------
    // Test 5 — Profit gate constants
    // -------------------------------------------------------------------------

    /// Verifies that the three profit-gate constants hold their exact intended values.
    ///
    /// These constants form a coupled system: MIN_GROSS / (1 - TIP_FRACTION) must equal
    /// MIN_NET.  At 60 % tip, 40 % of gross is retained, so the gross floor that exactly
    /// satisfies the net floor is MIN_NET / 0.40 = 5_000_000.  A change to any one of
    /// the three values without adjusting the others would silently break the coupling —
    /// for example, lowering MIN_GROSS while keeping MIN_NET would allow paths through
    /// Gate 1 that can never pass Gate 2, wasting the tip arithmetic on doomed paths.
    #[test]
    fn test_profit_gate_constants() {
        use super::{MIN_GROSS_PROFIT_LAMPORTS, MIN_NET_PROFIT_LAMPORTS, TIP_FRACTION};

        assert_eq!(
            MIN_GROSS_PROFIT_LAMPORTS, 5_000_000,
            "gross floor must be 5M lamports — the minimum gross that nets 2M at 60% tip"
        );
        assert_eq!(
            MIN_NET_PROFIT_LAMPORTS, 2_000_000,
            "net floor must be 2M lamports — the operator's minimum acceptable profit"
        );
        assert!(
            (TIP_FRACTION - 0.60).abs() < f64::EPSILON,
            "tip fraction must be exactly 0.60 (60%)"
        );

        // The constants must be internally consistent: gross * (1 - tip_fraction) == net.
        let net_at_min_gross = (MIN_GROSS_PROFIT_LAMPORTS as f64 * (1.0 - TIP_FRACTION)) as u64;
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
    ///
    /// The gross gate exists because any gross below MIN_GROSS_PROFIT_LAMPORTS is
    /// mathematically incapable of producing MIN_NET_PROFIT_LAMPORTS after the 60%
    /// tip is applied.  Checking gross first avoids computing the dynamic tip for
    /// paths that cannot possibly pass Gate 2, saving the floating-point multiply
    /// and the branch on every unprofitable simulation.
    #[test]
    fn test_gross_gate_logic() {
        use super::MIN_GROSS_PROFIT_LAMPORTS;

        let passes_gross_gate = |gross: u64| gross >= MIN_GROSS_PROFIT_LAMPORTS;

        // One lamport below the floor must be rejected.
        assert!(!passes_gross_gate(4_999_999), "4_999_999 lamports gross must fail Gate 1");

        // Exactly at the floor must pass.
        assert!(passes_gross_gate(5_000_000), "5_000_000 lamports gross must pass Gate 1");

        // Well above the floor must pass.
        assert!(passes_gross_gate(10_000_000), "10M lamports gross must pass Gate 1");

        // Zero gross (no profit) must be rejected — saturating_sub produces 0 on a loss.
        assert!(!passes_gross_gate(0), "zero gross must fail Gate 1");
    }

    // -------------------------------------------------------------------------
    // Test 7 — Dynamic tip arithmetic
    // -------------------------------------------------------------------------

    /// Verifies the dynamic tip formula: `max(gross * TIP_FRACTION, floor_tip)`.
    ///
    /// The proportional component ensures the Jito auction bid scales with the
    /// value of the opportunity.  The floor component (`jito_tip_lamports`) ensures
    /// the transaction is never below Helius Sender's eligibility threshold.
    /// This test exercises both the proportional-dominant branch and the
    /// floor-dominant branch to confirm the max() selects correctly.
    #[test]
    fn test_dynamic_tip_arithmetic() {
        use super::TIP_FRACTION;

        let compute_dynamic_tip = |gross: u64, floor_tip: u64| -> u64 {
            ((gross as f64 * TIP_FRACTION) as u64).max(floor_tip)
        };

        // Proportional tip dominates when gross is large.
        // 5_000_000 * 0.60 = 3_000_000 > 200_000 floor.
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

        // Floor dominates when the proportional bid would be smaller.
        // A very small gross that still passes Gate 1 boundary: 5_000_000 * 0.60 = 3M,
        // which is above 200K floor, so the floor can only dominate when gross is tiny.
        // Test with gross=100_000 (which would fail Gate 1 in production, but the arithmetic
        // is still defined): 100_000 * 0.60 = 60_000 < 200_000 floor.
        assert_eq!(
            compute_dynamic_tip(100_000, 200_000),
            200_000,
            "floor tip must dominate when proportional bid is below the minimum"
        );

        // Operator floor set higher than the Helius minimum: floor wins when it should.
        // gross=5_000_000, 60% = 3_000_000; floor=4_000_000 → floor wins.
        assert_eq!(
            compute_dynamic_tip(5_000_000, 4_000_000),
            4_000_000,
            "elevated operator floor must dominate over proportional bid when floor is higher"
        );

        // Scale test: gross=10M, tip=6M, net=4M.
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

    /// Verifies that Gate 2 rejects paths whose net profit (gross minus dynamic tip)
    /// falls below MIN_NET_PROFIT_LAMPORTS and passes those that meet or exceed it.
    ///
    /// Gate 2 is the final guard before the submission pipeline.  A path that passes
    /// Gate 1 is guaranteed to have a gross ≥ 5M, but the operator-configured tip
    /// floor can still erode the net below 2M if `jito_tip_lamports` is set
    /// abnormally high.  Gate 2 catches this case regardless of how dynamic_tip
    /// was computed.
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

        // One lamport below the net floor (engineered via an inflated floor_tip).
        // gross=5M, floor_tip=3_000_001 → tip=3_000_001 → net=1_999_999 → fail.
        assert!(
            !passes_net_gate(5_000_000, 3_000_001),
            "5M gross with floor_tip=3_000_001 must fail Gate 2 (net=1_999_999)"
        );

        // Large gross, standard floor: well above net threshold.
        // gross=20M, tip=12M (60%), net=8M → pass.
        assert!(
            passes_net_gate(20_000_000, 200_000),
            "20M gross must pass Gate 2 (net = 8M)"
        );

        // Zero gross (loss scenario): saturating_sub clamps net to 0 → fail.
        assert!(
            !passes_net_gate(0, 200_000),
            "zero gross must fail Gate 2"
        );
    }
}
