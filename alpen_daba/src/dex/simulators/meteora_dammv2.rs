use anyhow::Result;
use tracing::{info, warn};

use meteora_cp_amm::state::fee::FeeMode;
use meteora_cp_amm::state::{Pool as MeteoraPool, PoolStatus};
use meteora_cp_amm::params::swap::TradeDirection as MeteoraTradeDirection;
use anchor_lang::prelude::*;

// anchor_spl::token_interface accepts both the classic SPL Token program
// (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) and the Token-2022 program
// (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb) as valid account owners.
//
// The classic token::TokenAccount::try_deserialize validates the account owner
// against the classic program ID exclusively. If the vault happens to be owned
// by Token-2022 the check fails and the deserializer returns an error, causing
// the simulator to silently return Ok(0) and miss what may be a valid arb path.
//
// Meteora DAMM v2 vaults can be owned by either program because pool creators
// choose the token program at pool creation time. Using token_interface here
// makes the deserializer accept both owner IDs without any branching in our code.
use anchor_spl::token_interface::TokenAccount;

use solana_sdk::pubkey::Pubkey;

use crate::account_map::AccountMap;

pub fn calculate_meteora_dammv2_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,
    slot:           u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    info!("🔍 DAMMV2 calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    if amount_in == 0 {
        warn!("  ❌ DAMMV2 amount is zero");
        return Ok(0);
    }

    // ── pool state ──────────────────────────────────────────────────────────

    let pool_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ DAMMV2 pool account found"); acc }
        None      => { warn!("  ❌ DAMMV2 pool account missing: {}", pool_address); return Ok(0); }
    };

    let pool_state: MeteoraPool =
        bytemuck::pod_read_unaligned(&pool_account.data[8..8 + std::mem::size_of::<MeteoraPool>()]);
    info!("  ✅ DAMMV2 pool state deserialized");

    if pool_state.pool_status != PoolStatus::Enable as u8 {
        warn!("  ❌ DAMMV2 pool disabled");
        return Ok(0);
    }

    if pool_state.liquidity == 0 {
        warn!("  ❌ DAMMV2 zero liquidity");
        return Ok(0);
    }
    info!("  ✅ DAMMV2 liquidity: {}", pool_state.liquidity);

    // ── activation check — slot or timestamp from wire params ───────────────

    let current_point = match pool_state.activation_type {
        0 => {
            info!("  ✅ DAMMV2 activation type: Slot, current={}", slot);
            slot
        }
        1 => {
            info!("  ✅ DAMMV2 activation type: Timestamp, current={}", unix_timestamp);
            unix_timestamp
        }
        _ => {
            warn!("  ❌ DAMMV2 invalid activation type: {}", pool_state.activation_type);
            return Ok(0);
        }
    };

    if current_point < pool_state.activation_point {
        warn!(
            "  ❌ DAMMV2 pool not activated yet: current={}, activation={}",
            current_point, pool_state.activation_point
        );
        return Ok(0);
    }
    info!("  ✅ DAMMV2 pool is activated");

    // ── trade direction ─────────────────────────────────────────────────────

    // Explicit three-way match: token_in must equal one of the two pool mints.
    // An else branch that silently falls through to the second direction would
    // compute a valid-looking but wrong amount_out if token_in is neither mint —
    // caused by a routing bug, stale pool data, or a wrong pool address.
    // Returning Ok(0) on mismatch kills the path instead of feeding garbage
    // into the swap math.
    let trade_direction = if *token_in == pool_state.token_a_mint {
        MeteoraTradeDirection::AtoB
    } else if *token_in == pool_state.token_b_mint {
        MeteoraTradeDirection::BtoA
    } else {
        warn!("  ❌ DAMMV2 token_in does not match pool mints");
        return Ok(0);
    };
    info!("  🔄 DAMMV2 direction: {:?}", trade_direction);

    // ── vaults — keys come directly from pool state struct fields ───────────

    let vault_a_account = match accounts.get_account(&pool_state.token_a_vault) {
        Some(acc) => acc,
        None      => { warn!("  ❌ DAMMV2 vault A missing"); return Ok(0); }
    };

    let vault_b_account = match accounts.get_account(&pool_state.token_b_vault) {
        Some(acc) => acc,
        None      => { warn!("  ❌ DAMMV2 vault B missing"); return Ok(0); }
    };

    // TokenAccount::try_deserialize from token_interface validates the account
    // owner against both the classic SPL Token program ID and the Token-2022
    // program ID. The classic token::TokenAccount would reject any vault owned
    // by Token-2022, returning an error that collapses to Ok(0) above and silently
    // kills the path. DAMM v2 vaults are created with whatever token program the
    // pool creator chose, so both programs are valid owners here.
    let vault_a: TokenAccount = match TokenAccount::try_deserialize(&mut &vault_a_account.data[..]) {
        Ok(acc) => { info!("  ✅ DAMMV2 vault A deserialized: balance={}", acc.amount); acc }
        Err(e)  => { warn!("  ❌ DAMMV2 failed to deserialize vault A: {:?}", e); return Ok(0); }
    };

    let vault_b: TokenAccount = match TokenAccount::try_deserialize(&mut &vault_b_account.data[..]) {
        Ok(acc) => { info!("  ✅ DAMMV2 vault B deserialized: balance={}", acc.amount); acc }
        Err(e)  => { warn!("  ❌ DAMMV2 failed to deserialize vault B: {:?}", e); return Ok(0); }
    };

    // ── fee mode ────────────────────────────────────────────────────────────

    let fee_mode = match FeeMode::get_fee_mode(pool_state.collect_fee_mode, trade_direction, false) {
        Ok(fm) => {
            info!("  ✅ DAMMV2 fee mode: fees_on_input={}, fees_on_token_a={}", fm.fees_on_input, fm.fees_on_token_a);
            fm
        }
        Err(e) => { warn!("  ❌ DAMMV2 failed to get fee mode: {:?}", e); return Ok(0); }
    };

    // ── output vault balance check ───────────────────────────────────────────

    let output_vault_balance = if trade_direction == MeteoraTradeDirection::AtoB {
        vault_b.amount
    } else {
        vault_a.amount
    };

    // ── swap ─────────────────────────────────────────────────────────────────

    let swap_result = match pool_state.get_swap_result_from_exact_input(
        amount_in,
        &fee_mode,
        trade_direction,
        current_point,
    ) {
        Ok(result) => { info!("  ✅ DAMMV2 swap success: output={}", result.output_amount); result }
        Err(e)     => { warn!("  ❌ DAMMV2 swap calculation failed: {:?}", e); return Ok(0); }
    };

    if swap_result.output_amount == 0 {
        warn!("  ❌ DAMMV2 zero output");
        return Ok(0);
    }

    // A quote is only actionable if the pool's output vault actually holds enough
    // tokens to pay it. The swap math can produce a valid number for a drained or
    // heavily imbalanced pool — this check catches that before the quote reaches
    // the executor.
    if output_vault_balance < swap_result.output_amount {
        warn!(
            "  ❌ DAMMV2 insufficient vault balance: vault={}, needed={}",
            output_vault_balance, swap_result.output_amount
        );
        return Ok(0);
    }

    info!("✅ DAMMV2 output: {}", swap_result.output_amount);
    info!(
        "  📊 Details: trading_fee={}, protocol_fee={}, next_sqrt_price={}",
        swap_result.trading_fee, swap_result.protocol_fee, swap_result.next_sqrt_price
    );

    Ok(swap_result.output_amount)
}
