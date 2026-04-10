use crate::mev::loaders::pool_scanner::{discover_all_pools_grouped_by_mint, DiscoveredPools, MintDiscoveryResult};
use crate::mev::loaders::pool_parser;
use crate::mev::pools::MintPoolData;
// `ReadableAccount` is the trait that unlocks `.data()` and `.owner()` on
// `AccountSharedData`. In agave 4.x the trait lives in `solana_account` and is
// NOT automatically in scope — every module that calls those methods must import it
// explicitly. Without this import the compiler reports the methods as "private
// fields" because it finds the field-accessor form of `data` and `owner` on the
// concrete struct instead of the trait-provided method.
use solana_account::ReadableAccount;
use solana_runtime::bank::Bank;
use solana_pubkey::Pubkey;
use std::sync::Arc;
use anyhow::Result;
use tracing::info;

pub struct InitializedMint {
    pub mint: Pubkey,
    pub pool_data: MintPoolData,
}

/// Walk every DEX program's accounts in the bank and return a discovery result
/// grouped by the non-quote token mint. This is a read-only scan: it queries
/// `Bank::get_program_accounts` for each of the 14 known DEX programs and parses
/// every returned account's state header to extract the two token mints.
///
/// The bank parameter is the canonical active-slot bank held by BankForks. All
/// reads go through `AccountsDb::load_with_fixed_root`, which provides a
/// consistent snapshot of on-chain state at the given slot without any write-set
/// contamination from in-flight speculative execution.
pub fn scan_all_mints_no_init(bank: &Arc<Bank>) -> Result<MintDiscoveryResult> {
    info!(
        "Starting lightweight pool scan from bank at slot {}",
        bank.slot()
    );
    let result = discover_all_pools_grouped_by_mint(bank)?;
    info!(
        "Scan complete: {} unique mints, {} total pools",
        result.total_unique_mints,
        result.total_pools
    );
    Ok(result)
}

/// Parse and fully initialize all pool structs for a single mint that was
/// returned by a prior `scan_all_mints_no_init` call.
///
/// Each pool parser fetches the individual vault, tick-array, and sub-account
/// pubkeys from the bank so the instruction builder can use them directly without
/// any further network lookups at trade time. The bank must be the same bank that
/// was used for the preceding scan; using a different (newer) bank is safe but
/// may yield slightly different tick-array positions if the pool's active tick
/// shifted between the two bank states.
///
/// # Token-2022 detection
///
/// The mint account's `owner` field determines which SPL token program governs
/// the mint: `spl_token_interface::id()` for classic SPL Token, `spl_token_2022::id()` for
/// Token-2022. This owner is read directly from the bank — no RPC call required.
/// The detected program ID is stored in every pool struct so the instruction
/// builder can derive wallet ATAs with the correct program at build time.
///
/// # SPL Memo requirement
///
/// Token-2022 mints that use the transfer-hook extension require the SPL Memo
/// program to be included in the account list of every DEX swap that involves them.
/// The memo program pubkey is computed once here and stored in pool structs that
/// need it (CLMM, DLMM, Byreal, PancakeSwap, Whirlpool).
pub fn initialize_mint_from_discovered(
    mint: &Pubkey,
    discovered: DiscoveredPools,
    wallet_account: &Pubkey,
    bank: &Arc<Bank>,
) -> Result<InitializedMint> {
    let total_pools_for_mint = discovered.total_count();

    if total_pools_for_mint == 0 {
        return Err(anyhow::anyhow!("No pools for mint {}", mint));
    }

    let mint_account = bank
        .get_account(mint)
        .ok_or_else(|| anyhow::anyhow!("Mint account not found in bank: {}", mint))?;

    // The SPL Token program (`TokenkegQ…`) and Token-2022 program (`TokenzQd…`) are
    // the only two valid owners for a mint account on Solana. Reading the owner field
    // from the bank's in-memory account store and comparing it against the canonical
    // program IDs exposed by the `spl-token-interface` and `spl-token-2022` crates
    // determines which variant governs this mint.
    //
    // `spl_token_interface::id()` returns the classic SPL Token program ID as a
    // `solana_pubkey::Pubkey`, and `spl_token_2022::id()` returns the Token-2022
    // program ID — both sourced from the same modular crate family that agave 4.x
    // uses, so the `Pubkey` type is identical to the one used throughout this workspace.
    let token_2022_program_id = spl_token_2022::id();

    let token_program = if *mint_account.owner() == spl_token_interface::id() {
        spl_token_interface::id()
    } else if *mint_account.owner() == token_2022_program_id {
        token_2022_program_id
    } else {
        return Err(anyhow::anyhow!("Unknown token program for mint: {}", mint));
    };

    // SPL Memo is required alongside any Token-2022 transfer on DEXes that use the
    // token's transfer hook extension. The Memo program enforces that a memo CPI is
    // present inside the hook invocation. For plain SPL tokens this is None and callers
    // omit the memo account entirely. Whirlpool is an exception — the instruction builder
    // always hardcodes the memo program for Whirlpool regardless of this value.
    let memo_program_id: Option<Pubkey> = if token_program != spl_token_interface::id() {
        // compile-time decode — no runtime base58 parse, no unwrap
        Some(solana_pubkey::pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"))
    } else {
        None
    };

    let mut pool_data = MintPoolData::new(*mint, wallet_account, token_program);

    pool_data.raydium_pools          = pool_parser::parse_raydium_v4_pools(bank, &discovered.raydium_v4, mint)?;
    // Raydium CLMM, DLMM, Byreal, and PancakeSwap all receive memo_program_id so they
    // can embed the correct value into their pool structs at parse time. This avoids
    // re-deriving it on every instruction build, which is on the critical latency path.
    pool_data.raydium_clmm_pools     = pool_parser::parse_raydium_clmm_pools(bank, &discovered.raydium_clmm, mint, memo_program_id)?;
    pool_data.raydium_cp_pools       = pool_parser::parse_raydium_cpmm_pools(bank, &discovered.raydium_cpmm, mint)?;
    pool_data.meteora_damm_pools     = pool_parser::parse_meteora_damm_pools(bank, &discovered.meteora_damm, mint)?;
    pool_data.meteora_damm_v2_pools  = pool_parser::parse_meteora_damm_v2_pools(bank, &discovered.meteora_dammv2, mint)?;
    pool_data.dlmm_pairs             = pool_parser::parse_meteora_dlmm_pools(bank, &discovered.meteora_dlmm, mint, memo_program_id)?;
    // Whirlpool's instruction builder always hardcodes the memo program regardless of this
    // value (see add_orca_whirlpool_accounts in smb_instruction_builder.rs), so the stored
    // memo_program in WhirlpoolPool is not used at instruction build time. It is passed
    // through here for structural consistency with other CLMM-family pool parsers.
    pool_data.whirlpool_pools        = pool_parser::parse_orca_whirlpool_pools(bank, &discovered.whirlpool, mint, memo_program_id)?;
    pool_data.pump_pools             = pool_parser::parse_pump_swap_pools(bank, &discovered.pump, mint, wallet_account)?;
    pool_data.byreal_pools           = pool_parser::parse_byreal_pools(bank, &discovered.byreal, mint, memo_program_id)?;
    pool_data.pancakeswap_pools      = pool_parser::parse_pancakeswap_pools(bank, &discovered.pancakeswap, mint, memo_program_id)?;
    pool_data.humidifi_pools         = pool_parser::parse_humidifi_pools(bank, &discovered.humidifi, mint)?;
    pool_data.vertigo_pools          = pool_parser::parse_vertigo_pools(bank, &discovered.vertigo, mint)?;
    pool_data.heaven_pools           = pool_parser::parse_heaven_pools(bank, &discovered.heaven, mint)?;
    pool_data.futarchy_pools         = pool_parser::parse_futarchy_pools(bank, &discovered.futarchy, mint)?;

    let loaded_pools = pool_data.raydium_pools.len()
        + pool_data.raydium_clmm_pools.len()
        + pool_data.raydium_cp_pools.len()
        + pool_data.meteora_damm_pools.len()
        + pool_data.meteora_damm_v2_pools.len()
        + pool_data.dlmm_pairs.len()
        + pool_data.whirlpool_pools.len()
        + pool_data.pump_pools.len()
        + pool_data.byreal_pools.len()
        + pool_data.pancakeswap_pools.len()
        + pool_data.humidifi_pools.len()
        + pool_data.vertigo_pools.len()
        + pool_data.heaven_pools.len()
        + pool_data.futarchy_pools.len();

    if loaded_pools == 0 {
        return Err(anyhow::anyhow!("No pools successfully parsed for mint {}", mint));
    }

    info!(
        "Initialized mint {}: {} pools \
         ({} raydium_v4, {} raydium_cp, {} raydium_clmm, {} pump, \
          {} damm, {} damm_v2, {} dlmm, {} whirlpool, \
          {} byreal, {} pancakeswap, {} humidifi, {} vertigo, \
          {} heaven, {} futarchy)",
        mint,
        loaded_pools,
        pool_data.raydium_pools.len(),
        pool_data.raydium_cp_pools.len(),
        pool_data.raydium_clmm_pools.len(),
        pool_data.pump_pools.len(),
        pool_data.meteora_damm_pools.len(),
        pool_data.meteora_damm_v2_pools.len(),
        pool_data.dlmm_pairs.len(),
        pool_data.whirlpool_pools.len(),
        pool_data.byreal_pools.len(),
        pool_data.pancakeswap_pools.len(),
        pool_data.humidifi_pools.len(),
        pool_data.vertigo_pools.len(),
        pool_data.heaven_pools.len(),
        pool_data.futarchy_pools.len(),
    );

    Ok(InitializedMint {
        mint: *mint,
        pool_data,
    })
}
