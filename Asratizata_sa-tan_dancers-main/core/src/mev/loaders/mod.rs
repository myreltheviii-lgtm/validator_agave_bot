//! Pool loaders: discovery, parsing, scanning, and tracking.
//!
//! # Submodules
//!
//! - `pool_discovery` — `scan_all_mints_no_init`, `initialize_mint_from_discovered`
//! - `pool_parser`    — 14 parse_*_pools functions (each takes `bank: &Arc<Bank>`)
//! - `pool_scanner`   — `discover_all_pools_grouped_by_mint`, `DiscoveredPools`,
//!                       `MintDiscoveryResult`
//! - `pool_tracker`   — `PoolTracker` (Arc<RwLock<HashSet<Pubkey>>> wrapper)
//!
//! # Note on TrackingFilter / pre_populate_tracking_filter
//!
//! The original `loaders/mod.rs` re-exported `pre_populate_tracking_filter` from
//! `pool_scanner`. That function took a `&TrackingFilter` — a Geyser-plugin filter
//! object that told the external plugin which accounts to forward to the standalone
//! bot. In the integrated architecture, the equivalent filtering is done implicitly
//! by `MevEngine::handle_speculative_update` via its `account_to_mint` HashMap.
//! No `TrackingFilter` object is constructed or passed anywhere, so both
//! `pre_populate_tracking_filter` and the `tracking_filter` submodule are removed.

pub mod pool_discovery;
pub mod pool_parser;
pub mod pool_scanner;
pub mod pool_tracker;
pub mod pool_graduation;

pub use pool_discovery::{InitializedMint, initialize_mint_from_discovered, scan_all_mints_no_init};
pub use pool_scanner::{DiscoveredPools, MintDiscoveryResult, discover_all_pools_grouped_by_mint};
pub use pool_tracker::PoolTracker;
