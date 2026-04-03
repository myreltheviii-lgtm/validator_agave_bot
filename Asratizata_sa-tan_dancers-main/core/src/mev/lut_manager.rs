// `solana_message` owns `v0::Message`, `VersionedMessage`, and `AddressLookupTableAccount`
// in agave 4.x.  `v0::Message::try_compile` is the entry point for building versioned
// transactions that reference address lookup tables; it requires `AddressLookupTableAccount`
// slices to know which on-chain tables to embed into the message header.
use solana_message::{v0, AddressLookupTableAccount, VersionedMessage};
use solana_pubkey::Pubkey;
// `solana_hash` is the disaggregated crate for the SHA-256 `Hash` type used as a
// recent blockhash in transaction construction.
use solana_hash::Hash;
// `solana_instruction` owns the `Instruction` type in agave 4.x.
use solana_instruction::Instruction;
use solana_client::rpc_client::RpcClient;
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use tracing::{info, debug};
use std::sync::{Arc, RwLock};
use super::lut_helpers::fetch_multiple_address_lookup_tables;

#[derive(Clone, Debug)]
pub struct LutConfig {
    pub lut_addresses: Vec<Pubkey>,
    pub enabled: bool,
}

impl LutConfig {
    pub fn new(lut_addresses: Vec<Pubkey>) -> Self {
        Self {
            enabled: !lut_addresses.is_empty(),
            lut_addresses,
        }
    }

    pub fn disabled() -> Self {
        Self {
            lut_addresses: Vec::new(),
            enabled: false,
        }
    }
}

/// Internal state bag held behind a `RwLock`.
///
/// Splitting state from the public `LutManager` type allows read-heavy
/// operations (`create_v0_message`, `contains_account`) to acquire only a
/// shared read lock, while writes (`refresh_lut`, `refresh_all_luts`) acquire
/// an exclusive write lock for the shortest possible critical section.
struct LutState {
    /// On-chain address → decoded lookup table (pubkeys stored inside the
    /// table on-chain).
    luts: HashMap<Pubkey, AddressLookupTableAccount>,

    /// Reverse index: account pubkey → list of LUT addresses that contain it.
    ///
    /// Built by `rebuild_address_index` after every LUT load or refresh.
    /// Used by `select_optimal_luts` to score LUTs by how many accounts they
    /// compress for a given transaction, and by `contains_account` for O(1)
    /// membership tests.
    address_to_luts: HashMap<Pubkey, Vec<Pubkey>>,
}

impl LutState {
    fn new() -> Self {
        Self {
            luts: HashMap::new(),
            address_to_luts: HashMap::new(),
        }
    }

    /// Rebuild the reverse index from scratch after any change to `luts`.
    ///
    /// A full rebuild is correct here because LUT operations (load, refresh)
    /// are startup-time or operator-triggered; they are not on the hot path.
    fn rebuild_address_index(&mut self) {
        self.address_to_luts.clear();
        for (lut_addr, lut) in self.luts.iter() {
            for address in &lut.addresses {
                self.address_to_luts
                    .entry(*address)
                    .or_insert_with(Vec::new)
                    .push(*lut_addr);
            }
        }
    }
}

pub struct LutManager {
    state: RwLock<LutState>,
    pub config: LutConfig,
}

impl LutManager {
    /// Create a `LutManager` with no lookup tables loaded.
    ///
    /// Transactions built through an empty manager use only the static account
    /// list embedded directly in the v0 message header.  This works for simple
    /// single-hop paths but will fail at compile time for paths that exceed
    /// Solana's 35-account static limit.
    pub fn new(payer: Pubkey) -> Self {
        info!("LutManager initialized with no LUTs for payer: {}", payer);
        Self {
            state: RwLock::new(LutState::new()),
            config: LutConfig::disabled(),
        }
    }

    /// Create a `LutManager` by fetching `lut_addresses` from the cluster.
    ///
    /// Each address must refer to a deployed `AddressLookupTable` account
    /// whose on-chain `addresses` slice is fetched once and cached for the
    /// process lifetime.  Returns an error if any address is not found or if
    /// no valid tables are loaded.
    pub fn new_with_luts(
        lut_addresses: Vec<Pubkey>,
        rpc_client: &Arc<RpcClient>,
    ) -> Result<Self> {
        info!("Loading {} Lookup Tables", lut_addresses.len());

        let lut_accounts = fetch_multiple_address_lookup_tables(rpc_client, &lut_addresses)
            .context("Failed to fetch LUT accounts")?;

        let mut state = LutState::new();

        for lut_account in lut_accounts {
            info!(
                "Loaded LUT {} with {} addresses",
                lut_account.key,
                lut_account.addresses.len()
            );
            state.luts.insert(lut_account.key, lut_account);
        }

        if state.luts.is_empty() {
            return Err(anyhow::anyhow!("No valid LUTs loaded"));
        }

        state.rebuild_address_index();

        info!(
            "Successfully loaded {} LUTs covering {} unique addresses",
            state.luts.len(),
            state.address_to_luts.len()
        );

        Ok(Self {
            state: RwLock::new(state),
            config: LutConfig::new(lut_addresses),
        })
    }

    /// Build a versioned (v0) transaction message that compresses as many of
    /// the instruction's account references as possible using the loaded LUTs.
    ///
    /// `select_optimal_luts` scores every loaded LUT by how many accounts from
    /// `instructions` it covers, then picks the top-4 (Solana's per-transaction
    /// LUT limit).  `v0::Message::try_compile` replaces each account that
    /// appears in a selected LUT with a 1-byte index, freeing space for more
    /// unique accounts in the static header.
    pub fn create_v0_message(
        &self,
        instructions: &[Instruction],
        payer: &Pubkey,
        recent_blockhash: Hash,
    ) -> Result<VersionedMessage> {
        let mut all_accounts = HashSet::new();

        all_accounts.insert(*payer);

        for ix in instructions {
            all_accounts.insert(ix.program_id);
            for account_meta in &ix.accounts {
                all_accounts.insert(account_meta.pubkey);
            }
        }

        let accounts_vec: Vec<Pubkey> = all_accounts.into_iter().collect();
        let selected_luts = self.select_optimal_luts(&accounts_vec);

        debug!(
            "Selected {} LUTs for transaction with {} accounts",
            selected_luts.len(),
            accounts_vec.len()
        );

        let message = v0::Message::try_compile(
            payer,
            instructions,
            &selected_luts,
            recent_blockhash,
        ).context("Failed to compile V0 message")?;

        Ok(VersionedMessage::V0(message))
    }

    /// Choose the best subset of loaded LUTs for compressing `needed_accounts`.
    ///
    /// Each LUT is scored by how many of `needed_accounts` it covers.  The
    /// top-4 scoring LUTs are returned (Solana's maximum per-transaction limit).
    /// Only LUTs with at least one covered account are included — an empty LUT
    /// header slot wastes message space.
    fn select_optimal_luts(&self, needed_accounts: &[Pubkey]) -> Vec<AddressLookupTableAccount> {
        let state = match self.state.read() {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut lut_scores: HashMap<Pubkey, usize> = HashMap::new();

        for account in needed_accounts {
            if let Some(lut_addrs) = state.address_to_luts.get(account) {
                for lut_addr in lut_addrs {
                    *lut_scores.entry(*lut_addr).or_insert(0) += 1;
                }
            }
        }

        let mut scored_luts: Vec<_> = lut_scores.into_iter().collect();
        scored_luts.sort_by_key(|(_, score)| std::cmp::Reverse(*score));
        // Solana versioned transactions support at most 4 address lookup tables
        // per transaction.  Exceeding this limit causes `v0::Message::try_compile`
        // to return an error at build time rather than an on-chain rejection.
        scored_luts.truncate(4);

        scored_luts
            .into_iter()
            .filter_map(|(addr, score)| {
                if score > 0 {
                    state.luts.get(&addr).cloned()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_lut(&self, lut_address: &Pubkey) -> Option<AddressLookupTableAccount> {
        self.state.read().ok()?.luts.get(lut_address).cloned()
    }

    pub fn contains_account(&self, account: &Pubkey) -> bool {
        self.state.read()
            .map(|s| s.address_to_luts.contains_key(account))
            .unwrap_or(false)
    }

    /// Return `(lut_count, covered_address_count)` for logging and diagnostics.
    pub fn get_coverage_stats(&self) -> (usize, usize) {
        self.state.read()
            .map(|s| (s.luts.len(), s.address_to_luts.len()))
            .unwrap_or((0, 0))
    }

    /// Re-fetch a single LUT from the cluster and replace the cached version.
    ///
    /// Useful when an operator has extended a LUT on-chain and wants the engine
    /// to start using the new entries without a full restart.  The address index
    /// is rebuilt atomically under the write lock after the new data is inserted.
    pub fn refresh_lut(
        &self,
        lut_address: &Pubkey,
        rpc_client: &Arc<RpcClient>,
    ) -> Result<()> {
        let lut_account = super::lut_helpers::fetch_address_lookup_table_account(
            rpc_client,
            lut_address,
        )?;

        info!(
            "Refreshed LUT {} with {} addresses",
            lut_address,
            lut_account.addresses.len()
        );

        if let Ok(mut state) = self.state.write() {
            state.luts.insert(*lut_address, lut_account);
            state.rebuild_address_index();
        }

        Ok(())
    }

    /// Re-fetch every loaded LUT from the cluster in a single batch RPC call.
    ///
    /// The address index is rebuilt once after all tables are updated, paying
    /// the rebuild cost only once regardless of how many tables were refreshed.
    pub fn refresh_all_luts(&self, rpc_client: &Arc<RpcClient>) -> Result<()> {
        let lut_addresses: Vec<Pubkey> = self.state.read()
            .map(|s| s.luts.keys().copied().collect())
            .unwrap_or_default();

        let lut_accounts = fetch_multiple_address_lookup_tables(rpc_client, &lut_addresses)
            .context("Failed to refresh LUTs")?;

        if let Ok(mut state) = self.state.write() {
            for lut_account in lut_accounts {
                state.luts.insert(lut_account.key, lut_account);
            }
            state.rebuild_address_index();
            info!("Refreshed all {} LUTs", state.luts.len());
        }

        Ok(())
    }

    /// Return the number of `accounts` that are covered by at least one loaded
    /// LUT alongside the number that are not covered.
    ///
    /// Useful for operators diagnosing why transactions are approaching the
    /// 1232-byte MTU: high `not_covered` counts indicate that the LUTs need
    /// to be extended with the missing addresses.
    pub fn validate_accounts_coverage(&self, accounts: &[Pubkey]) -> (usize, usize) {
        let mut covered = 0;
        let mut not_covered = 0;

        for account in accounts {
            if self.contains_account(account) {
                covered += 1;
            } else {
                not_covered += 1;
            }
        }

        (covered, not_covered)
    }

    pub fn get_all_luts(&self) -> Vec<AddressLookupTableAccount> {
        self.state.read()
            .map(|s| s.luts.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_lut_count(&self) -> usize {
        self.state.read()
            .map(|s| s.luts.len())
            .unwrap_or(0)
    }

    pub fn get_covered_address_count(&self) -> usize {
        self.state.read()
            .map(|s| s.address_to_luts.len())
            .unwrap_or(0)
    }

    pub fn get_luts_containing_account(&self, account: &Pubkey) -> Vec<Pubkey> {
        self.state.read()
            .map(|s| s.address_to_luts.get(account).cloned().unwrap_or_default())
            .unwrap_or_default()
    }
}
