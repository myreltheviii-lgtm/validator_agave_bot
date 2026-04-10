//! Global token mint constants.
//!
//! Solana addresses are 32-byte Ed25519 public keys, conventionally written in
//! base58. The `solana_pubkey::pubkey!()` macro decodes base58 at compile time and
//! embeds the raw bytes directly into the binary. This is zero-cost at runtime:
//! the CPU never executes a base58 decode; it simply reads a constant from the
//! program's read-only data segment.
//!
//! Contrast with the previous approach, where `SOL_MINT` and `USDC_MINT` were
//! `&str` literals and callers in hot-path code called `Pubkey::from_str(...).unwrap()`
//! on every pool update event. Base58 decoding involves a multi-step character
//! mapping, a big-integer modular reduction, and a checksum verification — all
//! wasted cycles on a value that never changes.
//!
//! # The USD1 address discrepancy
//!
//! Two files in the original codebase encoded USD1 with different pubkeys:
//! - `token_flow_validator.rs` used `USDrbBQwQbQ2oWHUPfA8QBHcyVxKUq1xHyXsSLKdUq2`
//! - `smb_instruction_builder.rs` used `USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB`
//!
//! The canonical mainnet mint is `USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB`.
//! This is the address referenced by the deployed SOL↔USD1 Raydium pool, its
//! vault accounts, and the bridge instruction accounts in the instruction builder.
//! The other address (`USDrb…`) was a test or pre-deployment address. When the
//! mixed stablecoin mode fired (USDC present in pool 1, USD1 present in pool 2),
//! the validator and the builder were using different token identities, so the
//! validator approved a flow the builder could not construct, and the constructed
//! transaction failed on-chain.
//!
//! Centralising all four quote-token mints here makes divergence impossible: every
//! module imports from this crate, so a future address change propagates in one edit.

// `solana_pubkey` is the disaggregated crate extracted from the monolithic solana-sdk.
// It owns the `Pubkey` type and the `pubkey!()` macro. Because agave 4.x split the
// original sdk into fine-grained crates, imports that previously used
// `solana_sdk::pubkey::*` now target this dedicated crate directly.
use solana_pubkey::Pubkey;

/// Wrapped SOL mint (`So111…`). The SPL Token program governs this mint.
///
/// wSOL is the on-chain representation of native SOL inside the SPL token
/// framework. Swap programs accept it just like any other SPL token, and the
/// flashloan executor wraps/unwraps SOL automatically around each arbitrage
/// transaction so the wallet never holds a wSOL balance between slots.
pub const SOL_MINT: Pubkey =
    solana_pubkey::pubkey!("So11111111111111111111111111111111111111112");

/// USDC mint. The SPL Token program governs this mint.
///
/// USDC is the primary USD-pegged stablecoin on Solana and the second most
/// common quote token after SOL. Many DEX pools pair an arbitrary SPL token
/// against USDC rather than against SOL, so the arbitrage graph treats USDC
/// as a first-class quote token alongside SOL.
pub const USDC_MINT: Pubkey =
    solana_pubkey::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

/// USDT mint. The SPL Token program governs this mint.
///
/// Tether USD is less common than USDC as a Solana DEX quote token but still
/// has significant liquidity on Raydium V4 and Orca Whirlpool. The arbitrage
/// graph includes it as a quote token to capture paths that flow through
/// SOL/USDT or TOKEN/USDT pools.
pub const USDT_MINT: Pubkey =
    solana_pubkey::pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");

/// World Liberty Financial USD1 mint. The SPL Token program governs this mint.
///
/// USD1 is a dollar-pegged stablecoin launched by World Liberty Financial in
/// early 2025. It has significant on-chain liquidity on Raydium CLMM and
/// Meteora DLMM pools, making it a viable fourth quote token. The instruction
/// builder includes USD1-specific vault accounts and bridge accounts for the
/// mixed-stablecoin execution path (e.g. a path that buys a token with SOL
/// in the first hop and sells it for USD1 in the second hop, then bridges
/// USD1 back to SOL via the Raydium SOL↔USD1 pool).
pub const USD1_MINT: Pubkey =
    solana_pubkey::pubkey!("USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB");

/// Returns `SOL_MINT`. Retained for call-site compatibility with code that
/// invokes a function rather than referencing a constant directly. New code
/// should reference `SOL_MINT` directly to make the inline cost explicit.
#[inline(always)]
pub fn sol_mint() -> Pubkey {
    SOL_MINT
}

/// Returns `USDC_MINT`. Retained for call-site compatibility. New code should
/// reference `USDC_MINT` directly.
#[inline(always)]
pub fn usdc_mint() -> Pubkey {
    USDC_MINT
}
