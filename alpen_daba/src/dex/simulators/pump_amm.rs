// src/dex/simulators/pump_swap.rs
//
// Self-contained PumpSwap AMM output estimator.
//
// Every other simulator in this crate delegates its math and account decoding
// to a first-party open-source SDK crate (orca_whirlpools, raydium_cp_swap,
// etc.).  PumpSwap publishes no such crate, so this file is intentionally
// self-contained: it owns the account struct definitions, the raw-byte
// decoders, the PDA derivation helpers, and the constant-product swap math.
// Nothing from the `dex::pump::amm_info` module is imported — this file is
// its own authoritative copy of every piece of on-chain logic it needs.
//
// The public surface is a single function that matches the standard simulator
// API used by every DEX in `dex::simulators`:
//
//   pub fn calculate_pump_swap_output(
//       accounts:       &AccountMap,
//       pool_address:   &Pubkey,
//       _slot:          u64,
//       _unix_timestamp: u64,
//       amount_in:      u64,
//       token_in:       &Pubkey,
//   ) -> Result<u64>
//
// The `router::dispatch` function calls this entry point and converts any
// returned `Err` to 0, treating 0 as "path is dead".

use anyhow::{anyhow, Result};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use tracing::{info, warn};
// AccountDeserialize provides the try_deserialize method used by TokenAccount
// below. The trait must be explicitly in scope for Rust to resolve the method
// call even though the concrete type (TokenAccount) is what the caller writes.
use anchor_lang::AccountDeserialize;

// anchor_spl::token_interface accepts both the classic SPL Token program
// (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) and the Token-2022 program
// (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb) as valid account owners.
//
// PumpSwap pools are created with Token-2022 mints, so their vaults are owned
// by the Token-2022 program. Using token_interface here replaces the manual
// owner branch that previously called spl_token_2022::state::Account::unpack
// and spl_token::state::Account::unpack separately, consolidating both paths
// into a single deserializer that handles owner validation internally.
use anchor_spl::token_interface::TokenAccount;

use crate::account_map::AccountMap;

// ── Program IDs ──────────────────────────────────────────────────────────────

// The PumpSwap AMM program that creates and owns all pool accounts, the
// GlobalConfig singleton PDA, and the pool-derived vault PDAs.  This is
// distinct from the original pump.fun bonding-curve program (6EF8rr...) which
// handles token launches; pools only exist here after a token has graduated
// from the bonding curve.
//
// Pubkey::try_from parses the base58 string at call time. The function is
// #[inline(always)] so the compiler can hoist the parse out of hot loops and
// the result can be used anywhere a &Pubkey is needed without a const binding.
#[inline(always)]
fn pump_amm_program_id() -> Pubkey {
    Pubkey::try_from("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA")
        .expect("PUMP_AMM_PROGRAM_ID is a valid base58 constant")
}

// The separate fee program that owns the FeeConfig PDA.  It was introduced to
// support market-cap-aware fee tiers layered on top of GlobalConfig's flat
// rates, without requiring a change to the core AMM program interface.  Both
// buy and sell instructions pass the FeeConfig account as a readonly input so
// the AMM can cross-program-read the tier structure at swap time.
#[inline(always)]
fn pump_fee_program_id() -> Pubkey {
    Pubkey::try_from("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ")
        .expect("PUMP_FEE_PROGRAM_ID is a valid base58 constant")
}

// ── Anchor discriminators ────────────────────────────────────────────────────

// The Anchor account discriminator for a PumpSwap AMM pool account.
// Anchor computes the discriminator as the first 8 bytes of SHA-256("account:Pool").
// The PumpSwap program owns multiple account types — pool accounts, config accounts,
// authority PDAs, and potentially zero-byte placeholder accounts — all returned by
// get_filtered_indexed_accounts. Checking the discriminator rejects every non-pool
// account before the discriminator skip (`&data[8..]`) is ever reached, which means
// zero-byte and sub-8-byte accounts are caught by the length pre-check below and
// never reach any slice operation that could panic.
// PumpSwap has undergone layout upgrades adding fields (is_mayhem_mode,
// is_cashback_coin) to the pool account. The discriminator remains stable across
// these upgrades because it is tied to the account type name "Pool", not the field
// layout. The per-field bounds checks after the discriminator skip handle both old
// and new layout versions gracefully.
const POOL_DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

// The Anchor account discriminator for the PumpSwap GlobalConfig account.
// SHA-256("account:GlobalConfig")[0..8]. GlobalConfig is a singleton PDA that
// stores the protocol-level fee rates, the eight protocol fee recipient addresses,
// the admin authority, and the packed disable_flags bitfield that gates buy/sell/
// deposit/withdraw/create_pool operations. The MEV system reads this account once
// at startup via bank.get_account() and caches it; the fee rates sourced here are
// fed directly into the swap estimator rather than relying on hardcoded constants
// that become stale after an on-chain fee update.
const GLOBAL_CONFIG_DISCRIMINATOR: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];

// The Anchor account discriminator for the PumpSwap FeeConfig account.
// SHA-256("account:FeeConfig")[0..8]. FeeConfig is owned by a separate fee program
// (pfeeUx...) and introduced a market-cap-based tiered fee structure layered on top
// of the flat fees in GlobalConfig. Every buy and sell instruction now requires this
// account as a readonly input. The on-chain program reads the tiers at swap time and
// applies the first matching tier; when no tier matches it falls back to flat_fees.
// The MEV estimator uses flat_fees as a conservative upper bound — tiered fees are
// always equal to or lower than flat_fees for qualifying pools, so using flat_fees
// produces false positives that are caught at SVM simulation, never false negatives
// that would silently skip profitable trades.
const FEE_CONFIG_DISCRIMINATOR: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];

// ── PDA seeds ────────────────────────────────────────────────────────────────

// The ASCII seed used to derive the coin-creator vault authority PDA.  This PDA
// controls the vault ATA into which the creator's share of swap fees accumulates
// until they are claimed.
const COIN_CREATOR_VAULT_SEED: &[u8] = b"creator_vault";

// The two constant byte seeds that derive the canonical FeeConfig PDA under
// PUMP_FEE_PROGRAM_ID. The first seed is the ASCII string "fee_config". The second
// is a 32-byte constant embedded in the fee program at compile time — it binds the
// PDA to a specific deployment, making it impossible to spoof via an alternative
// seed combination. Together they produce a single well-known address that all
// PumpSwap buy/sell instructions reference regardless of which pool is being traded.
const FEE_CONFIG_SEED: &[u8] = b"fee_config";
const FEE_CONFIG_PROGRAM_SEED: &[u8] = &[
     12,  20, 222, 252, 130,  94, 198, 118, 148,  37,   8,  24, 187, 101,  64, 101,
    244,  41, 141,  49,  86, 213, 113, 180, 212, 248,   9,  12,  24, 233, 168,  99,
];

// ── Pool account ─────────────────────────────────────────────────────────────

/// The fields the MEV system reads from a PumpSwap AMM pool account.
///
/// A pool account is created by the PumpSwap AMM program at token graduation.
/// It records the two token mints, the addresses of the two vault token
/// accounts, and the coin creator whose fee vault receives a portion of every
/// swap.  The full on-chain layout also contains the pool authority bump, an
/// LP mint address, lp_supply, and the pool index — these are read during
/// decoding but not stored here because the simulator does not need them.
#[derive(Debug)]
struct PumpAmmInfo {
    pub base_mint:                  Pubkey,
    pub quote_mint:                 Pubkey,
    pub pool_base_token_account:    Pubkey,
    pub pool_quote_token_account:   Pubkey,
    pub coin_creator:               Pubkey,
    /// Program Derived Address that acts as the signing authority over the coin
    /// creator's fee vault. Derived as
    /// `find_program_address(&[b"creator_vault", coin_creator], pump_program_id)`.
    /// The PDA itself holds no tokens — it delegates authority to the vault ATA.
    pub coin_creator_vault_authority: Pubkey,
    pub is_mayhem_mode:             bool,
    pub is_cashback_coin:           bool,
}

impl PumpAmmInfo {
    fn load_checked(data: &[u8]) -> Result<Self> {
        // The discriminator occupies the first 8 bytes of every Anchor-managed account.
        // Reading it requires at least 8 bytes to be present. This check must come
        // before the discriminator comparison because that comparison indexes `data[0..8]`
        // directly — without this guard a zero-byte account would panic on that slice.
        // It must also come before `&data[8..]` which panics identically on short input.
        if data.len() < 8 {
            return Err(anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        // The discriminator is the type-level identity of an Anchor account. It is
        // computed once at program compile time as SHA-256("account:Pool")[0..8] and
        // is permanently stamped into every account the program initializes under that
        // type. The PumpSwap program owns config accounts, authority PDAs, and other
        // account types in addition to pool accounts — all are returned by
        // get_filtered_indexed_accounts. Checking the discriminator here rejects every
        // non-pool account before any field offset is touched.
        if data[0..8] != POOL_DISCRIMINATOR {
            return Err(anyhow!(
                "Account discriminator does not match PumpSwap AMM pool discriminator"
            ));
        }

        // Skip the 8-byte Anchor discriminator that prefixes every account
        // managed by an Anchor program. The discriminator is the first 8 bytes
        // of the SHA-256 hash of the account type name and is used by the runtime
        // to verify that the correct account type is being deserialized.
        let data = &data[8..];

        // PumpSwap Pool account layout — all byte offsets are relative to the
        // post-discriminator slice (i.e. byte 0 here = byte 8 in raw account data).
        //
        //   pool_bump:                u8       offset   0  (1 byte)
        //   index:                    u16 LE   offset   1  (2 bytes)
        //   creator:                  Pubkey   offset   3  (32 bytes)
        //   base_mint:                Pubkey   offset  35  (32 bytes)
        //   quote_mint:               Pubkey   offset  67  (32 bytes)
        //   lp_mint:                  Pubkey   offset  99  (32 bytes)
        //   pool_base_token_account:  Pubkey   offset 131  (32 bytes)
        //   pool_quote_token_account: Pubkey   offset 163  (32 bytes)
        //   lp_supply:                u64 LE   offset 195  (8 bytes)
        //   coin_creator:             Pubkey   offset 203  (32 bytes)
        //   is_mayhem_mode:           bool     offset 235  (1 byte)
        //   is_cashback_coin:         bool     offset 236  (1 byte)  [later upgrade]
        //
        // The IDL-canonical Pool struct ends at is_mayhem_mode (236 bytes body).
        // is_cashback_coin was added in a subsequent on-chain upgrade and is read
        // only when the buffer is long enough to contain it; older accounts return false.
        let base_mint_offset  = 1 + 2 + 32; // bump + index + creator = 35
        let quote_mint_offset = base_mint_offset + 32; // = 67
        let pool_base_offset  = quote_mint_offset + 32 + 32; // + lp_mint = 131
        let pool_quote_offset = pool_base_offset + 32; // = 163

        // The minimum buffer length required to read the five mandatory pubkeys above.
        // This covers everything through pool_quote_token_account inclusive.
        let min_len = pool_quote_offset + 32; // = 195

        if data.len() < min_len {
            return Err(anyhow!("Invalid data length for PumpAmmInfo"));
        }

        let base_mint = Pubkey::new_from_array(
            data[base_mint_offset..base_mint_offset + 32].try_into().unwrap(),
        );
        let quote_mint = Pubkey::new_from_array(
            data[quote_mint_offset..quote_mint_offset + 32].try_into().unwrap(),
        );
        let pool_base_token_account = Pubkey::new_from_array(
            data[pool_base_offset..pool_base_offset + 32].try_into().unwrap(),
        );
        let pool_quote_token_account = Pubkey::new_from_array(
            data[pool_quote_offset..pool_quote_offset + 32].try_into().unwrap(),
        );

        // coin_creator sits immediately after pool_quote_token_account (32 bytes)
        // and lp_supply (8 bytes), placing it at post-discriminator offset 203.
        let coin_creator_offset     = pool_quote_offset + 32 + 8; // pool_quote_token_account + lp_supply = 203
        let is_mayhem_mode_offset   = coin_creator_offset + 32;   // = 235
        let is_cashback_coin_offset = is_mayhem_mode_offset + 1;  // = 236

        // PumpSwap has undergone layout upgrades that added fields beyond the mandatory
        // core fields above. Older pool accounts on-chain do not have these fields —
        // they are shorter than the full current layout. Rather than rejecting older
        // pools with an exact size check, each optional field is read only if the
        // buffer extends far enough to contain it, otherwise a safe default is used.
        let coin_creator = if coin_creator_offset + 32 > data.len() {
            Pubkey::default()
        } else {
            Pubkey::new_from_array(
                data[coin_creator_offset..coin_creator_offset + 32].try_into().unwrap(),
            )
        };

        let is_mayhem_mode = if is_mayhem_mode_offset >= data.len() {
            false
        } else {
            data[is_mayhem_mode_offset] != 0
        };

        let is_cashback_coin = if is_cashback_coin_offset >= data.len() {
            false
        } else {
            data[is_cashback_coin_offset] != 0
        };

        // The coin creator vault authority is a Program Derived Address that
        // controls the coin creator's fee vault. The vault holds the creator's
        // share of swap fees until they are claimed. The PDA is keyed by the
        // coin creator's pubkey and the "creator_vault" seed so that each
        // creator has a unique vault address. This PDA is computed once at
        // graduation time and stored — it is never recomputed on the hot path.
        let coin_creator_vault_authority = if coin_creator == Pubkey::default() {
            Pubkey::default()
        } else {
            Pubkey::find_program_address(
                &[COIN_CREATOR_VAULT_SEED, coin_creator.as_ref()],
                &pump_amm_program_id(),
            )
            .0
        };

        Ok(Self {
            base_mint,
            quote_mint,
            pool_base_token_account,
            pool_quote_token_account,
            coin_creator,
            coin_creator_vault_authority,
            is_mayhem_mode,
            is_cashback_coin,
        })
    }
}

// ── GlobalConfig account ─────────────────────────────────────────────────────

/// A snapshot of the PumpSwap GlobalConfig account. GlobalConfig is a singleton
/// PDA seeded with `b"global_config"` under the AMM program. It holds the
/// protocol-level fee rates governing every swap, the eight protocol fee recipient
/// addresses required for instruction building, and a packed bitfield that can
/// disable individual operations globally.
///
/// The MEV system reads this account once via `bank.get_account()` at startup and
/// caches the parsed result. Live fee rates are passed directly to the swap estimator
/// from the cache, keeping estimates correct across on-chain fee updates without
/// requiring the validator to be restarted.
#[derive(Debug, Clone)]
struct PumpGlobalConfig {
    pub lp_fee_basis_points:              u64,
    pub protocol_fee_basis_points:        u64,
    /// Packed bitfield that enables or disables individual pool operations.
    ///
    ///   bit 0 — create_pool disabled
    ///   bit 1 — deposit disabled
    ///   bit 2 — withdraw disabled
    ///   bit 3 — buy disabled
    ///   bit 4 — sell disabled
    ///
    /// The MEV system checks is_buy_disabled and is_sell_disabled before building
    /// any transaction. A globally disabled buy or sell will always revert on-chain
    /// regardless of profitability, so the check saves priority fees.
    pub disable_flags:                    u8,
    pub coin_creator_fee_basis_points:    u64,
}

impl PumpGlobalConfig {
    fn load_checked(data: &[u8]) -> Result<Self> {
        if data.len() < 8 {
            return Err(anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        if data[0..8] != GLOBAL_CONFIG_DISCRIMINATOR {
            return Err(anyhow!(
                "Account discriminator does not match PumpSwap GlobalConfig discriminator"
            ));
        }

        // Skip the 8-byte Anchor discriminator before reading fields.
        let data = &data[8..];

        // GlobalConfig account layout — all byte offsets are relative to the
        // post-discriminator slice (i.e. byte 0 here = byte 8 in raw account data).
        //
        //   admin:                            Pubkey     offset   0  (32 bytes)
        //   lp_fee_basis_points:              u64 LE     offset  32  (8 bytes)
        //   protocol_fee_basis_points:        u64 LE     offset  40  (8 bytes)
        //   disable_flags:                    u8         offset  48  (1 byte)
        //   protocol_fee_recipients[0..8]:    [Pubkey;8] offset  49  (256 bytes)
        //   coin_creator_fee_basis_points:    u64 LE     offset 305  (8 bytes)
        //   admin_set_coin_creator_authority: Pubkey     offset 313  (32 bytes)
        //   whitelist_pda:                    Pubkey     offset 345  (32 bytes)  [upgrade]
        //   reserved_fee_recipient:           Pubkey     offset 377  (32 bytes)  [upgrade]
        //   mayhem_mode_enabled:              bool       offset 409  (1 byte)    [upgrade]
        //
        // The minimum required size covers the mandatory fields through
        // admin_set_coin_creator_authority. The three fields marked [upgrade] were
        // added later and are parsed conditionally based on actual buffer length.
        let min_len = 32 + 8 + 8 + 1 + 256 + 8 + 32; // = 345
        if data.len() < min_len {
            return Err(anyhow!(
                "GlobalConfig account data too short: {} post-discriminator bytes, need at least {}",
                data.len(),
                min_len
            ));
        }

        let lp_fee_basis_points       = u64::from_le_bytes(data[32..40].try_into().unwrap());
        let protocol_fee_basis_points = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let disable_flags             = data[48];
        let coin_creator_fee_basis_points =
            u64::from_le_bytes(data[305..313].try_into().unwrap());

        Ok(Self {
            lp_fee_basis_points,
            protocol_fee_basis_points,
            disable_flags,
            coin_creator_fee_basis_points,
        })
    }

    /// Returns true when the disable_flags bitfield has the buy-disabled bit set.
    /// A globally disabled buy reverts on-chain regardless of pool state. The MEV
    /// system checks this before building any buy transaction to save priority fees.
    #[inline(always)]
    fn is_buy_disabled(&self) -> bool {
        self.disable_flags & 0b00001000 != 0
    }

    /// Returns true when the disable_flags bitfield has the sell-disabled bit set.
    #[inline(always)]
    fn is_sell_disabled(&self) -> bool {
        self.disable_flags & 0b00010000 != 0
    }
}

// ── FeeConfig account ────────────────────────────────────────────────────────

/// The flat fee rates stored in the FeeConfig account. These apply when no
/// market-cap tier threshold is satisfied by the pool being swapped. For the
/// MEV estimator, flat_fees represents the upper bound on fees — tiered fees
/// can only be equal to or lower, so using flat_fees never underestimates cost.
#[derive(Debug, Clone, Copy)]
struct PumpFlatFees {
    pub lp_fee_bps:       u64,
    pub protocol_fee_bps: u64,
    pub creator_fee_bps:  u64,
}

/// A partial parse of the PumpSwap FeeConfig account that reads only the fixed
/// `flat_fees` structure at the front of the account. The `fee_tiers` field that
/// follows is a Borsh-encoded variable-length vector and is deliberately skipped —
/// the MEV estimator needs only the flat fallback rates, which sit at known fixed
/// offsets before the vector begins.
///
/// FeeConfig is owned by PUMP_FEE_PROGRAM_ID, not the AMM program. Its canonical
/// address is returned by `fee_config_address()` and must be passed as a readonly
/// account in every PumpSwap buy and sell instruction.
#[derive(Debug, Clone)]
struct PumpFeeConfig {
    /// The flat fallback fee rates used by the on-chain program when the pool's
    /// market cap does not satisfy any of the tiered thresholds in `fee_tiers`.
    pub flat_fees: PumpFlatFees,
}

impl PumpFeeConfig {
    fn load_checked(data: &[u8]) -> Result<Self> {
        if data.len() < 8 {
            return Err(anyhow!(
                "Account data length {} is too short to contain an Anchor discriminator",
                data.len()
            ));
        }

        if data[0..8] != FEE_CONFIG_DISCRIMINATOR {
            return Err(anyhow!(
                "Account discriminator does not match PumpSwap FeeConfig discriminator"
            ));
        }

        // Skip the 8-byte Anchor discriminator before reading fields.
        let data = &data[8..];

        // FeeConfig account layout — all byte offsets are relative to the
        // post-discriminator slice (i.e. byte 0 here = byte 8 in raw account data).
        //
        //   bump:                    u8     offset  0  (1 byte)
        //   admin:                   Pubkey offset  1  (32 bytes)
        //   flat_fees.lp_fee_bps:    u64 LE offset 33  (8 bytes)
        //   flat_fees.protocol_bps:  u64 LE offset 41  (8 bytes)
        //   flat_fees.creator_bps:   u64 LE offset 49  (8 bytes)
        //   fee_tiers (vec):                offset 57  (variable — not parsed)
        //
        // Minimum valid size to read through creator_fee_bps: 57 bytes
        // post-discriminator (65 bytes total). The variable-length fee_tiers
        // Borsh vector is not parsed; only flat_fees is needed by the estimator.
        let min_len = 1 + 32 + 8 + 8 + 8; // = 57
        if data.len() < min_len {
            return Err(anyhow!(
                "FeeConfig account data too short: {} post-discriminator bytes, need at least {}",
                data.len(),
                min_len
            ));
        }

        let lp_fee_bps       = u64::from_le_bytes(data[33..41].try_into().unwrap());
        let protocol_fee_bps = u64::from_le_bytes(data[41..49].try_into().unwrap());
        let creator_fee_bps  = u64::from_le_bytes(data[49..57].try_into().unwrap());

        Ok(Self {
            flat_fees: PumpFlatFees { lp_fee_bps, protocol_fee_bps, creator_fee_bps },
        })
    }
}

// ── PDA helpers ──────────────────────────────────────────────────────────────

/// Derive the canonical address of the PumpSwap GlobalConfig singleton PDA.
///
/// The seed is the ASCII string `"global_config"` under the AMM program.
/// GlobalConfig is a singleton — there is exactly one address per AMM program
/// deployment. Callers that invoke the estimator in a tight loop should compute
/// this once at startup and pass the result in, rather than calling
/// `find_program_address` on every estimation (each call runs an iterated
/// SHA-256 bump search).
fn global_config_pda() -> Pubkey {
    Pubkey::find_program_address(&[b"global_config"], &pump_amm_program_id()).0
}

/// Derive the canonical address of the PumpSwap FeeConfig account.
///
/// Both seeds are compile-time constants, so the resulting PDA is the same on every
/// call. Callers should compute this once at startup and cache the result — calling
/// find_program_address on the hot path is expensive due to the iterated SHA-256
/// bump search.
///
/// The returned address must be passed as the `fee_config` readonly account in every
/// PumpSwap buy and sell instruction. Passing the wrong address causes the on-chain
/// program to fail account validation before any swap math runs.
fn fee_config_address() -> Pubkey {
    Pubkey::find_program_address(
        &[FEE_CONFIG_SEED, FEE_CONFIG_PROGRAM_SEED],
        &pump_fee_program_id(),
    )
    .0
}

// ── Internal fee rates ────────────────────────────────────────────────────────

/// The three fee components applied to every PumpSwap trade, resolved from
/// whatever fee source is available.  All values are in basis points
/// (1 bps = 0.01 %).
#[derive(Debug, Clone, Copy)]
struct Fees {
    lp_fee_bps:       u64,
    protocol_fee_bps: u64,
    creator_fee_bps:  u64,
}

// ── Fee resolution ────────────────────────────────────────────────────────────

/// Resolve the effective fee rates for a swap from the available on-chain sources.
///
/// PumpSwap has two sources of fee rates:
///
/// 1. `GlobalConfig` — the original flat-rate configuration present on every pool
///    from day one.  Always available.
///
/// 2. `FeeConfig` — a later addition owned by a separate fee program.  It holds
///    a flat_fees fallback and a variable-length vec of market-cap-based fee tiers.
///    When a pool's market cap satisfies a tier threshold the on-chain program uses
///    that tier's rates; otherwise it falls back to flat_fees.
///
/// The MEV estimator intentionally uses flat_fees from FeeConfig when it is present,
/// bypassing the tiered lookup entirely.  Tiered fees are always equal to or lower
/// than flat_fees, which means this choice is conservative: it slightly underestimates
/// net output (higher apparent fees) and therefore never produces a false positive
/// that would submit an unprofitable bundle.  Any false positives are caught later
/// at SVM simulation where the exact fees are applied.
///
/// When FeeConfig is absent from the AccountMap the GlobalConfig rates serve as the
/// fallback.  The coin-creator fee is zeroed unconditionally when the pool has no
/// registered creator (coin_creator == Pubkey::default), because the on-chain
/// program skips the creator fee transfer in that case.
fn compute_fees(
    global_config: &PumpGlobalConfig,
    fee_config:    Option<&PumpFeeConfig>,
    coin_creator:  &Pubkey,
) -> Fees {
    let has_creator = *coin_creator != Pubkey::default();

    if let Some(fc) = fee_config {
        Fees {
            lp_fee_bps:       fc.flat_fees.lp_fee_bps,
            protocol_fee_bps: fc.flat_fees.protocol_fee_bps,
            // The coin-creator fee slot in flat_fees is always populated in the
            // FeeConfig account, but the on-chain program only transfers it when
            // the pool has a registered creator. Zero it out here so the math
            // does not silently inflate the fee deduction for creator-less pools.
            creator_fee_bps: if has_creator { fc.flat_fees.creator_fee_bps } else { 0 },
        }
    } else {
        Fees {
            lp_fee_bps:       global_config.lp_fee_basis_points,
            protocol_fee_bps: global_config.protocol_fee_basis_points,
            creator_fee_bps:  if has_creator { global_config.coin_creator_fee_basis_points } else { 0 },
        }
    }
}

// ── Swap math ────────────────────────────────────────────────────────────────

/// Ceiling division: the smallest integer ≥ a / b.
///
/// The on-chain PumpSwap program uses ceiling division when computing the quote
/// required for a buy (the pool charges you the ceiling so you never pay less
/// than the invariant demands). Matching the ceiling here keeps our estimate of
/// the required SOL input identical to what the program computes, preventing
/// off-by-one optimism that would manifest as failed on-chain transactions.
fn ceil_div(a: u64, b: u64) -> Result<u64> {
    if b == 0 {
        return Err(anyhow!("Division by zero in ceil_div"));
    }
    a.checked_add(b)
        .and_then(|sum| sum.checked_sub(1))
        .and_then(|val| val.checked_div(b))
        .ok_or_else(|| anyhow!("Overflow in ceil_div: a={}, b={}", a, b))
}

/// Compute the fee amount for a given trade size and rate in basis points,
/// rounding up to the nearest lamport.
///
/// Rounding up matches the on-chain program and ensures the estimator never
/// overstates net output.  The fee charged on-chain is always at least this
/// value, so profits computed here are a lower bound on actual profits.
fn calculate_fee(amount: u64, basis_points: u64) -> Result<u64> {
    let numerator = amount
        .checked_mul(basis_points)
        .ok_or_else(|| anyhow!("Overflow calculating fee: amount={} bps={}", amount, basis_points))?;
    ceil_div(numerator, 10_000)
}

/// Estimate base tokens received when depositing `quote_in` lamports of SOL.
///
/// PumpSwap uses a constant-product invariant (x · y = k).  On a buy the
/// trader deposits SOL (quote) and the pool releases base tokens.  Fees are
/// assessed on the quote side *before* the AMM sees the deposit, so the pool
/// invariant only sees the net (post-fee) SOL:
///
///   total_fee_bps   = lp_fee_bps + protocol_fee_bps + creator_fee_bps
///   effective_quote = quote_in × 10 000 / (10 000 + total_fee_bps)
///   base_out        = base_reserve × effective_quote
///                     / (quote_reserve + effective_quote)    [floor]
///
/// Floor division on the output matches the on-chain program, which keeps any
/// fractional token in the pool rather than rounding in the trader's favour.
fn buy_quote_input(
    quote_in:      u64,
    base_reserve:  u64,
    quote_reserve: u64,
    fees:          &Fees,
    coin_creator:  &Pubkey,
) -> Result<u64> {
    if base_reserve == 0 || quote_reserve == 0 {
        return Err(anyhow!("Pool reserves cannot be zero"));
    }

    let has_creator = *coin_creator != Pubkey::default();
    let total_fee_bps = fees
        .lp_fee_bps
        .checked_add(fees.protocol_fee_bps)
        .and_then(|v| {
            if has_creator {
                v.checked_add(fees.creator_fee_bps)
            } else {
                Some(v)
            }
        })
        .ok_or_else(|| anyhow!("Overflow accumulating total fee bps"))?;

    // The fee denominator grows by total_fee_bps so that effective_quote is
    // always strictly less than quote_in when fees > 0. This is the standard
    // "fee-on-input" pattern: the gross input implicitly covers both the net
    // deposit and all fee components.
    let fee_denominator = 10_000u64
        .checked_add(total_fee_bps)
        .ok_or_else(|| anyhow!("Overflow in buy fee denominator"))?;

    let effective_quote = quote_in
        .checked_mul(10_000)
        .and_then(|v| v.checked_div(fee_denominator))
        .ok_or_else(|| anyhow!("Overflow computing effective quote for buy"))?;

    // Constant-product output: base_out = base_reserve × effective_quote
    //                                     / (quote_reserve + effective_quote)
    let numerator = (base_reserve as u128)
        .checked_mul(effective_quote as u128)
        .ok_or_else(|| anyhow!("u128 overflow in buy numerator"))?;
    let denominator = (quote_reserve as u128)
        .checked_add(effective_quote as u128)
        .ok_or_else(|| anyhow!("u128 overflow in buy denominator"))?;

    let base_out = numerator
        .checked_div(denominator)
        .ok_or_else(|| anyhow!("Division by zero in buy constant-product"))?;

    u64::try_from(base_out)
        .map_err(|_| anyhow!("buy base_out overflows u64: {}", base_out))
}

/// Estimate SOL lamports received when depositing `base_in` base tokens.
///
/// On a sell the trader deposits base tokens and the pool releases SOL.  The
/// gross SOL output is computed first via the constant-product invariant, then
/// each fee component is subtracted from that gross amount:
///
///   gross_quote = quote_reserve × base_in
///                 / (base_reserve + base_in)    [floor]
///   net_quote   = gross_quote − lp_fee − protocol_fee − creator_fee
///
/// Each fee is computed independently on gross_quote.  Computing them
/// sequentially on the running net would compound rounding errors across the
/// three deductions and diverge from on-chain behaviour.
fn sell_base_input(
    base_in:       u64,
    base_reserve:  u64,
    quote_reserve: u64,
    fees:          &Fees,
    coin_creator:  &Pubkey,
) -> Result<u64> {
    if base_reserve == 0 || quote_reserve == 0 {
        return Err(anyhow!("Pool reserves cannot be zero"));
    }

    // Constant-product gross output before any fee deduction.
    let denominator = (base_reserve as u128)
        .checked_add(base_in as u128)
        .ok_or_else(|| anyhow!("u128 overflow in sell denominator"))?;
    let numerator = (quote_reserve as u128)
        .checked_mul(base_in as u128)
        .ok_or_else(|| anyhow!("u128 overflow in sell numerator"))?;
    let gross_quote = numerator
        .checked_div(denominator)
        .ok_or_else(|| anyhow!("Division by zero in sell constant-product"))?;
    let gross_quote = u64::try_from(gross_quote)
        .map_err(|_| anyhow!("sell gross_quote overflows u64: {}", gross_quote))?;

    let lp_fee       = calculate_fee(gross_quote, fees.lp_fee_bps)?;
    let protocol_fee = calculate_fee(gross_quote, fees.protocol_fee_bps)?;
    let creator_fee  = if *coin_creator != Pubkey::default() {
        calculate_fee(gross_quote, fees.creator_fee_bps)?
    } else {
        0
    };

    gross_quote
        .checked_sub(lp_fee)
        .and_then(|v| v.checked_sub(protocol_fee))
        .and_then(|v| v.checked_sub(creator_fee))
        .ok_or_else(|| anyhow!(
            "Fee deductions exceed gross_quote={} (lp={}, protocol={}, creator={})",
            gross_quote, lp_fee, protocol_fee, creator_fee
        ))
}

// ── SPL token vault helper ────────────────────────────────────────────────────

/// Read the token balance from an SPL vault account owned by either the classic
/// SPL Token program or the Token-2022 program.
///
/// Both programs share the same base `Account` layout for the amount field, so
/// a single deserializer handles both. anchor_spl::token_interface::TokenAccount
/// accepts accounts owned by either program — it replaces the manual owner branch
/// that previously imported spl_token and spl_token_2022 as standalone crates.
/// Token-2022 accounts may carry extension data beyond the base 165 bytes; the
/// underlying deserializer reads only the fixed base fields and ignores extensions.
fn get_token_balance_from_account(account: &Account) -> Result<u64> {
    let mut data: &[u8] = &account.data;
    let token_account = TokenAccount::try_deserialize(&mut data)
        .map_err(|e| anyhow!(
            "Failed to deserialize token vault (owner={}): {:?}",
            account.owner, e
        ))?;
    Ok(token_account.amount)
}

// ── Simulator ────────────────────────────────────────────────────────────────

/// Holds a reference to the flat account map provided by the Agave shard for
/// one simulation round.  All account reads go through this map; there are no
/// external RPC or cache calls.
struct PumpSwapSimulator<'a> {
    accounts: &'a AccountMap,
}

impl<'a> PumpSwapSimulator<'a> {
    fn new(accounts: &'a AccountMap) -> Self {
        Self { accounts }
    }

    fn simulate_swap(
        &self,
        pool_address: &Pubkey,
        amount_in:    u64,
        token_in:     &Pubkey,
    ) -> Result<u64> {
        info!(
            "🔍 PUMP calc start: pool={}, amount_in={}, token_in={}",
            pool_address, amount_in, token_in
        );

        if amount_in == 0 {
            warn!("  ❌ PUMP amount_in is zero");
            return Err(anyhow!("Zero amount_in"));
        }

        // ── Pool account ─────────────────────────────────────────────────────

        let pool_account = self.accounts.get_account(pool_address).ok_or_else(|| {
            warn!("  ❌ PUMP pool account missing: {}", pool_address);
            anyhow!("Pool account missing: {}", pool_address)
        })?;

        let pool = PumpAmmInfo::load_checked(&pool_account.data).map_err(|e| {
            warn!("  ❌ PUMP pool deserialization failed: {}", e);
            e
        })?;
        info!("  ✅ PUMP pool deserialized");

        // Determine swap direction before loading any other accounts so we can
        // reject unknown tokens immediately.  PumpSwap pools hold the launched
        // base token in the base vault and WSOL (or the configured quote token)
        // in the quote vault.  A buy deposits SOL to receive base tokens; a sell
        // deposits base tokens to receive SOL.
        let is_sell = *token_in == pool.base_mint;
        let is_buy  = *token_in == pool.quote_mint;

        if !is_sell && !is_buy {
            warn!(
                "  ❌ PUMP token_in {} matches neither base_mint {} nor quote_mint {}",
                token_in, pool.base_mint, pool.quote_mint
            );
            return Err(anyhow!(
                "token_in {} matches neither base_mint nor quote_mint of pool {}",
                token_in, pool_address
            ));
        }

        // ── GlobalConfig ─────────────────────────────────────────────────────

        let gc_pda      = global_config_pda();
        let gc_account  = self.accounts.get_account(&gc_pda).ok_or_else(|| {
            warn!("  ❌ PUMP GlobalConfig missing from AccountMap (pda={})", gc_pda);
            anyhow!("GlobalConfig account missing")
        })?;

        let global_config = PumpGlobalConfig::load_checked(&gc_account.data).map_err(|e| {
            warn!("  ❌ PUMP GlobalConfig deserialization failed: {}", e);
            e
        })?;
        info!("  ✅ PUMP GlobalConfig deserialized");

        // Guard against globally disabled operations before paying for any
        // further account reads.  A disabled buy or sell reverts on-chain
        // unconditionally regardless of pool state or profitability.
        if is_buy && global_config.is_buy_disabled() {
            warn!("  ❌ PUMP buys are globally disabled");
            return Err(anyhow!("PumpSwap buys are globally disabled"));
        }
        if is_sell && global_config.is_sell_disabled() {
            warn!("  ❌ PUMP sells are globally disabled");
            return Err(anyhow!("PumpSwap sells are globally disabled"));
        }

        // ── FeeConfig (optional) ─────────────────────────────────────────────

        // FeeConfig is the preferred fee source because it reflects the latest
        // on-chain fee schedule. If it is absent from the AccountMap (the shard
        // did not include it in the wire request, or the account was not indexed)
        // the estimator falls back to GlobalConfig rates, which are always present.
        let fc_pda            = fee_config_address();
        let fee_config: Option<PumpFeeConfig> = match self.accounts.get_account(&fc_pda) {
            Some(acc) => match PumpFeeConfig::load_checked(&acc.data) {
                Ok(fc) => {
                    info!("  ✅ PUMP FeeConfig deserialized");
                    Some(fc)
                }
                Err(e) => {
                    warn!(
                        "  ⚠️  PUMP FeeConfig deserialization failed — falling back to GlobalConfig rates: {}",
                        e
                    );
                    None
                }
            },
            None => {
                info!("  ℹ️  PUMP FeeConfig not in AccountMap — using GlobalConfig fee rates");
                None
            }
        };

        // ── Vault balances ───────────────────────────────────────────────────

        let base_vault_account = self
            .accounts
            .get_account(&pool.pool_base_token_account)
            .ok_or_else(|| {
                warn!("  ❌ PUMP base vault missing: {}", pool.pool_base_token_account);
                anyhow!("Base vault account missing: {}", pool.pool_base_token_account)
            })?;

        let quote_vault_account = self
            .accounts
            .get_account(&pool.pool_quote_token_account)
            .ok_or_else(|| {
                warn!("  ❌ PUMP quote vault missing: {}", pool.pool_quote_token_account);
                anyhow!("Quote vault account missing: {}", pool.pool_quote_token_account)
            })?;

        let base_reserve = get_token_balance_from_account(&base_vault_account).map_err(|e| {
            warn!("  ❌ PUMP failed to read base reserve: {}", e);
            e
        })?;

        let quote_reserve = get_token_balance_from_account(&quote_vault_account).map_err(|e| {
            warn!("  ❌ PUMP failed to read quote reserve: {}", e);
            e
        })?;

        info!("  ✅ PUMP reserves — base: {}, quote: {}", base_reserve, quote_reserve);

        if base_reserve == 0 || quote_reserve == 0 {
            warn!(
                "  ❌ PUMP pool has zero reserves — base={} quote={}",
                base_reserve, quote_reserve
            );
            return Err(anyhow!("Pool has zero reserves"));
        }

        // ── Fee resolution ────────────────────────────────────────────────────

        let fees = compute_fees(&global_config, fee_config.as_ref(), &pool.coin_creator);
        info!(
            "  ✅ PUMP fees — lp={}bps protocol={}bps creator={}bps",
            fees.lp_fee_bps, fees.protocol_fee_bps, fees.creator_fee_bps
        );

        // ── Constant-product swap ─────────────────────────────────────────────

        let amount_out = if is_buy {
            info!("  🔄 PUMP direction: BUY (quote → base)");
            buy_quote_input(
                amount_in,
                base_reserve,
                quote_reserve,
                &fees,
                &pool.coin_creator,
            )
            .map_err(|e| {
                warn!("  ❌ PUMP buy_quote_input failed: {}", e);
                e
            })?
        } else {
            // is_sell is guaranteed true here by the earlier direction check.
            info!("  🔄 PUMP direction: SELL (base → quote)");
            sell_base_input(
                amount_in,
                base_reserve,
                quote_reserve,
                &fees,
                &pool.coin_creator,
            )
            .map_err(|e| {
                warn!("  ❌ PUMP sell_base_input failed: {}", e);
                e
            })?
        };

        if amount_out == 0 {
            warn!("  ❌ PUMP swap produced zero output");
            return Err(anyhow!("Swap produced zero output"));
        }

        info!("✅ PUMP output: {}", amount_out);
        info!(
            "  📊 Details: direction={}, base_reserve={}, quote_reserve={}, amount_in={}, amount_out={}",
            if is_buy { "BUY" } else { "SELL" },
            base_reserve, quote_reserve, amount_in, amount_out
        );

        Ok(amount_out)
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Estimate the output of a single PumpSwap hop given a flat account snapshot.
///
/// Matches the standard simulator API used by every DEX in `dex::simulators`.
/// The `router::dispatch` function calls this entry point and maps any returned
/// `Err` to 0, treating 0 as "path is dead".
///
/// `_slot` and `_unix_timestamp` are accepted for API uniformity but are not
/// used by the PumpSwap estimator — the constant-product math is time-invariant
/// and PumpSwap has no time-gated oracle or adaptive-fee component.
///
/// # Arguments
/// * `accounts`        — Flat map of all accounts the Agave shard included in
///                       the wire request.  Must contain the pool account, the
///                       GlobalConfig singleton, both vault token accounts, and
///                       optionally the FeeConfig account.
/// * `pool_address`    — The on-chain address of the PumpSwap pool account.
/// * `_slot`           — Current slot (unused).
/// * `_unix_timestamp` — Current Unix timestamp in seconds (unused).
/// * `amount_in`       — Exact input token amount in the token's native decimals.
/// * `token_in`        — Mint address of the token being deposited.
pub fn calculate_pump_swap_output(
    accounts:         &AccountMap,
    pool_address:     &Pubkey,
    _slot:            u64,
    _unix_timestamp:  u64,
    amount_in:        u64,
    token_in:         &Pubkey,
) -> Result<u64> {
    PumpSwapSimulator::new(accounts)
        .simulate_swap(pool_address, amount_in, token_in)
}
