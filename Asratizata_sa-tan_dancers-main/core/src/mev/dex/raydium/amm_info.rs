use solana_pubkey::Pubkey;
use anyhow::Result;

const COIN_VAULT_OFFSET: usize = 336; // coinVault/tokenVaultA
const PC_VAULT_OFFSET: usize = 368; // pcVault/tokenVaultB
const COIN_MINT_OFFSET: usize = 400; // coinMint/tokenMintA
const PC_MINT_OFFSET: usize = 432; // pcMint/tokenMintB

// The Anchor account discriminator for a Raydium V4 AMM pool account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool")
// or the equivalent type name used in the Raydium V4 program. The Raydium V4 program
// owns many account types beyond pool state — open orders accounts, target orders,
// LP mint authority accounts — all returned by get_filtered_indexed_accounts.
// Checking the discriminator here rejects every non-pool account before any of the
// fixed-offset field reads below are reached, preventing both panics and silent
// garbage parsing from non-pool accounts that happen to be large enough to pass
// the minimum length check.
const POOL_DISCRIMINATOR: [u8; 8] = [33, 217, 2, 203, 184, 83, 235, 91];

#[derive(Debug)]
pub struct RaydiumAmmInfo {
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
}

impl RaydiumAmmInfo {
    pub fn load_checked(data: &[u8]) -> Result<Self> {
        // The discriminator occupies the first 8 bytes of every Anchor-managed account.
        // Reading it requires at least 8 bytes to be present. This check must come
        // before the discriminator comparison because that comparison indexes `data[0..8]`
        // directly — without this guard a zero-byte or sub-8-byte account would panic.
        if data.len() < 8 {
            return Err(anyhow::anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        // The discriminator is the type-level identity of an Anchor account. It is
        // computed once at program compile time as SHA-256("account:<TypeName>")[0..8]
        // and is permanently stamped into every account the program initializes under
        // that type. Raydium V4 owns open orders accounts, target orders accounts, and
        // other types in addition to pool state — all returned by
        // get_filtered_indexed_accounts. Checking the discriminator here rejects every
        // non-pool account before any of the fixed field offsets below are accessed.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow::anyhow!(
                "Account discriminator does not match Raydium V4 AMM pool discriminator"
            ));
        }

        // The minimum length check ensures the last field read (pc_mint at PC_MINT_OFFSET)
        // is fully contained within the buffer. All four field reads use fixed offsets
        // that are guaranteed safe once this check passes.
        if data.len() < PC_MINT_OFFSET + 32 {
            return Err(anyhow::anyhow!("Invalid data length for RaydiumAmmInfo"));
        }

        let coin_vault = Pubkey::new_from_array(data[COIN_VAULT_OFFSET..COIN_VAULT_OFFSET + 32].try_into().unwrap());
        let pc_vault = Pubkey::new_from_array(data[PC_VAULT_OFFSET..PC_VAULT_OFFSET + 32].try_into().unwrap());
        let coin_mint = Pubkey::new_from_array(data[COIN_MINT_OFFSET..COIN_MINT_OFFSET + 32].try_into().unwrap());
        let pc_mint = Pubkey::new_from_array(data[PC_MINT_OFFSET..PC_MINT_OFFSET + 32].try_into().unwrap());

        Ok(Self {
            coin_mint,
            pc_mint,
            coin_vault,
            pc_vault,
        })
    }
}
