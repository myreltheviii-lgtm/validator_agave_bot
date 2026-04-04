use anyhow::Result;
use solana_pubkey::Pubkey;

pub const TICK_ARRAY_SEED: &str = "tick_array";
pub const TICK_ARRAY_SIZE: i32 = 60;
pub const TICK_ARRAY_SIZE_USIZE: usize = 60;
pub const REWARD_NUM: usize = 3;

pub const POOL_TICK_ARRAY_BITMAP_SEED: &str = "pool_tick_array_bitmap_extension";

// The exact serialized byte length of a Raydium CLMM PoolState account on-chain,
// including the 8-byte Anchor discriminator prefix. Every account owned by the
// Raydium CLMM program that is a genuine PoolState will be exactly this size.
// Other account types owned by the same program — tick arrays, bitmap extensions,
// position accounts, fee accounts — are different sizes and will be rejected by
// the size check in load_checked before any field parsing occurs.
pub const RAYDIUM_CLMM_POOL_SIZE: usize = 1544;

// The Anchor account discriminator for a Raydium CLMM PoolState account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool").
// Every Raydium CLMM PoolState account on-chain begins with these exact 8 bytes.
// Other account types owned by the Raydium CLMM program — tick arrays, position
// accounts, bitmap extensions — carry different discriminators and are rejected
// immediately by the discriminator check before any field parsing occurs.
// The program ID filter in get_filtered_indexed_accounts already narrows the
// account set to Raydium CLMM-owned accounts only; the discriminator then narrows
// further to PoolState accounts specifically. Both filters together provide the
// strongest possible guarantee that what is being parsed is a genuine pool.
const POOL_DISCRIMINATOR: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];

pub enum RewardState {
    Uninitialized,
    Initialized,
    Opening,
    Ended,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct RewardInfo {
    pub reward_state: u8,
    pub open_time: u64,
    pub end_time: u64,
    pub last_update_time: u64,
    pub emissions_per_second_x64: u128,
    pub reward_total_emissioned: u64,
    pub reward_claimed: u64,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub authority: Pubkey,
    pub reward_growth_global_x64: u128,
}

impl RewardInfo {
    pub fn new(authority: Pubkey) -> Self {
        Self {
            authority,
            ..Default::default()
        }
    }

    pub fn initialized(&self) -> bool {
        self.token_mint.ne(&Pubkey::default())
    }
}

#[derive(Default, Debug)]
pub struct PoolState {
    pub bump: [u8; 1],
    pub amm_config: Pubkey,
    pub owner: Pubkey,

    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,

    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,

    pub observation_key: Pubkey,

    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,

    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,

    pub padding3: u16,
    pub padding4: u16,

    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,

    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,

    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,

    pub status: u8,
    pub padding: [u8; 7],

    pub reward_infos: [RewardInfo; REWARD_NUM],

    pub tick_array_bitmap: [u64; 16],

    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,

    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,

    pub open_time: u64,
    pub recent_epoch: u64,

    pub padding1: [u64; 24],
    pub padding2: [u64; 32],
}

impl PoolState {
    pub fn load_checked(data: &[u8]) -> Result<Self> {
        // The discriminator occupies the first 8 bytes of every Anchor-managed account.
        // Reading it requires at least 8 bytes to be present. This check must come
        // before both the discriminator comparison and the exact size check because
        // all three operations index into `data` — without this guard a zero-byte or
        // very short account would panic on any subsequent slice operation.
        if data.len() < 8 {
            return Err(anyhow::anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        // The discriminator is the type-level identity of an Anchor account. It is
        // computed once at program compile time as SHA-256("account:Pool")[0..8] and
        // is permanently stamped into every account the program initializes under that
        // type. The Raydium CLMM program owns many account types — PoolState, tick
        // arrays, bitmap extensions, position accounts — all returned by
        // get_filtered_indexed_accounts. Checking the discriminator here rejects every
        // non-PoolState account before a single field offset is touched.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow::anyhow!(
                "Account discriminator does not match Raydium CLMM PoolState discriminator"
            ));
        }

        // An exact size match is a second layer of defense on top of the discriminator
        // check. If a future program upgrade changes the PoolState layout and introduces
        // accounts of a different size, the size check catches them here rather than
        // silently parsing fields at wrong offsets and producing garbage pubkeys.
        if data.len() != RAYDIUM_CLMM_POOL_SIZE {
            return Err(anyhow::anyhow!(
                "Account data length {} does not match Raydium CLMM PoolState size {}",
                data.len(),
                RAYDIUM_CLMM_POOL_SIZE,
            ));
        }

        // All Anchor programs prefix every managed account with an 8-byte discriminator
        // computed as the first 8 bytes of SHA-256("account:<TypeName>"). Skipping it
        // here positions the cursor at the first real field of the PoolState layout.
        let data = &data[8..];
        let mut offset = 0;

        // bump: [u8; 1] — PDA bump seed, not needed for arb routing
        offset += 1;

        let mut amm_config = [0u8; 32];
        amm_config.copy_from_slice(&data[offset..offset + 32]);
        let amm_config = Pubkey::new_from_array(amm_config);
        offset += 32;

        // owner: Pubkey — pool owner, not needed for arb routing
        offset += 32;

        let mut token_mint_0 = [0u8; 32];
        token_mint_0.copy_from_slice(&data[offset..offset + 32]);
        let token_mint_0 = Pubkey::new_from_array(token_mint_0);
        offset += 32;

        let mut token_mint_1 = [0u8; 32];
        token_mint_1.copy_from_slice(&data[offset..offset + 32]);
        let token_mint_1 = Pubkey::new_from_array(token_mint_1);
        offset += 32;

        let mut token_vault_0 = [0u8; 32];
        token_vault_0.copy_from_slice(&data[offset..offset + 32]);
        let token_vault_0 = Pubkey::new_from_array(token_vault_0);
        offset += 32;

        let mut token_vault_1 = [0u8; 32];
        token_vault_1.copy_from_slice(&data[offset..offset + 32]);
        let token_vault_1 = Pubkey::new_from_array(token_vault_1);
        offset += 32;

        let mut observation_key = [0u8; 32];
        observation_key.copy_from_slice(&data[offset..offset + 32]);
        let observation_key = Pubkey::new_from_array(observation_key);
        offset += 32;

        // mint_decimals_0 (u8) + mint_decimals_1 (u8) — not needed for arb routing
        offset += 2;

        let mut tick_spacing_bytes = [0u8; 2];
        tick_spacing_bytes.copy_from_slice(&data[offset..offset + 2]);
        let tick_spacing = u16::from_le_bytes(tick_spacing_bytes);
        offset += 2;

        // liquidity: u128 — current in-range liquidity, not needed at scan time
        offset += 16;

        // sqrt_price_x64: u128 — current price as Q64.64 fixed point, not needed at scan time
        offset += 16;

        let mut tick_current_bytes = [0u8; 4];
        tick_current_bytes.copy_from_slice(&data[offset..offset + 4]);
        let tick_current = i32::from_le_bytes(tick_current_bytes);
        offset += 4;

        Ok(Self {
            amm_config,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_key,
            tick_spacing,
            tick_current,
            ..Default::default()
        })
    }
}

pub fn compute_tick_array_start_index(tick: i32, tick_spacing: u16) -> i32 {
    let ticks_in_array = TICK_ARRAY_SIZE * tick_spacing as i32;
    let mut start = tick / ticks_in_array;
    if tick < 0 && tick % ticks_in_array != 0 {
        start = start - 1
    }
    start * ticks_in_array
}

pub fn get_tick_array_pubkeys(
    pool_pubkey: &Pubkey,
    tick_current: i32,
    tick_spacing: u16,
    offsets: &[i32],
    raydium_clmm_program_id: &Pubkey,
) -> Result<Vec<Pubkey>> {
    let mut result = Vec::with_capacity(offsets.len());
    let ticks_in_array = TICK_ARRAY_SIZE * tick_spacing as i32;

    for &offset in offsets {
        let base_start_index = compute_tick_array_start_index(tick_current, tick_spacing);

        let offset_start_index = base_start_index + offset * ticks_in_array;

        let seeds = &[
            TICK_ARRAY_SEED.as_bytes(),
            pool_pubkey.as_ref(),
            &offset_start_index.to_be_bytes(),
        ];

        let (pubkey, _) = Pubkey::find_program_address(seeds, raydium_clmm_program_id);
        result.push(pubkey);
    }

    Ok(result)
}
