use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::clock::Clock;
use std::collections::HashMap;
use std::str::FromStr;
use bytemuck;
use tracing::{info, warn};

use meteora_dlmm_commons::{get_bin_array_pubkeys_for_swap, quote_exact_in};
use meteora_dlmm_commons::dlmm::accounts::{BinArray, BinArrayBitmapExtension, LbPair};

use crate::account_map::AccountMap;

// Meteora DLMM mainnet program ID.
const DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

// Seed used to derive the bitmap extension PDA.
const BITMAP_EXTENSION_SEED: &[u8] = b"bitmap_extension";

pub fn calculate_meteora_dlmm_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,   // lb_pair address
    slot:           u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    info!("🔍 DLMM calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

    // ── lb_pair state ────────────────────────────────────────────────────────

    let lb_pair_account = match accounts.get_account(pool_address) {
        Some(acc) => { info!("  ✅ DLMM lb_pair account found"); acc }
        None      => { warn!("  ❌ DLMM lb_pair account missing: {}", pool_address); return Ok(0); }
    };

    if lb_pair_account.data.len() < 8 + std::mem::size_of::<LbPair>() {
        warn!("  ❌ DLMM invalid lb_pair data length");
        return Ok(0);
    }

    let lb_pair: LbPair =
        bytemuck::pod_read_unaligned(&lb_pair_account.data[8..8 + std::mem::size_of::<LbPair>()]);
    info!("  ✅ DLMM lb_pair deserialized: active_id={}", lb_pair.active_id);

    // ── mints — keys come from lb_pair struct fields ─────────────────────────

    let mint_x_account = match accounts.get_account(&lb_pair.token_x_mint) {
        Some(acc) => { info!("  ✅ DLMM mint_x found"); acc }
        None      => { warn!("  ❌ DLMM mint_x missing"); return Ok(0); }
    };

    let mint_y_account = match accounts.get_account(&lb_pair.token_y_mint) {
        Some(acc) => { info!("  ✅ DLMM mint_y found"); acc }
        None      => { warn!("  ❌ DLMM mint_y missing"); return Ok(0); }
    };

    let swap_for_y = *token_in == lb_pair.token_x_mint;
    info!("  🔄 DLMM swap direction: swap_for_y={}", swap_for_y);

    // ── bitmap extension — PDA derived from pool_address ────────────────────
    //
    // The shard fetches this account and includes it in the AccountMap.
    // We derive the same PDA here to look it up.

    let dlmm_program_id = Pubkey::from_str(DLMM_PROGRAM_ID)
        .expect("DLMM_PROGRAM_ID is a valid constant");

    let (bitmap_extension_pda, _) = Pubkey::find_program_address(
        &[BITMAP_EXTENSION_SEED, pool_address.as_ref()],
        &dlmm_program_id,
    );

    let bitmap_extension = match accounts.get_account(&bitmap_extension_pda) {
        Some(acc) if acc.data.len() >= 8 + std::mem::size_of::<BinArrayBitmapExtension>() => {
            let ext: BinArrayBitmapExtension =
                bytemuck::pod_read_unaligned(&acc.data[8..8 + std::mem::size_of::<BinArrayBitmapExtension>()]);
            info!("  ✅ DLMM bitmap extension deserialized");
            Some(ext)
        }
        Some(_) => {
            warn!("  ⚠️ DLMM bitmap extension data too small");
            None
        }
        None => {
            info!("  ℹ️ DLMM bitmap extension not in accounts");
            None
        }
    };

    // ── bin arrays — keys derived by meteora_dlmm_commons ───────────────────

    let left_bin_array_pubkeys = match get_bin_array_pubkeys_for_swap(
        *pool_address,
        &lb_pair,
        bitmap_extension.as_ref(),
        true,
        3,
    ) {
        Ok(keys) => {
            info!("  ✅ DLMM derived {} left bin array pubkeys (active_id={})", keys.len(), lb_pair.active_id);
            keys
        }
        Err(e) => { warn!("  ❌ DLMM failed to get left bin array pubkeys: {}", e); return Ok(0); }
    };

    let right_bin_array_pubkeys = match get_bin_array_pubkeys_for_swap(
        *pool_address,
        &lb_pair,
        bitmap_extension.as_ref(),
        false,
        3,
    ) {
        Ok(keys) => {
            info!("  ✅ DLMM derived {} right bin array pubkeys", keys.len());
            keys
        }
        Err(e) => { warn!("  ❌ DLMM failed to get right bin array pubkeys: {}", e); return Ok(0); }
    };

    let all_bin_array_keys: Vec<Pubkey> = left_bin_array_pubkeys
        .into_iter()
        .chain(right_bin_array_pubkeys)
        .collect();

    let mut bin_arrays: HashMap<Pubkey, BinArray> = HashMap::new();

    for key in all_bin_array_keys {
        match accounts.get_account(&key) {
            Some(acc) if acc.data.len() >= 8 + std::mem::size_of::<BinArray>() => {
                let bin_array: BinArray =
                    bytemuck::pod_read_unaligned(&acc.data[8..8 + std::mem::size_of::<BinArray>()]);
                info!("  ✅ DLMM bin array loaded: {}", key);
                bin_arrays.insert(key, bin_array);
            }
            Some(_) => { warn!("  ⚠️ DLMM bin array data too small: {}", key); }
            None    => { warn!("  ⚠️ DLMM bin array not in accounts: {}", key); }
        }
    }

    if bin_arrays.is_empty() {
        warn!("  ❌ DLMM no bin arrays loaded");
        return Ok(0);
    }
    info!("  ✅ DLMM loaded {} bin arrays", bin_arrays.len());

    // ── clock — assembled from wire params ──────────────────────────────────
    // epoch is unused in DLMM swap math; set to 0.

    let sysvar_clock = Clock {
        slot,
        epoch_start_timestamp: 0,
        epoch:                  0,
        leader_schedule_epoch:  0,
        unix_timestamp:         unix_timestamp as i64,
    };

    // ── quote ────────────────────────────────────────────────────────────────
    // quote_exact_in takes &Account for mint_x/y to read decimals.
    // Our AccountMap::get_account() returns owned solana_sdk::account::Account.

    let quote_result = match quote_exact_in(
        *pool_address,
        &lb_pair,
        amount_in,
        swap_for_y,
        bin_arrays,
        bitmap_extension.as_ref(),
        &sysvar_clock,
        &mint_x_account,
        &mint_y_account,
    ) {
        Ok(result) => { info!("  ✅ DLMM quote success: amount_out={}", result.amount_out); result }
        Err(e)     => { warn!("  ❌ DLMM quote failed: {}", e); return Ok(0); }
    };

    info!("✅ DLMM output: {}", quote_result.amount_out);
    info!("  📊 Details: fee={}", quote_result.fee);

    Ok(quote_result.amount_out)
}
