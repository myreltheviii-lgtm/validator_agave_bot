// `solana_pubkey` is the disaggregated crate that owns the `Pubkey` type in agave 4.x.
// The old monolithic `solana_sdk::pubkey::Pubkey` path no longer resolves inside
// `solana-core` because `solana-sdk` is not listed as a dependency of this crate.
use solana_pubkey::Pubkey;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

/// Thread-safe set of account pubkeys that the MEV subsystem is watching.
///
/// `PoolTracker` wraps `Arc<RwLock<HashSet<Pubkey>>>`. Cloning a `PoolTracker`
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
    tracked_accounts: Arc<RwLock<HashSet<Pubkey>>>,
}

impl PoolTracker {
    pub fn new() -> Self {
        Self {
            tracked_accounts: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn add_account(&self, pubkey: Pubkey) {
        if let Ok(mut tracked) = self.tracked_accounts.write() {
            tracked.insert(pubkey);
        }
    }

    pub fn add_accounts(&self, pubkeys: &[Pubkey]) {
        if let Ok(mut tracked) = self.tracked_accounts.write() {
            for pubkey in pubkeys {
                tracked.insert(*pubkey);
            }
        }
    }

    pub fn remove_account(&self, pubkey: &Pubkey) {
        if let Ok(mut tracked) = self.tracked_accounts.write() {
            tracked.remove(pubkey);
        }
    }

    pub fn remove_accounts(&self, pubkeys: &[Pubkey]) {
        if let Ok(mut tracked) = self.tracked_accounts.write() {
            for pubkey in pubkeys {
                tracked.remove(pubkey);
            }
        }
    }

    pub fn contains(&self, pubkey: &Pubkey) -> bool {
        self.tracked_accounts.read()
            .map(|tracked| tracked.contains(pubkey))
            .unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        self.tracked_accounts.read()
            .map(|tracked| tracked.len())
            .unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_all_tracked(&self) -> Vec<Pubkey> {
        self.tracked_accounts.read()
            .map(|tracked| tracked.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn clear(&self) {
        if let Ok(mut tracked) = self.tracked_accounts.write() {
            tracked.clear();
        }
    }
}

impl Clone for PoolTracker {
    fn clone(&self) -> Self {
        Self {
            tracked_accounts: Arc::clone(&self.tracked_accounts),
        }
    }
}
