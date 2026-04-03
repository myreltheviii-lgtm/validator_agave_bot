// `solana_pubkey::pubkey!()` decodes base58 at compile time and embeds the raw
// 32-byte array directly into the binary's read-only segment. This replaces the
// old two-step pattern of storing an `&str` constant and calling
// `Pubkey::from_str(CONSTANT).unwrap()` at runtime — which performed base58
// decoding on every call and required `FromStr` in scope.
use solana_pubkey::Pubkey;

pub const MAX_TICK_INDEX: i32 = 443636;
pub const MIN_TICK_INDEX: i32 = -443636;

/// The Orca Whirlpool CLMM program.
pub fn whirlpool_program_id() -> Pubkey {
    solana_pubkey::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc")
}
