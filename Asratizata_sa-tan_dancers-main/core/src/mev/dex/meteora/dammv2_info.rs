use anyhow::Result;
use solana_pubkey::Pubkey;

// The exact serialized byte length of a Meteora DAMM V2 pool account on-chain.
// The Meteora DAMM V2 program owns multiple account types and
// get_filtered_indexed_accounts returns all of them. An exact size match ensures
// that only genuine pool state accounts are parsed — any account of a different
// size is a different account type owned by the same program and must be rejected
// before field parsing begins. Parsing a non-pool account at these offsets would
// silently produce garbage pubkeys as mint and vault addresses, registering phantom
// pools in the engine that will never match any real on-chain swap.
pub const METEORA_DAMMV2_POOL_SIZE: usize = 1112;

// The Anchor account discriminator for a Meteora DAMM V2 pool account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool").
// Multiple DEX programs that name their pool account type "Pool" will share this
// discriminator value — this is expected and correct. The program ID filter in
// get_filtered_indexed_accounts already isolates accounts to the Meteora DAMM V2
// program exclusively; the discriminator then identifies pool accounts within that
// program's owned account set. Together these two filters guarantee that only
// genuine Meteora DAMM V2 pool accounts reach the field parser below.
const POOL_DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

pub struct MeteoraDAmmV2Info {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
}

impl MeteoraDAmmV2Info {
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
        // type. Checking the discriminator here rejects every non-pool account owned
        // by the Meteora DAMM V2 program before any field offset is touched.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow::anyhow!(
                "Account discriminator does not match Meteora DAMM V2 pool discriminator"
            ));
        }

        // An exact size match is a second layer of defense on top of the discriminator
        // check. If a future program upgrade changes the pool layout and introduces
        // accounts of a different size, the size check catches them here rather than
        // silently parsing fields at wrong offsets and producing garbage pubkeys.
        if data.len() != METEORA_DAMMV2_POOL_SIZE {
            return Err(anyhow::anyhow!(
                "Account data length {} does not match Meteora DAMM V2 pool size {}",
                data.len(),
                METEORA_DAMMV2_POOL_SIZE,
            ));
        }

        let base_mint = Pubkey::new_from_array(data[168..200].try_into().unwrap());
        let quote_mint = Pubkey::new_from_array(data[200..232].try_into().unwrap());
        let base_vault = Pubkey::new_from_array(data[232..264].try_into().unwrap());
        let quote_vault = Pubkey::new_from_array(data[264..296].try_into().unwrap());

        Ok(Self {
            base_mint,
            quote_mint,
            base_vault,
            quote_vault,
        })
    }
}
