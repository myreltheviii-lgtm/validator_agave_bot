use anyhow::{anyhow, Result};
use solana_sdk::pubkey::Pubkey;
use std::mem;
use bytemuck;
use tracing::{info, warn};

use raydium_amm::state::AmmInfo;
use raydium_amm::math::{Calculator, CheckedCeilDiv, SwapDirection, U128};
use anchor_lang::prelude::*;

// anchor_spl::token_interface accepts both the classic SPL Token program
// (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) and the Token-2022 program
// (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb) as valid account owners.
//
// Raydium AMM v4 vaults are classic SPL Token in practice, but using
// token_interface here is consistent with every other simulator in this crate
// and adds owner validation that the raw byte offset read did not perform —
// a completely wrong account at the vault address would previously have
// returned a garbage u64 with no error.
use anchor_spl::token_interface::TokenAccount;

use crate::account_map::AccountMap;

pub fn calculate_raydium_amm_output(
    accounts:     &AccountMap,
    pool_address: &Pubkey,
    _slot:        u64,
    _timestamp:   u64,
    amount_in:    u64,
    token_in:     &Pubkey,
) -> Result<u64> {
    info!("🔍 RAYDIUM AMM calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    // ── pool state ──────────────────────────────────────────────────────────

    let pool_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ RAYDIUM AMM pool account found"); acc }
        None      => { warn!("  ❌ RAYDIUM AMM pool account missing: {}", pool_address); return Ok(0); }
    };

    if pool_account.data.len() < mem::size_of::<AmmInfo>() {
        warn!("  ❌ RAYDIUM AMM invalid pool account data length");
        return Ok(0);
    }

    let amm: &AmmInfo = match bytemuck::try_from_bytes(&pool_account.data[..mem::size_of::<AmmInfo>()]) {
        Ok(a)  => { info!("  ✅ RAYDIUM AMM pool deserialized"); a }
        Err(e) => { warn!("  ❌ RAYDIUM AMM failed to deserialize pool: {:?}", e); return Ok(0); }
    };

    // Status 6 (SwapOnly) and 7 (SwapOnlyNoLP) are the only states where swaps
    // work without orderbook involvement.
    let status_value = amm.status;
    if status_value != 6 && status_value != 7 {
        warn!("  ⚠️ RAYDIUM AMM pool has orderbook (status={}), skipping", status_value);
        return Ok(0);
    }
    info!("  ✅ RAYDIUM AMM safe non-orderbook pool (status={})", status_value);

    // ── swap direction ──────────────────────────────────────────────────────

    // Explicit three-way match: token_in must equal one of the two pool mints.
    // An else branch that silently falls through to the second direction would
    // compute a valid-looking but wrong amount_out if token_in is neither mint —
    // caused by a routing bug, stale pool data, or a wrong pool address.
    // Returning Ok(0) on mismatch kills the path instead of feeding garbage
    // into the AMM curve.
    let swap_direction = if *token_in == amm.coin_vault_mint {
        SwapDirection::Coin2PC
    } else if *token_in == amm.pc_vault_mint {
        SwapDirection::PC2Coin
    } else {
        warn!("  ❌ RAYDIUM AMM token_in matches neither pool mint");
        return Ok(0);
    };
    info!("  🔄 RAYDIUM AMM swap direction: {:?}", swap_direction);

    // ── vaults — keys read from AmmInfo struct fields ────────────────────────
    //
    // AmmInfo exposes the vault pubkeys as `coin_vault` and `pc_vault`
    // (see state.rs lines 681-683). No separate pool entry is needed.

    let coin_vault_account = match accounts.get_account(&amm.coin_vault) {
        Some(acc) => { info!("  ✅ RAYDIUM AMM coin vault found"); acc }
        None      => { warn!("  ❌ RAYDIUM AMM coin vault missing: {}", amm.coin_vault); return Ok(0); }
    };

    let pc_vault_account = match accounts.get_account(&amm.pc_vault) {
        Some(acc) => { info!("  ✅ RAYDIUM AMM pc vault found"); acc }
        None      => { warn!("  ❌ RAYDIUM AMM pc vault missing: {}", amm.pc_vault); return Ok(0); }
    };

    // TokenAccount::try_deserialize from token_interface validates the account
    // owner against both the classic SPL Token program ID and the Token-2022
    // program ID, and fully unpacks the account layout. The previous raw byte
    // offset read (data[64..72]) bypassed owner validation entirely — a wrong
    // account at the vault address would have silently returned a garbage amount.
    let coin_vault: TokenAccount = match TokenAccount::try_deserialize(&mut &coin_vault_account.data[..]) {
        Ok(acc) => { info!("  ✅ RAYDIUM AMM coin vault deserialized: balance={}", acc.amount); acc }
        Err(e)  => { warn!("  ❌ RAYDIUM AMM failed to deserialize coin vault: {:?}", e); return Ok(0); }
    };

    let pc_vault: TokenAccount = match TokenAccount::try_deserialize(&mut &pc_vault_account.data[..]) {
        Ok(acc) => { info!("  ✅ RAYDIUM AMM pc vault deserialized: balance={}", acc.amount); acc }
        Err(e)  => { warn!("  ❌ RAYDIUM AMM failed to deserialize pc vault: {:?}", e); return Ok(0); }
    };

    let coin_vault_amount = coin_vault.amount;
    let pc_vault_amount   = pc_vault.amount;

    info!("  ✅ RAYDIUM AMM vault amounts: coin={}, pc={}", coin_vault_amount, pc_vault_amount);

    // ── swap math ───────────────────────────────────────────────────────────

    let total_pc_without_take_pnl = pc_vault_amount
        .checked_sub(amm.state_data.need_take_pnl_pc)
        .ok_or_else(|| anyhow!("PC vault underflow"))?;

    let total_coin_without_take_pnl = coin_vault_amount
        .checked_sub(amm.state_data.need_take_pnl_coin)
        .ok_or_else(|| anyhow!("Coin vault underflow"))?;

    if total_pc_without_take_pnl == 0 || total_coin_without_take_pnl == 0 {
        warn!("  ❌ RAYDIUM AMM zero liquidity");
        return Ok(0);
    }
    info!("  ✅ RAYDIUM AMM liquidity: pc={}, coin={}", total_pc_without_take_pnl, total_coin_without_take_pnl);

    let swap_fee = U128::from(amount_in)
        .checked_mul(amm.fees.swap_fee_numerator.into())
        .ok_or_else(|| anyhow!("Fee multiplication overflow"))?
        .checked_ceil_div(amm.fees.swap_fee_denominator.into())
        .ok_or_else(|| anyhow!("Fee division failed"))?;

    let swap_in_after_deduct_fee = U128::from(amount_in)
        .checked_sub(swap_fee)
        .ok_or_else(|| anyhow!("Fee subtraction underflow"))?;

    let swap_amount_out = Calculator::swap_token_amount_base_in(
        swap_in_after_deduct_fee,
        total_pc_without_take_pnl.into(),
        total_coin_without_take_pnl.into(),
        swap_direction,
    )
    .as_u64();

    if swap_amount_out == 0 {
        warn!("  ❌ RAYDIUM AMM zero output");
        return Ok(0);
    }

    // A quote is only actionable if the pool's output vault actually holds enough
    // tokens to pay it. The constant-product curve can return a valid number for a
    // drained or heavily imbalanced pool — this check catches that before the
    // quote reaches the executor. Every other simulator in this crate performs
    // this check; AMM v4 reads both vaults for the curve inputs so the data is
    // already in scope at no extra cost.
    let output_vault_balance = match swap_direction {
        SwapDirection::Coin2PC => total_pc_without_take_pnl,
        SwapDirection::PC2Coin => total_coin_without_take_pnl,
    };

    if output_vault_balance < swap_amount_out {
        warn!(
            "  ❌ RAYDIUM AMM insufficient vault balance: vault={}, needed={}",
            output_vault_balance, swap_amount_out
        );
        return Ok(0);
    }

    info!("✅ RAYDIUM AMM output: {}", swap_amount_out);
    info!(
        "  📊 Details: fee={}, amount_after_fee={}",
        swap_fee.as_u64(),
        swap_in_after_deduct_fee.as_u64()
    );

    Ok(swap_amount_out)
}
