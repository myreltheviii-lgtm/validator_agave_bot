// `solana_pubkey` is the disaggregated crate that owns the `Pubkey` type in agave 4.x.
// The old monolithic `solana_sdk::pubkey::Pubkey` path no longer resolves inside
// `solana-core` because `solana-sdk` is not listed as a dependency of this crate.
use solana_pubkey::Pubkey;
// FxHashSet uses the Fx (a.k.a. rustc-hash) non-cryptographic hash function, which
// is approximately 3–4× faster than the default SipHash1-3 for fixed-size keys like
// Pubkey ([u8; 32]). SipHash is designed to resist HashDoS from adversarially crafted
// keys; since all keys here come from on-chain account addresses that the validator
// itself generates or validates, there is no external adversary, and the overhead of
// SipHash buys nothing. FxHash is the right choice for every internal Pubkey-keyed
// collection in this codebase — the same rationale that makes FxHashMap correct in
// arbitrage_graph.rs applies here.
use rustc_hash::FxHashSet;
use std::sync::{Arc, RwLock};

/// Thread-safe set of account pubkeys that the MEV subsystem is watching.
///
/// `PoolTracker` wraps `Arc<RwLock<FxHashSet<Pubkey>>>`. Cloning a `PoolTracker`
/// produces a second handle to the same underlying set — not a copy of the set.
/// Multiple handles can co-exist safely across threads; all writes serialise
/// through the RwLock.
///
/// # Role in the integrated architecture
///
/// In the standalone bot, `PoolTracker` was the source of truth for which
/// accounts the Geyser plugin should forward to the MEV process. Now that the
/// MEV engine runs inside the validator, `MevEngine::account_to_mint` (a plain
/// `HashMap<Pubkey, Pubkey>`) performs the equivalent lookup at zero extra cost.
/// `PoolTracker` is retained as a lightweight utility type for any subsystem that
/// needs a shared, concurrently-accessible account set without carrying the full
/// engine context.
pub struct PoolTracker {
    tracked_accounts: Arc<RwLock<FxHashSet<Pubkey>>>,
}

impl PoolTracker {
    pub fn new() -> Self {
        Self {
            tracked_accounts: Arc::new(RwLock::new(FxHashSet::default())),
        }
    }

    pub fn add_account(&self, pubkey: Pubkey) {
        // A poisoned lock means another thread panicked while holding it. Panicking
        // here rather than silently swallowing the error makes the failure loud and
        // diagnosable; silent wrong behaviour in a MEV pipeline is worse than a crash.
        self.tracked_accounts
            .write()
            .expect("PoolTracker RwLock poisoned")
            .insert(pubkey);
    }

    pub fn add_accounts(&self, pubkeys: &[Pubkey]) {
        let mut tracked = self.tracked_accounts
            .write()
            .expect("PoolTracker RwLock poisoned");
        for pubkey in pubkeys {
            tracked.insert(*pubkey);
        }
    }

    pub fn remove_account(&self, pubkey: &Pubkey) {
        self.tracked_accounts
            .write()
            .expect("PoolTracker RwLock poisoned")
            .remove(pubkey);
    }

    pub fn remove_accounts(&self, pubkeys: &[Pubkey]) {
        let mut tracked = self.tracked_accounts
            .write()
            .expect("PoolTracker RwLock poisoned");
        for pubkey in pubkeys {
            tracked.remove(pubkey);
        }
    }

    pub fn contains(&self, pubkey: &Pubkey) -> bool {
        self.tracked_accounts
            .read()
            .expect("PoolTracker RwLock poisoned")
            .contains(pubkey)
    }

    pub fn len(&self) -> usize {
        self.tracked_accounts
            .read()
            .expect("PoolTracker RwLock poisoned")
            .len()
    }

    pub fn is_empty(&self) -> bool {
        // Calls FxHashSet::is_empty() directly under the lock rather than routing
        // through self.len() == 0. Both acquire the read lock once, but the direct
        // path avoids the extra method-call chain and the usize intermediate value.
        self.tracked_accounts
            .read()
            .expect("PoolTracker RwLock poisoned")
            .is_empty()
    }

    pub fn get_all_tracked(&self) -> Vec<Pubkey> {
        self.tracked_accounts
            .read()
            .expect("PoolTracker RwLock poisoned")
            .iter()
            .copied()
            .collect()
    }

    pub fn clear(&self) {
        self.tracked_accounts
            .write()
            .expect("PoolTracker RwLock poisoned")
            .clear();
    }
}

impl Clone for PoolTracker {
    fn clone(&self) -> Self {
        Self {
            tracked_accounts: Arc::clone(&self.tracked_accounts),
        }
    }
}
