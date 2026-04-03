pub mod info;

pub use info::*;

use solana_pubkey::Pubkey;

pub fn futarchy_program_id() -> Pubkey {
    solana_pubkey::pubkey!("FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq")
}
