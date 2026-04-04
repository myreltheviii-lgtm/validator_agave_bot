use solana_pubkey::Pubkey;
use anyhow::Result;

const AMM_CONFIG_OFFSET: usize = 8; // amm_config
const POOL_CREATOR_OFFSET: usize = 40; // pool_creator
const TOKEN_0_VAULT_OFFSET: usize = 72; // token_0_vault
const TOKEN_1_VAULT_OFFSET: usize = 104; // token_1_vault
const LP_MINT_OFFSET: usize = 136; // lp_mint
const TOKEN_0_MINT_OFFSET: usize = 168; // token_0_mint
const TOKEN_1_MINT_OFFSET: usize = 200; // token_1_mint
const TOKEN_0_PROGRAM_OFFSET: usize = 232; // token_0_program
const TOKEN_1_PROGRAM_OFFSET: usize = 264; // token_1_program
const OBSERVATION_KEY_OFFSET: usize = 296; // observation_key

// The exact serialized byte length of a Raydium CPMM pool account on-chain,
// including the 8-byte Anchor discriminator prefix. Raydium CPMM pools are always
// exactly this size. Any account owned by the CPMM program that deviates from this
// size is not a pool state account and must be rejected before field parsing begins.
pub const RAYDIUM_CPMM_POOL_SIZE: usize = 637;

// The Anchor account discriminator for a Raydium CPMM pool account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool").
// Raydium CPMM and Raydium CLMM share the same discriminator value because both
// programs name their pool account type "Pool" — this is expected. The program ID
// filter in get_filtered_indexed_accounts already isolates accounts to the CPMM
// program exclusively, so the discriminator check narrows further to pool accounts
// specifically within that program's owned account set.
const POOL_DISCRIMINATOR: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];

#[derive(Debug)]
pub struct RaydiumCpAmmInfo {
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub amm_config: Pubkey,
    pub observation_key: Pubkey,
}

impl RaydiumCpAmmInfo {
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
        // computed once at program compile time as SHA-256("account:Pool")[0..8] and
        // is permanently stamped into every account the program initializes under that
        // type. Checking the discriminator here rejects every non-pool account owned
        // by the Raydium CPMM program before any field offset is touched.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow::anyhow!(
                "Account discriminator does not match Raydium CPMM pool discriminator"
            ));
        }

        // An exact size match is a second layer of defense on top of the discriminator
        // check. If a future program upgrade changes the pool layout and introduces
        // accounts of a different size, the size check catches them here rather than
        // silently parsing fields at wrong offsets and producing garbage pubkeys.
        if data.len() != RAYDIUM_CPMM_POOL_SIZE {
            return Err(anyhow::anyhow!(
                "Account data length {} does not match Raydium CPMM pool size {}",
                data.len(),
                RAYDIUM_CPMM_POOL_SIZE,
            ));
        }

        let token_0_vault = Pubkey::new_from_array(data[TOKEN_0_VAULT_OFFSET..TOKEN_0_VAULT_OFFSET + 32].try_into().unwrap());
        let token_1_vault = Pubkey::new_from_array(data[TOKEN_1_VAULT_OFFSET..TOKEN_1_VAULT_OFFSET + 32].try_into().unwrap());
        let token_0_mint = Pubkey::new_from_array(data[TOKEN_0_MINT_OFFSET..TOKEN_0_MINT_OFFSET + 32].try_into().unwrap());
        let token_1_mint = Pubkey::new_from_array(data[TOKEN_1_MINT_OFFSET..TOKEN_1_MINT_OFFSET + 32].try_into().unwrap());
        let amm_config = Pubkey::new_from_array(data[AMM_CONFIG_OFFSET..AMM_CONFIG_OFFSET + 32].try_into().unwrap());
        let observation_key = Pubkey::new_from_array(data[OBSERVATION_KEY_OFFSET..OBSERVATION_KEY_OFFSET + 32].try_into().unwrap());

        Ok(Self {
            token_0_mint,
            token_1_mint,
            token_0_vault,
            token_1_vault,
            amm_config,
            observation_key,
        })
    }
}
