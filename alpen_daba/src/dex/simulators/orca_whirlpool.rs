use anyhow::{anyhow, Result};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::pubkey::Pubkey;
use tracing::{info, warn};

use orca_whirlpools::ID as WHIRLPOOL_PROGRAM_ID;
use orca_whirlpools::state::{AdaptiveFeeInfo, Oracle, Whirlpool, load_tick_array_mut};
use orca_whirlpools::util::SwapTickSequence;
use orca_whirlpools::manager::swap_manager::swap;
use anchor_lang::AccountDeserialize;

use crate::account_map::AccountMap;

const ORACLE_SEED:         &[u8] = b"oracle";
const NO_SQRT_PRICE_LIMIT: u128  = 0;
const TICK_ARRAY_SIZE:     i32   = 88;

struct OrcaWhirlpoolSimulator<'a> {
    accounts:       &'a AccountMap,
    unix_timestamp: u64,
}

impl<'a> OrcaWhirlpoolSimulator<'a> {
    fn new(accounts: &'a AccountMap, unix_timestamp: u64) -> Self {
        Self { accounts, unix_timestamp }
    }

    fn simulate_swap(&self, pool_address: &Pubkey, amount_in: u64, token_in: &Pubkey) -> Result<u64> {
        info!("🔍 WHIRLPOOL calc start: pool={}, amount_in={}, token_in={}", pool_address, amount_in, token_in);

        if amount_in == 0 {
            warn!("  ❌ WHIRLPOOL amount is zero");
            return Err(anyhow!("Zero amount_in"));
        }

        let whirlpool = self.load_whirlpool(pool_address)?;

        if whirlpool.liquidity == 0 {
            warn!("  ❌ WHIRLPOOL zero liquidity");
            return Err(anyhow!("Zero liquidity in pool"));
        }
        info!("  ✅ WHIRLPOOL liquidity: {}", whirlpool.liquidity);

        // Direction from whirlpool state struct fields — no caller params needed.
        let a_to_b = if *token_in == whirlpool.token_mint_a {
            info!("  🔄 WHIRLPOOL direction: A to B");
            true
        } else if *token_in == whirlpool.token_mint_b {
            info!("  🔄 WHIRLPOOL direction: B to A");
            false
        } else {
            warn!(
                "  ❌ WHIRLPOOL token not in pool: token_in={}, mint_a={}, mint_b={}",
                token_in, whirlpool.token_mint_a, whirlpool.token_mint_b
            );
            return Err(anyhow!("Token not in pool"));
        };

        let adaptive_fee_info = self.load_and_validate_oracle(pool_address)?;
        if adaptive_fee_info.is_some() {
            info!("  ✅ WHIRLPOOL adaptive fee info loaded");
        } else {
            info!("  ℹ️ WHIRLPOOL no adaptive fee info");
        }

        // ── tick arrays — PDAs derived internally ────────────────────────────

        let required_start_indexes = self.get_required_tick_array_start_indexes(&whirlpool, a_to_b);
        info!(
            "  ✅ WHIRLPOOL calculated {} tick array start indexes: {:?}",
            required_start_indexes.len(), required_start_indexes
        );

        let required_tick_arrays: Vec<Pubkey> = required_start_indexes
            .iter()
            .map(|&idx| self.derive_tick_array_pda(pool_address, idx))
            .collect();

        if required_tick_arrays.is_empty() {
            warn!("  ❌ WHIRLPOOL no tick arrays calculated");
            return Err(anyhow!("No tick arrays calculated"));
        }

        let mut data_0 = self
            .accounts
            .get_account(&required_tick_arrays[0])
            .ok_or_else(|| {
                warn!("  ❌ WHIRLPOOL tick array 0 missing: {}", required_tick_arrays[0]);
                anyhow!("Tick array 0 missing: {}", required_tick_arrays[0])
            })?
            .data;
        info!("  ✅ WHIRLPOOL tick array 0 loaded");

        let mut data_1 = if required_tick_arrays.len() > 1 {
            self.accounts
                .get_account(&required_tick_arrays[1])
                .map(|acc| { info!("  ✅ WHIRLPOOL tick array 1 loaded"); acc.data })
                .unwrap_or_else(|| { info!("  ℹ️ WHIRLPOOL tick array 1 not in accounts"); Vec::new() })
        } else {
            Vec::new()
        };

        let mut data_2 = if required_tick_arrays.len() > 2 {
            self.accounts
                .get_account(&required_tick_arrays[2])
                .map(|acc| { info!("  ✅ WHIRLPOOL tick array 2 loaded"); acc.data })
                .unwrap_or_else(|| { info!("  ℹ️ WHIRLPOOL tick array 2 not in accounts"); Vec::new() })
        } else {
            Vec::new()
        };

        let mut lamports_0 = 0u64;
        let mut lamports_1 = 0u64;
        let mut lamports_2 = 0u64;

        let account_info_0 = AccountInfo::new(
            &required_tick_arrays[0],
            false, true, &mut lamports_0, &mut data_0,
            &WHIRLPOOL_PROGRAM_ID, false, 0,
        );

        let loaded_0 = load_tick_array_mut(&account_info_0, pool_address)
            .map_err(|e| { warn!("  ❌ WHIRLPOOL failed to load tick array 0: {}", e); anyhow!("Failed to load tick array 0: {}", e) })?;
        info!("  ✅ WHIRLPOOL tick array 0 validated");

        let has_data_1 = !data_1.is_empty() && required_tick_arrays.len() > 1;
        let has_data_2 = !data_2.is_empty() && required_tick_arrays.len() > 2;

        let account_info_1 = AccountInfo::new(
            if required_tick_arrays.len() > 1 { &required_tick_arrays[1] } else { &required_tick_arrays[0] },
            false, true, &mut lamports_1, &mut data_1,
            &WHIRLPOOL_PROGRAM_ID, false, 0,
        );

        let account_info_2 = AccountInfo::new(
            if required_tick_arrays.len() > 2 { &required_tick_arrays[2] } else { &required_tick_arrays[0] },
            false, true, &mut lamports_2, &mut data_2,
            &WHIRLPOOL_PROGRAM_ID, false, 0,
        );

        let loaded_1 = if has_data_1 { load_tick_array_mut(&account_info_1, pool_address).ok() } else { None };
        let loaded_2 = if has_data_2 { load_tick_array_mut(&account_info_2, pool_address).ok() } else { None };

        if loaded_1.is_some() { info!("  ✅ WHIRLPOOL tick array 1 validated"); }
        if loaded_2.is_some() { info!("  ✅ WHIRLPOOL tick array 2 validated"); }

        let mut swap_tick_sequence = SwapTickSequence::new(loaded_0, loaded_1, loaded_2);

        info!("  ✅ WHIRLPOOL starting swap simulation");

        let result = swap(
            &whirlpool,
            &mut swap_tick_sequence,
            amount_in,
            NO_SQRT_PRICE_LIMIT,
            true,
            a_to_b,
            self.unix_timestamp,
            &adaptive_fee_info,
        ).map_err(|e| {
            warn!("  ❌ WHIRLPOOL swap simulation failed: {}", e);
            anyhow!("Swap simulation failed: {}", e)
        })?;

        let amount_out = if a_to_b { result.amount_b } else { result.amount_a };

        if amount_out == 0 {
            warn!("  ❌ WHIRLPOOL zero output");
            return Err(anyhow!("Swap resulted in zero output"));
        }

        info!("✅ WHIRLPOOL output: {}", amount_out);
        info!("  📊 Details: amount_a={}, amount_b={}", result.amount_a, result.amount_b);

        Ok(amount_out)
    }

    fn get_required_tick_array_start_indexes(&self, whirlpool: &Whirlpool, a_to_b: bool) -> Vec<i32> {
        let tick_current_index = whirlpool.tick_current_index;
        let tick_spacing        = whirlpool.tick_spacing as i32;
        let ticks_in_array      = TICK_ARRAY_SIZE * tick_spacing;

        let d = tick_current_index / ticks_in_array;
        let r = tick_current_index % ticks_in_array;
        let start_tick_index_base = if r < 0 { (d - 1) * ticks_in_array } else { d * ticks_in_array };

        let offset = if a_to_b {
            vec![0, -1, -2]
        } else {
            let shifted = tick_current_index + tick_spacing >= start_tick_index_base + ticks_in_array;
            if shifted { vec![1, 2, 3] } else { vec![0, 1, 2] }
        };

        let mut start_indexes = Vec::new();
        for o in offset {
            let idx = start_tick_index_base + o * ticks_in_array;
            if self.check_is_valid_start_tick(idx, whirlpool.tick_spacing, ticks_in_array) {
                start_indexes.push(idx);
            }
        }
        start_indexes
    }

    fn check_is_valid_start_tick(&self, tick_index: i32, _tick_spacing: u16, ticks_in_array: i32) -> bool {
        const MIN_TICK_INDEX: i32 = -443636;
        const MAX_TICK_INDEX: i32 =  443636;

        if tick_index < MIN_TICK_INDEX || tick_index > MAX_TICK_INDEX {
            if tick_index > MIN_TICK_INDEX { return false; }
            let min_array_start = MIN_TICK_INDEX - (MIN_TICK_INDEX % ticks_in_array + ticks_in_array);
            return tick_index == min_array_start;
        }
        tick_index % ticks_in_array == 0
    }

    fn derive_tick_array_pda(&self, pool_address: &Pubkey, start_tick_index: i32) -> Pubkey {
        Pubkey::find_program_address(
            &[b"tick_array", pool_address.as_ref(), start_tick_index.to_string().as_bytes()],
            &WHIRLPOOL_PROGRAM_ID,
        ).0
    }

    fn load_whirlpool(&self, pool_address: &Pubkey) -> Result<Whirlpool> {
        let pool_account = self.accounts.get_account(pool_address).ok_or_else(|| {
            warn!("  ❌ WHIRLPOOL pool account missing: {}", pool_address);
            anyhow!("Pool account missing: {}", pool_address)
        })?;

        if pool_account.data.len() < 8 {
            return Err(anyhow!("Invalid pool account data length: {}", pool_account.data.len()));
        }

        let mut data_slice = &pool_account.data[..];
        let whirlpool = Whirlpool::try_deserialize(&mut data_slice).map_err(|e| {
            warn!("  ❌ WHIRLPOOL failed to deserialize pool: {:?}", e);
            anyhow!("Failed to deserialize Whirlpool: {:?}", e)
        })?;

        info!("  ✅ WHIRLPOOL pool deserialized");
        Ok(whirlpool)
    }

    fn load_and_validate_oracle(&self, pool_address: &Pubkey) -> Result<Option<AdaptiveFeeInfo>> {
        let (oracle_pda, _) =
            Pubkey::find_program_address(&[ORACLE_SEED, pool_address.as_ref()], &WHIRLPOOL_PROGRAM_ID);

        let oracle_account = match self.accounts.get_account(&oracle_pda) {
            Some(acc) => { info!("  ✅ WHIRLPOOL oracle account found"); acc }
            None      => { info!("  ℹ️ WHIRLPOOL no oracle account"); return Ok(None); }
        };

        if oracle_account.data.len() < 8 {
            info!("  ℹ️ WHIRLPOOL oracle data too small");
            return Ok(None);
        }

        let mut data_slice = &oracle_account.data[..];
        let oracle = Oracle::try_deserialize(&mut data_slice).map_err(|e| {
            warn!("  ❌ WHIRLPOOL failed to deserialize oracle: {:?}", e);
            anyhow!("Failed to deserialize Oracle: {:?}", e)
        })?;

        // Oracle is #[repr(packed)] — taking a reference to any of its fields
        // inside a macro that formats by reference is undefined behaviour under
        // Rust's alignment rules. Copy the field to a local stack slot first so
        // the formatter always receives a properly aligned reference.
        let trade_enable_ts = oracle.trade_enable_timestamp;
        if self.unix_timestamp < trade_enable_ts {
            warn!(
                "  ❌ WHIRLPOOL trading not enabled yet: current={}, required={}",
                self.unix_timestamp, trade_enable_ts
            );
            return Err(anyhow!(
                "Trading not enabled yet: current={}, required={}",
                self.unix_timestamp, trade_enable_ts
            ));
        }

        info!("  ✅ WHIRLPOOL trading enabled");
        Ok(Some(AdaptiveFeeInfo {
            constants: oracle.adaptive_fee_constants,
            variables: oracle.adaptive_fee_variables,
        }))
    }
}

pub fn calculate_orca_whirlpool_output(
    accounts:       &AccountMap,
    pool_address:   &Pubkey,
    _slot:          u64,
    unix_timestamp: u64,
    amount_in:      u64,
    token_in:       &Pubkey,
) -> Result<u64> {
    OrcaWhirlpoolSimulator::new(accounts, unix_timestamp)
        .simulate_swap(pool_address, amount_in, token_in)
}
