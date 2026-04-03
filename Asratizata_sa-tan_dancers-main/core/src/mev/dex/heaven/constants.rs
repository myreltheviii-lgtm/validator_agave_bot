// `solana_pubkey::pubkey!()` decodes base58 at compile time, embedding the raw
// 32-byte array in the binary's read-only segment. The old pattern stored
// `&str` constants and called `Pubkey::from_str(CONSTANT).unwrap()` at
// runtime, which required `FromStr` in scope and base58-decoded on every call.
use solana_pubkey::Pubkey;

/// The Heaven AMM swap program.
pub fn heaven_program_id() -> Pubkey {
    solana_pubkey::pubkey!("HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o")
}

/// Heaven protocol account 1 — a fixed protocol-owned account required as a
/// read-only account on every Heaven swap instruction.
pub fn heaven_protocol_account_1() -> Pubkey {
    solana_pubkey::pubkey!("HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny")
}

/// Heaven protocol account 2 — a second fixed protocol-owned account required
/// as a read-only account on every Heaven swap instruction.
pub fn heaven_protocol_account_2() -> Pubkey {
    solana_pubkey::pubkey!("CH31Xns5z3M1cTAbKW34jcxPPciazARpijcHj9rxtemt")
}
