//! MEV arbitrage subsystem for Jito-Agave.
//!
//! # Module hierarchy
//!
//! ```text
//! mev/
//!   mod.rs               ← this file
//!   arbitrage/           ← ArbitrageGraph, ArbitrageGraphConfig, ArbitragePath,
//!     mod.rs               PoolPair, PoolInfo, PoolType, MevPoolUpdateEvent
//!     arbitrage_graph.rs
//!   constants.rs         ← SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT
//!   engine.rs            ← MevEngine (event loop, per-mint routing)
//!   executor/            ← ArbitrageExecutor + instruction-building helpers
//!     mod.rs             ← ArbitrageExecutor (= arbitrage_executor.rs content)
//!     smb_instruction_builder.rs
//!     token_flow_validator.rs
//!   loaders/             ← pool discovery, parsing, scanning, tracking
//!     mod.rs             ← re-exports InitializedMint, scan_all_mints_no_init,
//!                           initialize_mint_from_discovered, DiscoveredPools,
//!                           MintDiscoveryResult, discover_all_pools_grouped_by_mint,
//!                           PoolTracker
//!     pool_discovery.rs
//!     pool_parser.rs
//!     pool_scanner.rs
//!     pool_tracker.rs
//!   lut_helpers.rs       ← fetch_address_lookup_table_account helpers
//!   lut_manager.rs       ← LutManager, LutConfig, load_lut_addresses_from_env
//!   pools.rs             ← MintPoolData, RaydiumPool, …, FutarchyPool
//!   asratizata__.rs      ← initialize_mev_components, MevStartupConfig, MevStartupResult
//! ```
//!
//! # Conditional activation
//!
//! The MEV engine is only initialised when `ValidatorConfig::mev_enabled` is `true`.
//! When disabled, `Validator::new` skips the `initialize_mev_components` call and
//! the `"solMevEngine"` thread is not spawned. All validator tests that construct
//! `ValidatorConfig::default_for_test()` are therefore unaffected.

/// Core arbitrage graph, path types, and the MevPoolUpdateEvent broadcast type.
///
/// The `arbitrage` module lives in the `arbitrage/` subdirectory. Rust resolves
/// `pub mod arbitrage` to `arbitrage/mod.rs`, which declares `mod arbitrage_graph`
/// and re-exports the public types from `arbitrage_graph.rs`. The module is exposed
/// as `crate::mev::arbitrage` throughout the codebase so that call sites use a
/// stable, intent-describing name.
pub mod arbitrage;

/// Compile-time quote-token Pubkey constants shared across all mev sub-crates.
pub mod constants;

/// DEX-specific program IDs, account layouts, and instruction helpers.
///
/// The `dex/` directory contains one submodule per supported DEX:
/// byreal, futarchy, heaven, humidifi, meteora, pancakeswap, pump,
/// raydium, vertigo, whirlpool. Every submodule exposes its program ID
/// function and the account struct(s) used by the pool parser and
/// instruction builder.
pub mod dex;

/// MevEngine: event loop that routes canonical bank notifications and graduation
/// events to per-mint ArbitrageExecutors.
pub mod engine;

/// ArbitrageExecutor and instruction-building helpers.
///
/// The executor/ directory contains:
/// - mod.rs (= arbitrage_executor.rs content + submodule declarations)
/// - smb_instruction_builder.rs
/// - token_flow_validator.rs
pub mod executor;

/// Pool discovery, parsing, scanning, and tracking.
pub mod loaders;

/// Address lookup table fetching helpers.
pub mod lut_helpers;

/// LutManager: selects and applies address lookup tables to v0 transactions.
pub mod lut_manager;

/// Per-mint pool data structs (one per DEX × one per mint combination).
pub mod pools;

/// Shredstream graduation bridge task.
///
/// Connects to the Jito shredstream proxy over gRPC, receives entry batches
/// as they are produced by the slot leader, and scans every instruction for
/// pool-creation discriminators.  When a supported DEX creates a tradeable
/// pool, a `DetectedPool` is forwarded into `MevEngine` via the crossbeam
/// graduation channel for Phase 2 processing.  The bridge does not perform
/// speculative execution — all account-state confirmation is handled by
/// `MevEngine::handle_mev_batch` and `MevEngine::handle_frozen_bank` via the
/// canonical blockstore pipeline.  Spawned as a Tokio task inside
/// `MevEngine::run_async()`.
pub mod shredstream_bridge;

/// One-time MEV initialisation sequence: wallet load, RPC client, LUT manager,
/// pool scan, pool parse loop.
///
/// The `#[path]` attribute maps this module to `asratizata__.rs` on disk.
/// Rust identifiers cannot begin with a lowercase letter when the convention
/// calls for an uppercase module name, so the module is declared as
/// `Asratizata__` while the actual filename on disk is `asratizata__.rs`.
#[path = "asratizata__.rs"]
pub mod Asratizata__;

// ---------------------------------------------------------------------------
// Top-level re-exports for convenience
// ---------------------------------------------------------------------------

pub use arbitrage::{
    ArbitrageGraph, ArbitrageGraphConfig, ArbitragePath, MevPoolUpdateEvent,
    PoolInfo, PoolPair, PoolType,
};
pub use engine::MevEngine;
pub use loaders::{
    DiscoveredPools, MintDiscoveryResult, PoolTracker,
    initialize_mint_from_discovered, scan_all_mints_no_init,
};
pub use lut_manager::LutManager;
pub use pools::MintPoolData;
pub use Asratizata__::{initialize_mev_components, MevStartupConfig, MevStartupResult};

