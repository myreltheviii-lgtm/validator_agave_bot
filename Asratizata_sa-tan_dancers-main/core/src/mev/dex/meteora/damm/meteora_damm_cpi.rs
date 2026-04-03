use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

// ---------------------------------------------------------------------------
// TokenMultiplier
//
// Meteora DAMM supports stable swap curves where both tokens represent the
// same underlying asset but have different decimal precisions (e.g., wBTC at
// 8 decimals vs. a synthetic BTC at 6 decimals). TokenMultiplier normalises
// both token amounts to a common precision before any curve math is applied,
// ensuring the invariant sees equal-scale values regardless of how each mint
// defines its decimal count.
//
// Fields:
//   token_a_multiplier — scaling factor applied to token A amounts so they
//     match the common precision.
//   token_b_multiplier — scaling factor applied to token B amounts.
//   precision_factor   — the exponent (power of 10) of the common precision.
//     E.g., a value of 8 means both tokens are scaled to 10^8 precision
//     before price calculations.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct TokenMultiplier {
    pub token_a_multiplier: u64,
    pub token_b_multiplier: u64,
    pub precision_factor: u8,
}

// ---------------------------------------------------------------------------
// PoolFees
//
// Stores the two fee tiers collected on every swap: the trade fee paid by
// the swapper and the owner (protocol) fee taken from that trade fee. Both
// are expressed as rational numbers (numerator / denominator) so they can
// represent arbitrary fee percentages without floating-point arithmetic.
//
// Fields:
//   trade_fee_numerator         — numerator of the LP fee fraction.
//   trade_fee_denominator       — denominator of the LP fee fraction.
//   owner_trade_fee_numerator   — numerator of the protocol fee fraction,
//     applied to the trade fee amount (not the gross swap amount).
//   owner_trade_fee_denominator — denominator of the protocol fee fraction.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct PoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
}

impl PoolFees {
    // Computes the LP trade fee for a given swap amount using the ratio:
    //   fee = trading_tokens * trade_fee_numerator / trade_fee_denominator
    //
    // A minimum of 1 token is charged whenever the fee would round to zero
    // but the numerator is non-zero, preventing fee-free micro swaps that
    // would otherwise extract value from LPs at no cost.
    pub fn trading_fee(&self, trading_tokens: u128) -> Option<u128> {
        calculate_fee(
            trading_tokens,
            u128::from(self.trade_fee_numerator),
            u128::from(self.trade_fee_denominator),
        )
    }

    // Computes the owner (protocol) fee, which is a fraction of the LP trade
    // fee — not a fraction of the gross swap amount. This means the protocol
    // never takes more than what the LP fee already captured; LPs and the
    // protocol share the fee pool rather than competing for it.
    pub fn owner_trading_fee(&self, trading_tokens: u128) -> Option<u128> {
        calculate_fee(
            trading_tokens,
            u128::from(self.owner_trade_fee_numerator),
            u128::from(self.owner_trade_fee_denominator),
        )
    }
}

// Helper for both fee calculations. Widening to u128 before multiplying
// prevents overflow for large token amounts — a u64 amount multiplied by a
// u64 numerator can exceed u64::MAX but not u128::MAX. Returns None only on
// arithmetic overflow, which cannot happen in practice because fee fractions
// are always less than 1.
pub fn calculate_fee(
    token_amount: u128,
    fee_numerator: u128,
    fee_denominator: u128,
) -> Option<u128> {
    if fee_numerator == 0 || token_amount == 0 {
        Some(0)
    } else {
        let fee = token_amount
            .checked_mul(fee_numerator)?
            .checked_div(fee_denominator)?;
        // Ensure at least 1 token is charged so the fee is never silently
        // waived on tiny swaps due to integer truncation.
        if fee == 0 { Some(1) } else { Some(fee) }
    }
}

// ---------------------------------------------------------------------------
// Depeg
//
// Configuration for stable pools that contain a liquid staking token (LST)
// whose value drifts relative to its underlying asset over time (e.g., mSOL
// accrues staking yield against SOL). The pool caches the current virtual
// price of the LST so it can price swaps correctly without performing a live
// oracle CPI on every trade, reducing compute unit cost and CPI depth.
//
// Fields:
//   base_virtual_price  — the cached exchange rate of the LST against its
//     base asset, expressed in the pool's precision units.
//   base_cache_updated  — Unix timestamp of the last virtual price refresh.
//     The pool rejects swaps if this cache is too stale.
//   depeg_type          — identifies which LST provider this pool targets,
//     which determines how the virtual price is fetched on refresh.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct Depeg {
    pub base_virtual_price: u64,
    pub base_cache_updated: u64,
    pub depeg_type: DepegType,
}

// ---------------------------------------------------------------------------
// Padding
//
// Reserved space in the Pool account layout. Anchor programs allocate a fixed
// account size at creation time. Padding fields hold that reserved space so
// future protocol upgrades can add new fields to the Pool struct without
// reallocating the account on-chain, which would require a migration
// instruction and break existing integrations.
//
// Fields:
//   padding0 — 15 bytes of byte-level padding for field alignment.
//   padding  — 29 u128 slots (464 bytes) of reserved future-use space.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct Padding {
    pub padding0: [u8; 15],
    pub padding: [u128; 29],
}

// ---------------------------------------------------------------------------
// RoundDirection
//
// Controls how fractional token amounts are rounded during swap math. The
// direction is chosen so that rounding always favours the pool over the user:
// when computing how much output the user receives, Floor is used (user gets
// slightly less); when computing how much input the pool requires, Ceiling is
// used (user pays slightly more). This prevents LPs from being drained by
// accumulated rounding errors across many small swaps.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum RoundDirection {
    #[default]
    Floor,
    Ceiling,
}

// ---------------------------------------------------------------------------
// TradeDirection
//
// Identifies which token is being sold into the pool. AtoB means the user
// is depositing token A and receiving token B; BtoA is the reverse. The pool
// uses this to select the correct source vault, destination vault, fee
// accumulator, and curve parameters for the swap without branching on pubkey
// comparisons at runtime.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum TradeDirection {
    #[default]
    AtoB,
    BtoA,
}

// ---------------------------------------------------------------------------
// NewCurveType
//
// An extension of CurveType introduced in a later protocol version, adding a
// third variant (NewCurve) as a placeholder for future AMM math. Defined in
// the IDL for forward-compatibility with tooling that reads pool accounts, but
// the on-chain swap handler does not yet route any logic through NewCurve.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum NewCurveType {
    #[default]
    ConstantProduct,
    Stable,
    NewCurve,
}

// ---------------------------------------------------------------------------
// CurveType
//
// The invariant formula the pool uses to price swaps and compute output
// amounts. This is the last serialized field in the Pool account.
//
// ConstantProduct — the classic x*y=k AMM. Price impact scales with trade
//   size relative to pool depth. Well-suited for uncorrelated asset pairs
//   where the fair market price fluctuates freely.
//
// Stable          — the StableSwap invariant (similar to Curve Finance).
//   Price impact is much lower near the peg price, making large trades
//   possible at near-zero slippage. Designed for assets that should trade
//   close to 1:1 such as stablecoin pairs or LST/SOL pairs.
//
// Borsh serializes this enum as a single u8 variant tag:
//   0 = ConstantProduct, 1 = Stable.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum CurveType {
    #[default]
    ConstantProduct,
    Stable,
}

// ---------------------------------------------------------------------------
// DepegType
//
// Identifies the liquid staking protocol whose exchange rate the pool tracks.
// The on-chain program uses this tag to know which program to CPI into when
// refreshing the virtual price cache stored in the Depeg field.
//
// None      — no LST involved; the pool is a regular stablecoin pair (e.g.,
//   USDC/USDT) that does not need virtual price logic.
// Marinade  — tracks mSOL, the Marinade Finance liquid staking token.
// Lido      — tracks stSOL, the Lido on Solana liquid staking token.
// SplStake  — tracks any SPL stake pool token not covered by the above two.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum DepegType {
    #[default]
    None,
    Marinade,
    Lido,
    SplStake,
}

// ---------------------------------------------------------------------------
// PoolType
//
// Controls who can provide liquidity to the pool.
//
// Permissioned    — only whitelisted addresses can deposit liquidity; used
//   for institutional or protocol-owned liquidity arrangements where the pool
//   operator wants full control over who participates as an LP.
// Permissionless  — anyone can deposit and withdraw; the standard public AMM
//   mode where liquidity provision is open to all participants.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub enum PoolType {
    #[default]
    Permissioned,
    Permissionless,
}

// ---------------------------------------------------------------------------
// Pool
//
// The on-chain account layout of a Meteora DAMM pool. Each pool holds two
// tokens (A and B) through a two-level structure: the Pool account stores
// references to two Meteora Vault accounts (a_vault and b_vault), and each
// Vault manages the actual SPL token accounts plus optional yield strategies.
// This indirection allows the pool to earn yield on idle liquidity without
// changing the Pool account structure — the vault layer handles all strategy
// interactions independently.
//
// Field order is fixed by the on-chain Anchor serialization layout. Borsh
// reads fields sequentially by byte offset with no named-field lookup, so
// any reordering produces silently incorrect deserialization where every
// field after the reordered pair reads the wrong bytes.
//
// Fields (in serialization order):
//   _discriminator    — the 8-byte Anchor account discriminator stamped at
//     the front of every Anchor account. Equals SHA256("account:Pool")[..8]
//     = [241, 154, 109, 4, 17, 177, 109, 188]. Consumed into this field
//     during borsh deserialization to keep the byte cursor aligned. Not
//     validated here because deserialize_unchecked is used at all call sites,
//     relying on program-ownership checks as the safety invariant instead.
//   lp_mint           — the mint of the LP token issued to liquidity
//     providers. LP tokens represent a proportional share of both vaults'
//     total managed liquidity.
//   token_a_mint      — the mint of token A (one side of the trading pair).
//   token_b_mint      — the mint of token B (the other side).
//   a_vault           — pubkey of the Meteora Vault account that holds and
//     manages token A liquidity. pool_parser fetches this account separately
//     to extract the vault's token_vault and lp_mint sub-accounts.
//   b_vault           — pubkey of the Meteora Vault account for token B.
//   a_vault_lp        — the SPL token account that holds the pool's LP
//     position in a_vault. Its token balance represents the pool's fractional
//     ownership of a_vault's total managed assets.
//   b_vault_lp        — same as a_vault_lp but for b_vault.
//   a_vault_lp_bump   — PDA bump seed for the a_vault_lp token account,
//     stored so the on-chain program can reconstruct the PDA signer without
//     iterating through nonces again.
//   enabled           — if false, the admin has frozen the pool and swaps
//     are rejected by the on-chain program.
//   admin_token_a_fee — SPL token account that accumulates the protocol's
//     share of trade fees collected on token A swaps.
//   admin_token_b_fee — same accumulator for token B fees.
//   admin             — authority pubkey that can reconfigure the pool,
//     update fees, freeze the pool, and collect accumulated admin fees.
//   fees              — the fee schedule (LP fee and protocol fee fractions).
//   pool_type         — whether liquidity provision is permissioned or open.
//   stake             — pubkey of a stake pool account; non-zero only for
//     pools that use liquid staking token virtual price logic (DepegType !=
//     None). Zero pubkey for standard stablecoin pools.
//   padding           — reserved account space for future protocol upgrades.
//   curve_type        — the AMM invariant (constant product or stable swap).
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub _discriminator: [u8; 8],
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_bump: u8,
    pub enabled: bool,
    pub admin_token_a_fee: Pubkey,
    pub admin_token_b_fee: Pubkey,
    pub admin: Pubkey,
    pub fees: PoolFees,
    pub pool_type: PoolType,
    pub stake: Pubkey,
    pub padding: Padding,
    pub curve_type: CurveType,
}

impl Pool {
    // Deserializes a Pool from a raw account data slice without validating
    // the 8-byte Anchor discriminator. The discriminator bytes are consumed
    // sequentially into _discriminator as part of normal borsh reading,
    // keeping the byte cursor aligned with all subsequent fields.
    //
    // "Unchecked" means no discriminator mismatch error is raised. The caller
    // is responsible for confirming the account is owned by the Meteora DAMM
    // program before calling this. Program ownership is the sufficient safety
    // invariant because only the DAMM program can write to accounts it owns,
    // and it always writes the Pool layout to its pool accounts.
    pub fn deserialize_unchecked(data: &[u8]) -> std::io::Result<Self> {
        let mut data_mut = data;
        Self::deserialize(&mut data_mut)
    }
}
