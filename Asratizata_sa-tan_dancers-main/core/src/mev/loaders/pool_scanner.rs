use crate::mev::constants::{SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT};
use crate::mev::dex::byreal::byreal_program_id;
use crate::mev::dex::futarchy::{futarchy_program_id, FutarchyInfo};
use crate::mev::dex::heaven::{heaven_program_id, HeavenPoolState};
use crate::mev::dex::humidifi::{humidifi_program_id, HumidifiInfo};
use crate::mev::dex::meteora::constants::{damm_program_id, damm_v2_program_id};
use crate::mev::dex::meteora::dammv2_info::MeteoraDAmmV2Info;
use crate::mev::dex::meteora::{constants::dlmm_program_id, dlmm_info::DlmmInfo};
use crate::mev::dex::meteora::damm::meteora_damm_cpi;
use crate::mev::dex::pancakeswap::pancakeswap_program_id;
use crate::mev::dex::pump::{pump_program_id, PumpAmmInfo};
use crate::mev::dex::raydium::{
    raydium_clmm_program_id, raydium_cp_program_id, raydium_program_id,
    PoolState, RaydiumAmmInfo, RaydiumCpAmmInfo,
};
use crate::mev::dex::vertigo::{vertigo_program_id, VertigoInfo};
use crate::mev::dex::whirlpool::{constants::whirlpool_program_id, state::Whirlpool};
use solana_account::ReadableAccount;
use solana_runtime::bank::Bank;
use solana_accounts_db::accounts_index::ScanConfig;
use solana_pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn};

#[derive(Clone)]
pub struct DiscoveredPools {
    pub raydium_v4: Vec<Pubkey>,
    pub raydium_clmm: Vec<Pubkey>,
    pub raydium_cpmm: Vec<Pubkey>,
    pub meteora_damm: Vec<Pubkey>,
    pub meteora_dammv2: Vec<Pubkey>,
    pub meteora_dlmm: Vec<Pubkey>,
    pub whirlpool: Vec<Pubkey>,
    pub pump: Vec<Pubkey>,
    pub byreal: Vec<Pubkey>,
    pub pancakeswap: Vec<Pubkey>,
    pub humidifi: Vec<Pubkey>,
    pub vertigo: Vec<Pubkey>,
    pub heaven: Vec<Pubkey>,
    pub futarchy: Vec<Pubkey>,
}

impl DiscoveredPools {
    pub fn new() -> Self {
        Self {
            raydium_v4: Vec::new(),
            raydium_clmm: Vec::new(),
            raydium_cpmm: Vec::new(),
            meteora_damm: Vec::new(),
            meteora_dammv2: Vec::new(),
            meteora_dlmm: Vec::new(),
            whirlpool: Vec::new(),
            pump: Vec::new(),
            byreal: Vec::new(),
            pancakeswap: Vec::new(),
            humidifi: Vec::new(),
            vertigo: Vec::new(),
            heaven: Vec::new(),
            futarchy: Vec::new(),
        }
    }

    pub fn total_count(&self) -> usize {
        self.raydium_v4.len()
            + self.raydium_clmm.len()
            + self.raydium_cpmm.len()
            + self.meteora_damm.len()
            + self.meteora_dammv2.len()
            + self.meteora_dlmm.len()
            + self.whirlpool.len()
            + self.pump.len()
            + self.byreal.len()
            + self.pancakeswap.len()
            + self.humidifi.len()
            + self.vertigo.len()
            + self.heaven.len()
            + self.futarchy.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    pub fn all_pool_state_pubkeys(&self) -> Vec<Pubkey> {
        let mut pubkeys = Vec::with_capacity(self.total_count());
        pubkeys.extend_from_slice(&self.raydium_v4);
        pubkeys.extend_from_slice(&self.raydium_clmm);
        pubkeys.extend_from_slice(&self.raydium_cpmm);
        pubkeys.extend_from_slice(&self.meteora_damm);
        pubkeys.extend_from_slice(&self.meteora_dammv2);
        pubkeys.extend_from_slice(&self.meteora_dlmm);
        pubkeys.extend_from_slice(&self.whirlpool);
        pubkeys.extend_from_slice(&self.pump);
        pubkeys.extend_from_slice(&self.byreal);
        pubkeys.extend_from_slice(&self.pancakeswap);
        pubkeys.extend_from_slice(&self.humidifi);
        pubkeys.extend_from_slice(&self.vertigo);
        pubkeys.extend_from_slice(&self.heaven);
        pubkeys.extend_from_slice(&self.futarchy);
        pubkeys
    }
}

pub struct MintDiscoveryResult {
    pub pools_by_mint: HashMap<Pubkey, DiscoveredPools>,
    pub total_unique_mints: usize,
    pub total_pools: usize,
}

impl MintDiscoveryResult {
    pub fn build_pool_state_to_mint_map(&self) -> HashMap<Pubkey, Pubkey> {
        let mut map = HashMap::new();
        for (mint, pools) in &self.pools_by_mint {
            for pubkey in pools.all_pool_state_pubkeys() {
                map.insert(pubkey, *mint);
            }
        }
        map
    }
}

/// Returns true if the given token is one of the four recognised quote currencies.
///
/// Pools whose BOTH tokens are quote currencies (e.g. SOL/USDC or USDC/USDT) are
/// not useful as standalone arb targets — the arb graph only needs pools where
/// one side is a speculative (non-quote) token. Filtering them here prevents the
/// scanner from creating thousands of useless mint entries in `pools_by_mint`.
#[inline(always)]
fn is_quote_token(t: &Pubkey) -> bool {
    *t == SOL_MINT || *t == USDC_MINT || *t == USDT_MINT || *t == USD1_MINT
}

/// Discovers every DEX pool on-chain and groups them by the non-quote token mint
/// they contain.
///
/// # How account discovery works inside the validator
///
/// The original implementation memory-mapped a 300-million-account flat file and
/// walked every account sequentially, testing each account's `owner` field against
/// the 14 known DEX program IDs. That linear scan was the only option when the bot
/// ran as a standalone process with no access to the validator's internal indices.
///
/// Inside the Agave validator process, `AccountsDb` maintains a program-ownership
/// index that maps each program ID to the set of accounts it owns. `Bank::get_program_accounts`
/// queries this index directly, returning only the accounts owned by a specific program.
/// This means instead of walking 300 million accounts once, we make 14 targeted queries
/// and visit only the pool state accounts — roughly 500K total across all DEXes. Every
/// returned account's owner is already known from the query that produced it, so no
/// per-account owner check is required inside the parse loop.
pub fn discover_all_pools_grouped_by_mint(bank: &Arc<Bank>) -> Result<MintDiscoveryResult> {
    info!(
        "Starting pool discovery from bank at slot {} — querying 14 DEX programs",
        bank.slot()
    );

    let scan_cfg = ScanConfig::default();

    let raydium_v4_program     = raydium_program_id();
    let raydium_clmm_program   = raydium_clmm_program_id();
    let raydium_cpmm_program   = raydium_cp_program_id();
    let meteora_damm_program   = damm_program_id();
    let meteora_dammv2_program = damm_v2_program_id();
    let meteora_dlmm_program   = dlmm_program_id();
    let whirlpool_program      = whirlpool_program_id();
    let pump_program           = pump_program_id();
    let byreal_program         = byreal_program_id();
    let pancakeswap_program    = pancakeswap_program_id();
    let humidifi_program       = humidifi_program_id();
    let vertigo_program        = vertigo_program_id();
    let heaven_program         = heaven_program_id();
    let futarchy_program       = futarchy_program_id();

    let mut pools_by_mint: HashMap<Pubkey, DiscoveredPools> = HashMap::new();
    let mut found = 0usize;
    let start = std::time::Instant::now();

    // -------------------------------------------------------------------------
    // Raydium V4 (AMM)
    // -------------------------------------------------------------------------
    // `before` is declared before every DEX section so each section's log line
    // reports only the delta for that DEX, not the cumulative total. For V4 this
    // equals zero since it is the first section, but the pattern is kept uniform
    // so all 14 log lines are directly comparable.
    let before = found;
    match bank.get_program_accounts(&raydium_v4_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = RaydiumAmmInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.coin_mint) {
                        pools_by_mint.entry(info.coin_mint).or_insert_with(DiscoveredPools::new).raydium_v4.push(pubkey);
                    }
                    if !is_quote_token(&info.pc_mint) {
                        pools_by_mint.entry(info.pc_mint).or_insert_with(DiscoveredPools::new).raydium_v4.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Raydium V4: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Raydium V4 program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Raydium CLMM
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&raydium_clmm_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = PoolState::load_checked(account.data()) {
                    if !is_quote_token(&info.token_mint_0) {
                        pools_by_mint.entry(info.token_mint_0).or_insert_with(DiscoveredPools::new).raydium_clmm.push(pubkey);
                    }
                    if !is_quote_token(&info.token_mint_1) {
                        pools_by_mint.entry(info.token_mint_1).or_insert_with(DiscoveredPools::new).raydium_clmm.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Raydium CLMM: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Raydium CLMM program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Raydium CPMM
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&raydium_cpmm_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = RaydiumCpAmmInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.token_0_mint) {
                        pools_by_mint.entry(info.token_0_mint).or_insert_with(DiscoveredPools::new).raydium_cpmm.push(pubkey);
                    }
                    if !is_quote_token(&info.token_1_mint) {
                        pools_by_mint.entry(info.token_1_mint).or_insert_with(DiscoveredPools::new).raydium_cpmm.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Raydium CPMM: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Raydium CPMM program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Meteora DAMM
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&meteora_damm_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                // Meteora DAMM uses Anchor's `deserialize_unchecked`, which takes
                // `buf: &mut &[u8]` — a mutable reference to a slice reference that acts
                // as a cursor advancing through the bytes as each field is decoded.
                // `AccountSharedData::data()` returns a `&[u8]` borrowed directly from
                // the validator's in-memory accounts database; binding it to a `mut`
                // local variable produces the `&mut &[u8]` that the function requires,
                // with no intermediate heap allocation or byte copy.
                let mut slice = account.data();
                if let Ok(pool) = meteora_damm_cpi::Pool::deserialize_unchecked(&mut slice) {
                    if !is_quote_token(&pool.token_a_mint) {
                        pools_by_mint.entry(pool.token_a_mint).or_insert_with(DiscoveredPools::new).meteora_damm.push(pubkey);
                    }
                    if !is_quote_token(&pool.token_b_mint) {
                        pools_by_mint.entry(pool.token_b_mint).or_insert_with(DiscoveredPools::new).meteora_damm.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Meteora DAMM: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Meteora DAMM program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Meteora DAMM V2
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&meteora_dammv2_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = MeteoraDAmmV2Info::load_checked(account.data()) {
                    if !is_quote_token(&info.base_mint) {
                        pools_by_mint.entry(info.base_mint).or_insert_with(DiscoveredPools::new).meteora_dammv2.push(pubkey);
                    }
                    if !is_quote_token(&info.quote_mint) {
                        pools_by_mint.entry(info.quote_mint).or_insert_with(DiscoveredPools::new).meteora_dammv2.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Meteora DAMM V2: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Meteora DAMM V2 program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Meteora DLMM
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&meteora_dlmm_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = DlmmInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.token_x_mint) {
                        pools_by_mint.entry(info.token_x_mint).or_insert_with(DiscoveredPools::new).meteora_dlmm.push(pubkey);
                    }
                    if !is_quote_token(&info.token_y_mint) {
                        pools_by_mint.entry(info.token_y_mint).or_insert_with(DiscoveredPools::new).meteora_dlmm.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Meteora DLMM: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Meteora DLMM program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Orca Whirlpool
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&whirlpool_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                // Orca's `try_deserialize` is an Anchor-generated method that validates
                // the eight-byte account discriminator stamped at the front of the data
                // before decoding the pool fields. Like `deserialize_unchecked`, its
                // signature takes `buf: &mut &[u8]` — a cursor that advances as fields
                // are read. `AccountSharedData::data()` is a zero-copy borrow from the
                // validator's accounts DB; binding it to a `mut` variable satisfies the
                // `&mut &[u8]` requirement without copying the underlying bytes.
                let mut slice = account.data();
                if let Ok(pool) = Whirlpool::try_deserialize(&mut slice) {
                    if !is_quote_token(&pool.token_mint_a) {
                        pools_by_mint.entry(pool.token_mint_a).or_insert_with(DiscoveredPools::new).whirlpool.push(pubkey);
                    }
                    if !is_quote_token(&pool.token_mint_b) {
                        pools_by_mint.entry(pool.token_mint_b).or_insert_with(DiscoveredPools::new).whirlpool.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Orca Whirlpool: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Orca Whirlpool program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // PumpSwap
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&pump_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = PumpAmmInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.base_mint) {
                        pools_by_mint.entry(info.base_mint).or_insert_with(DiscoveredPools::new).pump.push(pubkey);
                    }
                    if !is_quote_token(&info.quote_mint) {
                        pools_by_mint.entry(info.quote_mint).or_insert_with(DiscoveredPools::new).pump.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("PumpSwap: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query PumpSwap program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Byreal — shares Raydium CLMM's PoolState layout under a different program ID
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&byreal_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = PoolState::load_checked(account.data()) {
                    if !is_quote_token(&info.token_mint_0) {
                        pools_by_mint.entry(info.token_mint_0).or_insert_with(DiscoveredPools::new).byreal.push(pubkey);
                    }
                    if !is_quote_token(&info.token_mint_1) {
                        pools_by_mint.entry(info.token_mint_1).or_insert_with(DiscoveredPools::new).byreal.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Byreal: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Byreal program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // PancakeSwap — shares Raydium CLMM's PoolState layout under a different program ID
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&pancakeswap_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = PoolState::load_checked(account.data()) {
                    if !is_quote_token(&info.token_mint_0) {
                        pools_by_mint.entry(info.token_mint_0).or_insert_with(DiscoveredPools::new).pancakeswap.push(pubkey);
                    }
                    if !is_quote_token(&info.token_mint_1) {
                        pools_by_mint.entry(info.token_mint_1).or_insert_with(DiscoveredPools::new).pancakeswap.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("PancakeSwap: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query PancakeSwap program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Humidifi
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&humidifi_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = HumidifiInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.base_mint) {
                        pools_by_mint.entry(info.base_mint).or_insert_with(DiscoveredPools::new).humidifi.push(pubkey);
                    }
                    if !is_quote_token(&info.quote_mint) {
                        pools_by_mint.entry(info.quote_mint).or_insert_with(DiscoveredPools::new).humidifi.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Humidifi: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Humidifi program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Vertigo
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&vertigo_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = VertigoInfo::load_checked(account.data(), &pubkey) {
                    if !is_quote_token(&info.mint_a) {
                        pools_by_mint.entry(info.mint_a).or_insert_with(DiscoveredPools::new).vertigo.push(pubkey);
                    }
                    if !is_quote_token(&info.mint_b) {
                        pools_by_mint.entry(info.mint_b).or_insert_with(DiscoveredPools::new).vertigo.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Vertigo: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Vertigo program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Heaven
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&heaven_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Some(info) = HeavenPoolState::parse(account.data()) {
                    if !is_quote_token(&info.mint_a) {
                        pools_by_mint.entry(info.mint_a).or_insert_with(DiscoveredPools::new).heaven.push(pubkey);
                    }
                    if !is_quote_token(&info.mint_b) {
                        pools_by_mint.entry(info.mint_b).or_insert_with(DiscoveredPools::new).heaven.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Heaven: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Heaven program accounts: {:?}", e),
    }

    // -------------------------------------------------------------------------
    // Futarchy
    // -------------------------------------------------------------------------
    let before = found;
    match bank.get_program_accounts(&futarchy_program, &scan_cfg) {
        Ok(accounts) => {
            let count = accounts.len();
            for (pubkey, account) in accounts {
                if let Ok(info) = FutarchyInfo::load_checked(account.data()) {
                    if !is_quote_token(&info.base_mint) {
                        pools_by_mint.entry(info.base_mint).or_insert_with(DiscoveredPools::new).futarchy.push(pubkey);
                    }
                    if !is_quote_token(&info.quote_mint) {
                        pools_by_mint.entry(info.quote_mint).or_insert_with(DiscoveredPools::new).futarchy.push(pubkey);
                    }
                    found += 1;
                }
            }
            info!("Futarchy: {} accounts scanned, {} pools classified", count, found - before);
        }
        Err(e) => warn!("Failed to query Futarchy program accounts: {:?}", e),
    }

    let unique_mints = pools_by_mint.len();

    info!(
        "Pool discovery complete: {} total pools, {} unique mints, {:.1}s elapsed",
        found,
        unique_mints,
        start.elapsed().as_secs_f64()
    );

    info!("Top 10 mints by pool count:");
    let mut mint_pool_counts: Vec<(Pubkey, usize)> = pools_by_mint
        .iter()
        .map(|(mint, pools)| (*mint, pools.total_count()))
        .collect();
    mint_pool_counts.sort_by(|a, b| b.1.cmp(&a.1));

    for (rank, (mint, count)) in mint_pool_counts.iter().take(10).enumerate() {
        info!("  {}. {}: {} pools", rank + 1, mint, count);
    }

    Ok(MintDiscoveryResult {
        pools_by_mint,
        total_unique_mints: unique_mints,
        total_pools: found,
    })
}
