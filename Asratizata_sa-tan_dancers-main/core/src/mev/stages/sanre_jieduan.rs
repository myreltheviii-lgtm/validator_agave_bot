// src/mev/stages/sanre_jieduan.rs  (冷却阶段 — Cooling Stage)
//
// Prevents duplicate transaction submissions for the same arbitrage pair
// within a single Solana slot window.
//
// The MEV engine processes committed transaction batches in microseconds.
// A single pool update can trigger the same arbitrage opportunity dozens of
// times before the first submitted transaction lands on-chain — the validator
// sees each write to the pool account as a fresh event. Without rate-limiting,
// the shard would fire duplicate transactions for the same pair, all chasing
// the same arb, burning priority fees and consuming slot capacity on trades
// that the first submission already claimed.
//
// The cooling stage solves this by recording the last fire time per pair and
// blocking any re-submission within one slot window (400 ms). After the window
// expires the pair is "cool" again and eligible for a new submission if a fresh
// opportunity surfaces on a subsequent pool update.
//
// The map is bounded in size by the total number of arbitrage pairs registered
// across all mints on this shard. Pairs are created at startup and at pool
// graduation events — both are rare relative to the hot path. No explicit
// eviction is needed because stale entries are never visited again once their
// window expires; the elapsed-time check at access time is the only gate.

use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

/// One Solana slot is produced every 400 ms under normal network conditions.
/// Firing more than once per slot per pair serves no purpose: the first
/// transaction is still in flight (or already landed and resolved) for the
/// entire duration of the slot. Any re-fire within this window races the
/// first submission for the same arb profit, wasting the priority fee.
const COOLDOWN_DURATION: Duration = Duration::from_millis(400);

pub struct CoolingStage {
    /// Maps pair index → the wall-clock instant at which a transaction was
    /// last submitted for that pair. Pair indices are stable for the lifetime
    /// of the shard — they come from the ArbitrageGraph and never change once
    /// assigned, so they are safe to use as map keys without versioning.
    last_fired: FxHashMap<usize, Instant>,
}

impl CoolingStage {
    pub fn new() -> Self {
        Self {
            last_fired: FxHashMap::default(),
        }
    }

    /// Returns `true` and records the current instant if the pair is cool,
    /// meaning no submission has been fired for it within `COOLDOWN_DURATION`.
    ///
    /// Returns `false` if the pair is hot (fired within the last slot window).
    /// The caller must drop the opportunity when this returns `false`.
    ///
    /// The map entry is written only when `true` is returned, so a `false`
    /// return does not reset the cooldown clock — the window is measured from
    /// the original submission, not from the most recent suppressed event.
    pub fn check_and_mark(&mut self, pair_idx: usize) -> bool {
        let now = Instant::now();

        if let Some(&last) = self.last_fired.get(&pair_idx) {
            if now.duration_since(last) < COOLDOWN_DURATION {
                // Still within one slot window of the previous submission.
                // Suppress this event to avoid duplicate transactions.
                return false;
            }
        }

        // Pair is cool — record this instant as the new last-fired time
        // and allow the submission to proceed.
        self.last_fired.insert(pair_idx, now);
        true
    }
}
