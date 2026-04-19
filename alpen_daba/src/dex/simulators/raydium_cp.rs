use anyhow::Result;
use bytemuck;
use tracing::{info, warn};

use raydium_cp_swap::curve::calculator::CurveCalculator;
use raydium_cp_swap::curve::TradeDirection;
use raydium_cp_swap::states::{AmmConfig as RaydiumCpConfig, PoolState as RaydiumCpPool, PoolStatusBitIndex};
use anchor_lang::prelude::*;
// anchor_spl::token_interface::TokenAccount accepts vault accounts owned by either the
// classic SPL Token program (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) or the
// Token-2022 program (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb).
//
// The classic anchor_spl::token::TokenAccount hard-codes the classic SPL Token program
// ID as the only valid account owner inside try_deserialize. On Solana, every token
// account carries the program that created it as its owner field. When the Raydium
// CP-Swap pool was initialized with Token-2022 mints, the runtime created the vault
// token accounts under the Token-2022 program — their owner field is the Token-2022
// program ID, not the classic one. The classic deserializer sees a foreign owner and
// returns an error, causing the simulator to log a warning and return Ok(0), making
// every Token-2022 CP-Swap pool permanently invisible to the arbitrage graph.
//
// Raydium CP-Swap deliberately supports both token programs: PoolState carries
// token_0_program and token_1_program fields so the on-chain program knows which
// CPI target to use per vault. The simulator does not need to branch on those fields
// because token_interface::TokenAccount's try_deserialize performs the owner check
// against both program IDs internally, accepting whichever one the vault was created
// under without any conditional logic on our side.
use anchor_spl::token_interface::TokenAccount;
use solana_sdk::pubkey::Pubkey;

use crate::account_map::AccountMap;

pub fn calculate_raydium_cp_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,
    _slot:          u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    info!("🔍 RAYDIUM CP calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    // ── pool state ──────────────────────────────────────────────────────────

    let pool_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ RAYDIUM CP pool account found"); acc }
        None      => { warn!("  ❌ RAYDIUM CP pool account missing: {}", pool_address); return Ok(0); }
    };

    let pool_state: RaydiumCpPool = match bytemuck::try_from_bytes(&pool_account.data[8..]) {
        Ok(p)  => { info!("  ✅ RAYDIUM CP pool state deserialized"); *p }
        Err(e) => { warn!("  ❌ RAYDIUM CP failed to deserialize pool: {:?}", e); return Ok(0); }
    };

    if !pool_state.get_status_by_bit(PoolStatusBitIndex::Swap) {
        warn!("  ❌ RAYDIUM CP swaps disabled");
        return Ok(0);
    }

    if unix_timestamp < pool_state.open_time {
        warn!("  ❌ RAYDIUM CP pool not open yet");
        return Ok(0);
    }

    // ── amm config ──────────────────────────────────────────────────────────

    let config_account = match accounts.get_account(&pool_state.amm_config) {
        Some(acc) => { info!("  ✅ RAYDIUM CP config found"); acc }
        None      => { warn!("  ❌ RAYDIUM CP config missing"); return Ok(0); }
    };

    let amm_config: RaydiumCpConfig = match RaydiumCpConfig::try_deserialize(&mut &config_account.data[..]) {
        Ok(c)  => { info!("  ✅ RAYDIUM CP config deserialized"); c }
        Err(e) => { warn!("  ❌ RAYDIUM CP failed to deserialize config: {:?}", e); return Ok(0); }
    };

    // ── vaults — keys come from pool state struct fields ────────────────────

    let is_token_0_input = *token_in == pool_state.token_0_mint;

    let input_vault_key  = if is_token_0_input { &pool_state.token_0_vault } else { &pool_state.token_1_vault };
    let output_vault_key = if is_token_0_input { &pool_state.token_1_vault } else { &pool_state.token_0_vault };

    let input_vault_account = match accounts.get_account(input_vault_key) {
        Some(acc) => { info!("  ✅ RAYDIUM CP input vault found"); acc }
        None      => { warn!("  ❌ RAYDIUM CP input vault missing"); return Ok(0); }
    };

    let output_vault_account = match accounts.get_account(output_vault_key) {
        Some(acc) => { info!("  ✅ RAYDIUM CP output vault found"); acc }
        None      => { warn!("  ❌ RAYDIUM CP output vault missing"); return Ok(0); }
    };

    let input_vault: TokenAccount = match TokenAccount::try_deserialize(&mut &input_vault_account.data[..]) {
        Ok(acc) => { info!("  ✅ RAYDIUM CP input vault deserialized"); acc }
        Err(e)  => { warn!("  ❌ RAYDIUM CP failed to deserialize input vault: {:?}", e); return Ok(0); }
    };

    let output_vault: TokenAccount = match TokenAccount::try_deserialize(&mut &output_vault_account.data[..]) {
        Ok(acc) => { info!("  ✅ RAYDIUM CP output vault deserialized"); acc }
        Err(e)  => { warn!("  ❌ RAYDIUM CP failed to deserialize output vault: {:?}", e); return Ok(0); }
    };

    // ── swap math ───────────────────────────────────────────────────────────

    let vault_0_amount = if is_token_0_input { input_vault.amount  } else { output_vault.amount };
    let vault_1_amount = if is_token_0_input { output_vault.amount } else { input_vault.amount  };

    let (vault_0_without_fee, vault_1_without_fee) = match pool_state.vault_amount_without_fee(vault_0_amount, vault_1_amount) {
        Ok(amounts) => { info!("  ✅ RAYDIUM CP vault amounts calculated"); amounts }
        Err(e)      => { warn!("  ❌ RAYDIUM CP failed to calculate vault amounts: {}", e); return Ok(0); }
    };

    let (input_vault_amount, output_vault_amount) = if is_token_0_input {
        (vault_0_without_fee, vault_1_without_fee)
    } else {
        (vault_1_without_fee, vault_0_without_fee)
    };

    if input_vault_amount == 0 || output_vault_amount == 0 {
        warn!("  ❌ RAYDIUM CP zero vault amounts");
        return Ok(0);
    }

    info!("  ✅ RAYDIUM CP vault amounts: input={}, output={}", input_vault_amount, output_vault_amount);

    let trade_direction = if is_token_0_input { TradeDirection::ZeroForOne } else { TradeDirection::OneForZero };

    let is_creator_fee_on_input = match pool_state.is_creator_fee_on_input(trade_direction) {
        Ok(flag) => { info!("  ✅ RAYDIUM CP creator fee on input: {}", flag); flag }
        Err(e)   => { warn!("  ❌ RAYDIUM CP failed to determine creator fee placement: {}", e); return Ok(0); }
    };

    let creator_fee_rate = pool_state.adjust_creator_fee_rate(amm_config.creator_fee_rate);

    info!(
        "  📊 RAYDIUM CP fees: trade={}, protocol={}, fund={}, creator={}",
        amm_config.trade_fee_rate, amm_config.protocol_fee_rate,
        amm_config.fund_fee_rate, creator_fee_rate
    );

    let result = match CurveCalculator::swap_base_input(
        amount_in as u128,
        input_vault_amount as u128,
        output_vault_amount as u128,
        amm_config.trade_fee_rate,
        creator_fee_rate,
        amm_config.protocol_fee_rate,
        amm_config.fund_fee_rate,
        is_creator_fee_on_input,
    ) {
        Some(r) => { info!("  ✅ RAYDIUM CP calculation success"); r }
        None    => { warn!("  ❌ RAYDIUM CP calculation returned None"); return Ok(0); }
    };

    let amount_out = result.output_amount as u64;

    if amount_out == 0 {
        warn!("  ❌ RAYDIUM CP zero output");
        return Ok(0);
    }

    if output_vault.amount < amount_out {
        warn!("  ❌ RAYDIUM CP insufficient vault balance: vault={}, needed={}", output_vault.amount, amount_out);
        return Ok(0);
    }

    info!("✅ RAYDIUM CP output: {}", amount_out);
    info!(
        "  📊 Details: trade_fee={}, protocol_fee={}, fund_fee={}, creator_fee={}",
        result.trade_fee, result.protocol_fee, result.fund_fee, result.creator_fee
    );

    Ok(amount_out)
}
