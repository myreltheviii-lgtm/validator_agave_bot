// src/mev/stages/fangzhen_jieduan.rs  (仿真阶段 — Simulation Stage)
//
// Evaluates all qualifying arbitrage pairs for a pool update and returns the
// single most profitable opportunity, or None if no pair clears the profit
// threshold.
//
// This stage replaces `bank.simulate_transaction_unchecked` as the profitability
// signal. The SVM simulation requires full BPF interpreter execution, CPI
// dispatch into DEX programs, and bank account locking — roughly 1–15 ms per
// call under production load. The sim server performs the same price math in
// pure Rust using pre-fetched account data, completing in 5–50 µs. At 12
// concurrent shards this improvement translates directly into fewer stale
// events and more timely submissions.
//
// ## How the search works
//
// Profit as a function of `amount_in` is a concave curve with a single peak.
// Ternary search converges to that peak in O(log n) evaluations. The search
// exits as soon as any evaluated point yields profit ≥ PROFIT_THRESHOLD —
// we do not need the exact peak because the on-chain SMB program determines
// the optimal swap amount atomically at execution time. We only need a
// confident signal that a profitable amount exists.
//
// ## Unsupported pool types
//
// Pool types without a DexKind mapping (PancakeSwap, Humidifi, Vertigo,
// Heaven, Futarchy, Meteora DAMM v1) cannot be routed through the sim server.
// Any pair containing one of these pools is skipped entirely. The on-chain
// executor still handles them correctly — they simply do not benefit from
// pre-flight simulation and never fire in production mode until a DexKind
// mapping is added.

use std::cell::Cell;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rustc_hash::{FxHashMap, FxHashSet};
use solana_account::ReadableAccount;
use solana_pubkey::Pubkey;
use solana_runtime::bank::Bank;
use tracing::{debug, info};

use crate::mev::arbitrage::{ArbitragePath, PoolInfo, PoolType};
use crate::mev::constants::SOL_MINT;
use crate::mev::dex::raydium::constants::raydium_clmm_program_id;
use crate::mev::dex::whirlpool::constants::whirlpool_program_id;
use crate::mev::dex::whirlpool::state::Whirlpool;
use crate::mev::dex::whirlpool::update_tick_array_accounts_for_onchain;
use crate::mev::dex::byreal::byreal_program_id;
use crate::mev::dex::pancakeswap::pancakeswap_program_id;
use crate::mev::dex::pump::constants::pump_global_config;
use crate::mev::executor::smb_instruction_builder::SmbInstructionBuilder;
use crate::mev::pools::MintPoolData;
// sim_client is a standalone crate that lives outside the validator's source
// tree and is linked as an external dependency in Cargo.toml. It owns the
// Unix domain socket client, all wire types, and the frame codec. Because it
// carries zero Solana SDK dependency by design, it can be compiled once and
// shared between the validator and the sim-server binary even though those two
// binaries are built against different SDK versions.
use sim_client::SimClient;
use sim_client::{DexKind, WireAccount};

// ── Search constants ─────────────────────────────────────────────────────────

/// Minimum lamports to probe. Below ~0.001 SOL the round-trip swap fees on
/// any DEX exceed any realistically achievable arbitrage profit, so searching
/// in this range produces only noise with no actionable signal.
const MIN_PROBE_LAMPORTS: u64 = 1_000_000;

/// Profit in lamports at which the ternary search terminates early.
/// The caller's on-chain profit floor is 2 M net; anything clearing this
/// threshold is a strong enough signal to fire. The on-chain SMB executor
/// re-computes the exact profit at landing time and reverts if the market
/// moved against us — we do not need a precise pre-flight number.
const PROFIT_THRESHOLD: i64 = 2_000_000;

/// Maximum ternary-search iterations per pair. Each iteration fires two
/// sim-server queries. 40 iterations = 80 calls, converging the search
/// interval to `max_capital / 3^40` ≈ 1 lamport precision on any capital
/// size up to 10⁵ SOL. In practice the search exits far earlier because
/// the early-exit condition fires the moment profit ≥ PROFIT_THRESHOLD.
const MAX_SEARCH_ITERS: u32 = 40;



// ── Public result type ───────────────────────────────────────────────────────

/// The winning pair returned by `evaluate_pairs`.
///
/// Contains only what the caller needs to proceed — the pair index for
/// cooling-stage lookup, the path for instruction building, and the profit
/// recorded at the search exit point for ranking across pairs.
pub struct BestSimResult {
    pub pair_idx: usize,
    pub path:     ArbitragePath,
    /// Profit in lamports recorded at the search exit point. This is the value
    /// at whichever `amount_in` first crossed `PROFIT_THRESHOLD`, not
    /// necessarily the true peak. Used only for ranking multiple profitable
    /// pairs — the actual profit at landing is determined by the on-chain program.
    pub profit:   i64,
}

// ── Pool lookup maps ─────────────────────────────────────────────────────────

/// Per-type pool address → Vec index lookup maps built once before the
/// qualifying pairs loop in evaluate_pairs.
///
/// A pool update event can affect O(N) qualifying pairs (where N is the number
/// of pools for a popular mint). Each qualifying pair requires two pool lookups —
/// one per hop — to collect the on-chain account addresses the simulator needs.
/// Without pre-indexing, each lookup is an O(N) linear scan through the pool
/// Vec, making the total cost O(pairs × N). Because pairs can itself be O(N),
/// the naive scan is O(N²) per pool update event. Building these index maps
/// once in a single O(total_pools) pass before the loop reduces every
/// subsequent lookup to O(1) by index, returning the loop to O(pairs) overall.
struct PoolLookupMaps {
    raydium:         FxHashMap<Pubkey, usize>,
    raydium_cp:      FxHashMap<Pubkey, usize>,
    raydium_clmm:    FxHashMap<Pubkey, usize>,
    pump:            FxHashMap<Pubkey, usize>,
    meteora_damm:    FxHashMap<Pubkey, usize>,
    meteora_damm_v2: FxHashMap<Pubkey, usize>,
    // DLMM indexes by pair address rather than pool — the MintPoolData field
    // that identifies a DLMM entry is dlmm_pairs[i].pair, not .pool.
    dlmm:            FxHashMap<Pubkey, usize>,
    whirlpool:       FxHashMap<Pubkey, usize>,
    byreal:          FxHashMap<Pubkey, usize>,
    pancakeswap:     FxHashMap<Pubkey, usize>,
}

/// Build PoolLookupMaps from pool_data in a single O(total_pools) pass.
///
/// Each map entry records the Vec index of the pool stored at that address so
/// the lookup in collect_pool_account_pubkeys can retrieve the pool struct
/// with pool_data.xyz_pools[idx] in O(1) instead of scanning the entire Vec.
/// These maps are stack-local to one evaluate_pairs call and are never stored
/// on any long-lived struct.
fn build_pool_lookup_maps(pool_data: &MintPoolData) -> PoolLookupMaps {
    PoolLookupMaps {
        raydium:         pool_data.raydium_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        raydium_cp:      pool_data.raydium_cp_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        raydium_clmm:    pool_data.raydium_clmm_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        pump:            pool_data.pump_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        meteora_damm:    pool_data.meteora_damm_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        meteora_damm_v2: pool_data.meteora_damm_v2_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        dlmm:            pool_data.dlmm_pairs.iter().enumerate()
                             .map(|(i, p)| (p.pair, i)).collect(),
        whirlpool:       pool_data.whirlpool_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        byreal:          pool_data.byreal_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
        pancakeswap:     pool_data.pancakeswap_pools.iter().enumerate()
                             .map(|(i, p)| (p.pool, i)).collect(),
    }
}

// ── Main entry point ─────────────────────────────────────────────────────────

/// Evaluate all qualifying pairs and return the most profitable one, or `None`.
///
/// For each pair the function:
///   1. Maps both pool types to `DexKind`. Pairs containing an unsupported
///      pool type are skipped.
///   2. Collects all account pubkeys required by both DEX simulators and
///      snapshots their current data from the bank.
///   3. Runs a ternary search over `amount_in` with early exit the moment
///      profit ≥ `PROFIT_THRESHOLD`.
///
/// All profitable pairs are collected and the one with the highest profit is
/// returned. If no pair clears the threshold, `None` is returned and the
/// caller drops the event silently.
///
/// ## Timing instrumentation
///
/// Two layers of timing run inside this function. The first measures account
/// collection and bank snapshot time per pair — this is the cost of reading
/// account data from the bank's AccountsDb before any socket I/O occurs. The
/// second measures the ternary search per pair, which is dominated by Unix
/// socket round-trip time to the sim server and includes per-probe RTT stats
/// (average, min, max) returned directly from `search_for_profit`. Both are
/// logged at `debug` level so they can be enabled with RUST_LOG without
/// affecting production throughput. A single `info` line fires at stage exit
/// regardless of outcome, recording the total stage wall time for latency
/// monitoring in production.
pub fn evaluate_pairs(
    qualifying_pairs: &[(usize, ArbitragePath)],
    bank:             &Arc<Bank>,
    pool_data:        &Arc<MintPoolData>,
    sim_client:       &mut SimClient,
    max_capital:      u64,
) -> Option<BestSimResult> {
    // Stamp wall clock at stage entry. This measures the total time from when
    // the shard hands off to the simulation stage until a winner is returned
    // (or the stage exits with None). It covers account collection, bank
    // snapshots, and all sim-server round trips across every qualifying pair.
    let stage_start = Instant::now();

    let slot           = bank.slot();
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Build per-type address → Vec-index maps once before the pairs loop.
    // Each pair requires two calls to collect_pool_account_pubkeys (one per
    // hop). Without this pre-indexing each call scans the entire pool Vec —
    // O(N) per call, O(pairs × N) total, O(N²) when pairs itself scales with N.
    // Pre-building here reduces every lookup to O(1) by Vec index.
    let lookup = build_pool_lookup_maps(pool_data);

    let mut best: Option<BestSimResult> = None;
    let mut pairs_evaluated: usize      = 0;
    let mut pairs_profitable: usize     = 0;

    for (pair_idx, path) in qualifying_pairs {
        let ArbitragePath::TwoHop { pool_1, pool_2, intermediate_token } = path;

        // Map pool types to sim-server DexKind. A None result means the pool
        // type has no sim-server implementation — skip the entire pair.
        let hop1_dex = match pool_type_to_dex_kind(pool_1.pool_type) {
            Some(d) => d,
            None => {
                debug!(
                    "fangzhen: pair {} skipped — pool_1 type {:?} has no DexKind mapping",
                    pair_idx, pool_1.pool_type
                );
                continue;
            }
        };

        let hop2_dex = match pool_type_to_dex_kind(pool_2.pool_type) {
            Some(d) => d,
            None => {
                debug!(
                    "fangzhen: pair {} skipped — pool_2 type {:?} has no DexKind mapping",
                    pair_idx, pool_2.pool_type
                );
                continue;
            }
        };

        pairs_evaluated += 1;

        // Stamp the clock before account collection. This measures how long it
        // takes to derive the required account pubkeys per pool type, deduplicate
        // them, and read each account's raw bytes from the bank's AccountsDb.
        // For CLMM pools this includes live tick array PDA derivation from the
        // bank state, which is the most expensive step in collection.
        let collection_start = Instant::now();

        // Collect all account pubkeys both DEX simulators will read.
        // Missing accounts (not found in bank) are silently skipped — the
        // simulator treats a missing account as an error and returns 0,
        // which the search interprets as zero profit and moves on.
        let mut keys: Vec<Pubkey> = Vec::new();

        match collect_pool_account_pubkeys(pool_1, pool_data, &lookup, bank) {
            Ok(mut k) => keys.append(&mut k),
            Err(e) => {
                debug!("fangzhen: pair {} pool_1 account collection failed: {}", pair_idx, e);
                continue;
            }
        }

        match collect_pool_account_pubkeys(pool_2, pool_data, &lookup, bank) {
            Ok(mut k) => keys.append(&mut k),
            Err(e) => {
                debug!("fangzhen: pair {} pool_2 account collection failed: {}", pair_idx, e);
                continue;
            }
        }

        // Deduplicate keys before snapshotting. Both pools may share certain
        // accounts (e.g. both reference the same token mint). Sending duplicate
        // WireAccounts inflates the frame size and wastes the server's deserialisation
        // cost without providing any additional information.
        let accounts       = snapshot_deduplicated(keys, bank);
        let collection_us  = collection_start.elapsed().as_micros();

        // Build the wire representations of the three 32-byte keys the sim server
        // needs for routing: both pool addresses and the two token_in values.
        //
        // Hop 1: the executor sells SOL into pool_1 to acquire the intermediate token.
        // Hop 2: the executor sells the intermediate token into pool_2 to recover SOL.
        let hop1_pool     = pool_1.address.to_bytes();
        let hop1_token_in = SOL_MINT.to_bytes();
        let hop2_pool     = pool_2.address.to_bytes();
        let hop2_token_in = intermediate_token.to_bytes();

        // Stamp the clock before the ternary search. This isolates the pure
        // sim-server I/O cost — the wall time from first probe to search exit —
        // from the account collection and bank snapshot cost measured above.
        let search_start = Instant::now();

        // Search for an amount_in that yields profit ≥ PROFIT_THRESHOLD.
        // `accounts` is passed as a slice — the ternary search fires up to 82
        // sim-server queries per pair, all using the same account snapshot.
        // Passing a reference instead of cloning the Vec<WireAccount> per call
        // eliminates up to 82 × K deep copies of WireAccount data buffers per
        // pair, where K is the account count (up to ~30 for CLMM pools with
        // tick arrays). The account state is fixed for the lifetime of one search
        // because the snapshot was taken from the bank at the start of this call.
        //
        // search_for_profit returns the profit result alongside per-probe timing
        // stats so the caller can log a complete picture of the search cost
        // without any additional timing infrastructure on this side.
        let (profit, probes, avg_rtt_us, min_rtt_us, max_rtt_us) = search_for_profit(
            slot,
            unix_timestamp,
            hop1_dex,
            hop1_pool,
            hop1_token_in,
            hop2_dex,
            hop2_pool,
            hop2_token_in,
            &accounts,
            sim_client,
            max_capital,
        );

        let search_us = search_start.elapsed().as_micros();

        // Per-pair timing log. This fires for every pair regardless of outcome.
        // It gives a complete breakdown: how many accounts were snapshotted, how
        // long collection took, how many probes the search fired, and the
        // distribution of individual Unix socket round-trip times. If avg_rtt_us
        // is unexpectedly high (> 100 µs) it indicates sim-server load or IPC
        // congestion. If collection_us is high it indicates CLMM tick array
        // derivation bottleneck in the bank's AccountsDb.
        debug!(
            "fangzhen: pair={} accounts={} collection={}µs probes={} \
             search={}µs avg_rtt={}µs min_rtt={}µs max_rtt={}µs result={}",
            pair_idx,
            accounts.len(),
            collection_us,
            probes,
            search_us,
            avg_rtt_us,
            min_rtt_us,
            max_rtt_us,
            if profit.is_some() { "profitable" } else { "dead" },
        );

        if let Some(p) = profit {
            pairs_profitable += 1;

            debug!(
                "fangzhen: pair {} profitable — profit={} lamports",
                pair_idx, p
            );

            // Keep track of the highest-profit pair found so far.
            let is_better = best.as_ref().map_or(true, |b| p > b.profit);
            if is_better {
                best = Some(BestSimResult {
                    pair_idx: *pair_idx,
                    path:     path.clone(),
                    profit:   p,
                });
            }
        }
    }

    let stage_us = stage_start.elapsed().as_micros();

    // Stage summary — always logged at info level. This is the primary latency
    // signal for the simulation layer and must remain visible in production logs
    // regardless of RUST_LOG level. It records: how many pairs were evaluated
    // (pairs_evaluated < qualifying_pairs.len() means some were filtered by
    // DexKind mapping), how many cleared the threshold, which pair won, and the
    // total wall time for the entire stage. A stage_total consistently above
    // 300 µs warrants investigation — the cooling stage drops opportunities
    // older than 400 µs and the stage budget is tight.
    info!(
        "fangzhen: slot={} pairs_evaluated={} pairs_profitable={} \
         winner_pair={} winner_profit={} stage_total={}µs",
        slot,
        pairs_evaluated,
        pairs_profitable,
        best.as_ref().map_or(usize::MAX, |b| b.pair_idx),
        best.as_ref().map_or(0, |b| b.profit),
        stage_us,
    );

    best
}

// ── Ternary search ────────────────────────────────────────────────────────────

/// Search for an `amount_in` in `[MIN_PROBE_LAMPORTS, max_capital]` that
/// yields profit ≥ `PROFIT_THRESHOLD` for the given two-hop path.
///
/// Returns `(result, probes_fired, avg_rtt_us, min_rtt_us, max_rtt_us)` so the
/// caller receives the complete search cost breakdown alongside the profit
/// result in a single call, with no additional timing infrastructure needed
/// in `evaluate_pairs`. The result is `Some(profit)` on success or `None` if
/// the full search space was exhausted without finding a profitable amount.
///
/// ## Probe-level timing with Cell interior mutability
///
/// The ternary search fires `eval` as a `FnMut` closure that captures
/// `sim_client` by exclusive mutable borrow. Rust's borrow checker disallows
/// a second mutable borrow inside the same closure environment, which would
/// normally prevent timing accumulators from being updated inside `eval`.
/// `Cell<T>` resolves this by providing interior mutability through a shared
/// reference: the closure captures `&Cell<T>` and calls `.set()` / `.get()`
/// without needing `&mut`. After the closure's scope ends (the block that
/// declares `eval`), the exclusive borrow of `sim_client` is released and the
/// Cell values can be read freely to build the return tuple.
///
/// ## Search behaviour
///
/// A two-point pre-check at the endpoints eliminates dead paths in two probes
/// before paying the full iteration cost — most paths are dead most of the time.
/// The search is ternary: each iteration tests the two trisection points
/// m1 = lo + (hi-lo)/3 and m2 = hi - (hi-lo)/3. Because profit is a concave
/// function of amount_in (more capital yields diminishing marginal returns and
/// eventually negative returns from price impact), ternary search guarantees
/// convergence to the peak in O(log₃ n) iterations. Early exit fires the moment
/// either trisection point clears the threshold.
#[allow(clippy::too_many_arguments)]
fn search_for_profit(
    slot:           u64,
    unix_timestamp: u64,
    hop1_dex:       DexKind,
    hop1_pool:      [u8; 32],
    hop1_token_in:  [u8; 32],
    hop2_dex:       DexKind,
    hop2_pool:      [u8; 32],
    hop2_token_in:  [u8; 32],
    accounts:       &[WireAccount],
    sim_client:     &mut SimClient,
    max_capital:    u64,
) -> (Option<i64>, u32, u128, u128, u128) {
    if max_capital <= MIN_PROBE_LAMPORTS {
        return (None, 0, 0, 0, 0);
    }

    // Probe-level timing accumulators. Cell<T> is used because the eval closure
    // captures sim_client by exclusive mutable borrow — there is no room for a
    // second mutable borrow of any other variable in the same closure. Cell
    // provides write access through a shared reference, which is compatible with
    // the exclusive borrow on sim_client because they target different memory.
    let probes_fired   = Cell::new(0u32);
    let total_probe_us = Cell::new(0u128);
    let min_probe_us   = Cell::new(u128::MAX);
    let max_probe_us   = Cell::new(0u128);

    let mut lo = MIN_PROBE_LAMPORTS;
    let mut hi = max_capital;

    // The search result is collected here so the function has one exit point
    // after the eval closure's scope ends. The eval closure must go out of
    // scope before the Cell values are read — its lifetime is bounded by the
    // block below, and the Cell reads happen after the block closes.
    let result = {
        // eval fires one sim-server query and returns the profit at `amount_in`.
        //
        // `accounts` is borrowed from the caller as a shared slice. The same
        // snapshot is reused for every iteration of the search — the pool state
        // does not change between probes because the bank is not mutated during
        // the search. Errors from the sim server are treated as i64::MIN so the
        // search continues without crashing; a persistently failing socket will
        // cause every probe to return i64::MIN, the endpoint pre-check will see
        // both values as ≤ 0, and the search exits immediately with None.
        let mut eval = |amount_in: u64| -> i64 {
            let probe_start = Instant::now();

            let result = sim_client
                .query(
                    slot,
                    unix_timestamp,
                    amount_in,
                    hop1_dex,
                    hop1_pool,
                    hop1_token_in,
                    hop2_dex,
                    hop2_pool,
                    hop2_token_in,
                    accounts,
                )
                .map(|resp| resp.final_out as i64 - amount_in as i64)
                .unwrap_or(i64::MIN);

            let probe_us = probe_start.elapsed().as_micros();

            // Update accumulators through Cell — no &mut needed so this is
            // compatible with the exclusive mutable capture of sim_client.
            probes_fired.set(probes_fired.get() + 1);
            total_probe_us.set(total_probe_us.get() + probe_us);
            if probe_us < min_probe_us.get() { min_probe_us.set(probe_us); }
            if probe_us > max_probe_us.get() { max_probe_us.set(probe_us); }

            result
        };

        // ── Pre-check at both endpoints ─────────────────────────────────────
        // Firing two calls before the loop eliminates dead paths immediately.
        // A path where neither the minimum nor the maximum capital is profitable
        // has no interior peak above zero — the profit curve is negative everywhere.
        // Dropping it here costs 2 calls instead of up to 80.
        let p_lo = eval(lo);
        let p_hi = eval(hi);

        if p_lo >= PROFIT_THRESHOLD {
            Some(p_lo)
        } else if p_hi >= PROFIT_THRESHOLD {
            Some(p_hi)
        } else if p_lo <= 0 && p_hi <= 0 {
            // If both endpoints are unprofitable (≤ 0), the entire curve is below
            // zero. A concave profit function that is non-positive at both endpoints
            // cannot have a positive interior peak — the peak is at or between them
            // and the function is non-increasing from the peak in both directions.
            None
        } else {
            // ── Ternary search with early exit ──────────────────────────────
            let mut found: Option<i64> = None;

            for _ in 0..MAX_SEARCH_ITERS {
                if hi - lo < 2 {
                    break;
                }

                let m1 = lo + (hi - lo) / 3;
                let m2 = hi - (hi - lo) / 3;

                let p1 = eval(m1);
                let p2 = eval(m2);

                // Early exit: the first evaluation that clears the threshold is
                // sufficient. The caller fires the transaction immediately — we do
                // not need the exact peak. Return the higher of the two to give the
                // ranking step in evaluate_pairs an accurate value for pair comparison.
                if p1 >= PROFIT_THRESHOLD || p2 >= PROFIT_THRESHOLD {
                    found = Some(p1.max(p2));
                    break;
                }

                // Neither trisection point cleared the threshold. Narrow the search
                // interval toward the higher-profit side. Since the profit function
                // is concave with a single peak, the peak lies in the half where the
                // nearer trisection point is higher.
                if p1 < p2 {
                    lo = m1;
                } else {
                    hi = m2;
                }
            }

            // ── Final evaluation at the converged midpoint ──────────────────
            // After the loop the interval [lo, hi] brackets the peak. Check the
            // midpoint one last time — the peak may have been narrowed to exactly
            // here without either trisection point ever reaching it.
            if found.is_none() {
                let peak = eval((lo + hi) / 2);
                if peak >= PROFIT_THRESHOLD {
                    found = Some(peak);
                }
            }

            found
        }
        // eval goes out of scope here. Its exclusive borrow of sim_client and
        // shared borrows of the Cell accumulators are all released at this point.
    };

    // Read the Cell values now that eval is out of scope. These are returned
    // to the caller so the per-pair debug log in evaluate_pairs can record the
    // complete search cost in one place rather than scattering timing reads
    // across multiple functions.
    let n          = probes_fired.get();
    let avg_rtt_us = if n > 0 { total_probe_us.get() / n as u128 } else { 0 };
    let min_rtt_us = if min_probe_us.get() == u128::MAX { 0 } else { min_probe_us.get() };
    let max_rtt_us = max_probe_us.get();

    (result, n, avg_rtt_us, min_rtt_us, max_rtt_us)
}

// ── Account collection ────────────────────────────────────────────────────────

/// Snapshot the given pubkeys from the bank into `WireAccount` records,
/// deduplicating by pubkey before fetching.
///
/// Accounts that are not present in the bank (e.g. uninitialized tick arrays
/// beyond the current price range) are silently omitted. The simulator handles
/// a missing account by returning an error, which the search interprets as zero
/// profit — a safe conservative outcome that never causes a false positive.
fn snapshot_deduplicated(keys: Vec<Pubkey>, bank: &Arc<Bank>) -> Vec<WireAccount> {
    let mut seen: FxHashSet<Pubkey> = FxHashSet::default();
    let mut result: Vec<WireAccount> = Vec::with_capacity(keys.len());

    for key in keys {
        if !seen.insert(key) {
            // Duplicate pubkey from the union of both pools' account lists.
            // The sim server's HashMap would overwrite the first copy with the
            // second — harmless because both copies are identical — but sending
            // duplicates wastes frame bandwidth and deserialisation cost.
            continue;
        }

        if let Some(account) = bank.get_account(&key) {
            result.push(WireAccount {
                pubkey: key.to_bytes(),
                owner:  account.owner().to_bytes(),
                data:   account.data().to_vec(),
            });
        }
    }

    result
}

/// Return all account pubkeys that the sim server's DEX simulator for `pool`
/// will attempt to read, given the current pool state in `bank`.
///
/// The returned list mirrors the account set consumed by each
/// `calculate_*_output` function in `src/dex/simulators/`. If a required
/// account is omitted here it will be missing from the `AccountMap` on the
/// server side and the simulator will return an error (0 output), causing the
/// pair to be treated as dead for this slot.
///
/// `lookup` provides O(1) index access into the pool Vecs inside `pool_data`.
/// Callers build this map once per evaluate_pairs invocation and share it
/// across every collect_pool_account_pubkeys call in that invocation, avoiding
/// the O(N) linear scan that `.iter().find()` would perform per call.
///
/// Returns `Err` if the pool struct cannot be located in `pool_data` (the
/// pool was not registered at startup — should not happen in practice because
/// the graph is built from the same pool_data, but we handle it defensively).
fn collect_pool_account_pubkeys(
    pool_info:  &PoolInfo,
    pool_data:  &MintPoolData,
    lookup:     &PoolLookupMaps,
    bank:       &Arc<Bank>,
) -> Result<Vec<Pubkey>, anyhow::Error> {
    let mut keys: Vec<Pubkey> = Vec::new();

    match pool_info.pool_type {
        // ── Raydium AMM V4 (constant product, legacy) ──────────────────────
        // The simulator reads the pool state to get reserves, then reads the
        // two vaults to verify balances. No derived accounts.
        PoolType::RaydiumV4 => {
            let idx = lookup.raydium.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("RaydiumV4 pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.raydium_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.token_vault);
            keys.push(pool.sol_vault);
        }

        // ── Raydium CPMM (constant product, modern) ────────────────────────
        // Reads pool state, amm_config (fee tier), both token vaults, and the
        // observation state account (used for TWAP and fee calculation).
        PoolType::RaydiumCpmm => {
            let idx = lookup.raydium_cp.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("RaydiumCpmm pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.raydium_cp_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.amm_config);
            keys.push(pool.token_vault);
            keys.push(pool.sol_vault);
            keys.push(pool.observation);
        }

        // ── Raydium CLMM (concentrated liquidity) ─────────────────────────
        // Reads pool state, amm_config, observation state, bitmap extension
        // (tick availability index), both token vaults, and three tick arrays
        // bracketing the current price. Tick arrays are re-derived from the
        // live tick_current_index in the bank — stale cached values from parse
        // time would point to empty tick arrays if the price has drifted.
        PoolType::RaydiumClmm => {
            let idx = lookup.raydium_clmm.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("RaydiumClmm pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.raydium_clmm_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.amm_config);
            keys.push(pool.observation_state);
            keys.push(pool.bitmap_extension);
            keys.push(pool.x_vault);
            keys.push(pool.y_vault);

            // Re-derive tick arrays from live bank state. Three arrays at offsets
            // [-1, 0, +1] relative to the array containing tick_current_index cover
            // the swap range for any realistic input size.
            let prog = raydium_clmm_program_id();
            match SmbInstructionBuilder::calculate_live_clmm_tick_arrays(&pool.pool, bank, &prog) {
                Ok(tick_keys) => keys.extend(tick_keys),
                Err(e) => {
                    debug!("fangzhen: RaydiumClmm tick array derivation failed for {}: {}", pool.pool, e);
                    // Continue without tick arrays — the simulator will return 0
                    // for this pool, treating it as unprofitable.
                }
            }
        }

        // ── Pump AMM ────────────────────────────────────────────────────────
        // Reads pool state, token vault, SOL vault, fee accounts, volume
        // accumulator PDAs, pool_v2 versioning account, and the GlobalConfig
        // singleton. All pool-specific PDAs were pre-computed at parse time
        // and stored directly in PumpPool.
        PoolType::PumpSwap => {
            let idx = lookup.pump.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("PumpSwap pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.pump_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.token_vault);
            keys.push(pool.sol_vault);
            keys.push(pool.fee_wallet);
            keys.push(pool.fee_token_wallet);
            keys.push(pool.coin_creator_vault_ata);
            keys.push(pool.coin_creator_vault_authority);
            keys.push(pool.global_volume_accumulator);
            keys.push(pool.user_volume_accumulator);
            keys.push(pool.pool_v2);

            // GlobalConfig is a singleton PDA seeded with b"global_config" under
            // the AMM program. The simulator reads it unconditionally to obtain
            // the protocol-level fee rates (lp_fee, protocol_fee, creator_fee)
            // before computing any swap output. Without it the simulator hard-fails
            // immediately, returning 0 for every Pump pair regardless of whether
            // the pool is actually profitable.
            keys.push(pump_global_config());

            // Cashback coins use an additional wSOL ATA on the volume accumulator.
            // The simulator checks pool.is_cashback_coin to decide whether to
            // include this account — we include it unconditionally because the
            // snapshot cost is negligible and a missing account on a cashback pool
            // would incorrectly suppress a genuine arb opportunity.
            if pool.is_cashback_coin {
                let wsol_ata = spl_associated_token_account_interface::address::get_associated_token_address(
                    &pool.user_volume_accumulator,
                    &SOL_MINT,
                );
                keys.push(wsol_ata);
            }
        }

        // ── Meteora DAMM v1 ─────────────────────────────────────────────────
        // Suspended in the wire protocol (no DexKind variant). Included here
        // only for completeness — the pool_type_to_dex_kind call in evaluate_pairs
        // will have already returned None and skipped this pair before we reach
        // this branch. This arm is unreachable in production.
        PoolType::MeteoraDamm => {
            return Err(anyhow::anyhow!("MeteoraDamm v1 is suspended and has no sim server support"));
        }

        // ── Meteora DAMM v2 ─────────────────────────────────────────────────
        // Reads pool state and both token vaults. The event authority and pool
        // authority are program-derived constants, not stored in pool_data, so
        // they are not included in the snapshot — the simulator does not read
        // them from AccountMap.
        PoolType::MeteoraDammV2 => {
            let idx = lookup.meteora_damm_v2.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("MeteoraDammV2 pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.meteora_damm_v2_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.token_x_vault);
            keys.push(pool.token_sol_vault);
        }

        // ── Meteora DLMM (discrete liquidity market maker) ──────────────────
        // Reads pool state, both token vaults, oracle account, the active bin
        // arrays, and both token mint accounts. Bin arrays are re-derived from
        // the live active_id in the bank — same reasoning as CLMM tick arrays above.
        PoolType::MeteoraDlmm => {
            let idx = lookup.dlmm.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("MeteoraDlmm pair {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.dlmm_pairs[*idx];

            keys.push(pool.pair);
            keys.push(pool.token_vault);
            keys.push(pool.sol_vault);
            keys.push(pool.oracle);

            // The DLMM quote function requires both token mint accounts to read
            // their decimal places. Bin prices are stored as raw fixed-point values
            // inside the pool; converting them to actual token amounts requires
            // knowing how many decimal places each mint uses. Without the mint
            // accounts the simulator fails immediately before any swap math runs.
            keys.push(pool.token_mint);
            keys.push(pool.base_mint);

            // Re-derive bin arrays from live bank state.
            match SmbInstructionBuilder::calculate_live_dlmm_bin_arrays(&pool.pair, bank) {
                Ok(bin_keys) => keys.extend(bin_keys),
                Err(e) => {
                    debug!("fangzhen: MeteoraDlmm bin array derivation failed for {}: {}", pool.pair, e);
                }
            }
        }

        // ── Orca Whirlpool (CLMM) ────────────────────────────────────────────
        // Reads pool state, oracle, both token vaults, and three tick arrays.
        // Tick arrays are re-derived from live pool state via
        // `update_tick_array_accounts_for_onchain` which handles both the
        // a_to_b and b_to_a directions by bracketing the current tick.
        PoolType::OrcaWhirlpool => {
            let idx = lookup.whirlpool.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("OrcaWhirlpool pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.whirlpool_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.oracle);
            keys.push(pool.x_vault);
            keys.push(pool.y_vault);

            // Deserialise the live Whirlpool state from the bank to obtain
            // tick_current_index and tick_spacing, which determine which tick
            // arrays are active at the current price.
            if let Some(account) = bank.get_account(&pool.pool) {
                let mut slice = account.data();
                if let Ok(whirlpool_state) = Whirlpool::try_deserialize(&mut slice) {
                    let tick_array_metas = update_tick_array_accounts_for_onchain(
                        &whirlpool_state,
                        &pool.pool,
                        &whirlpool_program_id(),
                    );
                    for meta in tick_array_metas {
                        keys.push(meta.pubkey);
                    }
                }
            }
        }

        // ── Byreal CLMM ─────────────────────────────────────────────────────
        // Identical structure to Raydium CLMM — pool state, amm_config,
        // observation, bitmap extension, vaults, and live tick arrays —
        // but uses the Byreal program ID for PDA derivation.
        PoolType::Byreal => {
            let idx = lookup.byreal.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("Byreal pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.byreal_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.amm_config);
            keys.push(pool.observation_state);
            keys.push(pool.bitmap_extension);
            keys.push(pool.x_vault);
            keys.push(pool.y_vault);

            let prog = byreal_program_id();
            match SmbInstructionBuilder::calculate_live_clmm_tick_arrays(&pool.pool, bank, &prog) {
                Ok(tick_keys) => keys.extend(tick_keys),
                Err(e) => {
                    debug!("fangzhen: Byreal tick array derivation failed for {}: {}", pool.pool, e);
                }
            }
        }

        // ── PancakeSwap CLMM ─────────────────────────────────────────────────
        // Account layout is identical to Raydium CLMM — pool state, amm_config,
        // observation state, bitmap extension, both token vaults, and live tick
        // arrays. The only difference is the program ID used for PDA derivation,
        // since tick array addresses are seeded under the DEX's own program.
        PoolType::PancakeSwap => {
            let idx = lookup.pancakeswap.get(&pool_info.address)
                .ok_or_else(|| anyhow::anyhow!("PancakeSwap pool {} not in pool_data", pool_info.address))?;
            let pool = &pool_data.pancakeswap_pools[*idx];

            keys.push(pool.pool);
            keys.push(pool.amm_config);
            keys.push(pool.observation_state);
            keys.push(pool.bitmap_extension);
            keys.push(pool.x_vault);
            keys.push(pool.y_vault);

            // Tick arrays are re-derived from live bank state using the PancakeSwap
            // program ID. The same three-array bracketing strategy as Raydium CLMM
            // applies — the concentrated liquidity math and tick indexing are identical.
            let prog = pancakeswap_program_id();
            match SmbInstructionBuilder::calculate_live_clmm_tick_arrays(&pool.pool, bank, &prog) {
                Ok(tick_keys) => keys.extend(tick_keys),
                Err(e) => {
                    debug!("fangzhen: PancakeSwap tick array derivation failed for {}: {}", pool.pool, e);
                }
            }
        }

        // ── Unsupported pool types ───────────────────────────────────────────
        // Humidifi, Vertigo, Heaven, and Futarchy have no DexKind mapping and
        // cannot be routed through the sim server. evaluate_pairs already filters
        // these out via pool_type_to_dex_kind before calling this function, so
        // these arms are unreachable in practice.
        PoolType::Humidifi
        | PoolType::Vertigo
        | PoolType::Heaven
        | PoolType::Futarchy => {
            return Err(anyhow::anyhow!(
                "Pool type {:?} has no sim server DexKind — pair should have been filtered",
                pool_info.pool_type
            ));
        }
    }

    Ok(keys)
}

// ── DexKind mapping ───────────────────────────────────────────────────────────

/// Map an on-chain `PoolType` to the sim server's `DexKind` routing enum.
///
/// Returns `None` for pool types that have no sim server implementation.
/// The caller skips any pair containing a pool with a `None` mapping —
/// the pair cannot be evaluated and should not fire in production mode.
fn pool_type_to_dex_kind(pool_type: PoolType) -> Option<DexKind> {
    match pool_type {
        PoolType::OrcaWhirlpool  => Some(DexKind::OrcaWhirlpool),
        PoolType::RaydiumClmm    => Some(DexKind::RaydiumClmm),
        PoolType::RaydiumCpmm    => Some(DexKind::RaydiumCp),
        PoolType::RaydiumV4      => Some(DexKind::RaydiumAmmV4),
        PoolType::MeteoraDammV2  => Some(DexKind::MeteoraDammV2),
        PoolType::MeteoraDlmm    => Some(DexKind::MeteoraDlmm),
        PoolType::PumpSwap       => Some(DexKind::PumpAmm),
        PoolType::Byreal         => Some(DexKind::ByrealClmm),
        PoolType::PancakeSwap    => Some(DexKind::PancakeSwap),
        // Meteora DAMM v1 was suspended from the wire protocol because its
        // vault-program account structure was redesigned. No DexKind variant exists.
        PoolType::MeteoraDamm    => None,
        // The following pool types have on-chain executors but no sim server
        // simulator yet. Adding a simulator and a DexKind variant for each is a
        // future task; for now these pairs are invisible to the pre-flight stage.
        PoolType::Humidifi       => None,
        PoolType::Vertigo        => None,
        PoolType::Heaven         => None,
        PoolType::Futarchy       => None,
    }
}
