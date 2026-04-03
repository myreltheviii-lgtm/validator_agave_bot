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

// ---------------------------------------------------------------------------
// SPL program ID constants — inlined to avoid version conflicts
//
// The SPL implementation crates (spl-token, spl-token-2022,
// spl-associated-token-account) depend on `solana-pubkey` at a version older
// than the one this workspace uses (4.1.0). Cargo's resolver brings in both
// versions simultaneously, producing two structurally identical but
// type-incompatible `Pubkey` types. Every call site that passes our
// `solana_pubkey::Pubkey` values to the SPL crate functions then fails with
// "expected `__Pubkey`, found `Address`".
//
// The fix: inline the three program IDs as compile-time constants and
// replicate the two ATA derivation functions using `Pubkey::find_program_address`,
// which only depends on `solana_pubkey` at the workspace version. This removes
// the spl-token, spl-token-2022, and spl-associated-token-account crates from
// the dependency graph entirely.
// ---------------------------------------------------------------------------

/// SPL Token program ID (`TokenkegQfe…`).
pub const SPL_TOKEN_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// SPL Token-2022 program ID (`TokenzQdBN…`).
pub const SPL_TOKEN_2022_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

/// SPL Associated Token Account program ID (`ATokenGPvb…`).
pub const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe8bv");

/// Derives the Associated Token Account address for `wallet_address` and
/// `token_mint_address` under the standard SPL Token program.
#[inline(always)]
pub fn get_associated_token_address(wallet_address: &Pubkey, token_mint_address: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(
        wallet_address,
        token_mint_address,
        &SPL_TOKEN_PROGRAM_ID,
    )
}

/// Derives the Associated Token Account address for `wallet_address` and
/// `token_mint_address` under the given `token_program_id`. Handles both
/// SPL Token and Token-2022.
///
/// A program-derived address is a public key that lives outside the Ed25519
/// curve — no private key can sign for it. The on-chain Associated Token
/// Account program owns and creates these accounts. The three seeds
/// `[wallet, token_program_id, mint]` in this exact order are the canonical
/// layout the program uses, and `find_program_address` searches for a bump
/// seed that pushes the derived key off the curve. The `.0` extracts only the
/// address; the bump seed is not needed by callers that pass the address as a
/// writable account in a transaction.
#[inline(always)]
pub fn get_associated_token_address_with_program_id(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &[
            &wallet_address.to_bytes(),
            &token_program_id.to_bytes(),
            &token_mint_address.to_bytes(),
        ],
        &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
    )
    .0
}
