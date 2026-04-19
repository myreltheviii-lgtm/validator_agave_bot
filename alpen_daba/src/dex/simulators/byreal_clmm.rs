use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::cell::{RefCell, RefMut};
use std::mem;
use std::collections::VecDeque;
use std::str::FromStr;
use bytemuck;
use arrayref::array_ref;
use tracing::{info, warn};
use anchor_lang::AccountDeserialize;

// anchor_spl::token_interface accepts both the classic SPL Token program
// (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) and the Token-2022 program
// (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb) as valid account owners.
//
// Byreal CLMM vaults can be owned by either program because the pool creator
// chooses the token program at initialization time. Using token_interface here
// makes the deserializer accept both owner IDs without any conditional logic in
// our code, and validates the account owner field — a raw byte-offset read would
// silently return a garbage amount for any account that is not actually a token vault.
use anchor_spl::token_interface::TokenAccount;

use byreal_clmm::states::{
    AmmConfig as ByrealConfig,
    DynTickArrayState,
    ObservationState as ByrealObservation,
    PoolState as ByrealPoolState,
    PoolStatusBitIndex,
    TickArrayBitmapExtension,
    TickArrayContainerRefMut,
    TickArrayState as ByrealTickArray,
    TickState,
};
use byreal_clmm::instructions::swap::swap_internal;
use byreal_clmm::libraries::tick_math;

use crate::account_map::AccountMap;

const BYREAL_CLMM_PROGRAM_ID: &str = "REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2";
const TICK_ARRAY_SEED:               &[u8] = b"tick_array";
const OBSERVATION_SEED:              &[u8] = b"observation";
const POOL_TICK_ARRAY_BITMAP_SEED:   &str  = "pool_tick_array_bitmap_extension";
const MAX_TICK_ARRAYS_TO_LOAD:       usize = 20;

pub fn calculate_byreal_clmm_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,
    _slot:          u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    info!("🔍 BYREAL CLMM calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    if amount_in == 0 {
        warn!("  ❌ BYREAL CLMM amount_in is zero");
        return Ok(0);
    }

    let program_id = Pubkey::from_str(BYREAL_CLMM_PROGRAM_ID)
        .expect("BYREAL_CLMM_PROGRAM_ID is a valid constant");

    // ── pool state ──────────────────────────────────────────────────────────

    let pool_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ BYREAL CLMM pool account found"); acc }
        None      => { warn!("  ❌ BYREAL CLMM pool account missing: {}", pool_address); return Ok(0); }
    };

    if pool_account.data.len() < 8 + mem::size_of::<ByrealPoolState>() {
        warn!("  ❌ BYREAL CLMM invalid pool account data length: {}", pool_account.data.len());
        return Ok(0);
    }

    let pool_state_data: ByrealPoolState =
        match bytemuck::try_from_bytes(&pool_account.data[8..8 + mem::size_of::<ByrealPoolState>()]) {
            Ok(p)  => { info!("  ✅ BYREAL CLMM pool state deserialized"); *p }
            Err(e) => { warn!("  ❌ BYREAL CLMM failed to deserialize pool: {:?}", e); return Ok(0); }
        };

    if pool_state_data.liquidity == 0 {
        warn!("  ❌ BYREAL CLMM zero liquidity");
        return Ok(0);
    }

    if !pool_state_data.get_status_by_bit(PoolStatusBitIndex::Swap) {
        warn!("  ❌ BYREAL CLMM swap is disabled");
        return Ok(0);
    }

    if unix_timestamp <= pool_state_data.open_time {
        warn!(
            "  ❌ BYREAL CLMM pool not yet open: current={}, open_time={}",
            unix_timestamp, pool_state_data.open_time
        );
        return Ok(0);
    }

    info!(
        "  ✅ BYREAL CLMM pool is open — liquidity={}, tick_current={}",
        pool_state_data.liquidity, pool_state_data.tick_current
    );

    // ── amm config — key comes from pool state struct field ─────────────────

    let config_account = match accounts.get_account(&pool_state_data.amm_config) {
        Some(acc) => { info!("  ✅ BYREAL CLMM config account found"); acc }
        None      => { warn!("  ❌ BYREAL CLMM config missing: {}", pool_state_data.amm_config); return Ok(0); }
    };

    if config_account.data.len() < 8 {
        warn!("  ❌ BYREAL CLMM invalid config account data length: {}", config_account.data.len());
        return Ok(0);
    }

    let mut data_slice: &[u8] = &config_account.data;
    let config: ByrealConfig = match ByrealConfig::try_deserialize(&mut data_slice) {
        Ok(c)  => { info!("  ✅ BYREAL CLMM config deserialized"); c }
        Err(e) => { warn!("  ❌ BYREAL CLMM failed to deserialize config: {:?}", e); return Ok(0); }
    };

    // ── swap direction ───────────────────────────────────────────────────────

    // Explicit three-way match: token_in must equal one of the two pool mints.
    // An else branch that silently falls through to the second direction would
    // compute a valid-looking but wrong amount_out if token_in is neither mint —
    // caused by a routing bug, stale pool data, or a wrong pool address.
    // Returning Ok(0) on mismatch kills the path instead of feeding garbage
    // into the CLMM tick traversal.
    let zero_for_one = if *token_in == pool_state_data.token_mint_0 {
        true
    } else if *token_in == pool_state_data.token_mint_1 {
        false
    } else {
        warn!(
            "  ❌ BYREAL CLMM token_in={} matches neither mint_0={} nor mint_1={}",
            token_in, pool_state_data.token_mint_0, pool_state_data.token_mint_1
        );
        return Ok(0);
    };

    info!("  🔄 BYREAL CLMM direction: zero_for_one={}", zero_for_one);

    // ── output vault ─────────────────────────────────────────────────────────
    //
    // The output vault balance check is the final guard before returning a quote.
    // The CLMM tick traversal can produce a mathematically valid amount_out for
    // a drained or heavily imbalanced vault — the curve math has no knowledge of
    // the actual token balance sitting in the vault account. Without this check,
    // the executor would attempt a swap that the on-chain program would reject at
    // runtime because the vault cannot pay the quoted output, wasting the tx fee
    // and the slot position.

    let output_vault_key = if zero_for_one {
        &pool_state_data.token_vault_1
    } else {
        &pool_state_data.token_vault_0
    };

    let output_vault_account = match accounts.get_account(output_vault_key) {
        Some(acc) => { info!("  ✅ BYREAL CLMM output vault found: {}", output_vault_key); acc }
        None      => { warn!("  ❌ BYREAL CLMM output vault missing: {}", output_vault_key); return Ok(0); }
    };

    // TokenAccount::try_deserialize from token_interface validates the account
    // owner against both the classic SPL Token program ID and the Token-2022
    // program ID before unpacking. A raw byte-offset read (data[64..72]) would
    // bypass owner validation entirely — a wrong account at the vault address
    // would silently return a garbage balance and the swap math would proceed
    // on completely invalid inputs.
    let output_vault: TokenAccount = match TokenAccount::try_deserialize(&mut &output_vault_account.data[..]) {
        Ok(acc) => { info!("  ✅ BYREAL CLMM output vault deserialized: balance={}", acc.amount); acc }
        Err(e)  => { warn!("  ❌ BYREAL CLMM failed to deserialize output vault: {:?}", e); return Ok(0); }
    };

    // ── observation — PDA derived from pool_address ─────────────────────────

    let (observation_pda, _) =
        Pubkey::find_program_address(&[OBSERVATION_SEED, pool_address.as_ref()], &program_id);

    let observation_account = match accounts.get_account(&observation_pda) {
        Some(acc) => { info!("  ✅ BYREAL CLMM observation account found"); acc }
        None      => { warn!("  ❌ BYREAL CLMM observation missing: {}", observation_pda); return Ok(0); }
    };

    if observation_account.data.len() < 8 + mem::size_of::<ByrealObservation>() {
        warn!("  ❌ BYREAL CLMM invalid observation account data length: {}", observation_account.data.len());
        return Ok(0);
    }

    let observation_data: ByrealObservation =
        match bytemuck::try_from_bytes(&observation_account.data[8..8 + mem::size_of::<ByrealObservation>()]) {
            Ok(o)  => { info!("  ✅ BYREAL CLMM observation deserialized"); *o }
            Err(e) => { warn!("  ❌ BYREAL CLMM failed to deserialize observation: {:?}", e); return Ok(0); }
        };

    if observation_data.pool_id != *pool_address {
        warn!(
            "  ❌ BYREAL CLMM observation pool_id mismatch: expected={}, got={}",
            pool_address, observation_data.pool_id
        );
        return Ok(0);
    }

    info!("  ✅ BYREAL CLMM observation validated");

    // ── bitmap extension — PDA derived, conditionally fetched ───────────────
    //
    // The bitmap extension account holds tick availability flags for tick arrays
    // that fall outside the default bitmap range embedded in the pool state. It is
    // only required when the current tick is in the overflow region — i.e. when
    // the price has moved far enough from center that the pool's internal bitmap
    // can no longer describe the next initialized tick array. For pools trading
    // near the center of their range the extension is never needed, so we skip the
    // account fetch when is_overflow returns false.

    let bitmap_extension_pda = Pubkey::find_program_address(
        &[POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(), pool_address.as_ref()],
        &program_id,
    ).0;

    let is_overflow = pool_state_data.is_overflow_default_tickarray_bitmap(
        vec![pool_state_data.tick_current],
    );

    let bitmap_extension: Option<TickArrayBitmapExtension> = if is_overflow {
        match accounts.get_account(&bitmap_extension_pda) {
            Some(acc) if acc.data.len() >= 8 + mem::size_of::<TickArrayBitmapExtension>() => {
                match bytemuck::try_from_bytes::<TickArrayBitmapExtension>(
                    &acc.data[8..8 + mem::size_of::<TickArrayBitmapExtension>()],
                ) {
                    Ok(ext) => { info!("  ✅ BYREAL CLMM bitmap extension loaded"); Some(*ext) }
                    Err(_)  => { warn!("  ❌ BYREAL CLMM bitmap extension required but deserialization failed"); return Ok(0); }
                }
            }
            _ => { warn!("  ❌ BYREAL CLMM bitmap extension required but missing or too small"); return Ok(0); }
        }
    } else {
        info!("  ℹ️ BYREAL CLMM bitmap extension not needed (tick within default range)");
        None
    };

    // ── tick arrays ─────────────────────────────────────────────────────────
    //
    // Concentrated liquidity pools store liquidity in discrete price intervals
    // called tick arrays. A swap traverses arrays sequentially from the current
    // price toward the swap limit. Each array covers a contiguous range of ticks
    // and holds the liquidity deltas at each initialized boundary. We load up to
    // MAX_TICK_ARRAYS_TO_LOAD arrays starting from the first initialized one in
    // the swap direction — larger swaps that exhaust the near arrays fall through
    // to farther ones until the input is consumed or arrays run out.

    let (_, first_tick_array_start_index) =
        match pool_state_data.get_first_initialized_tick_array(&bitmap_extension, zero_for_one) {
            Ok(result) => { info!("  ✅ BYREAL CLMM first tick array start_index={}", result.1); result }
            Err(e)     => { warn!("  ❌ BYREAL CLMM failed to get first tick array: {:?}", e); return Ok(0); }
        };

    let tick_array_indices = match collect_tick_array_start_indices(
        &pool_state_data,
        &bitmap_extension,
        first_tick_array_start_index,
        zero_for_one,
        MAX_TICK_ARRAYS_TO_LOAD,
    ) {
        Ok(indices) => { info!("  ✅ BYREAL CLMM collected {} tick array indices", indices.len()); indices }
        Err(e)      => { warn!("  ❌ BYREAL CLMM failed to collect tick arrays: {:?}", e); return Ok(0); }
    };

    if tick_array_indices.is_empty() {
        warn!("  ❌ BYREAL CLMM no tick array indices found");
        return Ok(0);
    }

    info!("  ✅ BYREAL CLMM loading {} tick arrays", tick_array_indices.len());

    let mut tick_array_data_cells: Vec<RefCell<Vec<u8>>> = Vec::new();

    for &start_index in &tick_array_indices {
        let (tick_array_pda, _) = Pubkey::find_program_address(
            &[TICK_ARRAY_SEED, pool_address.as_ref(), &start_index.to_be_bytes()],
            &program_id,
        );

        match accounts.get_account(&tick_array_pda) {
            Some(acc) => {
                info!("  ✅ BYREAL CLMM tick array loaded: start_index={}, pda={}", start_index, tick_array_pda);
                tick_array_data_cells.push(RefCell::new(acc.data));
            }
            None => {
                warn!("  ⚠️ BYREAL CLMM tick array not found: start_index={}, pda={} — stopping load", start_index, tick_array_pda);
                break;
            }
        }
    }

    if tick_array_data_cells.is_empty() {
        warn!("  ❌ BYREAL CLMM no tick arrays loaded");
        return Ok(0);
    }

    info!("  ✅ BYREAL CLMM loaded {} tick arrays", tick_array_data_cells.len());

    let mut tick_array_containers: VecDeque<TickArrayContainerRefMut> = VecDeque::new();

    for data_cell in &tick_array_data_cells {
        match create_byreal_tick_array_container(data_cell) {
            Ok(container) => {
                if container.get_pool_id() != *pool_address {
                    warn!("  ❌ BYREAL CLMM tick array pool_id mismatch");
                    return Ok(0);
                }
                tick_array_containers.push_back(container);
            }
            Err(e) => {
                warn!("  ❌ BYREAL CLMM failed to create tick array container: {:?}", e);
                return Ok(0);
            }
        }
    }

    // ── swap ─────────────────────────────────────────────────────────────────

    let pool_cell        = RefCell::new(pool_state_data);
    let observation_cell = RefCell::new(observation_data);
    let mut pool_borrow        = pool_cell.borrow_mut();
    let mut observation_borrow = observation_cell.borrow_mut();

    // The sqrt price limit acts as a safety stop for the swap traversal.
    // Setting it to MIN+1 (when selling token_0) or MAX-1 (when selling token_1)
    // allows the swap to traverse the full tick range without hitting the limit
    // prematurely. The on-chain executor uses the same boundary values —
    // a more restrictive limit here would cause the simulator to stop early and
    // underestimate the output, producing false-negative profitability signals.
    let sqrt_price_limit = if zero_for_one {
        tick_math::MIN_SQRT_PRICE_X64 + 1
    } else {
        tick_math::MAX_SQRT_PRICE_X64 - 1
    };

    let swap_price_before = pool_borrow.sqrt_price_x64;

    info!("  ✅ BYREAL CLMM starting swap simulation");

    let result = match swap_internal(
        &config,
        &mut pool_borrow,
        &mut tick_array_containers,
        &mut observation_borrow,
        &bitmap_extension,
        amount_in,
        sqrt_price_limit,
        zero_for_one,
        true,
        unix_timestamp as u32,
    ) {
        Ok(r)  => { info!("  ✅ BYREAL CLMM swap simulation success"); r }
        Err(e) => { warn!("  ❌ BYREAL CLMM swap simulation failed: {:?}", e); return Ok(0); }
    };

    let (amount_0, amount_1) = result;

    if amount_0 == 0 || amount_1 == 0 {
        warn!("  ❌ BYREAL CLMM swap resulted in zero amounts: amount_0={}, amount_1={}", amount_0, amount_1);
        return Ok(0);
    }

    let amount_out = if zero_for_one { amount_1 } else { amount_0 };

    // A CLMM swap always moves the price in one direction. If the price moved the
    // wrong way the swap_internal result is nonsensical — likely a tick array
    // ordering issue or a zero-liquidity range that produced no actual movement.
    let swap_price_after = pool_borrow.sqrt_price_x64;

    if zero_for_one && swap_price_after >= swap_price_before {
        warn!(
            "  ❌ BYREAL CLMM price moved wrong direction (zero_for_one=true): before={}, after={}",
            swap_price_before, swap_price_after
        );
        return Ok(0);
    }
    if !zero_for_one && swap_price_after <= swap_price_before {
        warn!(
            "  ❌ BYREAL CLMM price moved wrong direction (zero_for_one=false): before={}, after={}",
            swap_price_before, swap_price_after
        );
        return Ok(0);
    }

    // A quote is only actionable if the pool's output vault actually holds enough
    // tokens to pay it. The CLMM tick traversal can produce a valid number for a
    // drained or heavily imbalanced vault — the curve math has no knowledge of the
    // actual token balance sitting in the vault account. This check catches that
    // before the quote reaches the executor, preventing a submission that the
    // on-chain program would reject due to insufficient vault balance.
    if output_vault.amount < amount_out {
        warn!(
            "  ❌ BYREAL CLMM insufficient vault balance: vault={}, needed={}",
            output_vault.amount, amount_out
        );
        return Ok(0);
    }

    info!("✅ BYREAL CLMM output: {}", amount_out);
    info!(
        "  📊 Details: amount_0={}, amount_1={}, price_before={}, price_after={}",
        amount_0, amount_1, swap_price_before, swap_price_after
    );

    Ok(amount_out)
}

// ─── helpers ──────────────────────────────────────────────────────────────────

fn collect_tick_array_start_indices(
    pool_state:        &ByrealPoolState,
    bitmap_extension:  &Option<TickArrayBitmapExtension>,
    first_start_index: i32,
    zero_for_one:      bool,
    max_count:         usize,
) -> Result<Vec<i32>> {
    let mut indices = vec![first_start_index];
    let mut current_index = first_start_index;

    for _ in 1..max_count {
        match pool_state.next_initialized_tick_array_start_index(
            bitmap_extension,
            current_index,
            zero_for_one,
        ) {
            Ok(Some(next_index)) => {
                if next_index < tick_math::MIN_TICK || next_index > tick_math::MAX_TICK {
                    break;
                }
                indices.push(next_index);
                current_index = next_index;
            }
            Ok(None) => break,
            Err(e)   => return Err(anyhow::anyhow!("Failed to find next tick array: {:?}", e)),
        }
    }

    Ok(indices)
}

fn create_byreal_tick_array_container<'a>(
    data_cell: &'a RefCell<Vec<u8>>,
) -> Result<TickArrayContainerRefMut<'a>> {
    let disc_bytes = {
        let data = data_cell.borrow();
        if data.len() < 8 {
            return Err(anyhow::anyhow!("Tick array data too small"));
        }
        *array_ref![data, 0, 8]
    };

    if disc_bytes == ByrealTickArray::DISCRIMINATOR {
        let tick_array_ref = RefMut::map(data_cell.borrow_mut(), |data| {
            bytemuck::from_bytes_mut(&mut data[8..8 + mem::size_of::<ByrealTickArray>()])
        });
        Ok(TickArrayContainerRefMut::Fixed(tick_array_ref))
    } else if disc_bytes == DynTickArrayState::DISCRIMINATOR {
        let data_len = data_cell.borrow().len();

        let (header, ticks) = RefMut::map_split(data_cell.borrow_mut(), |data| {
            let (header_bytes, ticks_bytes) = data.split_at_mut(DynTickArrayState::HEADER_LEN);
            let header: &mut DynTickArrayState = bytemuck::from_bytes_mut(&mut header_bytes[8..]);

            // bytemuck::try_cast_slice_mut fails when the byte slice length is not
            // a multiple of size_of::<TickState>(), or when the slice is not
            // properly aligned. Both conditions indicate malformed account data that
            // the on-chain program would also reject. Propagating the error through
            // the ? operator surfaces it at the call site where it is logged and
            // converted to Ok(0), rather than unwinding the thread via panic.
            // A panic here would crash the MEV execution thread inside the validator
            // binary, which shares a process with consensus threads.
            let ticks: &mut [TickState] = bytemuck::try_cast_slice_mut(ticks_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to cast to TickState slice: {:?}", e))?;
            (header, ticks)
        });

        if data_len != header.all_data_len() {
            return Err(anyhow::anyhow!(
                "Invalid dynamic tick array size: expected {}, got {}",
                header.all_data_len(),
                data_len
            ));
        }

        Ok(TickArrayContainerRefMut::Dynamic((header, ticks)))
    } else {
        Err(anyhow::anyhow!("Invalid tick array discriminator: {:?}", disc_bytes))
    }
}
