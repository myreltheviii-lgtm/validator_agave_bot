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

// The Anchor account discriminator for a PumpSwap AMM pool account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool").
// The PumpSwap program owns multiple account types — pool accounts, config accounts,
// authority PDAs, and potentially zero-byte placeholder accounts — all returned by
// get_filtered_indexed_accounts. Checking the discriminator rejects every non-pool
// account before the discriminator skip (`&data[8..]`) is ever reached, which means
// zero-byte and sub-8-byte accounts are caught by the length pre-check below and
// never reach any slice operation that could panic.
// PumpSwap has undergone layout upgrades adding fields (is_mayhem_mode,
// is_cashback_coin) to the pool account. The discriminator remains stable across
// these upgrades because it is tied to the account type name "Pool", not the field
// layout. The per-field bounds checks after the discriminator skip handle both old
// and new layout versions gracefully.
const POOL_DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

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
        // The discriminator occupies the first 8 bytes of every Anchor-managed account.
        // Reading it requires at least 8 bytes to be present. This check must come
        // before the discriminator comparison because that comparison indexes `data[0..8]`
        // directly — without this guard a zero-byte account would panic on that slice.
        // It must also come before `&data[8..]` which panics identically on short input.
        if data.len() < 8 {
            return Err(anyhow::anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        // The discriminator is the type-level identity of an Anchor account. It is
        // computed once at program compile time as SHA-256("account:Pool")[0..8] and
        // is permanently stamped into every account the program initializes under that
        // type. The PumpSwap program owns config accounts, authority PDAs, and other
        // account types in addition to pool accounts — all are returned by
        // get_filtered_indexed_accounts. Checking the discriminator here rejects every
        // non-pool account before any field offset is touched.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow::anyhow!(
                "Account discriminator does not match PumpSwap AMM pool discriminator"
            ));
        }

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

        // PumpSwap has undergone layout upgrades that added fields beyond the mandatory
        // core fields above. Older pool accounts on-chain do not have these fields —
        // they are shorter than the full current layout. Rather than rejecting older
        // pools with an exact size check, each optional field is read only if the
        // buffer extends far enough to contain it, otherwise a safe default is used.
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
