//! One-time MEV startup sequence run inside the validator process.
//!
//! When the arbitrage system was a standalone bot, `main.rs` owned the startup
//! sequence: NUMA thread-pool affinity, wallet loading, RPC client construction,
//! LUT manager initialisation, pool scan, and pool parsing. Now that the system
//! runs inside the Jito/Agave validator process, that sequence cannot live in a
//! standalone `main` function — the validator already has one. Instead, every
//! responsibility from the old `main.rs` has been extracted into the public
//! functions and types of this module.
//!
//! # Call site
//!
//! `validator.rs` calls `initialize_mev_components` once inside `Validator::new`,
//! after `bank_forks` is ready (i.e. after `load_blockstore` returns) but before
//! `MevEngine::new`. The `MevStartupResult` it returns is unpacked and its fields
//! are passed directly to `MevEngine::new` as its non-channel parameters.
//!
//! # Account filtering
//!
//! The old bot pre-populated a `TrackingFilter` for the Geyser plugin so the
//! plugin only forwarded account updates that belonged to known pools. In the
//! integrated architecture, `SpeculativeSlotExecutor.execute()` emits every
//! account that changed during speculative replay, and `MevEngine` performs the
//! equivalent filter implicitly: `handle_speculative_update` looks up each account
//! in its `account_to_mint` reverse index and silently skips any account that is
//! not in the map. No external `TrackingFilter` or `PoolTracker` object needs to
//! be constructed or passed anywhere — the map IS the filter.
//!
//! # Lazy initialisation
//!
//! The old bot had a `lazy_init_worker` tokio task that initialised mints on
//! demand as new pool-state accounts appeared in the Geyser event stream.
//! In the integrated architecture, `MevEngine::register_mint` fills the same role:
//! it is `pub` and can be called from the engine's run loop whenever a previously
//! unknown account appears in a `SpeculativeAccountUpdate`. The initial scan at
//! startup covers every pool that exists on-chain at the moment the validator
//! loads its snapshot, so the lazy path is only hit for mints created after the
//! snapshot slot — a negligible fraction of traffic on mainnet.

use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_runtime::bank::Bank;
use solana_runtime::bank_forks::BankForks;
use solana_pubkey::Pubkey;
use solana_keypair::{read_keypair_file, Keypair};
// `Signer` provides `.pubkey()` on `Arc<Keypair>`. Without this trait in scope
// the compiler cannot resolve the `.pubkey()` method call.
use solana_signer::Signer;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tracing::{info, warn};

use crate::mev::loaders::pool_discovery::{initialize_mint_from_discovered, scan_all_mints_no_init};
use crate::mev::lut_manager::LutManager;
use crate::mev::pools::MintPoolData;

// ---------------------------------------------------------------------------
// Config and result types
// ---------------------------------------------------------------------------

/// MEV-specific configuration read by `initialize_mev_components`.
///
/// Every field maps directly to a `--mev-*` CLI flag in `args.rs` and a
/// matching field on `ValidatorConfig` in `validator.rs`.  Keeping these
/// values in a dedicated struct rather than scattering them as individual
/// parameters makes the call site compact and keeps the boundary between
/// MEV startup and validator startup explicit.
pub struct MevStartupConfig {
    /// Path to the keypair file used to sign arbitrage transactions.
    /// Must be pre-funded with enough SOL to cover priority fees and wSOL costs.
    pub wallet_path: PathBuf,

    /// JSON-RPC endpoint used for transaction submission and LUT account fetching.
    /// Should point to a local validator RPC port (zero latency, no rate limits)
    /// or a high-availability cluster RPC for production use.
    pub rpc_url: String,

    /// On-chain addresses of address lookup tables to load at startup.
    ///
    /// Each pubkey here identifies a deployed `AddressLookupTable` account whose
    /// `addresses` slice is fetched from the cluster during startup and cached for
    /// the process lifetime.  Passing an empty vector is valid — the engine will
    /// still operate, but every transaction will use the static account list only,
    /// which limits complex multi-pool arbitrage paths.  See the `mev_lut_addresses`
    /// field on `ValidatorConfig` for the full description of why LUTs are needed.
    pub lut_addresses: Vec<Pubkey>,

    /// gRPC endpoint of the Jito shredstream proxy.
    ///
    /// The shredstream bridge task connects to this address on startup and
    /// reconnects automatically with a 2-second back-off if the connection drops.
    /// For validators that co-locate the shredstream proxy on the same machine,
    /// the default value `http://127.0.0.1:8100` eliminates network round-trips
    /// entirely.  Remote proxies are supported for setups where the proxy runs
    /// on a dedicated machine.
    pub shredstream_url: String,
}

/// Everything `MevEngine::new` needs beyond the channel parameters that
/// `validator.rs` already holds at the time it constructs the engine
/// (the channel receivers, `bank_forks`, `speculative_executor`, and the
/// dead-slot channel).
pub struct MevStartupResult {
    /// Keypair used to sign arbitrage transactions.
    pub wallet: Arc<Keypair>,

    /// RPC client for transaction submission and LUT refresh.
    pub rpc_client: Arc<RpcClient>,

    /// Address lookup table manager.  Populated with up to ten LUTs whose
    /// on-chain pubkeys were supplied via `--mev-lut-address` CLI flags.
    /// Each LUT encodes the pool vault, tick-array, and oracle addresses that
    /// would otherwise consume v0 transaction account slots.
    pub lut_manager: Arc<LutManager>,

    /// One `MintPoolData` per tracked mint, fully parsed from the bank at
    /// startup.  Each struct holds vault, tick-array, and oracle pubkeys for
    /// every DEX pool that quotes this mint against SOL, USDC, USDT, or USD1.
    /// The instruction builder reads these at trade time with no further bank
    /// or RPC lookups — they are resolved once here and cached for the process
    /// lifetime.
    pub mint_pool_data: Vec<Arc<MintPoolData>>,
}

// ---------------------------------------------------------------------------
// NUMA topology helper
// ---------------------------------------------------------------------------

/// Read `/proc/self/numa_maps` and return the NUMA node that backs the
/// process's shared-memory segments (where AccountsDb mmaps its files).
///
/// NUMA awareness matters for MEV because the pool scan and account reads in
/// `initialize_mint_from_discovered` walk through AccountsDb memory.  If that
/// memory is on a remote NUMA node, every random-access read pays a 2–4× latency
/// penalty.  This function detects and logs the node so operators can tune OS
/// huge-page allocation and `numactl --membind` settings.
///
/// Returns 0 on any parse or I/O error — safe fallback on non-Linux kernels
/// and inside Docker containers that do not expose this procfs path.
fn detect_numa_node_from_maps() -> usize {
    let content = match std::fs::read_to_string("/proc/self/numa_maps") {
        Ok(c) => c,
        Err(_) => return 0,
    };
    for line in content.lines() {
        if line.contains("shmem") || line.contains("shm") {
            for part in line.split_whitespace() {
                if let Some(rest) = part.strip_prefix('N') {
                    if let Some(eq) = rest.find('=') {
                        if let Ok(node) = rest[..eq].parse::<usize>() {
                            return node;
                        }
                    }
                }
            }
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Pool count helper (logging only)
// ---------------------------------------------------------------------------

fn count_total_pools(pool_data: &MintPoolData) -> usize {
    pool_data.raydium_pools.len()
        + pool_data.raydium_cp_pools.len()
        + pool_data.raydium_clmm_pools.len()
        + pool_data.pump_pools.len()
        + pool_data.meteora_damm_pools.len()
        + pool_data.meteora_damm_v2_pools.len()
        + pool_data.dlmm_pairs.len()
        + pool_data.whirlpool_pools.len()
        + pool_data.byreal_pools.len()
        + pool_data.pancakeswap_pools.len()
        + pool_data.humidifi_pools.len()
        + pool_data.vertigo_pools.len()
        + pool_data.heaven_pools.len()
        + pool_data.futarchy_pools.len()
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// Run the one-time MEV startup sequence and return everything `MevEngine::new`
/// needs.  Called from `Validator::new` in `validator.rs` after `load_blockstore`
/// has returned and `bank_forks` is populated, but before `MevEngine::new`.
///
/// # Steps
///
/// 1. Detect and log the NUMA node where AccountsDb storage is resident.
/// 2. Load the wallet keypair from `config.wallet_path`.
/// 3. Construct an `RpcClient` pointing at `config.rpc_url`.
/// 4. Load LUT addresses from `config.lut_addresses` and build a `LutManager`.
///    If the vector is empty, the manager is initialised without tables —
///    transactions still succeed but may exceed size limits for pools whose
///    accounts do not fit in the base account list without LUT compression.
/// 5. Obtain the working bank from `bank_forks` and run a read-only scan of
///    all 14 known DEX programs to enumerate every pool at the snapshot slot.
///    The scan does not parse pool state — it only collects pubkeys.
/// 6. Parse every discovered pool's vault, tick-array, and oracle accounts from
///    the bank, producing one `MintPoolData` per mint.  Mints whose parse fails
///    are logged and skipped — a single malformed pool account cannot prevent
///    the engine from tracking the rest.
///
/// # Errors
///
/// Returns an error if the wallet cannot be loaded, the bank RwLock is
/// poisoned, the pool scan fails, or zero mints are successfully parsed.
/// Zero mints almost always means the validator loaded a devnet or
/// misconfigured snapshot rather than a mainnet one; aborting early is
/// preferable to silently running an engine that never fires.
pub fn initialize_mev_components(
    bank_forks: &Arc<RwLock<BankForks>>,
    config: &MevStartupConfig,
) -> Result<MevStartupResult> {
    // Step 1 — NUMA logging
    let numa_node = detect_numa_node_from_maps();
    info!("MEV startup: AccountsDb NUMA node = {}", numa_node);

    // Step 2 — wallet
    let wallet = Arc::new(
        // read_keypair_file returns Result<Keypair, Box<dyn std::error::Error>>.
        // Box<dyn Error> does not implement Send + Sync, so anyhow::Context cannot
        // be applied to it directly.  map_err converts the boxed error into an
        // anyhow::Error by formatting it, satisfying the Send + Sync bound.
        read_keypair_file(&config.wallet_path)
            .map_err(|e| anyhow::anyhow!(
                "MEV startup: failed to read wallet keypair from {:?}: {:?}",
                config.wallet_path, e
            ))?,
    );
    info!("MEV startup: wallet pubkey = {}", wallet.pubkey());

    // Step 3 — RPC client
    let rpc_client = Arc::new(RpcClient::new(config.rpc_url.clone()));
    info!("MEV startup: RPC endpoint = {}", config.rpc_url);

    // Step 4 — LUT manager
    //
    // LUT addresses are provided through `MevStartupConfig` rather than
    // environment variables.  This keeps the entire MEV configuration visible
    // in a single place — the validator's CLI arguments — and eliminates a
    // hidden runtime dependency on environment variable state that could go
    // unnoticed during deployment.
    let lut_manager = if !config.lut_addresses.is_empty() {
        info!(
            "MEV startup: loading {} LUT(s) from CLI configuration",
            config.lut_addresses.len()
        );
        Arc::new(
            LutManager::new_with_luts(config.lut_addresses.clone(), &Arc::new(RpcClient::new("https://mainnet.helius-rpc.com/?api-key=db75ab85-690e-483d-b351-dc1bd0a2e9b3".to_string())))
                .context("MEV startup: failed to load address lookup tables")?,
        )
    } else {
        info!(
            "MEV startup: no --mev-lut-address arguments supplied; \
             starting without lookup table compression"
        );
        Arc::new(LutManager::new(wallet.pubkey()))
    };
    let (lut_count, covered) = lut_manager.get_coverage_stats();
    info!(
        "MEV startup: LUT manager ready — {} table(s), {} covered addresses",
        lut_count, covered
    );

    // Step 5 — obtain the working bank for the scan.
    //
    // `working_bank()` returns the unfrozen bank at the highest slot, which
    // has the most recent account state visible to the validator.  Holding the
    // read guard only long enough to clone the Arc avoids blocking any
    // concurrent BankForks writes during the (potentially slow) scan below.
    let bank: Arc<Bank> = {
        let forks = bank_forks
            .read()
            .map_err(|_| anyhow::anyhow!("MEV startup: BankForks RwLock is poisoned"))?;
        forks.working_bank()
    };
    info!(
        "MEV startup: scanning DEX program accounts in bank at slot {}",
        bank.slot()
    );

    // Step 5 cont. — read-only pool discovery
    //
    // `scan_all_mints_no_init` calls `bank.get_program_accounts` for each of
    // the 14 known DEX programs.  On mainnet with ~500 K pool accounts this
    // scan takes several seconds and runs synchronously inside `Validator::new`.
    // This is a known trade-off: scanning at startup gives us a complete
    // picture of all existing pools before the engine begins processing
    // shredstream entries, eliminating the need for a cold-start discovery
    // window.  Pools created after this snapshot are discovered incrementally
    // by the background task in `MevEngine`.
    let discovery = scan_all_mints_no_init(&bank)
        .context("MEV startup: pool scan failed")?;
    info!(
        "MEV startup: scan complete — {} unique mints, {} total pools",
        discovery.total_unique_mints, discovery.total_pools
    );

    // Step 6 — parse pool state into MintPoolData
    //
    // Each call reads vault, tick-array, oracle, and sub-account pubkeys from
    // the bank for every pool belonging to one mint.  Results are stored in
    // MintPoolData so the instruction builder can resolve them in O(1) on the
    // hot path without any further bank or RPC access.
    //
    // The map is consumed with `into_iter()` rather than iterated by reference.
    // `initialize_mint_from_discovered` takes `DiscoveredPools` by value; iterating
    // by reference (`&discovery.pools_by_mint`) would force a `.clone()` of each
    // `DiscoveredPools` value on every iteration — one allocation of 14 Vec<Pubkey>
    // fields per mint.  Moving out of the map with `into_iter()` is zero-cost.
    let wallet_pubkey = wallet.pubkey();
    // Capture the total mint count before consuming the map with into_iter().
    // After the loop, discovery.pools_by_mint no longer exists so its .len()
    // would be a compile error.  This single usize read is free.
    let total_mints_discovered = discovery.pools_by_mint.len();
    let mut mint_pool_data: Vec<Arc<MintPoolData>> =
        Vec::with_capacity(total_mints_discovered);

    for (mint, discovered_pools) in discovery.pools_by_mint {
        match initialize_mint_from_discovered(
            &mint,
            discovered_pools,
            &wallet_pubkey,
            &bank,
        ) {
            Ok(init) => {
                let n = count_total_pools(&init.pool_data);
                info!("MEV startup: parsed mint {} — {} pool(s)", mint, n);
                mint_pool_data.push(Arc::new(init.pool_data));
            }
            Err(e) => {
                // A single malformed account must not abort the whole startup.
                // Log and continue so the remaining mints remain available.
                warn!("MEV startup: skipping mint {} — parse error: {}", mint, e);
            }
        }
    }

    info!(
        "MEV startup: {} mint(s) fully initialised ({} skipped)",
        mint_pool_data.len(),
        total_mints_discovered.saturating_sub(mint_pool_data.len()),
    );

    // Zero parsed mints means no DEX pool accounts were found in the bank.
    // On mainnet there are always hundreds of thousands of pools; zero mints
    // almost certainly indicates the validator loaded a devnet or localnet
    // snapshot, or that the RPC and wallet flags point to the wrong cluster.
    // Continuing would create an engine that never fires and gives no signal
    // that anything is wrong — a hard abort here surfaces the misconfiguration
    // immediately with a clear diagnostic.
    if mint_pool_data.is_empty() {
        return Err(anyhow::anyhow!(
            "MEV startup: zero mints initialised after scanning {} pool(s). \
             Verify that the validator loaded a mainnet snapshot, that \
             --mev-wallet-path points to a valid keypair, and that \
             --mev-rpc-url is reachable.",
            discovery.total_pools,
        ));
    }

    Ok(MevStartupResult {
        wallet,
        rpc_client,
        lut_manager,
        mint_pool_data,
    })
}
