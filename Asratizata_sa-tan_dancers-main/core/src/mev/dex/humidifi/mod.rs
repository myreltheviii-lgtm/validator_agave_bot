pub mod info;

pub use info::*;

use solana_pubkey::Pubkey;

pub fn humidifi_program_id() -> Pubkey {
    solana_pubkey::pubkey!("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp")
}
