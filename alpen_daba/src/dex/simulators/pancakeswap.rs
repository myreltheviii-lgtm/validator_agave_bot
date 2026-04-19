// src/dex/simulators/pancakeswap.rs
//
// PancakeSwap CLMM output estimator.
//
// PancakeSwap launched a Concentrated Liquidity Market Maker on Solana that
// shares the same on-chain state layout as Raydium CLMM — identical account
// structs, identical tick-array derivation seeds, identical swap math, and the
// same Anchor-compatible discriminator format. Because of this the raydium_clmm
// SDK crate can be used directly for all state deserialization and swap
// simulation; the only thing that differs from the Raydium CLMM simulator is
// the program ID used to derive PDAs.
//
// The public surface is a single function that matches the standard simulator
// API used by every DEX in `dex::simulators`:
//
//   pub fn calculate_pancakeswap_output(
//       accounts:        &AccountMap,
//       pool_address:    &Pubkey,
//       _slot:           u64,
//       unix_timestamp:  u64,
//       amount_in:       u64,
//       token_in:        &Pubkey,
//   ) -> Result<u64>

use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::collections::VecDeque;
use bytemuck;
use tracing::{info, warn};

use raydium_clmm::states::{
    AmmConfig         as PancakeswapConfig,
    ObservationState  as PancakeswapObservation,
    PoolState         as PancakeswapClmmPool,
    PoolStatusBitIndex,
    TickArrayBitmapExtension,
    TickArrayState    as PancakeswapTickArray,
    POOL_TICK_ARRAY_BITMAP_SEED,
    TICK_ARRAY_SEED,
};
use raydium_clmm::instructions::swap::swap_internal;
use raydium_clmm::libraries::tick_math::{MAX_SQRT_PRICE_X64, MIN_SQRT_PRICE_X64};
use anchor_lang::prelude::*;

// anchor_spl::token_interface accepts both the classic SPL Token program
// (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) and the Token-2022 program
// (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb) as valid account owners.
//
// PancakeSwap CLMM vaults can be owned by either program because the pool
// creator chooses the token program at initialization time. Using
// token_interface here makes the deserializer accept both owner IDs without
// any conditional logic, consistent with every other CLMM simulator in this
// crate.
use anchor_spl::token_interface::TokenAccount;

use solana_sdk::pubkey::Pubkey;

use crate::account_map::AccountMap;

// The on-chain address of the PancakeSwap CLMM program on Solana mainnet.
// Stored as a &str and parsed to Pubkey at call time via Pubkey::try_from —
// the same pattern used by every other simulator in this crate to avoid a
// dependency on the solana-pubkey crate for the pubkey! macro.
const PANCAKESWAP_CLMM_PROGRAM_ID: &str = "HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq";

pub fn calculate_pancakeswap_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,
    _slot:          u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    info!("🔍 PANCAKESWAP CLMM calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    let program_id = Pubkey::try_from(PANCAKESWAP_CLMM_PROGRAM_ID).unwrap();

    // ── pool state ──────────────────────────────────────────────────────────

    let pool_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ PANCAKESWAP CLMM pool account found"); acc }
        None      => { warn!("  ❌ PANCAKESWAP CLMM pool account missing: {}", pool_address); return Err(anyhow!("Pool account missing")); }
    };

    if pool_account.data.len() < 8 + std::mem::size_of::<PancakeswapClmmPool>() {
        warn!("  ❌ PANCAKESWAP CLMM invalid pool account data length");
        return Err(anyhow!("Invalid pool data length"));
    }

    let pool_state_data: PancakeswapClmmPool =
        match bytemuck::try_from_bytes(&pool_account.data[8..8 + std::mem::size_of::<PancakeswapClmmPool>()]) {
            Ok(p)  => { info!("  ✅ PANCAKESWAP CLMM pool state deserialized"); *p }
            Err(e) => { warn!("  ❌ PANCAKESWAP CLMM failed to deserialize pool: {:?}", e); return Err(anyhow!("Failed to deserialize pool")); }
        };

    if pool_state_data.liquidity == 0 {
        warn!("  ❌ PANCAKESWAP CLMM zero liquidity");
        return Err(anyhow!("Zero liquidity"));
    }

    if !pool_state_data.get_status_by_bit(PoolStatusBitIndex::Swap) {
        warn!("  ❌ PANCAKESWAP CLMM swaps disabled");
        return Err(anyhow!("Swaps disabled"));
    }

    // PancakeswapClmmPool is #[repr(C, packed)] — taking a reference to any of
    // its fields inside a macro that formats by reference triggers undefined
    // behaviour under Rust's alignment rules. Copying the primitive field values
    // into local stack variables first gives the formatter a properly aligned
    // reference to an ordinary stack slot.
    let open_time = pool_state_data.open_time;
    if unix_timestamp <= open_time {
        warn!("  ❌ PANCAKESWAP CLMM pool not open yet: current={}, open={}", unix_timestamp, open_time);
        return Err(anyhow!("Pool not open yet"));
    }
    let liquidity = pool_state_data.liquidity;
    info!("  ✅ PANCAKESWAP CLMM pool is open, liquidity={}", liquidity);

    // ── amm config — key from pool state struct field ───────────────────────

    let config_account = match accounts.get_account(&pool_state_data.amm_config) {
        Some(acc) => { info!("  ✅ PANCAKESWAP CLMM config found"); acc }
        None      => { warn!("  ❌ PANCAKESWAP CLMM config missing"); return Err(anyhow!("Config missing")); }
    };

    let config: PancakeswapConfig = match PancakeswapConfig::try_deserialize(&mut &config_account.data[..]) {
        Ok(c)  => { info!("  ✅ PANCAKESWAP CLMM config deserialized"); c }
        Err(e) => { warn!("  ❌ PANCAKESWAP CLMM failed to deserialize config: {:?}", e); return Err(anyhow!("Failed to deserialize config")); }
    };

    // ── direction and output vault ───────────────────────────────────────────

    // Explicit three-way match: token_in must equal one of the two pool mints.
    // An else branch that silently falls through to the second direction would
    // compute a valid-looking but wrong amount_out if token_in is neither mint —
    // caused by a routing bug, stale pool data, or a wrong pool address.
    // Returning an error on mismatch kills the path instead of feeding garbage
    // into the CLMM tick traversal.
    let zero_for_one = if *token_in == pool_state_data.token_mint_0 {
        true
    } else if *token_in == pool_state_data.token_mint_1 {
        false
    } else {
        warn!("  ❌ PANCAKESWAP CLMM token_in matches neither pool mint");
        return Err(anyhow!("token_in matches neither pool mint"));
    };
    info!("  🔄 PANCAKESWAP CLMM direction: zero_for_one={}", zero_for_one);

    let output_vault_key = if zero_for_one { &pool_state_data.token_vault_1 } else { &pool_state_data.token_vault_0 };

    let output_vault_account = match accounts.get_account(output_vault_key) {
        Some(acc) => { info!("  ✅ PANCAKESWAP CLMM output vault found"); acc }
        None      => { warn!("  ❌ PANCAKESWAP CLMM output vault missing"); return Err(anyhow!("Output vault missing")); }
    };

    // TokenAccount::try_deserialize from token_interface validates the account
    // owner against both the classic SPL Token program ID and the Token-2022
    // program ID. The classic token::TokenAccount would reject any vault owned
    // by Token-2022, returning an error that propagates up as a missed arb path.
    // PancakeSwap CLMM vaults are created with whatever token program the pool
    // creator chose, so both programs are valid owners here.
    let output_vault: TokenAccount =
        match TokenAccount::try_deserialize(&mut &output_vault_account.data[..]) {
            Ok(acc) => { info!("  ✅ PANCAKESWAP CLMM output vault deserialized"); acc }
            Err(e)  => { warn!("  ❌ PANCAKESWAP CLMM failed to deserialize output vault: {:?}", e); return Err(anyhow!("Failed to deserialize output vault")); }
        };

    // ── observation — key from pool state struct field ───────────────────────

    let observation_account = match accounts.get_account(&pool_state_data.observation_key) {
        Some(acc) => { info!("  ✅ PANCAKESWAP CLMM observation found"); acc }
        None      => { warn!("  ❌ PANCAKESWAP CLMM observation missing"); return Err(anyhow!("Observation missing")); }
    };

    if observation_account.data.len() < 8 + std::mem::size_of::<PancakeswapObservation>() {
        return Err(anyhow!("Invalid observation data length"));
    }

    let observation_data: PancakeswapObservation =
        match bytemuck::try_from_bytes(&observation_account.data[8..8 + std::mem::size_of::<PancakeswapObservation>()]) {
            Ok(o)  => { info!("  ✅ PANCAKESWAP CLMM observation deserialized"); *o }
            Err(e) => { warn!("  ❌ PANCAKESWAP CLMM failed to deserialize observation: {:?}", e); return Err(anyhow!("Failed to deserialize observation")); }
        };

    if observation_data.pool_id != *pool_address {
        warn!("  ❌ PANCAKESWAP CLMM observation pool_id mismatch");
        return Err(anyhow!("Observation pool_id mismatch"));
    }

    // ── bitmap extension — PDA derived ──────────────────────────────────────

    let tickarray_bitmap_extension_key = Pubkey::find_program_address(
        &[POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(), pool_address.as_ref()],
        &program_id,
    ).0;

    let mut tickarray_bitmap_extension: Option<TickArrayBitmapExtension> = None;

    if let Some(ext_acc) = accounts.get_account(&tickarray_bitmap_extension_key) {
        if ext_acc.data.len() >= 8 + std::mem::size_of::<TickArrayBitmapExtension>() {
            if let Ok(ext) = bytemuck::try_from_bytes::<TickArrayBitmapExtension>(
                &ext_acc.data[8..8 + std::mem::size_of::<TickArrayBitmapExtension>()],
            ) {
                info!("  ✅ PANCAKESWAP CLMM bitmap extension loaded");
                tickarray_bitmap_extension = Some(*ext);
            }
        }
    } else {
        info!("  ℹ️ PANCAKESWAP CLMM no bitmap extension");
    }

    // ── tick arrays ─────────────────────────────────────────────────────────

    let (_, first_valid_tick_array_start_index) =
        pool_state_data.get_first_initialized_tick_array(&tickarray_bitmap_extension, zero_for_one)
            .map_err(|e| anyhow!("Failed to get first tick array: {:?}", e))?;

    info!("  ✅ PANCAKESWAP CLMM first tick array start_index={}", first_valid_tick_array_start_index);

    let mut tick_array_indices = vec![first_valid_tick_array_start_index];
    let mut current_index = first_valid_tick_array_start_index;

    for _ in 0..10 {
        match pool_state_data.next_initialized_tick_array_start_index(
            &tickarray_bitmap_extension,
            current_index,
            zero_for_one,
        ).map_err(|e| anyhow!("Failed to get next tick array: {:?}", e))? {
            Some(next_index) if !tick_array_indices.contains(&next_index) => {
                tick_array_indices.push(next_index);
                current_index = next_index;
            }
            _ => break,
        }
    }

    info!("  ✅ PANCAKESWAP CLMM loading {} tick arrays", tick_array_indices.len());

    let mut tick_array_states_data = Vec::new();

    for tick_array_start_index in &tick_array_indices {
        let (tick_array_pda, _) = Pubkey::find_program_address(
            &[
                TICK_ARRAY_SEED.as_bytes(),
                pool_address.as_ref(),
                &tick_array_start_index.to_be_bytes(),
            ],
            &program_id,
        );

        let tick_array_account = match accounts.get_account(&tick_array_pda) {
            Some(acc) => acc,
            None      => { info!("  ⚠️ PANCAKESWAP CLMM tick array {} not in accounts, stopping", tick_array_start_index); break; }
        };

        if tick_array_account.data.len() < 8 + std::mem::size_of::<PancakeswapTickArray>() {
            break;
        }

        let tick_array_data: PancakeswapTickArray =
            match bytemuck::try_from_bytes(&tick_array_account.data[8..8 + std::mem::size_of::<PancakeswapTickArray>()]) {
                Ok(ta) => *ta,
                Err(_) => break,
            };

        if tick_array_data.pool_id != *pool_address {
            warn!("  ❌ PANCAKESWAP CLMM tick array pool_id mismatch for index {}", tick_array_start_index);
            return Err(anyhow!("Tick array pool_id mismatch"));
        }

        // PancakeswapTickArray is packed; copy the field to a stack local before
        // passing it to the format macro to avoid a misaligned reference.
        let expected_start = *tick_array_start_index;
        let actual_start   = tick_array_data.start_tick_index;
        if actual_start != expected_start {
            warn!(
                "  ❌ PANCAKESWAP CLMM tick array start_index mismatch: expected {}, got {}",
                expected_start, actual_start
            );
            return Err(anyhow!("Tick array start_index mismatch"));
        }

        tick_array_states_data.push(tick_array_data);
        info!("  ✅ PANCAKESWAP CLMM tick array {} loaded", tick_array_start_index);
    }

    if tick_array_states_data.is_empty() {
        warn!("  ❌ PANCAKESWAP CLMM no tick arrays loaded");
        return Err(anyhow!("No tick arrays loaded"));
    }

    // ── swap ─────────────────────────────────────────────────────────────────

    let pool_cell        = RefCell::new(pool_state_data);
    let observation_cell = RefCell::new(observation_data);
    let mut pool_borrow        = pool_cell.borrow_mut();
    let mut observation_borrow = observation_cell.borrow_mut();

    let tick_array_cells: Vec<RefCell<PancakeswapTickArray>> =
        tick_array_states_data.into_iter().map(RefCell::new).collect();
    let mut tick_array_vec = VecDeque::new();
    for cell in tick_array_cells.iter() {
        tick_array_vec.push_back(cell.borrow_mut());
    }

    let sqrt_price_limit = if zero_for_one { MIN_SQRT_PRICE_X64 + 1 } else { MAX_SQRT_PRICE_X64 - 1 };

    let swap_price_before = pool_borrow.sqrt_price_x64;

    let result = match swap_internal(
        &config,
        &mut pool_borrow,
        &mut tick_array_vec,
        &mut observation_borrow,
        &tickarray_bitmap_extension,
        amount_in,
        sqrt_price_limit,
        zero_for_one,
        true,
        unix_timestamp as u32,
    ) {
        Ok(r)  => { info!("  ✅ PANCAKESWAP CLMM swap simulation success"); r }
        Err(e) => { warn!("  ❌ PANCAKESWAP CLMM swap simulation failed: {:?}", e); return Err(anyhow!("Swap simulation failed: {:?}", e)); }
    };

    let (amount_0, amount_1) = result;

    if amount_0 == 0 || amount_1 == 0 {
        warn!("  ❌ PANCAKESWAP CLMM swap resulted in zero amounts");
        return Err(anyhow!("Swap resulted in zero amounts"));
    }

    let amount_out = if zero_for_one { amount_1 } else { amount_0 };

    // A quote is only actionable if the pool's output vault actually holds enough
    // tokens to pay it. The CLMM tick traversal can produce a valid number for a
    // drained or heavily imbalanced vault — this check catches that before the
    // quote reaches the executor.
    if output_vault.amount < amount_out {
        warn!("  ❌ PANCAKESWAP CLMM insufficient vault balance: vault={}, needed={}", output_vault.amount, amount_out);
        return Err(anyhow!("Insufficient vault balance"));
    }

    let swap_price_after = pool_borrow.sqrt_price_x64;

    // A CLMM swap always moves the price in one direction. If price went the
    // wrong way the swap_internal result is nonsensical — likely a tick array
    // ordering issue or a zero-liquidity range that produced no actual movement.
    if zero_for_one && swap_price_after >= swap_price_before {
        return Err(anyhow!("Price moved in wrong direction"));
    }
    if !zero_for_one && swap_price_after <= swap_price_before {
        return Err(anyhow!("Price moved in wrong direction"));
    }

    info!("✅ PANCAKESWAP CLMM output: {}", amount_out);
    info!("  📊 Price change: {} -> {}", swap_price_before, swap_price_after);

    Ok(amount_out)
}
