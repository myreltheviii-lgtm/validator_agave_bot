use std::collections::HashMap;

use solana_clock::Slot;
use solana_pubkey::Pubkey;
use tracing::warn;

use crate::mev::constants::{SOL_MINT, USDC_MINT, USD1_MINT, USDT_MINT};
use crate::mev::dex::meteora::constants::{damm_v2_program_id, dlmm_program_id};
use crate::mev::dex::pump::pump_program_id;
use crate::mev::dex::raydium::{raydium_clmm_program_id, raydium_cp_program_id, raydium_program_id};
use crate::mev::dex::whirlpool::constants::whirlpool_program_id;

// ---------------------------------------------------------------------------
// Two-phase graduation detection — threading model
//
// This module runs across two OS threads and must never require synchronization
// between them.  The design achieves this by assigning each phase to exactly
// one thread:
//
//   Phase 1 — detect_instruction (bridge thread only)
//     The shredstream bridge calls this for every instruction in every entry
//     batch.  At this point the transaction has NOT been applied to any bank —
//     bank.get_account() would return None for any account the transaction
//     intends to write.  Phase 1 does zero bank access.  It inspects raw
//     instruction bytes, extracts pool addresses and mints, and returns
//     Option<DetectedPool> when a pool is ready.
//
//   Phase 2 — caller's responsibility (engine thread only)
//     The bridge sends DetectedPool over a crossbeam channel to the engine.
//     The engine stores it in a pending_ready map keyed by pool_address.
//     When a SpeculativeAccountUpdate arrives for that pubkey, the speculative
//     executor has already applied the transaction to the bank, so the account
//     exists in the speculative write cache. The engine parses it using the
//     GraduationSource discriminant to select the correct DEX parser, then
//     injects the pool into the running ArbitrageGraph.
//
// Because GraduationDetector is owned exclusively by the bridge task, no
// Mutex or Arc is needed.  The crossbeam channel connecting Phase 1 output
// to Phase 2 processing is the only cross-thread communication, and crossbeam
// channels are already lock-free and safe for concurrent send/recv.
//
// Race condition between graduation_tx and update_tx
//   For any DEX, Phase 1 fires (graduation_tx.send) before executor.execute()
//   runs on the same batch, and execute() produces the update_tx.send.  So by
//   the time the engine's update_rx receives a message, the corresponding
//   graduation event is already in graduation_rx.
//
//   However, crossbeam_channel::select! is non-deterministic among ready arms.
//   Even though graduation was sent before the update, select! may pick update_rx
//   first.  The engine handles this by draining graduation_rx with try_recv() in
//   the None arm of handle_speculative_update — the arm that fires when an account
//   is not yet tracked.  This drain runs before checking pending_ready, so even
//   in the race case the DetectedPool is absorbed from the channel and the new
//   pool is registered within the same handle_speculative_update call.
//
// Stale entry management
//   For single-event DEXes, Phase 1 fires before the transaction executes.
//   If the transaction fails, no SpeculativeAccountUpdate arrives for the pool
//   address and the engine's pending_ready entry remains. The engine's
//   dead_slot_rx handler sweeps pending_ready for all entries belonging to the
//   dead slot, using the `slot` field on DetectedPool. This bounds stale
//   accumulation to at most one slot's worth of unconfirmed pool creations.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Instruction discriminators
//
// Anchor programs compute a discriminator as sha256("global:<ix_name>")[..8].
// This is a stable, deployment-time constant — it never changes once the
// program is deployed on-chain.  Two programs that name an instruction the
// same way will produce the same discriminator bytes.
//
// Raydium AMM V4 predates Anchor entirely and uses a legacy scheme: a single
// byte at position 0 of the instruction data identifies the command.
//
// Discriminator collisions in this file (all safe — program_id checked first):
//
//   Collision 1: PumpSwap create_pool == Raydium CLMM create_pool
//                [233, 146, 209, 142, 207, 104, 64, 188]
//                Both programs name their instruction "create_pool".
//                Anchor computes sha256("global:create_pool")[..8] identically
//                for both, producing the same 8 bytes.
//
//   Collision 2: Whirlpool initialize_pool (v1) == Meteora DAMM V2 initialize_pool
//                [0x5f, 0xb4, 0x0a, 0xac, 0x54, 0xae, 0xe8, 0x28]
//                Both programs name their instruction "initialize_pool".
//
// In detect_instruction, each `if pid == <program>` block is a mutually
// exclusive branch with an early return.  A DLMM instruction can never match
// the Whirlpool branch because program IDs differ.  The collisions are therefore
// harmless at runtime.
// ---------------------------------------------------------------------------

/// PumpSwap `create_pool` / Raydium CLMM `create_pool`.
/// Same Anchor hash because both programs use the instruction name "create_pool".
/// Source: events.rs discriminators::CREATE_POOL_IX
///         events_3.rs discriminators::CREATE_POOL
const DISC_CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];

/// Raydium CLMM `increase_liquidity_v2`.
/// A newly created CLMM pool has empty tick arrays.  Swap output is zero until
/// at least one tick range is funded.  This instruction is the gate — its
/// presence means real reserves have been deposited and swaps are executable.
/// Source: events_3.rs discriminators::INCREASE_LIQUIDITY_V2
const DISC_CLMM_INCREASE_LIQ_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];

/// Raydium CPMM `initialize`.
/// Source: events_2.rs discriminators::INITIALIZE
const DISC_CPMM_INIT: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

/// Raydium AMM V4 `initialize2`.
/// This program predates Anchor and uses a single-byte opcode scheme rather
/// than 8-byte discriminators.  Opcode 0x01 == initialize2.  The value 0x01
/// is not globally unique — any program whose first instruction byte is 0x01
/// would produce a false match if the program_id guard were absent.  The
/// `if pid == raydium_program_id()` branch ensures this opcode is only
/// interpreted in the context of the AMM V4 program.
/// Source: events_4.rs discriminators::INITIALIZE2
const DISC_AMM_V4_INIT2: u8 = 1;

/// Meteora DAMM V2 `initialize_pool`.
/// Identical discriminator to Whirlpool `initialize_pool` v1 (collision 2).
/// Source: events_6.rs discriminators::INITIALIZE_POOL_IX
const DISC_METEORA_DAMM_INIT_POOL: [u8; 8] = [0x5f, 0xb4, 0x0a, 0xac, 0x54, 0xae, 0xe8, 0x28];

/// Meteora DAMM V2 `initialize_customizable_pool`.
/// Accepts per-pool fee parameters instead of inheriting from a shared config
/// account.  The pool and mint positions differ from initialize_pool because
/// the config account is absent, shifting everything one slot earlier.
/// Source: events_6.rs discriminators::INITIALIZE_CUSTOMIZABLE_POOL_IX
const DISC_METEORA_DAMM_INIT_CUSTOM: [u8; 8] = [0x14, 0xa1, 0xf1, 0x18, 0xbd, 0xdd, 0xb4, 0x02];

/// Meteora DAMM V2 `initialize_pool_with_dynamic_config`.
/// Uses a dynamic fee config account that allows fee rates to adjust
/// algorithmically.  Both `pool_creator_authority` and `config` accounts are
/// present, pushing the pool address two slots later vs the customizable variant.
/// Source: events_6.rs discriminators::INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX
const DISC_METEORA_DAMM_INIT_DYNAMIC: [u8; 8] = [0x95, 0x52, 0x48, 0xc5, 0xfd, 0xfc, 0x44, 0x0f];

/// Meteora DLMM `initialize_lb_pair`.
/// Source: initialize_lb_pair.rs
///         #[carbon(discriminator = "0x2d9aedd2dd0fa65c")]
const DISC_DLMM_INIT_LB_PAIR: [u8; 8] = [0x2d, 0x9a, 0xed, 0xd2, 0xdd, 0x0f, 0xa6, 0x5c];

/// Meteora DLMM `initialize_lb_pair2`.
/// Token-2022 compatible variant.  Adds token badge accounts after the funder
/// position (slot 8), but lb_pair and both mints remain at their original
/// positions (0, 2, 3), so the same position constants apply to both variants.
/// Source: initialize_lb_pair2.rs
///         #[carbon(discriminator = "0x493b2478ed536cc6")]
const DISC_DLMM_INIT_LB_PAIR2: [u8; 8] = [0x49, 0x3b, 0x24, 0x78, 0xed, 0x53, 0x6c, 0xc6];

/// Meteora DLMM `initialize_customizable_permissionless_lb_pair`.
/// Accepts a CustomizableParams struct rather than referencing a preset
/// parameter account.  lb_pair and mint positions are identical to initialize_lb_pair.
/// Source: initialize_customizable_permissionless_lb_pair.rs
///         #[carbon(discriminator = "0x2e2729876fb7c840")]
const DISC_DLMM_INIT_CUSTOM: [u8; 8] = [0x2e, 0x27, 0x29, 0x87, 0x6f, 0xb7, 0xc8, 0x40];

/// Meteora DLMM `initialize_customizable_permissionless_lb_pair2`.
/// Token-2022 compatible customizable variant.
/// Source: initialize_customizable_permissionless_lb_pair2.rs
///         #[carbon(discriminator = "0xf349817e3313f16b")]
const DISC_DLMM_INIT_CUSTOM2: [u8; 8] = [0xf3, 0x49, 0x81, 0x7e, 0x33, 0x13, 0xf1, 0x6b];

/// Meteora DLMM `add_liquidity`.
/// DLMM (Liquidity Book) pools store reserves in discrete price bins.  At pool
/// creation all bins are empty — a swap would read zero reserves and fail.
/// add_liquidity deposits tokens into specific bin ranges, making those ranges
/// tradeable for the first time.  This is the liquidity gate for all four DLMM
/// initialization variants.
/// Source: add_liquidity.rs
///         #[carbon(discriminator = "0xb59d59438fb63448")]
const DISC_DLMM_ADD_LIQUIDITY: [u8; 8] = [0xb5, 0x9d, 0x59, 0x43, 0x8f, 0xb6, 0x34, 0x48];

/// Meteora DLMM `add_liquidity2`.
/// Token-2022 compatible variant of add_liquidity.  The lb_pair account is at
/// position 1 in both variants, so both share DLMM_ADD_LIQ_LB_PAIR.
/// Source: add_liquidity2.rs
///         #[carbon(discriminator = "0xe4a24e1c46db7473")]
const DISC_DLMM_ADD_LIQUIDITY2: [u8; 8] = [0xe4, 0xa2, 0x4e, 0x1c, 0x46, 0xdb, 0x74, 0x73];

/// Orca Whirlpool `initialize_pool` (v1, SPL Token only).
/// Identical discriminator to DISC_METEORA_DAMM_INIT_POOL (collision 2).
/// V1 has no token badge accounts: the whirlpool address is at position 4.
/// Source: initialize_pool.rs
///         #[carbon(discriminator = "0x5fb40aac54aee828")]
const DISC_WHIRLPOOL_INIT_POOL_V1: [u8; 8] = [0x5f, 0xb4, 0x0a, 0xac, 0x54, 0xae, 0xe8, 0x28];

/// Orca Whirlpool `initialize_pool_v2` (Token-2022 compatible).
/// V2 inserts token_badge_a and token_badge_b before funder, shifting the
/// whirlpool address from position 4 (v1) to position 6 (v2).  Both mint
/// positions remain at 1 and 2 in both versions.
/// Source: initialize_pool_v2.rs
///         #[carbon(discriminator = "0xcf2d57f21b3fcc43")]
const DISC_WHIRLPOOL_INIT_POOL_V2: [u8; 8] = [0xcf, 0x2d, 0x57, 0xf2, 0x1b, 0x3f, 0xcc, 0x43];

/// Orca Whirlpool `increase_liquidity`.
/// Whirlpool is a concentrated liquidity protocol.  Tick arrays are empty at
/// pool creation — no price range is tradeable until a position is opened and
/// funded.  This instruction deposits tokens into a specific tick range via a
/// position NFT, making that range executable for swaps.
/// Source: increase_liquidity.rs
///         #[carbon(discriminator = "0x2e9cf3760dcdfbb2")]
const DISC_WHIRLPOOL_INCREASE_LIQ: [u8; 8] = [0x2e, 0x9c, 0xf3, 0x76, 0x0d, 0xcd, 0xfb, 0xb2];

/// Orca Whirlpool `increase_liquidity_v2`.
/// Token-2022 compatible variant.  The whirlpool address is at position 0 in
/// both v1 and v2, so both share WHIRLPOOL_INCREASE_LIQ_POOL.
/// Source: increase_liquidity_v2.rs
///         #[carbon(discriminator = "0x851d59df45eeb00a")]
const DISC_WHIRLPOOL_INCREASE_LIQ_V2: [u8; 8] = [0x85, 0x1d, 0x59, 0xdf, 0x45, 0xee, 0xb0, 0x0a];

// ---------------------------------------------------------------------------
// Account position constants
//
// All positions are 0-indexed into the instruction's resolved account slice.
// "Resolved" means the transaction message's flat account key array has already
// been dereferenced by the caller — `accounts[i]` is a concrete Pubkey, not a
// message index.
//
// Each constant is verified against the upstream source file's arrange_accounts
// field ordering, which exactly matches the IDL account declaration order.
// ---------------------------------------------------------------------------

// ── PumpSwap create_pool ─────────────────────────────────────────────────────
// Source: parser.rs parse_create_pool_instruction
//   0: pool           ← new pool address
//   1: global_config
//   2: creator
//   3: base_mint      ← token A
//   4: quote_mint     ← token B
//   5: lp_mint
//   6: user_base_token_account
//   7: user_quote_token_account
//   8: user_pool_token_account
//   9: pool_base_token_account
//  10: pool_quote_token_account
const PUMPSWAP_POOL: usize = 0;
const PUMPSWAP_BASE_MINT: usize = 3;
const PUMPSWAP_QUOTE_MINT: usize = 4;

// ── Raydium CLMM create_pool ─────────────────────────────────────────────────
// Source: events_3.rs RaydiumClmmCreatePoolEvent / parser_3.rs
//   0: pool_creator
//   1: amm_config
//   2: pool_state     ← new pool address
//   3: token_mint0   ← token A
//   4: token_mint1   ← token B
//   5: token_vault0
//   6: token_vault1
//   7: observation_state
//   8: tick_array_bitmap
//   9: token_program0
//  10: token_program1
//  11: system_program
//  12: rent
const CLMM_CREATE_POOL_STATE: usize = 2;
const CLMM_CREATE_MINT0: usize = 3;
const CLMM_CREATE_MINT1: usize = 4;

// ── Raydium CLMM increase_liquidity_v2 ───────────────────────────────────────
// Source: parser_3.rs parse_increase_liquidity_v2_instruction
//   0: nft_owner
//   1: nft_account
//   2: pool_state     ← the pool receiving liquidity
//   3: protocol_position
//   4: personal_position
//   5: tick_array_lower
//   6: tick_array_upper
//   7: token_account0
//   8: token_account1
//   9: token_vault0
//  10: token_vault1
//  11: token_program
//  12: token_program2022
//  13: vault0_mint
//  14: vault1_mint
const CLMM_INCR_LIQ_POOL_STATE: usize = 2;

// ── Raydium CPMM initialize ───────────────────────────────────────────────────
// Source: parser_2.rs parse_initialize_instruction
//   0: creator
//   1: amm_config
//   2: authority
//   3: pool_state     ← new pool address
//   4: token_0_mint  ← token A
//   5: token_1_mint  ← token B
//   6: lp_mint
//   7..19: vaults, programs, system accounts
const CPMM_INIT_POOL_STATE: usize = 3;
const CPMM_INIT_MINT0: usize = 4;
const CPMM_INIT_MINT1: usize = 5;

// ── Raydium AMM V4 initialize2 ───────────────────────────────────────────────
// Source: parser_4.rs parse_initialize2_instruction
//   0: token_program
//   1: spl_associated_token_account
//   2: system_program
//   3: rent
//   4: amm            ← new pool address
//   5: amm_authority
//   6: amm_open_orders
//   7: lp_mint
//   8: coin_mint      ← token A
//   9: pc_mint        ← token B
//  10..20: token accounts, serum accounts
const AMM_V4_INIT_POOL: usize = 4;
const AMM_V4_INIT_COIN_MINT: usize = 8;
const AMM_V4_INIT_PC_MINT: usize = 9;

// ── Meteora DAMM V2 initialize_pool ──────────────────────────────────────────
// Source: parser_6.rs parse_initialize_pool_instruction
//   0: creator
//   1: position_nft_mint
//   2: position_nft_account
//   3: payer
//   4: config
//   5: pool_authority
//   6: pool           ← new pool address
//   7: position
//   8: token_a_mint  ← token A
//   9: token_b_mint  ← token B
//  10..19: vaults, programs, event_authority, program
const METEORA_DAMM_INIT_POOL: usize = 6;
const METEORA_DAMM_INIT_MINT_A: usize = 8;
const METEORA_DAMM_INIT_MINT_B: usize = 9;

// ── Meteora DAMM V2 initialize_customizable_pool ─────────────────────────────
// Source: parser_6.rs parse_initialize_customizable_pool_instruction
// The config account is absent here compared to initialize_pool.  pool_authority
// therefore occupies position 4 (not 5), and the pool address moves to position 5
// (not 6).  Mint positions shift one slot earlier for the same reason.
//   0: creator
//   1: position_nft_mint
//   2: position_nft_account
//   3: payer
//   4: pool_authority    ← no config account before this
//   5: pool              ← new pool address
//   6: position
//   7: token_a_mint     ← token A
//   8: token_b_mint     ← token B
//   9..18: vaults, programs, event_authority, program
const METEORA_DAMM_CUSTOM_POOL: usize = 5;
const METEORA_DAMM_CUSTOM_MINT_A: usize = 7;
const METEORA_DAMM_CUSTOM_MINT_B: usize = 8;

// ── Meteora DAMM V2 initialize_pool_with_dynamic_config ──────────────────────
// Source: parser_6.rs parse_initialize_pool_with_dynamic_config_instruction
// Both pool_creator_authority and config are present, which shifts every
// subsequent account two positions later compared to the customizable variant.
//   0: creator
//   1: position_nft_mint
//   2: position_nft_account
//   3: payer
//   4: pool_creator_authority   ← extra account vs customizable
//   5: config                   ← extra account vs customizable
//   6: pool_authority
//   7: pool              ← new pool address
//   8: position
//   9: token_a_mint     ← token A
//  10: token_b_mint     ← token B
//  11..20: vaults, programs, event_authority, program
const METEORA_DAMM_DYNAMIC_POOL: usize = 7;
const METEORA_DAMM_DYNAMIC_MINT_A: usize = 9;
const METEORA_DAMM_DYNAMIC_MINT_B: usize = 10;

// ── Meteora DLMM initialize_lb_pair / lb_pair2 / customizable / customizable2 ─
// Source: initialize_lb_pair.rs / initialize_lb_pair2.rs /
//         initialize_customizable_permissionless_lb_pair.rs /
//         initialize_customizable_permissionless_lb_pair2.rs
//
// All four variants share the same positions for lb_pair, token_mint_x, and
// token_mint_y.  The V2 variants add token badge accounts after position 8
// (funder), but the first nine positions are identical across all four.
//   0: lb_pair          ← new pool address
//   1: bin_array_bitmap_extension
//   2: token_mint_x    ← token A
//   3: token_mint_y    ← token B
//   4: reserve_x
//   5: reserve_y
//   6: oracle
//   7: preset_parameter (v1) / user_token_x (customizable variants)
//   8: funder
//   [V2 only: 9: token_badge_x, 10: token_badge_y, then programs]
const DLMM_INIT_LB_PAIR: usize = 0;
const DLMM_INIT_MINT_X: usize = 2;
const DLMM_INIT_MINT_Y: usize = 3;

// ── Meteora DLMM add_liquidity / add_liquidity2 ──────────────────────────────
// Source: add_liquidity.rs / add_liquidity2.rs arrange_accounts
// Position 0 is the user's position account, not the pool itself.
// lb_pair is at position 1 in both the original and Token-2022 variants.
//   0: position
//   1: lb_pair          ← the pool receiving liquidity
//   2: bin_array_bitmap_extension
//   ...
const DLMM_ADD_LIQ_LB_PAIR: usize = 1;

// ── Orca Whirlpool initialize_pool (v1) ──────────────────────────────────────
// Source: initialize_pool.rs arrange_accounts
// V1 has no token badge accounts.  funder is at position 3 and whirlpool
// immediately follows at position 4.
//   0: whirlpools_config
//   1: token_mint_a    ← token A
//   2: token_mint_b    ← token B
//   3: funder
//   4: whirlpool       ← new pool address
//   5: token_vault_a
//   6: token_vault_b
//   7: fee_tier
//   8: token_program
//   9: system_program
//  10: rent
const WHIRLPOOL_V1_POOL: usize = 4;

// ── Orca Whirlpool initialize_pool_v2 ────────────────────────────────────────
// Source: initialize_pool_v2.rs arrange_accounts
// V2 inserts token_badge_a and token_badge_b before funder.  Mint positions
// are unchanged (1, 2) but the whirlpool address moves from position 4 to
// position 6 because two badge accounts are inserted at positions 3 and 4.
//   0: whirlpools_config
//   1: token_mint_a    ← token A  (same as v1)
//   2: token_mint_b    ← token B  (same as v1)
//   3: token_badge_a   ← absent in v1
//   4: token_badge_b   ← absent in v1
//   5: funder
//   6: whirlpool       ← new pool address (was 4 in v1)
//   7: token_vault_a
//   8: token_vault_b
//   9: fee_tier
//  10: token_program_a
//  11: token_program_b
//  12: system_program
//  13: rent
const WHIRLPOOL_V2_POOL: usize = 6;
// Mint positions are the same in both v1 and v2.
// WHIRLPOOL_MINT_A = 1, WHIRLPOOL_MINT_B = 2 for both versions.
const WHIRLPOOL_MINT_A: usize = 1;
const WHIRLPOOL_MINT_B: usize = 2;

// ── Orca Whirlpool increase_liquidity / increase_liquidity_v2 ────────────────
// Source: increase_liquidity.rs / increase_liquidity_v2.rs arrange_accounts
// Both variants place the whirlpool address at position 0.
//   0: whirlpool       ← the pool receiving liquidity
//   1: token_program (v1) / token_program_a (v2)
//   ...
const WHIRLPOOL_INCREASE_LIQ_POOL: usize = 0;

// ---------------------------------------------------------------------------
// Per-DEX pending map size caps
//
// Pool creation is permissionless on every supported DEX.  An adversary can
// spam creation transactions to grow a pending map without bound.  The caps
// prevent a single DEX's spam from exhausting validator memory.
//
// Each concentrated-liquidity DEX has its own dedicated map and its own cap,
// so spam targeting one DEX cannot cause the detection of another DEX's pools
// to be dropped.  This isolation is the reason three separate maps exist
// rather than one shared map — a shared map with per-DEX caps would allow one
// DEX to fill the map and starve all others.
// ---------------------------------------------------------------------------

/// Maximum pending Raydium CLMM pools awaiting increase_liquidity_v2.
const MAX_PENDING_CLMM: usize = 1024;

/// Maximum pending Orca Whirlpool pools awaiting increase_liquidity or
/// increase_liquidity_v2.
const MAX_PENDING_WHIRLPOOL: usize = 1024;

/// Maximum pending Meteora DLMM lb_pairs awaiting add_liquidity or add_liquidity2.
const MAX_PENDING_DLMM: usize = 1024;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The output of Phase 1.  When detect_instruction returns Some(DetectedPool),
/// the bridge sends this over its crossbeam channel to the engine.
///
/// The engine stores it in a HashMap<Pubkey, DetectedPool> keyed by pool_address
/// (pending_ready).  When a SpeculativeAccountUpdate arrives for that pubkey,
/// the engine invokes the appropriate pool parser and injects the pool into the
/// running ArbitrageGraph for the matching mint.
///
/// `slot` allows the engine's dead_slot_rx handler to sweep pending_ready when
/// a slot is declared dead by canonical replay.  Without the slot field, stale
/// entries from transactions that failed within a dead slot would remain in
/// pending_ready indefinitely, bounded only by the DEX-specific pending map caps.
#[derive(Debug, Clone)]
pub struct DetectedPool {
    /// On-chain address of the newly created pool.
    pub pool_address: Pubkey,
    /// First mint of the trading pair (base token or token A depending on DEX).
    pub mint0: Pubkey,
    /// Second mint of the trading pair (quote token or token B depending on DEX).
    pub mint1: Pubkey,
    /// Which DEX created this pool.  The engine uses this to dispatch to the
    /// correct pool parser — a CLMM PoolState cannot deserialize a Whirlpool
    /// account and vice versa.
    pub source: GraduationSource,
    /// The slot in which this pool creation instruction was observed.
    /// Used by the engine's dead_slot_rx handler to sweep pending_ready:
    /// when a slot is marked dead, all DetectedPool entries for that slot
    /// are removed because the creation transaction never landed.
    pub slot: Slot,
}

/// Identifies which DEX produced a pool detection event.
///
/// The engine uses this to dispatch to the correct pool parser because each
/// DEX stores pool state in a different on-chain account layout.  A CLMM
/// PoolState struct cannot be used to deserialise a Whirlpool account and
/// vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraduationSource {
    /// PumpSwap AMM pool created from a Pump.fun bonding curve graduation.
    /// The graduation transaction atomically closes the bonding curve and opens
    /// the AMM pool with the bonding curve's accumulated SOL and tokens as
    /// initial liquidity.  A single instruction is sufficient.
    PumpSwap,

    /// Raydium Concentrated Liquidity Market Maker.
    /// Only emitted after BOTH create_pool AND increase_liquidity_v2 have been
    /// observed for the same pool address.  CLMM tick arrays are empty at
    /// creation — the pool cannot execute swaps until at least one tick range
    /// is funded.
    RaydiumClmm,

    /// Raydium Constant Product Market Maker (constant product, x*y=k).
    /// Liquidity is seeded atomically within the initialize instruction.
    /// A single instruction is sufficient.
    RaydiumCpmm,

    /// Raydium legacy AMM V4 (constant product, pre-Anchor).
    /// Liquidity is seeded within the initialize2 instruction.
    /// A single instruction is sufficient.
    RaydiumAmmV4,

    /// Meteora Dynamic AMM V2.
    /// Any of the three initialize variants (standard, customizable, dynamic
    /// config) seeds liquidity atomically at initialization.
    /// A single instruction is sufficient regardless of which variant.
    MeteoraDammV2,

    /// Meteora DLMM (Liquidity Book).
    /// Only emitted after BOTH initialize_lb_pair (any of the four variants)
    /// AND add_liquidity (any variant) have been observed for the same lb_pair.
    /// DLMM bins are empty at creation — swaps fail with zero reserves until
    /// a liquidity provider deposits into at least one bin range.
    MeteoraDlmm,

    /// Orca Whirlpool (Concentrated Liquidity).
    /// Only emitted after BOTH initialize_pool (v1 or v2) AND increase_liquidity
    /// (v1 or v2) have been observed for the same whirlpool.  Tick arrays are
    /// empty at creation — the pool is not usable until a position is opened
    /// and funded.
    OrcaWhirlpool,
}

/// A concentrated-liquidity pool that has been created on-chain but has not
/// yet received its first liquidity deposit.
///
/// Held in the appropriate per-DEX pending map from the moment the creation
/// instruction is detected until the liquidity gate instruction is observed
/// for the same pool address.
///
/// `slot` records which slot this creation was detected in.  `clear_dead_slot`
/// uses it to sweep entries whose creation transaction will never land because
/// canonical replay declared the slot dead.
struct PendingConcentratedPool {
    mint0: Pubkey,
    mint1: Pubkey,
    source: GraduationSource,
    slot: Slot,
}

// ---------------------------------------------------------------------------
// GraduationDetector
// ---------------------------------------------------------------------------

/// Bridge-side Phase 1 of the two-phase graduation detection pipeline.
///
/// This struct is owned exclusively by the shredstream bridge task.
/// It has no internal locks and is intentionally not shared across threads —
/// `detect_instruction` is called only from the bridge's async task context.
///
/// ## Usage
///
/// For every instruction in every entry batch, call detect_instruction.
/// If it returns Some(DetectedPool), send that over the bridge's crossbeam
/// channel to the engine for Phase 2 processing.
///
/// ## Concentrated liquidity two-step sequence
///
/// Raydium CLMM, Orca Whirlpool, and Meteora DLMM separate pool creation and
/// liquidity seeding into two distinct transactions.  detect_instruction
/// maintains three per-DEX pending maps to track pools that have been created
/// but not yet funded.  The sequence is:
///
///   Step 1: Creation instruction detected
///     → extract pool address and mints, store in per-DEX pending map
///     → return None (pool not yet tradeable)
///
///   Step 2: Liquidity gate instruction detected for a pool in the pending map
///     → remove from pending map
///     → return Some(DetectedPool) for the engine to handle
///
/// If a liquidity provider bundles the creation and initial deposit in the same
/// atomic transaction, the bridge processes both instructions sequentially from
/// the same entry batch: step 1 inserts into the pending map, step 2 immediately
/// promotes from it, and Some(DetectedPool) is returned within the same batch.
/// The pending map is purely an intra-task local buffer with no synchronization
/// overhead, so same-transaction bundling is handled with zero additional cost.
pub struct GraduationDetector {
    /// Raydium CLMM pools created but awaiting increase_liquidity_v2.
    /// Capped at MAX_PENDING_CLMM.  Only CLMM creation instructions write to
    /// this map — DLMM and Whirlpool spam cannot fill it.
    pending_clmm: HashMap<Pubkey, PendingConcentratedPool>,

    /// Orca Whirlpool pools created but awaiting increase_liquidity.
    /// Capped at MAX_PENDING_WHIRLPOOL.
    pending_whirlpool: HashMap<Pubkey, PendingConcentratedPool>,

    /// Meteora DLMM lb_pairs created but awaiting add_liquidity.
    /// Capped at MAX_PENDING_DLMM.
    pending_dlmm: HashMap<Pubkey, PendingConcentratedPool>,
}

impl GraduationDetector {
    pub fn new() -> Self {
        Self {
            pending_clmm: HashMap::with_capacity(256),
            pending_whirlpool: HashMap::with_capacity(256),
            pending_dlmm: HashMap::with_capacity(256),
        }
    }

    /// Phase 1: scan one instruction from the raw entry stream.
    ///
    /// Must be called on the bridge task only.  Does not access any bank.
    ///
    /// `slot`       — the slot this instruction belongs to, embedded in
    ///                DetectedPool so the engine can sweep stale entries on
    ///                dead-slot notification.
    /// `program_id` — the program this instruction targets.
    /// `data`       — raw instruction data bytes, including the 8-byte
    ///                discriminator prefix for Anchor programs.
    /// `accounts`   — the instruction's account list, already resolved from
    ///                the transaction message's flat key array into concrete
    ///                Pubkeys by the caller.
    ///
    /// Returns `Some(DetectedPool)` when a pool is ready for the engine.
    /// Returns `None` in all other cases — which is the dominant path, since
    /// the vast majority of instructions are swaps or liquidity management on
    /// existing pools, not new pool creation.
    pub fn detect_instruction(
        &mut self,
        slot: Slot,
        program_id: &Pubkey,
        data: &[u8],
        accounts: &[Pubkey],
    ) -> Option<DetectedPool> {
        let pid = *program_id;

        // ── PumpSwap ─────────────────────────────────────────────────────────
        // The graduation transaction atomically closes the Pump.fun bonding
        // curve and opens the AMM pool with the bonding curve's accumulated
        // reserves as initial liquidity.  A single instruction is sufficient.
        if pid == pump_program_id() {
            if disc8(data) == Some(DISC_CREATE_POOL) {
                return make_detected(
                    slot,
                    accounts,
                    PUMPSWAP_POOL,
                    PUMPSWAP_BASE_MINT,
                    PUMPSWAP_QUOTE_MINT,
                    GraduationSource::PumpSwap,
                );
            }
            return None;
        }

        // ── Raydium CLMM ─────────────────────────────────────────────────────
        // Two-step: create_pool allocates the PoolState account but leaves all
        // tick arrays empty.  Executing a swap against a pool with no funded
        // ticks would return zero output — the simulation would never pass.
        // increase_liquidity_v2 is the gate that tells us real reserves exist.
        //
        // The discriminator is computed once and reused for both branches to
        // avoid reading the same 8 bytes twice on the else-if path.
        if pid == raydium_clmm_program_id() {
            let d = disc8(data);
            if d == Some(DISC_CREATE_POOL) {
                if let Some(entry) = extract_concentrated(
                    accounts,
                    CLMM_CREATE_POOL_STATE,
                    CLMM_CREATE_MINT0,
                    CLMM_CREATE_MINT1,
                    GraduationSource::RaydiumClmm,
                    slot,
                ) {
                    let pool = entry.pool;
                    if self.pending_clmm.len() >= MAX_PENDING_CLMM {
                        warn!(
                            "GraduationDetector: pending_clmm at cap {MAX_PENDING_CLMM}, \
                             dropping create for CLMM pool {pool}"
                        );
                    } else {
                        self.pending_clmm.insert(pool, entry.pending);
                    }
                }
                return None;
            }
            if d == Some(DISC_CLMM_INCREASE_LIQ_V2) {
                return self.promote_clmm(
                    slot,
                    accounts,
                    CLMM_INCR_LIQ_POOL_STATE,
                );
            }
            return None;
        }

        // ── Raydium CPMM ─────────────────────────────────────────────────────
        // The initialize instruction seeds both token vaults in the same
        // transaction.  The pool is immediately tradeable after this single
        // instruction completes successfully.
        if pid == raydium_cp_program_id() {
            if disc8(data) == Some(DISC_CPMM_INIT) {
                return make_detected(
                    slot,
                    accounts,
                    CPMM_INIT_POOL_STATE,
                    CPMM_INIT_MINT0,
                    CPMM_INIT_MINT1,
                    GraduationSource::RaydiumCpmm,
                );
            }
            return None;
        }

        // ── Raydium AMM V4 ───────────────────────────────────────────────────
        // Legacy single-byte opcode.  Only matched after confirming program_id
        // because 0x01 as a first byte is not unique across all Solana programs.
        if pid == raydium_program_id() {
            if data.first() == Some(&DISC_AMM_V4_INIT2) {
                return make_detected(
                    slot,
                    accounts,
                    AMM_V4_INIT_POOL,
                    AMM_V4_INIT_COIN_MINT,
                    AMM_V4_INIT_PC_MINT,
                    GraduationSource::RaydiumAmmV4,
                );
            }
            return None;
        }

        // ── Meteora DAMM V2 ───────────────────────────────────────────────────
        // All three initialize variants seed liquidity atomically.  The pool
        // is immediately tradeable after any of the three succeeds.  The three
        // variants differ only in how fee parameters are specified — from the
        // graduation detector's perspective they are equivalent.
        if pid == damm_v2_program_id() {
            let d = disc8(data);
            if d == Some(DISC_METEORA_DAMM_INIT_POOL) {
                return make_detected(
                    slot,
                    accounts,
                    METEORA_DAMM_INIT_POOL,
                    METEORA_DAMM_INIT_MINT_A,
                    METEORA_DAMM_INIT_MINT_B,
                    GraduationSource::MeteoraDammV2,
                );
            }
            if d == Some(DISC_METEORA_DAMM_INIT_CUSTOM) {
                return make_detected(
                    slot,
                    accounts,
                    METEORA_DAMM_CUSTOM_POOL,
                    METEORA_DAMM_CUSTOM_MINT_A,
                    METEORA_DAMM_CUSTOM_MINT_B,
                    GraduationSource::MeteoraDammV2,
                );
            }
            if d == Some(DISC_METEORA_DAMM_INIT_DYNAMIC) {
                return make_detected(
                    slot,
                    accounts,
                    METEORA_DAMM_DYNAMIC_POOL,
                    METEORA_DAMM_DYNAMIC_MINT_A,
                    METEORA_DAMM_DYNAMIC_MINT_B,
                    GraduationSource::MeteoraDammV2,
                );
            }
            return None;
        }

        // ── Meteora DLMM ─────────────────────────────────────────────────────
        // Two-step: all four initialization variants create an lb_pair with
        // empty bins.  Reserves are zero until a liquidity provider deposits
        // into at least one bin range via add_liquidity or add_liquidity2.
        // All four init variants share the same lb_pair, mint_x, and mint_y
        // positions, so a single promote path handles all of them.
        if pid == dlmm_program_id() {
            let d = disc8(data);
            if matches!(
                d,
                Some(DISC_DLMM_INIT_LB_PAIR)
                    | Some(DISC_DLMM_INIT_LB_PAIR2)
                    | Some(DISC_DLMM_INIT_CUSTOM)
                    | Some(DISC_DLMM_INIT_CUSTOM2)
            ) {
                if let Some(entry) = extract_concentrated(
                    accounts,
                    DLMM_INIT_LB_PAIR,
                    DLMM_INIT_MINT_X,
                    DLMM_INIT_MINT_Y,
                    GraduationSource::MeteoraDlmm,
                    slot,
                ) {
                    let pool = entry.pool;
                    if self.pending_dlmm.len() >= MAX_PENDING_DLMM {
                        warn!(
                            "GraduationDetector: pending_dlmm at cap {MAX_PENDING_DLMM}, \
                             dropping create for DLMM lb_pair {pool}"
                        );
                    } else {
                        self.pending_dlmm.insert(pool, entry.pending);
                    }
                }
                return None;
            }
            if matches!(d, Some(DISC_DLMM_ADD_LIQUIDITY) | Some(DISC_DLMM_ADD_LIQUIDITY2)) {
                return self.promote_dlmm(slot, accounts, DLMM_ADD_LIQ_LB_PAIR);
            }
            return None;
        }

        // ── Orca Whirlpool ────────────────────────────────────────────────────
        // Two-step: both initialize_pool variants allocate the whirlpool account
        // but leave all tick arrays empty.  increase_liquidity or
        // increase_liquidity_v2 is the gate — at least one position has been
        // opened and funded, making that tick range tradeable.
        //
        // V1 and V2 differ only in the whirlpool address position (4 vs 6).
        // Mint positions are identical in both versions.
        if pid == whirlpool_program_id() {
            let d = disc8(data);
            if d == Some(DISC_WHIRLPOOL_INIT_POOL_V1) {
                if let Some(entry) = extract_concentrated(
                    accounts,
                    WHIRLPOOL_V1_POOL,
                    WHIRLPOOL_MINT_A,
                    WHIRLPOOL_MINT_B,
                    GraduationSource::OrcaWhirlpool,
                    slot,
                ) {
                    let pool = entry.pool;
                    if self.pending_whirlpool.len() >= MAX_PENDING_WHIRLPOOL {
                        warn!(
                            "GraduationDetector: pending_whirlpool at cap \
                             {MAX_PENDING_WHIRLPOOL}, dropping create for \
                             Whirlpool v1 {pool}"
                        );
                    } else {
                        self.pending_whirlpool.insert(pool, entry.pending);
                    }
                }
                return None;
            }
            if d == Some(DISC_WHIRLPOOL_INIT_POOL_V2) {
                if let Some(entry) = extract_concentrated(
                    accounts,
                    WHIRLPOOL_V2_POOL,
                    WHIRLPOOL_MINT_A,
                    WHIRLPOOL_MINT_B,
                    GraduationSource::OrcaWhirlpool,
                    slot,
                ) {
                    let pool = entry.pool;
                    if self.pending_whirlpool.len() >= MAX_PENDING_WHIRLPOOL {
                        warn!(
                            "GraduationDetector: pending_whirlpool at cap \
                             {MAX_PENDING_WHIRLPOOL}, dropping create for \
                             Whirlpool v2 {pool}"
                        );
                    } else {
                        self.pending_whirlpool.insert(pool, entry.pending);
                    }
                }
                return None;
            }
            if matches!(
                d,
                Some(DISC_WHIRLPOOL_INCREASE_LIQ) | Some(DISC_WHIRLPOOL_INCREASE_LIQ_V2)
            ) {
                return self.promote_whirlpool(
                    slot,
                    accounts,
                    WHIRLPOOL_INCREASE_LIQ_POOL,
                );
            }
        }

        None
    }

    // ── Per-DEX promotion helpers ─────────────────────────────────────────────
    //
    // Each method moves a pool from its specific pending map to a DetectedPool
    // when the DEX's liquidity gate instruction fires.  If the pool address is
    // not in the map, this is a deposit on an established pool — return None
    // silently and let it travel the normal SpeculativeAccountUpdate path.
    //
    // A single generic `promote_from(&mut self, map: &mut HashMap<...>)` would
    // require simultaneously holding `&mut self` (method receiver) and
    // `&mut self.pending_xxx` (the map argument).  These are overlapping mutable
    // borrows of the same allocation — a compile error in Rust regardless of
    // two-phase borrow support.  Per-field methods avoid the overlap entirely:
    // each method names its own field directly so the borrow checker sees only
    // one mutable borrow of `self` at a time.

    fn promote_clmm(&mut self, slot: Slot, accounts: &[Pubkey], pool_idx: usize) -> Option<DetectedPool> {
        let pool = get(accounts, pool_idx)?;
        let entry = self.pending_clmm.remove(&pool)?;
        Some(DetectedPool { pool_address: pool, mint0: entry.mint0, mint1: entry.mint1, source: entry.source, slot })
    }

    fn promote_dlmm(&mut self, slot: Slot, accounts: &[Pubkey], pool_idx: usize) -> Option<DetectedPool> {
        let pool = get(accounts, pool_idx)?;
        let entry = self.pending_dlmm.remove(&pool)?;
        Some(DetectedPool { pool_address: pool, mint0: entry.mint0, mint1: entry.mint1, source: entry.source, slot })
    }

    fn promote_whirlpool(&mut self, slot: Slot, accounts: &[Pubkey], pool_idx: usize) -> Option<DetectedPool> {
        let pool = get(accounts, pool_idx)?;
        let entry = self.pending_whirlpool.remove(&pool)?;
        Some(DetectedPool { pool_address: pool, mint0: entry.mint0, mint1: entry.mint1, source: entry.source, slot })
    }

    /// Sweep every pending map entry whose creation was seen in `dead_slot`.
    ///
    /// The canonical replay pipeline calls `mark_dead_slot` when it permanently
    /// rejects a slot due to invalid PoH, bad Ed25519 batch, SVM execution error,
    /// or chained block-ID mismatch.  Any pool-creation transaction that was part
    /// of a dead slot will never land on-chain, so its `PendingConcentratedPool`
    /// entry is garbage.  Without this sweep the three pending maps accumulate
    /// dead entries over time, eventually reaching their per-map caps and silently
    /// dropping detections for genuinely new pools.
    ///
    /// Called by the shredstream bridge task when it receives a `dead_slot` value
    /// from the engine's `mev_dead_slot_sender` channel clone.
    pub fn clear_dead_slot(&mut self, dead_slot: Slot) {
        // `HashMap::retain` iterates the map once and removes matching entries in-place.
        // Each call is O(n) in the map size, which is bounded by the per-DEX cap.
        self.pending_clmm.retain(|_, entry| entry.slot != dead_slot);
        self.pending_whirlpool.retain(|_, entry| entry.slot != dead_slot);
        self.pending_dlmm.retain(|_, entry| entry.slot != dead_slot);
    }
}

// ---------------------------------------------------------------------------
// Module-private helpers
// ---------------------------------------------------------------------------

/// Intermediate result from extract_concentrated, bundling the pool address
/// together with the PendingConcentratedPool that will be inserted into a
/// per-DEX map.
struct ExtractedConcentrated {
    pool: Pubkey,
    pending: PendingConcentratedPool,
}

/// Extract pool address and both mints from the accounts slice and apply the
/// quote-token filter.  Returns None if any index is out of bounds or neither
/// mint is a recognised quote token.
///
/// This function does NOT insert into any map — the caller decides which map
/// to use and whether the cap has been reached.  Separating extraction from
/// insertion keeps the borrow checker happy (no double &mut self borrows).
fn extract_concentrated(
    accounts: &[Pubkey],
    pool_idx: usize,
    mint0_idx: usize,
    mint1_idx: usize,
    source: GraduationSource,
    slot: Slot,
) -> Option<ExtractedConcentrated> {
    let pool = get(accounts, pool_idx)?;
    let mint0 = get(accounts, mint0_idx)?;
    let mint1 = get(accounts, mint1_idx)?;

    if !has_quote_token(mint0, mint1) {
        return None;
    }

    Some(ExtractedConcentrated {
        pool,
        pending: PendingConcentratedPool { mint0, mint1, source, slot },
    })
}

/// Build a DetectedPool directly from an instruction's accounts slice.
///
/// Used for single-event DEXes (PumpSwap, CPMM, AMM V4, DAMM V2) where pool
/// creation and liquidity seeding are atomic.  No pending map is involved.
///
/// Returns None if any account index is out of bounds or neither mint is a
/// recognised quote token.
fn make_detected(
    slot: Slot,
    accounts: &[Pubkey],
    pool_idx: usize,
    mint0_idx: usize,
    mint1_idx: usize,
    source: GraduationSource,
) -> Option<DetectedPool> {
    let pool_address = get(accounts, pool_idx)?;
    let mint0 = get(accounts, mint0_idx)?;
    let mint1 = get(accounts, mint1_idx)?;

    if !has_quote_token(mint0, mint1) {
        return None;
    }

    Some(DetectedPool { pool_address, mint0, mint1, source, slot })
}

/// Extract the first 8 bytes of `data` as a fixed-size discriminator array.
///
/// Anchor prefixes every instruction with 8 bytes computed as
/// sha256("global:<instruction_name>")[..8].  Instructions shorter than 8
/// bytes cannot be valid Anchor calls and are treated as non-matching by
/// returning None.
#[inline(always)]
fn disc8(data: &[u8]) -> Option<[u8; 8]> {
    data.get(..8)?.try_into().ok()
}

/// Safely index into an accounts slice, returning None on out-of-bounds.
///
/// Instructions arrive from the network and are not trusted.  An instruction
/// that claims to be a pool-creation call but carries fewer accounts than the
/// IDL specifies is malformed.  Returning None rather than panicking keeps the
/// validator stable under adversarial or corrupted transactions.
#[inline(always)]
fn get(accounts: &[Pubkey], idx: usize) -> Option<Pubkey> {
    accounts.get(idx).copied()
}

/// Returns true if at least one of the two mints is a recognised quote token.
///
/// The engine constructs arbitrage paths exclusively as two-hop SOL-in/SOL-out
/// cycles.  Each hop crosses one pool.  For a pool to participate in any two-
/// hop cycle, one of its two mints must be reachable from SOL in a single hop.
/// SOL itself qualifies directly.  USDC, USDT, and USD1 qualify because each
/// has deep, liquid SOL pairs on multiple DEXes already in the graph.  A pool
/// where both mints are arbitrary SPL tokens has no bridge to SOL and can never
/// appear in a profitable two-hop path — excluding it here saves all downstream
/// work.
#[inline(always)]
fn has_quote_token(mint0: Pubkey, mint1: Pubkey) -> bool {
    mint0 == SOL_MINT
        || mint1 == SOL_MINT
        || mint0 == USDC_MINT
        || mint1 == USDC_MINT
        || mint0 == USDT_MINT
        || mint1 == USDT_MINT
        || mint0 == USD1_MINT
        || mint1 == USD1_MINT
}

/// Returns true if the given program ID is one of the DEX programs whose
/// pool-creation instructions the graduation detector can recognize.
///
/// The shredstream bridge uses this as a pre-filter before allocating the
/// resolved-accounts Vec that `detect_instruction` consumes.  The vast majority
/// of instructions on mainnet target programs that are not on this list — swap
/// programs like System, Token, ComputeBudget, and every DEX's swap instruction
/// path.  Checking program membership here means the bridge pays the Vec<Pubkey>
/// allocation cost only for instructions that have any chance of being a pool
/// creation event, eliminating millions of pointless allocations per slot.
#[inline(always)]
pub fn is_graduation_program(program_id: &Pubkey) -> bool {
    *program_id == pump_program_id()
        || *program_id == raydium_clmm_program_id()
        || *program_id == raydium_cp_program_id()
        || *program_id == raydium_program_id()
        || *program_id == damm_v2_program_id()
        || *program_id == dlmm_program_id()
        || *program_id == whirlpool_program_id()
}
