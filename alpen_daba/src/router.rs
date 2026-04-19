// src/router.rs
//
// Dispatches a single hop to the correct simulator based on DexKind.
// Called twice per TwoHopSimRequest (once per hop), with hop1_out
// forwarded as amount_in for hop2.

use solana_sdk::pubkey::Pubkey;
use tracing::warn;

use crate::account_map::AccountMap;

// DexKind is defined in the sim-client crate alongside the wire types.
// The validator encodes each hop's DEX identity as a DexKind discriminant
// before sending the request across the socket. The router reads that
// discriminant here and branches to the correct simulator. Because both
// sides share the same DexKind definition from sim-client, the numeric
// discriminants are guaranteed to match — there is no risk of the validator
// encoding variant 3 while the router decodes it as a different variant.
use sim_client::DexKind;

use crate::dex::simulators::{
    calculate_byreal_clmm_output,
    calculate_meteora_dammv2_output,
    calculate_meteora_dlmm_output,
    calculate_orca_whirlpool_output,
    calculate_pancakeswap_output,
    calculate_pump_swap_output,
    calculate_raydium_amm_output,
    calculate_raydium_clmm_output,
    calculate_raydium_cp_output,
};

/// Run one hop of a two-hop simulation.
///
/// Returns 0 on any error — the caller treats 0 as "path is dead".
pub fn dispatch(
    dex:       DexKind,
    accounts:  &AccountMap,
    pool:      &[u8; 32],
    slot:      u64,
    timestamp: u64,
    amount_in: u64,
    token_in:  &[u8; 32],
) -> u64 {
    let pool_key     = Pubkey::new_from_array(*pool);
    let token_in_key = Pubkey::new_from_array(*token_in);

    let result = match dex {
        DexKind::OrcaWhirlpool => calculate_orca_whirlpool_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::RaydiumClmm => calculate_raydium_clmm_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::RaydiumCp => calculate_raydium_cp_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::RaydiumAmmV4 => calculate_raydium_amm_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::MeteoraDammV2 => calculate_meteora_dammv2_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::MeteoraDlmm => calculate_meteora_dlmm_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::PumpAmm => calculate_pump_swap_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        DexKind::ByrealClmm => calculate_byreal_clmm_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
        // PancakeSwap CLMM shares the same on-chain state layout as Raydium
        // CLMM — the simulator delegates to the same raydium_clmm SDK crate
        // and differs only in the program ID used for PDA derivation.
        DexKind::PancakeSwap => calculate_pancakeswap_output(
            accounts, &pool_key, slot, timestamp, amount_in, &token_in_key,
        ),
    };

    match result {
        Ok(out) => out,
        Err(e) => {
            warn!("sim dispatch error [{:?}] pool={}: {}", dex, pool_key, e);
            0
        }
    }
}
