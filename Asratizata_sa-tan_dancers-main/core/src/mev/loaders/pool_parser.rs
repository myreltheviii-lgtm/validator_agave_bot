use crate::mev::constants::SOL_MINT;
use crate::mev::dex::byreal::byreal_program_id;
use crate::mev::dex::futarchy::FutarchyInfo;
use crate::mev::dex::heaven::HeavenPoolState;
use crate::mev::dex::humidifi::HumidifiInfo;

use crate::mev::dex::meteora::dammv2_info::MeteoraDAmmV2Info;
use crate::mev::dex::meteora::dlmm_info::DlmmInfo;
use crate::mev::dex::meteora::damm::meteora_damm_cpi;
use crate::mev::dex::meteora::damm_vault::meteora_vault_cpi;
use crate::mev::dex::pancakeswap::pancakeswap_program_id;
use crate::mev::dex::pump::{pump_fee_wallet, pump_mayhem_fee_wallet, PumpAmmInfo};
use crate::mev::dex::raydium::{
    get_tick_array_pubkeys, raydium_clmm_program_id,
    PoolState, RaydiumAmmInfo, RaydiumCpAmmInfo,
};
use crate::mev::dex::vertigo::{derive_vault_address, VertigoInfo};
use crate::mev::dex::whirlpool::{
    constants::whirlpool_program_id, state::Whirlpool, update_tick_array_accounts_for_onchain,
};
use crate::mev::pools::{
    RaydiumPool, RaydiumClmmPool, RaydiumCpPool,
    MeteoraDAmmPool, MeteoraDAmmV2Pool, DlmmPool,
    WhirlpoolPool, PumpPool, ByrealPool, PancakeswapPool, HumidifiPool,
    VertigoPool, HeavenPool, FutarchyPool,
};
use solana_account::ReadableAccount;
use solana_runtime::bank::Bank;
use solana_pubkey::Pubkey;
use std::sync::Arc;
use anyhow::Result;
use tracing::{warn, error, debug};

// All Pump pool ATA derivations in this module — the fee wallet's quote-mint ATA and the
// coin creator's vault ATA — target accounts that are always governed by the classic SPL
// Token program. The single-argument form `get_associated_token_address(wallet, mint)` is
// the correct call here: internally it fixes the token program seed to the classic SPL Token
// program ID, which is the only valid program for these accounts in the Pump AMM protocol.
// `spl_associated_token_account_interface` is the canonical source for this derivation —
// its seed layout, program address, and bump-search strategy are guaranteed to match what
// the on-chain Pump program computes for the same inputs.
use spl_associated_token_account_interface::address::get_associated_token_address;


pub fn parse_raydium_v4_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<RaydiumPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let amm_info = match RaydiumAmmInfo::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if amm_info.coin_mint != *mint && amm_info.pc_mint != *mint {
            warn!("Mint {} not present in Raydium pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        // Raydium V4 stores one token as "coin" and the other as "pc" (price currency).
        // The vault assignment mirrors every other parser in this file: SOL (the quote
        // currency this system arbs against) is identified explicitly by checking which
        // side matches SOL_MINT. Without this explicit check, a USDC/TOKEN or USDT/TOKEN
        // pool would enter the else branch and silently assign the wrong vault — the
        // speculative-token vault would end up in sol_vault and the USDC vault in
        // token_vault, inverting the accounts the instruction builder expects.
        let (sol_vault, token_vault) = if sol == amm_info.coin_mint {
            (amm_info.coin_vault, amm_info.pc_vault)
        } else if sol == amm_info.pc_mint {
            (amm_info.pc_vault, amm_info.coin_vault)
        } else {
            // Neither token is SOL. The CLMM, CPMM, Whirlpool, Byreal, and PancakeSwap
            // parsers all reject non-SOL pools for the same reason: the instruction
            // builder's vault slot labelled "sol_vault" must hold the native-SOL token
            // account. USDC/USDT/USD1-paired V4 pools are therefore unsupported until
            // the instruction builder is extended to handle stablecoin quote sides.
            error!("SOL is not present in Raydium V4 pool {}", pool_addresses[idx]);
            continue;
        };

        let (token_mint, base_mint) = if *mint == amm_info.coin_mint {
            (amm_info.coin_mint, amm_info.pc_mint)
        } else {
            (amm_info.pc_mint, amm_info.coin_mint)
        };

        results.push(RaydiumPool {
            pool: pool_addresses[idx],
            token_vault,
            sol_vault,
            token_mint,
            base_mint,
        });

        // Per-pool confirmation is debug-level. At startup with thousands of V4 pools
        // this line would otherwise encode thousands of Pubkeys to base58 at info level,
        // adding hundreds of milliseconds of pure formatting overhead before the
        // validator is ready to trade. Use RUST_LOG=debug to enable per-pool tracing.
        debug!("Raydium pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_raydium_cpmm_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<RaydiumCpPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let amm_info = match RaydiumCpAmmInfo::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if amm_info.token_0_mint != *mint && amm_info.token_1_mint != *mint {
            warn!("Mint {} not present in Raydium CP pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (sol_vault, token_vault) = if sol == amm_info.token_0_mint {
            (amm_info.token_0_vault, amm_info.token_1_vault)
        } else if sol == amm_info.token_1_mint {
            (amm_info.token_1_vault, amm_info.token_0_vault)
        } else {
            error!("SOL is not present in Raydium CP pool {}", pool_addresses[idx]);
            continue;
        };

        let (token_mint, base_mint) = if *mint == amm_info.token_0_mint {
            (amm_info.token_0_mint, amm_info.token_1_mint)
        } else {
            (amm_info.token_1_mint, amm_info.token_0_mint)
        };

        results.push(RaydiumCpPool {
            pool: pool_addresses[idx],
            token_vault,
            sol_vault,
            amm_config: amm_info.amm_config,
            observation: amm_info.observation_key,
            token_mint,
            base_mint,
        });

        debug!("Raydium CP pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

// memo_program is Some(memo_pubkey) when the mint uses Token-2022, otherwise None.
// The Raydium CLMM on-chain program mandates the SPL Memo program's presence in the
// account list for Token-2022 transfers because Token-2022's transfer-hook extension
// requires an attached memo as part of its CPI chain. Without it the transaction fails
// with an account-not-found error inside the transfer hook.
pub fn parse_raydium_clmm_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    memo_program: Option<Pubkey>,
) -> Result<Vec<RaydiumClmmPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let raydium_clmm_prog_id = raydium_clmm_program_id();
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let pool_state = match PoolState::load_checked(account.data()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if pool_state.token_mint_0 != *mint && pool_state.token_mint_1 != *mint {
            warn!("Mint {} not present in Raydium CLMM pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        // CLMM vaults are ordered token_0 and token_1. Whichever side is SOL is sol_vault.
        let (token_vault, sol_vault) = if sol == pool_state.token_mint_0 {
            (pool_state.token_vault_1, pool_state.token_vault_0)
        } else if sol == pool_state.token_mint_1 {
            (pool_state.token_vault_0, pool_state.token_vault_1)
        } else {
            error!("SOL is not present in Raydium CLMM pool {}", pool_addresses[idx]);
            continue;
        };

        // The tick arrays covering offsets [-1, 0, +1] relative to the current tick are the
        // three pages most likely to be touched by a swap. The on-chain executor always
        // receives exactly three tick array accounts in this order.
        let tick_arrays = match get_tick_array_pubkeys(
            &pool_addresses[idx],
            pool_state.tick_current,
            pool_state.tick_spacing,
            &[-1, 0, 1],
            &raydium_clmm_prog_id,
        ) {
            Ok(arrays) => arrays,
            Err(e) => {
                error!("Error calculating tick arrays for Raydium CLMM pool {}: {:?}", pool_addresses[idx], e);
                continue;
            }
        };

        let (token_mint, base_mint) = if *mint == pool_state.token_mint_0 {
            (pool_state.token_mint_0, pool_state.token_mint_1)
        } else {
            (pool_state.token_mint_1, pool_state.token_mint_0)
        };

        results.push(RaydiumClmmPool {
            pool: pool_addresses[idx],
            amm_config: pool_state.amm_config,
            observation_state: pool_state.observation_key,
            x_vault: token_vault,
            y_vault: sol_vault,
            // tick_arrays is the sole owner of this Vec at this point — get_tick_array_pubkeys
            // returned it by value and it is not referenced elsewhere in this iteration.
            // Moving here rather than cloning eliminates a Vec<Pubkey> heap allocation per pool.
            tick_arrays,
            // memo_program propagated from the mint-level token program detection performed
            // in pool_discovery before this function is called.
            memo_program,
            token_mint,
            base_mint,
            bitmap_extension: {
                use crate::mev::dex::raydium::clmm_info::POOL_TICK_ARRAY_BITMAP_SEED;
                Pubkey::find_program_address(
                    &[POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(), pool_addresses[idx].as_ref()],
                    &raydium_clmm_prog_id,
                ).0
            },
        });

        debug!("Raydium CLMM pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_pump_swap_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    wallet_pubkey: &Pubkey,
) -> Result<Vec<PumpPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    // Pre-compute PDAs that are the same for every pool under this executor wallet.
    //
    // pump_pda_authority is the program that owns the Pump AMM PDA accounts.  It is
    // distinct from pump_program_id() which is the swap instruction program.
    //
    // Doing this once outside the loop (rather than once per pool inside the loop) means
    // the 256-SHA256 find_program_address search for global_volume_accumulator and
    // user_volume_accumulator runs exactly once regardless of how many Pump pools are
    // parsed for this mint.
    let pump_pda_authority =
        solana_pubkey::pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

    let (global_volume_accumulator, _) = Pubkey::find_program_address(
        &[b"global_volume_accumulator"],
        &pump_pda_authority,
    );

    let (user_volume_accumulator, _) = Pubkey::find_program_address(
        &[b"user_volume_accumulator", wallet_pubkey.as_ref()],
        &pump_pda_authority,
    );

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let amm_info = match PumpAmmInfo::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if amm_info.base_mint != *mint && amm_info.quote_mint != *mint {
            warn!("Mint {} not present in Pump pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (sol_vault, token_vault) = if sol == amm_info.base_mint {
            (amm_info.pool_base_token_account, amm_info.pool_quote_token_account)
        } else if sol == amm_info.quote_mint {
            (amm_info.pool_quote_token_account, amm_info.pool_base_token_account)
        } else {
            (amm_info.pool_quote_token_account, amm_info.pool_base_token_account)
        };

        // Pump's mayhem mode uses a different fee wallet than standard mode. The fee_token_wallet
        // is always the fee wallet's ATA for the quote mint (which is SOL/wSOL in practice).
        let (fee_wallet, fee_token_wallet) = if amm_info.is_mayhem_mode {
            let wallet = pump_mayhem_fee_wallet();
            (
                wallet,
                get_associated_token_address(
                    &wallet,
                    &amm_info.quote_mint,
                ),
            )
        } else {
            let wallet = pump_fee_wallet();
            (
                wallet,
                get_associated_token_address(
                    &wallet,
                    &amm_info.quote_mint,
                ),
            )
        };

        // The coin creator receives a portion of fees through their vault ATA for the quote mint.
        let coin_creator_vault_ata = get_associated_token_address(
            &amm_info.coin_creator_vault_authority,
            &amm_info.quote_mint,
        );

        let (token_mint, base_mint) = if *mint == amm_info.base_mint {
            (amm_info.base_mint, amm_info.quote_mint)
        } else {
            (amm_info.quote_mint, amm_info.base_mint)
        };

        // pool_v2 is per-pool (keyed on the speculative token mint). Derived here once
        // at parse time so the instruction builder never calls find_program_address at
        // simulation time.
        let pool_v2 = Pubkey::find_program_address(
            &[b"pool-v2", token_mint.as_ref()],
            &pump_pda_authority,
        )
        .0;

        results.push(PumpPool {
            pool: pool_addresses[idx],
            token_vault,
            sol_vault,
            fee_wallet,
            fee_token_wallet,
            coin_creator_vault_ata,
            coin_creator_vault_authority: amm_info.coin_creator_vault_authority,
            token_mint,
            base_mint,
            is_mayhem_mode: amm_info.is_mayhem_mode,
            // When the token opts into the cashback program, every swap must carry two extra
            // accounts so the protocol can credit wSOL back to the user's volume accumulator.
            is_cashback_coin: amm_info.is_cashback_coin,
            global_volume_accumulator,
            user_volume_accumulator,
            pool_v2,
        });

        debug!("Pump pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_meteora_damm_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<MeteoraDAmmPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        // Meteora DAMM pool state is serialized with Anchor's `deserialize_unchecked`,
        // which skips the eight-byte discriminator check and decodes fields sequentially
        // using a `buf: &mut &[u8]` cursor. The cursor is a mutable reference to a slice
        // reference: Anchor advances the inner `&[u8]` pointer past each field as it reads,
        // so the caller's slice shrinks to reflect consumed bytes. `AccountSharedData::data()`
        // already returns a `&[u8]` borrow into the validator's in-memory accounts store;
        // binding it to a `mut` local variable yields the `&mut &[u8]` that `deserialize_unchecked`
        // requires without allocating a new Vec or copying the raw bytes.
        let mut slice = account.data();
        let pool = match meteora_damm_cpi::Pool::deserialize_unchecked(&mut slice) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if pool.token_a_mint != *mint && pool.token_b_mint != *mint {
            warn!("Mint {} not present in Meteora DAMM pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        if pool.token_a_mint != sol && pool.token_b_mint != sol {
            warn!("SOL not present in Meteora DAMM pool {}, skipping", pool_addresses[idx]);
            continue;
        }

        // Meteora DAMM uses a two-level vault structure: the pool holds a_vault and b_vault
        // which are Meteora Vault program accounts. Each vault account in turn holds a
        // token_vault (the actual SPL token account holding tokens), lp_mint (for liquidity
        // tracking), and admin fee accounts. These sub-accounts must be fetched from the bank
        // individually because they are stored under the Vault program, not the DAMM program.
        let (x_vault, sol_vault) = if sol == pool.token_a_mint {
            (pool.b_vault, pool.a_vault)
        } else {
            (pool.a_vault, pool.b_vault)
        };

        let x_vault_account = match bank.get_account(&x_vault) {
            Some(acc) => acc,
            None => {
                error!("Could not find Meteora DAMM x_vault {} for pool {}", x_vault, pool_addresses[idx]);
                continue;
            }
        };

        let sol_vault_account = match bank.get_account(&sol_vault) {
            Some(acc) => acc,
            None => {
                error!("Could not find Meteora DAMM sol_vault {} for pool {}", sol_vault, pool_addresses[idx]);
                continue;
            }
        };

        // The Meteora Vault program's `Vault` struct is also Anchor-serialized, so the same
        // `&mut &[u8]` cursor pattern applies. Each vault account is borrowed directly from
        // `AccountSharedData` — no intermediate Vec copy needed for either the token vault
        // or the SOL vault deserialization.
        let mut x_slice = x_vault_account.data();
        let x_vault_obj = match meteora_vault_cpi::Vault::deserialize_unchecked(&mut x_slice) {
            Ok(v) => v,
            Err(e) => {
                error!("Error deserializing Meteora DAMM x_vault {}: {:?}", x_vault, e);
                continue;
            }
        };

        let mut sol_slice = sol_vault_account.data();
        let sol_vault_obj = match meteora_vault_cpi::Vault::deserialize_unchecked(&mut sol_slice) {
            Ok(v) => v,
            Err(e) => {
                error!("Error deserializing Meteora DAMM sol_vault {}: {:?}", sol_vault, e);
                continue;
            }
        };

        let x_token_vault = x_vault_obj.token_vault;
        let sol_token_vault = sol_vault_obj.token_vault;

        let x_lp_mint = x_vault_obj.lp_mint;
        let sol_lp_mint = sol_vault_obj.lp_mint;

        let (x_pool_lp, sol_pool_lp) = if sol == pool.token_a_mint {
            (pool.b_vault_lp, pool.a_vault_lp)
        } else {
            (pool.a_vault_lp, pool.b_vault_lp)
        };

        let (x_admin_fee, sol_admin_fee) = if sol == pool.token_a_mint {
            (pool.admin_token_b_fee, pool.admin_token_a_fee)
        } else {
            (pool.admin_token_a_fee, pool.admin_token_b_fee)
        };

        let (token_mint, base_mint) = if *mint == pool.token_a_mint {
            (pool.token_a_mint, pool.token_b_mint)
        } else {
            (pool.token_b_mint, pool.token_a_mint)
        };

        results.push(MeteoraDAmmPool {
            pool: pool_addresses[idx],
            token_x_vault: x_vault,
            token_sol_vault: sol_vault,
            token_x_token_vault: x_token_vault,
            token_sol_token_vault: sol_token_vault,
            token_x_lp_mint: x_lp_mint,
            token_sol_lp_mint: sol_lp_mint,
            token_x_pool_lp: x_pool_lp,
            token_sol_pool_lp: sol_pool_lp,
            admin_token_fee_x: x_admin_fee,
            admin_token_fee_sol: sol_admin_fee,
            token_mint,
            base_mint,
        });

        debug!("Meteora DAMM pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_meteora_damm_v2_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<MeteoraDAmmV2Pool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let info = match MeteoraDAmmV2Info::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if info.base_mint != *mint && info.quote_mint != *mint {
            warn!("Mint {} not present in Meteora DAMM V2 pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let token_x_vault = if sol == info.base_mint {
            info.quote_vault
        } else {
            info.base_vault
        };

        let token_sol_vault = if sol == info.base_mint {
            info.base_vault
        } else {
            info.quote_vault
        };

        let (token_mint, base_mint) = if *mint == info.base_mint {
            (info.base_mint, info.quote_mint)
        } else {
            (info.quote_mint, info.base_mint)
        };

        results.push(MeteoraDAmmV2Pool {
            pool: pool_addresses[idx],
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });

        debug!("Meteora DAMM V2 pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

// memo_program is Some when the mint uses Token-2022. Meteora DLMM requires the SPL Memo
// program for Token-2022 transfers for the same reason as Raydium CLMM — the token's
// transfer hook extension chains through Memo during CPI.
pub fn parse_meteora_dlmm_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    memo_program: Option<Pubkey>,
) -> Result<Vec<DlmmPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let amm_info = match DlmmInfo::load_checked(account.data()) {
            Ok(info) => info,
            Err(_) => continue,
        };

        if amm_info.token_x_mint != *mint && amm_info.token_y_mint != *mint {
            warn!("Mint {} not present in DLMM pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_vault, sol_vault) = amm_info.get_token_and_sol_vaults(mint, &sol);

        // Bin arrays are computed from the active bin index at parse time. At execution time
        // the instruction builder re-reads the live active bin from the bank to get the
        // most current bin arrays, so these stored arrays serve only as a fallback seed.
        let bin_arrays = match amm_info.calculate_bin_arrays(&pool_addresses[idx]) {
            Ok(arrays) => arrays,
            Err(e) => {
                error!("Error calculating bin arrays for DLMM pool {}: {:?}", pool_addresses[idx], e);
                continue;
            }
        };

        let (token_mint, base_mint) = if *mint == amm_info.token_x_mint {
            (amm_info.token_x_mint, amm_info.token_y_mint)
        } else {
            (amm_info.token_y_mint, amm_info.token_x_mint)
        };

        results.push(DlmmPool {
            pair: pool_addresses[idx],
            token_vault,
            sol_vault,
            oracle: amm_info.oracle,
            bin_arrays,
            memo_program,
            token_mint,
            base_mint,
        });

        debug!("DLMM pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

// Whirlpool always requires the SPL Memo program regardless of token standard — the
// Orca program unconditionally includes a memo CPI in every swap. memo_program is
// therefore always Some(memo_pubkey) for any Whirlpool, never None.
pub fn parse_orca_whirlpool_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    memo_program: Option<Pubkey>,
) -> Result<Vec<WhirlpoolPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let whirlpool_prog_id = whirlpool_program_id();
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        // Orca's `try_deserialize` validates the Anchor eight-byte discriminator stamped
        // at the start of the account data before decoding the Whirlpool fields. Its
        // `buf: &mut &[u8]` cursor parameter advances through the raw bytes as each field
        // is read. `AccountSharedData::data()` returns a direct `&[u8]` borrow from the
        // validator's in-memory store; binding it to a `mut` variable produces the
        // `&mut &[u8]` cursor without allocating a new Vec or copying the account bytes.
        let mut slice = account.data();
        let whirlpool = match Whirlpool::try_deserialize(&mut slice) {
            Ok(w) => w,
            Err(_) => continue,
        };

        if whirlpool.token_mint_a != *mint && whirlpool.token_mint_b != *mint {
            warn!("Mint {} not present in Whirlpool pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (sol_vault, token_vault) = if sol == whirlpool.token_mint_a {
            (whirlpool.token_vault_a, whirlpool.token_vault_b)
        } else if sol == whirlpool.token_mint_b {
            (whirlpool.token_vault_b, whirlpool.token_vault_a)
        } else {
            error!("SOL is not present in Whirlpool pool {}", pool_addresses[idx]);
            continue;
        };

        // Whirlpool's oracle PDA is derived from the pool address with the "oracle" seed.
        // The oracle is writable in swaps — the program updates TWAP data on every trade.
        let oracle = Pubkey::find_program_address(
            &[b"oracle", pool_addresses[idx].as_ref()],
            &whirlpool_prog_id,
        ).0;

        // update_tick_array_accounts_for_onchain produces exactly 3 tick array accounts
        // covering the current tick's array and one array to each side. Three is the number
        // the on-chain executor expects; providing more or fewer causes account index misalignment.
        let whirlpool_tick_arrays = update_tick_array_accounts_for_onchain(
            &whirlpool,
            &pool_addresses[idx],
            &whirlpool_prog_id,
        );

        let tick_arrays: Vec<Pubkey> = whirlpool_tick_arrays
            .iter()
            .map(|meta| meta.pubkey)
            .collect();

        let (token_mint, base_mint) = if *mint == whirlpool.token_mint_a {
            (whirlpool.token_mint_a, whirlpool.token_mint_b)
        } else {
            (whirlpool.token_mint_b, whirlpool.token_mint_a)
        };

        results.push(WhirlpoolPool {
            pool: pool_addresses[idx],
            oracle,
            x_vault: token_vault,
            y_vault: sol_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
        });

        debug!("Whirlpool pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

// memo_program is Some when the mint uses Token-2022. Byreal shares Raydium CLMM's
// PoolState layout and the same Token-2022 Memo requirement.
pub fn parse_byreal_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    memo_program: Option<Pubkey>,
) -> Result<Vec<ByrealPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let byreal_prog_id = byreal_program_id();
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        // Byreal shares Raydium's PoolState layout — same struct, different program ID.
        let pool_state = match PoolState::load_checked(account.data()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if pool_state.token_mint_0 != *mint && pool_state.token_mint_1 != *mint {
            warn!("Mint {} not present in Byreal pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_vault, sol_vault) = if sol == pool_state.token_mint_0 {
            (pool_state.token_vault_1, pool_state.token_vault_0)
        } else if sol == pool_state.token_mint_1 {
            (pool_state.token_vault_0, pool_state.token_vault_1)
        } else {
            error!("SOL is not present in Byreal pool {}", pool_addresses[idx]);
            continue;
        };

        let tick_arrays = match get_tick_array_pubkeys(
            &pool_addresses[idx],
            pool_state.tick_current,
            pool_state.tick_spacing,
            &[-1, 0, 1],
            &byreal_prog_id,
        ) {
            Ok(arrays) => arrays,
            Err(e) => {
                error!("Error calculating tick arrays for Byreal pool {}: {:?}", pool_addresses[idx], e);
                continue;
            }
        };

        let (token_mint, base_mint) = if *mint == pool_state.token_mint_0 {
            (pool_state.token_mint_0, pool_state.token_mint_1)
        } else {
            (pool_state.token_mint_1, pool_state.token_mint_0)
        };

        results.push(ByrealPool {
            pool: pool_addresses[idx],
            amm_config: pool_state.amm_config,
            observation_state: pool_state.observation_key,
            x_vault: token_vault,
            y_vault: sol_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
            bitmap_extension: {
                const SEED: &str = "pool_tick_array_bitmap_extension";
                Pubkey::find_program_address(
                    &[SEED.as_bytes(), pool_addresses[idx].as_ref()],
                    &byreal_prog_id,
                ).0
            },
        });

        debug!("Byreal pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

// memo_program is Some when the mint uses Token-2022. PancakeSwap shares Raydium CLMM's
// PoolState layout and the same Token-2022 Memo requirement.
pub fn parse_pancakeswap_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
    memo_program: Option<Pubkey>,
) -> Result<Vec<PancakeswapPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let pancakeswap_prog_id = pancakeswap_program_id();
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        // PancakeSwap shares Raydium's PoolState layout — same struct, different program ID.
        let pool_state = match PoolState::load_checked(account.data()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if pool_state.token_mint_0 != *mint && pool_state.token_mint_1 != *mint {
            warn!("Mint {} not present in PancakeSwap pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_vault, sol_vault) = if sol == pool_state.token_mint_0 {
            (pool_state.token_vault_1, pool_state.token_vault_0)
        } else if sol == pool_state.token_mint_1 {
            (pool_state.token_vault_0, pool_state.token_vault_1)
        } else {
            error!("SOL is not present in PancakeSwap pool {}", pool_addresses[idx]);
            continue;
        };

        let tick_arrays = match get_tick_array_pubkeys(
            &pool_addresses[idx],
            pool_state.tick_current,
            pool_state.tick_spacing,
            &[-1, 0, 1],
            &pancakeswap_prog_id,
        ) {
            Ok(arrays) => arrays,
            Err(e) => {
                error!("Error calculating tick arrays for PancakeSwap pool {}: {:?}", pool_addresses[idx], e);
                continue;
            }
        };

        let (token_mint, base_mint) = if *mint == pool_state.token_mint_0 {
            (pool_state.token_mint_0, pool_state.token_mint_1)
        } else {
            (pool_state.token_mint_1, pool_state.token_mint_0)
        };

        results.push(PancakeswapPool {
            pool: pool_addresses[idx],
            amm_config: pool_state.amm_config,
            observation_state: pool_state.observation_key,
            x_vault: token_vault,
            y_vault: sol_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
            bitmap_extension: {
                const SEED: &str = "pool_tick_array_bitmap_extension";
                Pubkey::find_program_address(
                    &[SEED.as_bytes(), pool_addresses[idx].as_ref()],
                    &pancakeswap_prog_id,
                ).0
            },
        });

        debug!("PancakeSwap pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_humidifi_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<HumidifiPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let info = match HumidifiInfo::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if info.base_mint != *mint && info.quote_mint != *mint {
            warn!("Mint {} not present in Humidifi pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_x_vault, token_sol_vault) = if sol == info.base_mint {
            (info.quote_vault, info.base_vault)
        } else {
            (info.base_vault, info.quote_vault)
        };

        let (token_mint, base_mint) = if *mint == info.base_mint {
            (info.base_mint, info.quote_mint)
        } else {
            (info.quote_mint, info.base_mint)
        };

        results.push(HumidifiPool {
            pool: pool_addresses[idx],
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });

        debug!("Humidifi pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_vertigo_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<VertigoPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let info = match VertigoInfo::load_checked(account.data(), &pool_addresses[idx]) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if info.mint_a != *mint && info.mint_b != *mint {
            warn!("Mint {} not present in Vertigo pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        // Vertigo vaults are PDAs derived from the pool address and the respective mint.
        // derive_vault_address encapsulates that PDA derivation. The naming convention is
        // "token_x" for the non-SOL token and "token_sol" for the SOL side.
        let non_base_vault = if *mint == info.mint_a {
            derive_vault_address(&pool_addresses[idx], &info.mint_b).0
        } else {
            derive_vault_address(&pool_addresses[idx], &info.mint_a).0
        };

        let base_vault = if *mint == info.mint_a {
            derive_vault_address(&pool_addresses[idx], &info.mint_a).0
        } else {
            derive_vault_address(&pool_addresses[idx], &info.mint_b).0
        };

        let token_x_vault = base_vault;
        let token_sol_vault = non_base_vault;

        let (token_mint, base_mint) = if *mint == info.mint_a {
            (info.mint_a, info.mint_b)
        } else {
            (info.mint_b, info.mint_a)
        };

        results.push(VertigoPool {
            pool: pool_addresses[idx],
            pool_owner: info.pool,
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });

        debug!("Vertigo pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_heaven_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<HeavenPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let info = match HeavenPoolState::parse(account.data()) {
            Some(i) => i,
            None => {
                warn!("Failed to parse Heaven pool {}, skipping", pool_addresses[idx]);
                continue;
            }
        };

        if info.mint_a != *mint && info.mint_b != *mint {
            warn!("Mint {} not present in Heaven pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_x_vault, token_base_vault) = if *mint == info.mint_a {
            (info.vault_a, info.vault_b)
        } else {
            (info.vault_b, info.vault_a)
        };

        let (token_mint, base_mint) = if *mint == info.mint_a {
            (info.mint_a, info.mint_b)
        } else {
            (info.mint_b, info.mint_a)
        };

        results.push(HeavenPool {
            pool: pool_addresses[idx],
            protocol_config: info.protocol_config,
            token_x_vault,
            token_base_vault,
            token_mint,
            base_mint,
            // Heaven pools use standard SPL Token. `spl_token_interface::id()` returns the
            // classic token program ID sourced from the `spl-token-interface` crate — a
            // dedicated interface crate that exposes only program IDs and type definitions
            // without pulling in the heavy implementation code. The `Pubkey` type it returns
            // is the same workspace-versioned type used throughout this codebase, so no
            // type coercion is needed. The field is retained for forward-compatibility if
            // Heaven ever extends support to Token-2022 vaults.
            token_program: spl_token_interface::id(),
        });

        debug!("Heaven pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}

pub fn parse_futarchy_pools(
    bank: &Arc<Bank>,
    pool_addresses: &[Pubkey],
    mint: &Pubkey,
) -> Result<Vec<FutarchyPool>> {
    if pool_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let sol = SOL_MINT;
    let mut results = Vec::new();

    for (idx, pubkey) in pool_addresses.iter().enumerate() {
        let account = match bank.get_account(pubkey) {
            Some(acc) => acc,
            None => continue,
        };

        let info = match FutarchyInfo::load_checked(account.data()) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if info.base_mint != *mint && info.quote_mint != *mint {
            warn!("Mint {} not present in Futarchy pool {}, skipping", mint, pool_addresses[idx]);
            continue;
        }

        let (token_x_vault, token_sol_vault) = if sol == info.base_mint {
            (info.quote_vault, info.base_vault)
        } else {
            (info.base_vault, info.quote_vault)
        };

        let (token_mint, base_mint) = if *mint == info.base_mint {
            (info.base_mint, info.quote_mint)
        } else {
            (info.quote_mint, info.base_mint)
        };

        results.push(FutarchyPool {
            // Futarchy identifies its pool by the DAO account, not a separate pool address.
            dao: pool_addresses[idx],
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });

        debug!("Futarchy pool added: {}", pool_addresses[idx]);
    }

    Ok(results)
}
