// `solana_pubkey::pubkey!()` decodes a base58 address at compile time and embeds the
// raw 32-byte array into the binary's read-only data segment. This replaces the old
// `Pubkey::from_str("...").unwrap()` pattern, which performed base58 decoding at
// runtime, required the `FromStr` trait in scope, and panicked on typos instead of
// failing at compile time.
use solana_pubkey::Pubkey;

// ---------------------------------------------------------------------------
// Pump AMM — Program and Fixed Protocol Addresses
// ---------------------------------------------------------------------------

/// The Pump AMM swap program that governs pool creation, token swaps,
/// fee collection, and cashback distribution for every Pump pool.
pub fn pump_program_id() -> Pubkey {
    solana_pubkey::pubkey!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA")
}

/// The primary fee collection wallet for standard Pump pools.
pub fn pump_fee_wallet() -> Pubkey {
    solana_pubkey::pubkey!("JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU")
}

/// The Mayhem-mode fee wallet. Pump Mayhem pools route fees here instead of the
/// standard fee wallet; the `is_mayhem_mode` flag in the pool state selects which
/// wallet receives fees.
pub fn pump_mayhem_fee_wallet() -> Pubkey {
    solana_pubkey::pubkey!("GesfTA3X2arioaHp8bbKdjG9vJtskViWACZoYvxp4twS")
}

/// The global configuration account for the Pump AMM program.
///
/// This is a **fixed on-chain address**, not a derived address. It stores
/// protocol-wide settings — default swap fees, LP fee rates, and authority keys.
/// The on-chain executor reads it as a non-signer, read-only account on every swap.
/// The address was obtained from the deployed program's configuration and confirmed
/// against the reference bot implementation in `transaction.rs`.
pub fn pump_global_config() -> Pubkey {
    solana_pubkey::pubkey!("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw")
}

/// The vault-and-LP-mint authority for the Pump AMM program.
///
/// This is a **fixed on-chain address**, not a derived address. It is the account
/// that holds signing authority over all Pump token vaults and LP mint operations.
/// Every pool's token vaults are controlled by this account rather than directly by
/// the program ID. The address was confirmed against the reference bot.
pub fn pump_authority() -> Pubkey {
    solana_pubkey::pubkey!("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR")
}
