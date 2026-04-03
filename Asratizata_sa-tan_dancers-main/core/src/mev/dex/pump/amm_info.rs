use anyhow::Result;
// `solana_pubkey` is the disaggregated crate that owns `Pubkey` in agave 4.x.
// `Pubkey::new(&[u8])` was removed from the API — raw bytes must now be passed
// as a fixed `[u8; 32]` array via `Pubkey::new_from_array`. The `.try_into()`
// call converts the `&[u8]` slice produced by byte-range indexing into a `[u8;32]`
// array, and `.unwrap()` is safe here because every call site slices exactly 32 bytes
// from a buffer that has already been bounds-checked above.
use solana_pubkey::Pubkey;

use super::constants::pump_program_id;

const COIN_CREATOR_VAULT_SEED: &[u8] = b"creator_vault";

#[derive(Debug)]
pub struct PumpAmmInfo {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_vault_authority: Pubkey,
    pub is_mayhem_mode: bool,
    pub is_cashback_coin: bool,
}

impl PumpAmmInfo {
    pub fn load_checked(data: &[u8]) -> Result<Self> {
        // Skip the 8-byte Anchor discriminator that prefixes every account
        // managed by an Anchor program. The discriminator is the first 8 bytes
        // of the SHA-256 hash of the account type name and is used by the runtime
        // to verify that the correct account type is being deserialized.
        let data = &data[8..];
        let base_mint_offset = 1 + 2 + 32; // bump + index + creator
        let quote_mint_offset = base_mint_offset + 32;
        let pool_base_offset = quote_mint_offset + 32 + 32; // + lp mint
        let pool_quote_offset = pool_base_offset + 32;
        let min_len = pool_quote_offset + 32;

        if data.len() < min_len {
            return Err(anyhow::anyhow!("Invalid data length for PumpAmmInfo"));
        }

        let base_mint =
            Pubkey::new_from_array(data[base_mint_offset..base_mint_offset + 32].try_into().unwrap());
        let quote_mint =
            Pubkey::new_from_array(data[quote_mint_offset..quote_mint_offset + 32].try_into().unwrap());
        let pool_base_token_account =
            Pubkey::new_from_array(data[pool_base_offset..pool_base_offset + 32].try_into().unwrap());
        let pool_quote_token_account =
            Pubkey::new_from_array(data[pool_quote_offset..pool_quote_offset + 32].try_into().unwrap());

        let coin_creator_offset = pool_quote_offset + 8 + 32; // lp_supply + last_trade_timestamp
        let is_mayhem_mode_offset = coin_creator_offset + 32;
        let is_cashback_coin_offset = is_mayhem_mode_offset + 1;

        let coin_creator = if coin_creator_offset + 32 > data.len() {
            Pubkey::default()
        } else {
            Pubkey::new_from_array(data[coin_creator_offset..coin_creator_offset + 32].try_into().unwrap())
        };

        let is_mayhem_mode = if is_mayhem_mode_offset >= data.len() {
            false
        } else {
            data[is_mayhem_mode_offset] != 0
        };
        let is_cashback_coin = if is_cashback_coin_offset >= data.len() {
            false
        } else {
            data[is_cashback_coin_offset] != 0
        };

        // The coin creator vault authority is a Program Derived Address that
        // controls the coin creator's fee vault. The vault holds the creator's
        // share of swap fees until they are claimed. The PDA is keyed by the
        // coin creator's pubkey and the "creator_vault" seed so that each
        // creator has a unique vault address.
        let coin_creator_vault_authority = if coin_creator == Pubkey::default() {
            Pubkey::default()
        } else {
            Pubkey::find_program_address(
                &[COIN_CREATOR_VAULT_SEED, coin_creator.as_ref()],
                &pump_program_id(),
            )
            .0
        };

        Ok(Self {
            base_mint,
            quote_mint,
            pool_base_token_account,
            pool_quote_token_account,
            coin_creator,
            coin_creator_vault_authority,
            is_mayhem_mode,
            is_cashback_coin,
        })
    }
}
