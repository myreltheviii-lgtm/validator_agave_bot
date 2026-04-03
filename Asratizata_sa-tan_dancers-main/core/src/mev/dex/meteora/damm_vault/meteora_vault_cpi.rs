use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

// ---------------------------------------------------------------------------
// VaultBumps
//
// Anchor programs derive program-derived addresses (PDAs) and store the bump
// seeds so they can be reproduced cheaply on-chain without iterating through
// nonces again on every instruction. The vault PDA bump and the token-vault
// PDA bump are stored here so the on-chain program can sign CPIs with a
// known, stable nonce without paying the compute cost of find_program_address
// on every swap.
//
// Fields:
//   vault_bump       — the nonce that makes the vault account itself a valid
//     PDA under the Meteora Vault program.
//   token_vault_bump — the nonce for the SPL token account PDA that holds
//     the vault's liquid reserve (tokens not deployed to any strategy).
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct VaultBumps {
    pub vault_bump: u8,
    pub token_vault_bump: u8,
}

// ---------------------------------------------------------------------------
// LockedProfitTracker
//
// Meteora Vault implements a time-decaying locked-profit mechanism to prevent
// sandwich attacks on yield harvests. When the vault harvests yield from an
// underlying strategy, those tokens are not immediately available to LPs —
// they are released linearly over a degradation window defined by
// locked_profit_degradation. This prevents an attacker from depositing just
// before a harvest, claiming the yield, and withdrawing immediately, which
// would steal value from long-term LPs.
//
// Fields:
//   last_updated_locked_profit — the total locked profit at the time of the
//     most recent harvest, in raw token units.
//   last_report                — Unix timestamp of the most recent harvest.
//     Used to compute how much time has elapsed since locking began.
//   locked_profit_degradation  — the rate at which locked profit is released
//     per second, expressed as a fraction of
//     LOCKED_PROFIT_DEGRADATION_DENOMINATOR (1_000_000_000_000). A higher
//     value means profit unlocks faster; at the maximum value the entire
//     locked profit is released after exactly one second.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct LockedProfitTracker {
    pub last_updated_locked_profit: u64,
    pub last_report: u64,
    pub locked_profit_degradation: u64,
}

// ---------------------------------------------------------------------------
// LOCKED_PROFIT_DEGRADATION_DENOMINATOR
//
// The fixed denominator used when computing what fraction of the locked
// profit has degraded (unlocked) over time since the last harvest. The ratio:
//
//   elapsed_seconds * locked_profit_degradation
//   ─────────────────────────────────────────────
//   LOCKED_PROFIT_DEGRADATION_DENOMINATOR
//
// gives the fraction of the original locked amount that has been released.
// Once this ratio reaches or exceeds 1 (i.e., the numerator equals or
// exceeds this constant), the entire locked profit is considered unlocked
// and is fully accessible to LPs.
// ---------------------------------------------------------------------------
pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

impl LockedProfitTracker {
    // Computes how many tokens are still locked at the given current_time.
    //
    // The remaining locked amount is the original locked profit scaled by
    // the fraction of the degradation window that has not yet elapsed:
    //
    //   remaining = last_updated_locked_profit
    //               * (DENOMINATOR - elapsed * degradation_rate)
    //               / DENOMINATOR
    //
    // Returns None if current_time is before last_report (clock anomaly) or
    // if any intermediate multiplication overflows u128, which cannot happen
    // in practice because all input values are u64-bounded.
    pub fn calculate_locked_profit(&self, current_time: u64) -> Option<u64> {
        let duration = u128::from(current_time.checked_sub(self.last_report)?);
        let locked_profit_degradation = u128::from(self.locked_profit_degradation);
        let locked_fund_ratio = duration.checked_mul(locked_profit_degradation)?;

        // Once the elapsed degradation ratio meets or exceeds the denominator,
        // the entire locked amount has been released — return zero so all
        // vault assets are treated as immediately available to LPs.
        if locked_fund_ratio > LOCKED_PROFIT_DEGRADATION_DENOMINATOR {
            return Some(0);
        }

        let locked_profit = u128::from(self.last_updated_locked_profit);
        let locked_profit = (locked_profit
            .checked_mul(LOCKED_PROFIT_DEGRADATION_DENOMINATOR - locked_fund_ratio)?)
            .checked_div(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)?;

        u64::try_from(locked_profit).ok()
    }
}

// ---------------------------------------------------------------------------
// Vault
//
// The on-chain account layout of a Meteora Vault program account. The vault
// sits between the DAMM liquidity pool and the underlying yield strategies
// (e.g., Solend, Tulip, Marginfi). The DAMM pool holds a_vault and b_vault
// references that point to two of these Vault accounts — one for each side
// of the trading pair — so idle liquidity can earn yield while remaining
// available for swaps.
//
// Field order is fixed by the on-chain Anchor serialization layout. Borsh
// reads fields sequentially by byte offset with no named-field lookup, so
// any reordering produces silently incorrect deserialization.
//
// Fields (in serialization order):
//   _discriminator        — the 8-byte Anchor account discriminator stamped
//     at the front of every Anchor account. Equals SHA256("account:Vault")
//     [..8] = [211, 8, 232, 43, 2, 152, 117, 119]. Consumed into this field
//     during borsh deserialization to maintain byte alignment with the rest
//     of the fields. Not validated here because deserialize_unchecked is used
//     at all call sites, relying on program-ownership checks instead.
//   enabled               — admin flag; 0 means the vault accepts withdrawals
//     only (deposits blocked); any non-zero value means normal operation.
//   bumps                 — PDA bump seeds for the vault and token_vault PDAs
//     stored for cheap on-chain reconstruction without brute-forcing nonces.
//   total_amount          — total tokens managed by the vault across all
//     active strategies plus the liquid token_vault reserve, in raw token
//     units. This is the gross amount before subtracting locked profit.
//   token_vault           — the SPL token account PDA that holds the vault's
//     liquid reserve (tokens not yet deployed to any strategy). This is the
//     field read by pool_parser to populate MeteoraDAmmPool.token_x_token_vault
//     and token_sol_token_vault.
//   fee_vault             — the SPL token account that accumulates protocol
//     performance fees charged on harvested yield.
//   token_mint            — the mint of the underlying token this vault
//     manages.
//   lp_mint               — the mint of the LP token issued to depositors.
//     The DAMM pool holds a_vault_lp and b_vault_lp token accounts for this
//     mint, representing the pool's share of each vault's total liquidity.
//     This is the field read by pool_parser as x_lp_mint and sol_lp_mint.
//   strategies            — up to 30 public keys of active yield strategy
//     accounts. The vault CPI's into each strategy to deposit and withdraw
//     liquidity. Unused slots are the zero pubkey.
//   base                  — the base keypair pubkey used as a seed to derive
//     the vault PDA deterministically.
//   admin                 — authority that can pause the vault, add or remove
//     strategies, and update fee parameters.
//   operator              — authority that can rebalance liquidity between
//     strategies and trigger yield harvests.
//   locked_profit_tracker — tracks the time-decaying locked profit from the
//     most recent yield harvest.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub _discriminator: [u8; 8],
    pub enabled: u8,
    pub bumps: VaultBumps,
    pub total_amount: u64,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub token_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub strategies: [Pubkey; 30],
    pub base: Pubkey,
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub locked_profit_tracker: LockedProfitTracker,
}

impl Vault {
    // Deserializes a Vault from a raw account data slice without validating
    // the 8-byte Anchor discriminator. The discriminator bytes are consumed
    // sequentially into _discriminator as part of normal borsh reading,
    // keeping the byte cursor aligned with all subsequent fields.
    //
    // "Unchecked" means no discriminator mismatch error is raised. The caller
    // is responsible for confirming the account is owned by the Meteora Vault
    // program before calling this. Program ownership is the sufficient safety
    // invariant because only the Vault program can write to accounts it owns,
    // and it always writes the Vault layout to its vault accounts.
    pub fn deserialize_unchecked(data: &[u8]) -> std::io::Result<Self> {
        let mut data_mut = data;
        Self::deserialize(&mut data_mut)
    }

    // Returns the number of tokens immediately available to LPs, which is
    // the total managed amount minus the portion still locked from the most
    // recent yield harvest. The locked portion decays linearly to zero over
    // the degradation window defined in locked_profit_tracker.
    //
    // Returns None if the locked profit calculation overflows or if
    // current_time is before the last harvest timestamp.
    pub fn get_unlocked_amount(&self, current_time: u64) -> Option<u64> {
        self.total_amount.checked_sub(
            self.locked_profit_tracker
                .calculate_locked_profit(current_time)?,
        )
    }

    // Converts a quantity of LP shares into the equivalent underlying token
    // amount at the given timestamp using the ratio:
    //   token_amount = share * unlocked_total / total_lp_supply
    //
    // Widening to u128 before multiplying prevents overflow — a u64 share
    // times a u64 total_amount can exceed u64::MAX. Returns None on overflow
    // or if total_supply is zero (division by zero).
    pub fn get_amount_by_share(
        &self,
        current_time: u64,
        share: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(share)
                .checked_mul(u128::from(total_amount))?
                .checked_div(u128::from(total_supply))?,
        )
        .ok()
    }

    // Computes how many LP tokens must be burned to withdraw out_token
    // underlying tokens. This is the inverse of get_amount_by_share:
    //   lp_to_burn = out_token * total_lp_supply / unlocked_total
    //
    // Returns None on overflow or if total_supply or unlocked_total is zero.
    pub fn get_unmint_amount(
        &self,
        current_time: u64,
        out_token: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(out_token)
                .checked_mul(u128::from(total_supply))?
                .checked_div(u128::from(total_amount))?,
        )
        .ok()
    }
}
