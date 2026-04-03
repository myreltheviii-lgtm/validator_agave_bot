// `super::` navigates one level up from `pools.rs` (which lives at `core/src/mev/pools.rs`)
// to `core/src/mev/`, resolving into the sibling modules `constants` and `dex`.
// This is the correct relative path now that the code is integrated into solana-core;
// `crate::` would resolve to the solana-core crate root, which is two levels up from
// the `mev/` directory.
use super::{
    constants::sol_mint,
    dex::{
        byreal::byreal_program_id,
        pancakeswap::pancakeswap_program_id,
        raydium::{clmm_info::POOL_TICK_ARRAY_BITMAP_SEED, raydium_clmm_program_id},
    },
};

// PancakeSwap and Byreal share Raydium CLMM's PoolState layout but derive their bitmap
// extension PDAs using their own program IDs and this seed string (not the imported constant).
const POOL_TICK_ARRAY_BITMAP_SEED_CLMM: &str = "pool_tick_array_bitmap_extension";

use solana_pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct RaydiumPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct RaydiumCpPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub amm_config: Pubkey,
    pub observation: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct PumpPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub fee_wallet: Pubkey,
    pub fee_token_wallet: Pubkey,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_vault_authority: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
    pub is_mayhem_mode: bool,
    // When a Pump token opts into the protocol's cashback program, every swap rewards
    // the user's volume accumulator with wrapped SOL. The executor needs two additional
    // accounts at the tail of the Pump account list: the user_volume_accumulator's wSOL
    // ATA and the accumulator itself (again, writable). This flag gates their inclusion.
    pub is_cashback_coin: bool,

    // PDAs pre-computed at parse time so the instruction builder can use them at
    // zero cost on every simulation and submission call.
    //
    // `find_program_address` iterates up to 256 bump seeds, each performing a
    // SHA-256 hash.  Re-deriving these three addresses on every instruction build
    // (twice per arb attempt — Phase 1 simulation + Phase 2 submission) adds a
    // measurable CPU cost on the microsecond-critical hot path.  Pre-computing
    // them once at startup removes that cost entirely for the process lifetime.
    //
    // global_volume_accumulator: protocol-wide singleton PDA tracking aggregate
    //   swap volume.  The same pubkey for every pool under the same program.
    // user_volume_accumulator: per-wallet PDA used for rebate eligibility.
    //   Derived from the executor's wallet pubkey at parse time.
    // pool_v2: per-pool PDA introduced in Pump AMM v2.  The on-chain executor
    //   reads it to select the correct swap path for this pool.
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub pool_v2: Pubkey,
}

#[derive(Debug, Clone)]
pub struct DlmmPool {
    pub pair: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub oracle: Pubkey,
    pub bin_arrays: Vec<Pubkey>,
    // The Meteora DLMM program requires the SPL Memo program to be listed as an account
    // whenever the token being swapped uses the Token-2022 standard. For plain SPL tokens
    // this is None and no memo account appears in the instruction.
    pub memo_program: Option<Pubkey>,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct WhirlpoolPool {
    pub pool: Pubkey,
    pub oracle: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
    // Orca Whirlpool unconditionally requires the SPL Memo program in every swap
    // regardless of token standard, so this field is always Some(...) at initialization
    // time. It is stored here so the instruction builder can always push it without
    // needing to reconstruct the pubkey.
    pub memo_program: Option<Pubkey>,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct RaydiumClmmPool {
    pub pool: Pubkey,
    pub amm_config: Pubkey,
    pub observation_state: Pubkey,
    // The bitmap extension is a PDA that tracks which tick-array bitmap pages exist
    // beyond the first 15 pages. It is always required by the on-chain executor even
    // if the current tick sits in the first page, so it is derived once at init time.
    pub bitmap_extension: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
    // Raydium CLMM requires the SPL Memo program only when the token uses Token-2022.
    pub memo_program: Option<Pubkey>,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct MeteoraDAmmPool {
    pub pool: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_x_token_vault: Pubkey,
    pub token_sol_token_vault: Pubkey,
    pub token_x_lp_mint: Pubkey,
    pub token_sol_lp_mint: Pubkey,
    pub token_x_pool_lp: Pubkey,
    pub token_sol_pool_lp: Pubkey,
    pub admin_token_fee_x: Pubkey,
    pub admin_token_fee_sol: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct MeteoraDAmmV2Pool {
    pub pool: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct VertigoPool {
    pub pool: Pubkey,
    // pool_owner is the wallet that created and controls the Vertigo pool. It is a
    // readonly account in swaps — the executor uses it to locate the pool authority
    // for fee routing. It is derived as a PDA at initialization time.
    pub pool_owner: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct HeavenPool {
    pub pool: Pubkey,
    // protocol_config is the Heaven protocol's global configuration account. It is
    // writable in swaps because the protocol updates fee accumulators on every trade.
    pub protocol_config: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_base_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
    // Heaven pools can hold Token-2022 assets. The token_program field records which
    // SPL token variant governs this pool's vaults so the instruction builder can
    // pass the correct program account.
    pub token_program: Pubkey,
}

#[derive(Debug, Clone)]
pub struct FutarchyPool {
    // Futarchy uses a DAO account as the pool anchor rather than a pool address.
    // The dao pubkey acts as the canonical identifier for this pool.
    pub dao: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct HumidifiPool {
    pub pool: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct PancakeswapPool {
    pub pool: Pubkey,
    pub amm_config: Pubkey,
    pub observation_state: Pubkey,
    // PancakeSwap shares Raydium CLMM's PoolState layout but derives its bitmap extension
    // PDA using the pancakeswap_program_id, not the raydium_clmm_program_id.
    pub bitmap_extension: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
    // Requires SPL Memo for Token-2022 tokens, same rule as Raydium CLMM.
    pub memo_program: Option<Pubkey>,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

#[derive(Debug, Clone)]
pub struct ByrealPool {
    pub pool: Pubkey,
    pub amm_config: Pubkey,
    pub observation_state: Pubkey,
    // Byreal shares Raydium CLMM's PoolState layout but derives its bitmap extension
    // PDA using the byreal_program_id, not the raydium_clmm_program_id.
    pub bitmap_extension: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
    // Requires SPL Memo for Token-2022 tokens, same rule as Raydium CLMM.
    pub memo_program: Option<Pubkey>,
    pub token_mint: Pubkey,
    pub base_mint: Pubkey,
}

// `Clone` is derived so that a MintPoolData value can be moved behind an
// `Arc<RwLock<MintPoolData>>` at registration time without requiring the
// caller to transfer ownership of the original Arc. The clone happens exactly
// once per mint at startup or first discovery — never on the hot path.
#[derive(Debug, Clone)]
pub struct MintPoolData {
    pub mint: Pubkey,
    // Both SPL Token and Token-2022 programs are valid owners for mint accounts.
    // This field records which program governs this mint so the instruction builder
    // can derive wallet ATAs with the correct program ID.
    pub token_program: Pubkey,
    pub wallet_account: Pubkey,
    pub wallet_wsol_account: Pubkey,
    pub raydium_pools: Vec<RaydiumPool>,
    pub raydium_cp_pools: Vec<RaydiumCpPool>,
    pub pump_pools: Vec<PumpPool>,
    pub dlmm_pairs: Vec<DlmmPool>,
    pub whirlpool_pools: Vec<WhirlpoolPool>,
    pub raydium_clmm_pools: Vec<RaydiumClmmPool>,
    pub meteora_damm_pools: Vec<MeteoraDAmmPool>,
    pub meteora_damm_v2_pools: Vec<MeteoraDAmmV2Pool>,
    pub vertigo_pools: Vec<VertigoPool>,
    pub heaven_pools: Vec<HeavenPool>,
    pub futarchy_pools: Vec<FutarchyPool>,
    pub humidifi_pools: Vec<HumidifiPool>,
    pub pancakeswap_pools: Vec<PancakeswapPool>,
    pub byreal_pools: Vec<ByrealPool>,
}

impl MintPoolData {
    pub fn new(mint: Pubkey, wallet_account: &Pubkey, token_program: Pubkey) -> Self {
        let sol = sol_mint();
        // The wallet's wSOL ATA is always derived against the standard SPL Token program
        // regardless of the target token's program, because wSOL is always a plain SPL token.
        let wallet_wsol_pk =
            crate::mev::constants::get_associated_token_address(wallet_account, &sol);
        Self {
            mint,
            token_program,
            wallet_account: *wallet_account,
            wallet_wsol_account: wallet_wsol_pk,
            raydium_pools: Vec::new(),
            raydium_cp_pools: Vec::new(),
            pump_pools: Vec::new(),
            dlmm_pairs: Vec::new(),
            whirlpool_pools: Vec::new(),
            raydium_clmm_pools: Vec::new(),
            meteora_damm_pools: Vec::new(),
            meteora_damm_v2_pools: Vec::new(),
            vertigo_pools: Vec::new(),
            heaven_pools: Vec::new(),
            futarchy_pools: Vec::new(),
            humidifi_pools: Vec::new(),
            pancakeswap_pools: Vec::new(),
            byreal_pools: Vec::new(),
        }
    }

    pub fn add_raydium_pool(
        &mut self,
        pool: Pubkey,
        token_vault: Pubkey,
        sol_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.raydium_pools.push(RaydiumPool {
            pool,
            token_vault,
            sol_vault,
            token_mint,
            base_mint,
        });
    }

    pub fn add_raydium_cp_pool(
        &mut self,
        pool: Pubkey,
        token_vault: Pubkey,
        sol_vault: Pubkey,
        amm_config: Pubkey,
        observation: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.raydium_cp_pools.push(RaydiumCpPool {
            pool,
            token_vault,
            sol_vault,
            amm_config,
            observation,
            token_mint,
            base_mint,
        });
    }

    pub fn add_pump_pool(
        &mut self,
        pool: Pubkey,
        token_vault: Pubkey,
        sol_vault: Pubkey,
        fee_wallet: Pubkey,
        fee_token_wallet: Pubkey,
        coin_creator_vault_ata: Pubkey,
        coin_creator_vault_authority: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
        is_mayhem_mode: bool,
        is_cashback_coin: bool,
    ) {
        // The pump_program_id_internal is the canonical PDA authority for the Pump AMM.
        // It is distinct from pump_program_id() (the swap instruction program).
        let pump_pda_authority =
            solana_pubkey::pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

        // global_volume_accumulator: protocol-wide singleton — same for every pool.
        let (global_volume_accumulator, _) = Pubkey::find_program_address(
            &[b"global_volume_accumulator"],
            &pump_pda_authority,
        );

        // user_volume_accumulator: per-executor-wallet PDA. self.wallet_account is the
        // fixed executor keypair set at MintPoolData::new time.
        let (user_volume_accumulator, _) = Pubkey::find_program_address(
            &[b"user_volume_accumulator", self.wallet_account.as_ref()],
            &pump_pda_authority,
        );

        // pool_v2: per-pool PDA keyed on the speculative (non-quote) token mint.
        let pool_v2 =
            Pubkey::find_program_address(&[b"pool-v2", token_mint.as_ref()], &pump_pda_authority)
                .0;

        self.pump_pools.push(PumpPool {
            pool,
            token_vault,
            sol_vault,
            fee_wallet,
            fee_token_wallet,
            coin_creator_vault_ata,
            coin_creator_vault_authority,
            token_mint,
            base_mint,
            is_mayhem_mode,
            is_cashback_coin,
            global_volume_accumulator,
            user_volume_accumulator,
            pool_v2,
        });
    }

    pub fn add_dlmm_pool(
        &mut self,
        pair: Pubkey,
        token_vault: Pubkey,
        sol_vault: Pubkey,
        oracle: Pubkey,
        bin_arrays: Vec<Pubkey>,
        memo_program: Option<Pubkey>,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.dlmm_pairs.push(DlmmPool {
            pair,
            token_vault,
            sol_vault,
            oracle,
            bin_arrays,
            memo_program,
            token_mint,
            base_mint,
        });
    }

    pub fn add_whirlpool_pool(
        &mut self,
        pool: Pubkey,
        oracle: Pubkey,
        x_vault: Pubkey,
        y_vault: Pubkey,
        tick_arrays: Vec<Pubkey>,
        memo_program: Option<Pubkey>,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.whirlpool_pools.push(WhirlpoolPool {
            pool,
            oracle,
            x_vault,
            y_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
        });
    }

    pub fn add_raydium_clmm_pool(
        &mut self,
        pool: Pubkey,
        amm_config: Pubkey,
        observation_state: Pubkey,
        x_vault: Pubkey,
        y_vault: Pubkey,
        tick_arrays: Vec<Pubkey>,
        memo_program: Option<Pubkey>,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        // Bitmap extension PDA is deterministic from the pool address and the Raydium CLMM
        // program ID. Deriving it once here avoids repeating find_program_address at
        // instruction build time (which is on the hot path).
        let bitmap_extension = Pubkey::find_program_address(
            &[
                POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
                pool.as_ref(),
            ],
            &raydium_clmm_program_id(),
        )
        .0;

        self.raydium_clmm_pools.push(RaydiumClmmPool {
            pool,
            amm_config,
            observation_state,
            x_vault,
            y_vault,
            bitmap_extension,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
        });
    }

    pub fn add_meteora_damm_pool(
        &mut self,
        pool: Pubkey,
        token_x_vault: Pubkey,
        token_sol_vault: Pubkey,
        token_x_token_vault: Pubkey,
        token_sol_token_vault: Pubkey,
        token_x_lp_mint: Pubkey,
        token_sol_lp_mint: Pubkey,
        token_x_pool_lp: Pubkey,
        token_sol_pool_lp: Pubkey,
        admin_token_fee_x: Pubkey,
        admin_token_fee_sol: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.meteora_damm_pools.push(MeteoraDAmmPool {
            pool,
            token_x_vault,
            token_sol_vault,
            token_x_token_vault,
            token_sol_token_vault,
            token_x_lp_mint,
            token_sol_lp_mint,
            token_x_pool_lp,
            token_sol_pool_lp,
            admin_token_fee_x,
            admin_token_fee_sol,
            token_mint,
            base_mint,
        });
    }

    pub fn add_meteora_damm_v2_pool(
        &mut self,
        pool: Pubkey,
        token_x_vault: Pubkey,
        token_sol_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.meteora_damm_v2_pools.push(MeteoraDAmmV2Pool {
            pool,
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });
    }

    pub fn add_vertigo_pool(
        &mut self,
        pool: Pubkey,
        pool_owner: Pubkey,
        token_x_vault: Pubkey,
        token_sol_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.vertigo_pools.push(VertigoPool {
            pool,
            pool_owner,
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });
    }

    pub fn add_heaven_pool(
        &mut self,
        pool: Pubkey,
        protocol_config: Pubkey,
        token_x_vault: Pubkey,
        token_base_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
        token_program: Pubkey,
    ) {
        self.heaven_pools.push(HeavenPool {
            pool,
            protocol_config,
            token_x_vault,
            token_base_vault,
            token_mint,
            base_mint,
            token_program,
        });
    }

    pub fn add_futarchy_pool(
        &mut self,
        dao: Pubkey,
        token_x_vault: Pubkey,
        token_sol_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.futarchy_pools.push(FutarchyPool {
            dao,
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });
    }

    pub fn add_humidifi_pool(
        &mut self,
        pool: Pubkey,
        token_x_vault: Pubkey,
        token_sol_vault: Pubkey,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        self.humidifi_pools.push(HumidifiPool {
            pool,
            token_x_vault,
            token_sol_vault,
            token_mint,
            base_mint,
        });
    }

    pub fn add_pancakeswap_pool(
        &mut self,
        pool: Pubkey,
        amm_config: Pubkey,
        observation_state: Pubkey,
        x_vault: Pubkey,
        y_vault: Pubkey,
        tick_arrays: Vec<Pubkey>,
        memo_program: Option<Pubkey>,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        // PancakeSwap uses the same bitmap extension seed as Raydium CLMM but its PDA
        // is derived under the pancakeswap_program_id. Using the wrong program ID here
        // produces a valid-looking but incorrect pubkey that the on-chain program will reject.
        let bitmap_extension = Pubkey::find_program_address(
            &[POOL_TICK_ARRAY_BITMAP_SEED_CLMM.as_bytes(), pool.as_ref()],
            &pancakeswap_program_id(),
        )
        .0;

        self.pancakeswap_pools.push(PancakeswapPool {
            pool,
            amm_config,
            observation_state,
            bitmap_extension,
            x_vault,
            y_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
        });
    }

    /// Merge all pool entries from `other` into this `MintPoolData`.
    ///
    /// Only the pool-type Vec fields are extended. The base fields — `mint`,
    /// `token_program`, `wallet_account`, `wallet_wsol_account` — are NOT
    /// overwritten because they belong to the existing mint registration and
    /// must remain stable. The `other` value is consumed.
    ///
    /// This method is called during the known-mint graduation path. When a new
    /// DEX pool is created for a token that is already tracked by a running
    /// `ArbitrageExecutor`, `initialize_mint_from_discovered` builds a fresh
    /// `MintPoolData` containing the new pool's vault addresses, tick arrays,
    /// and oracle accounts. `merge_pools_from` appends those entries into the
    /// existing data so the SMB instruction builder can construct valid swap
    /// transactions through the new pool without rebuilding from scratch.
    pub fn merge_pools_from(&mut self, other: MintPoolData) {
        self.raydium_pools.extend(other.raydium_pools);
        self.raydium_cp_pools.extend(other.raydium_cp_pools);
        self.pump_pools.extend(other.pump_pools);
        self.dlmm_pairs.extend(other.dlmm_pairs);
        self.whirlpool_pools.extend(other.whirlpool_pools);
        self.raydium_clmm_pools.extend(other.raydium_clmm_pools);
        self.meteora_damm_pools.extend(other.meteora_damm_pools);
        self.meteora_damm_v2_pools.extend(other.meteora_damm_v2_pools);
        self.vertigo_pools.extend(other.vertigo_pools);
        self.heaven_pools.extend(other.heaven_pools);
        self.futarchy_pools.extend(other.futarchy_pools);
        self.humidifi_pools.extend(other.humidifi_pools);
        self.pancakeswap_pools.extend(other.pancakeswap_pools);
        self.byreal_pools.extend(other.byreal_pools);
    }

    pub fn add_byreal_pool(
        &mut self,
        pool: Pubkey,
        amm_config: Pubkey,
        observation_state: Pubkey,
        x_vault: Pubkey,
        y_vault: Pubkey,
        tick_arrays: Vec<Pubkey>,
        memo_program: Option<Pubkey>,
        token_mint: Pubkey,
        base_mint: Pubkey,
    ) {
        // Byreal uses the same bitmap extension seed as Raydium CLMM but its PDA is
        // derived under the byreal_program_id, identical reasoning to PancakeSwap above.
        let bitmap_extension = Pubkey::find_program_address(
            &[POOL_TICK_ARRAY_BITMAP_SEED_CLMM.as_bytes(), pool.as_ref()],
            &byreal_program_id(),
        )
        .0;

        self.byreal_pools.push(ByrealPool {
            pool,
            amm_config,
            observation_state,
            bitmap_extension,
            x_vault,
            y_vault,
            tick_arrays,
            memo_program,
            token_mint,
            base_mint,
        });
    }
}
