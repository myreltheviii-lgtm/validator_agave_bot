use solana_pubkey::Pubkey;
use anyhow::Result;

const COIN_VAULT_OFFSET: usize = 336; // coinVault/tokenVaultA
const PC_VAULT_OFFSET: usize = 368;   // pcVault/tokenVaultB
const COIN_MINT_OFFSET: usize = 400;  // coinMint/tokenMintA
const PC_MINT_OFFSET: usize = 432;    // pcMint/tokenMintB

#[derive(Debug)]
pub struct RaydiumAmmInfo {
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
}

impl RaydiumAmmInfo {
    pub fn load_checked(data: &[u8]) -> Result<Self> {
        // Raydium V4 AMM is a native Solana program, not an Anchor program.
        // Native programs do not prepend an 8-byte account discriminator to their
        // account data — that convention is Anchor-specific. The pool state layout
        // is defined purely by the program's own serialization logic, and the fields
        // we need (vaults and mints) live at fixed byte offsets within that layout.
        // Attempting a discriminator check against Raydium V4 accounts will reject
        // every valid pool account and yield zero results during scanning.
        //
        // The minimum length check here is the correct and sufficient guard: it
        // ensures that pc_mint — the furthest field we read — is fully contained
        // within the buffer before any offset indexing is performed. All four reads
        // are safe once this single bound is established.
        if data.len() < PC_MINT_OFFSET + 32 {
            return Err(anyhow::anyhow!(
                "Account data length {} is too short to contain a valid Raydium V4 AMM pool layout",
                data.len()
            ));
        }

        let coin_vault = Pubkey::new_from_array(data[COIN_VAULT_OFFSET..COIN_VAULT_OFFSET + 32].try_into().unwrap());
        let pc_vault   = Pubkey::new_from_array(data[PC_VAULT_OFFSET..PC_VAULT_OFFSET + 32].try_into().unwrap());
        let coin_mint  = Pubkey::new_from_array(data[COIN_MINT_OFFSET..COIN_MINT_OFFSET + 32].try_into().unwrap());
        let pc_mint    = Pubkey::new_from_array(data[PC_MINT_OFFSET..PC_MINT_OFFSET + 32].try_into().unwrap());

        Ok(Self {
            coin_mint,
            pc_mint,
            coin_vault,
            pc_vault,
        })
    }
}
