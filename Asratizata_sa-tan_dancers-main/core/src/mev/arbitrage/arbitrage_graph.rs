use rustc_hash::FxHashMap;
use solana_pubkey::Pubkey;
use tracing::info;

// Quote-token constants are centralised in crate::mev::constants and imported here
// instead of being redeclared locally. Duplicate declarations create a drift hazard:
// if the authoritative constant in constants.rs is ever corrected (as USD1 was), a
// local shadowing definition would silently persist with the stale value.
use crate::mev::constants::{SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT};


// ---------------------------------------------------------------------------
// MevPoolUpdateEvent — defined here so both engine.rs and arbitrage_executor.rs
// can import it from crate::mev::arbitrage without a circular dependency.
// (executor imports ArbitrageGraph/ArbitragePath from here; this struct must
// therefore live here, not in executor, to avoid engine → executor → arbitrage
// → executor cycles.)
// ---------------------------------------------------------------------------

/// Signals that a pool account was modified by speculative or canonical execution
/// and that affected arbitrage pairs should be re-evaluated.
#[derive(Clone)]
pub struct MevPoolUpdateEvent {
    /// The pool state address (vault, tick array, etc.) that changed.
    pub pool_address: Pubkey,

    /// The speculative bank whose write cache already holds the post-execution
    /// state of all accounts touched by this shredstream batch. Simulating
    /// against this bank gives the forward-looking pool view ~200 ms before
    /// canonical confirmation. `None` for canonical-source events.
    pub speculative_bank: Option<std::sync::Arc<solana_runtime::bank::Bank>>,

    /// Blockhash from the speculative bank (or canonical bank if no speculative
    /// bank is available) used when building the arbitrage transaction.
    pub blockhash: solana_hash::Hash,

    /// Whether this update originated from speculative shredstream execution
    /// (`true`) or from a canonical correction (`false`). Logged for latency
    /// tracking.
    pub from_speculative_execution: bool,

    /// Whether this event is a canonical rebase correction (`true`) or an
    /// incremental batch delta (`false`).
    ///
    /// A speculative pipeline for a given slot produces two kinds of events
    /// through the same broadcast channel:
    ///
    ///   · `false` — an incremental delta from one shredstream batch delivery.
    ///     The executor accumulates these as the slot progresses. Each event
    ///     reports only what changed during that specific delivery.
    ///
    ///   · `true` — a rebase correction produced by `confirm_slot` after the
    ///     parent slot was canonically verified and all stored child-slot batches
    ///     were re-executed against the verified parent. The accounts map in the
    ///     originating `SpeculativeAccountUpdate` holds the TOTAL accumulated
    ///     effect of every batch for this child slot, measured from the canonical
    ///     parent. The executor must REPLACE all prior cached state it holds for
    ///     this slot with exactly the state produced by this correction — any
    ///     accumulated incremental deltas from earlier in the same slot are now
    ///     superseded and must be discarded.
    ///
    /// Treating a correction as another incremental accumulation is a silent
    /// state corruption: the post-rebase result would be added on top of the
    /// prior speculative state rather than replacing it, producing a pool view
    /// that no transaction ever actually created.
    pub is_correction: bool,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PoolType {
    RaydiumV4,
    RaydiumCpmm,
    RaydiumClmm,
    PumpSwap,
    MeteoraDamm,
    MeteoraDammV2,
    MeteoraDlmm,
    OrcaWhirlpool,
    Byreal,
    PancakeSwap,
    Humidifi,
    Vertigo,
    Heaven,
    Futarchy,
}

#[derive(Clone, Debug)]
pub struct PoolInfo {
    pub address: Pubkey,
    pub pool_type: PoolType,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
}

#[derive(Clone, Debug)]
pub struct PoolUpdateEvent {
    pub pool_address: Pubkey,
    pub speculative_bank: std::sync::Arc<solana_runtime::bank::Bank>,
    pub slot: solana_clock::Slot,
}

impl PoolUpdateEvent {
    pub fn new(
        pool_address: Pubkey,
        speculative_bank: std::sync::Arc<solana_runtime::bank::Bank>,
        slot: solana_clock::Slot,
    ) -> Self {
        Self {
            pool_address,
            speculative_bank,
            slot,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ArbitragePath {
    TwoHop {
        pool_1: PoolInfo,
        pool_2: PoolInfo,
        intermediate_token: Pubkey,
    },
}

impl ArbitragePath {
    pub fn start_token(&self, base_mint: &Pubkey) -> Pubkey {
        *base_mint
    }

    /// Returns both pools in the two-hop path as a fixed-size array.
    ///
    /// `ArbitragePath` is always `TwoHop` — there are no other variants in this
    /// codebase. Returning `[&PoolInfo; 2]` instead of `Vec<&PoolInfo>` eliminates
    /// a heap allocation on every call. The two callers (`SmbInstructionBuilder` and
    /// `TokenFlowValidator`) use only `.len()` and index `[0]` — both operations are
    /// identical on a fixed-size array and on a Vec, so this change is backward-
    /// compatible at the call sites.
    pub fn pools(&self) -> [&PoolInfo; 2] {
        match self {
            Self::TwoHop { pool_1, pool_2, .. } => [pool_1, pool_2],
        }
    }

    /// Returns the on-chain addresses of both pools.
    ///
    /// Implemented by matching directly rather than calling `self.pools()` so only
    /// one Vec is allocated. If this method called `pools()` it would allocate a
    /// `[&PoolInfo; 2]` stack value, then iterate it to produce a `Vec<Pubkey>` —
    /// effectively paying for two collection operations where one suffices.
    pub fn pool_addresses(&self) -> Vec<Pubkey> {
        match self {
            Self::TwoHop { pool_1, pool_2, .. } => vec![pool_1.address, pool_2.address],
        }
    }

    pub fn intermediate_tokens(&self) -> Vec<Pubkey> {
        match self {
            Self::TwoHop { intermediate_token, .. } => vec![*intermediate_token],
        }
    }

    pub fn hop_count(&self) -> usize {
        match self {
            Self::TwoHop { .. } => 2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PoolPair {
    pub pool_1: PoolInfo,
    pub pool_2: PoolInfo,
    pub shared_token_a: Pubkey,
    pub shared_token_b: Pubkey,
}

impl PoolPair {
    pub fn to_path(&self) -> ArbitragePath {
        // Select the non-quote token as the intermediate. A valid two-hop arbitrage path
        // starts and ends at the same quote currency (SOL, USDC, USDT, or USD1) and passes
        // through exactly one speculative token in the middle. If shared_token_a is one of
        // the four recognised quote currencies, shared_token_b must be the speculative token,
        // and vice versa. Both tokens in a pair are guaranteed to differ, so exactly one of
        // the two will be non-quote.
        let intermediate_token = if self.shared_token_a != SOL_MINT
            && self.shared_token_a != USDC_MINT
            && self.shared_token_a != USDT_MINT
            && self.shared_token_a != USD1_MINT
        {
            self.shared_token_a
        } else {
            self.shared_token_b
        };

        ArbitragePath::TwoHop {
            pool_1: self.pool_1.clone(),
            pool_2: self.pool_2.clone(),
            intermediate_token,
        }
    }
}

pub struct ArbitrageGraphConfig {
    pub allow_same_dex_pairs: bool,
}

impl Default for ArbitrageGraphConfig {
    fn default() -> Self {
        Self {
            allow_same_dex_pairs: true,
        }
    }
}

pub struct ArbitrageGraph {
    pool_to_pairs: FxHashMap<Pubkey, Vec<usize>>,
    pairs: Vec<PoolPair>,
    account_to_pool: FxHashMap<Pubkey, Pubkey>,
    // The complete list of every PoolInfo ever added to the graph, preserved
    // so that add_pool can pair a new pool against all existing pools by
    // iterating this vec. Without it we would need to reconstruct PoolInfos
    // from the pairs vec, which has O(pairs) cost and would require de-duplicating
    // pools that appear in multiple pairs.
    all_pools: Vec<PoolInfo>,
    config: ArbitrageGraphConfig,
}

// Returns true when the token is one of the four recognised quote currencies.
// Quote currencies are the denominators in which arbitrage profit is measured.
// Any token that is NOT a quote currency is a speculative (intermediate) token.
fn is_quote_token(token: &Pubkey) -> bool {
    *token == SOL_MINT
        || *token == USDC_MINT
        || *token == USDT_MINT
        || *token == USD1_MINT
}

// Returns true if at least one side of the pool is a quote currency. Only pools
// that have a quote side can participate in two-hop arbitrage because the executor
// needs to borrow the quote currency, trade into the intermediate token, and trade
// back — which requires one leg of each pool to be the same quote currency.
fn pool_has_quote(pool: &PoolInfo) -> bool {
    is_quote_token(&pool.token_x) || is_quote_token(&pool.token_y)
}

// Returns true if two pools trade the same pair of tokens in any orientation.
// Pool A trades X/Y and pool B trades X/Y or Y/X — both constitute a matching pair.
fn tokens_match(pool_a: &PoolInfo, pool_b: &PoolInfo) -> bool {
    (pool_a.token_x == pool_b.token_x && pool_a.token_y == pool_b.token_y)
        || (pool_a.token_x == pool_b.token_y && pool_a.token_y == pool_b.token_x)
}

// Returns the two tokens (shared_token_a, shared_token_b) for the pair, where
// shared_token_a is the token that appears in both pools at the same or swapped
// position. The caller uses these to determine which token is the quote currency
// (base) and which is the intermediate (speculative) token.
fn get_shared_tokens(pool_a: &PoolInfo, pool_b: &PoolInfo) -> (Pubkey, Pubkey) {
    if pool_a.token_x == pool_b.token_x {
        (pool_a.token_x, pool_a.token_y)
    } else if pool_a.token_x == pool_b.token_y {
        (pool_a.token_x, pool_a.token_y)
    } else if pool_a.token_y == pool_b.token_x {
        (pool_a.token_y, pool_a.token_x)
    } else {
        (pool_a.token_y, pool_a.token_x)
    }
}

// Build the reverse index: every account pubkey that appears in any parsed pool
// struct maps to that pool's canonical address. When SpeculativeSlotExecutor fires
// a SpeculativeAccountUpdate, the engine can perform an O(1) lookup of the vault,
// tick array, or pool account to find the owning pool without iterating all pools.
fn build_account_to_pool_map(pool_data: &crate::mev::pools::MintPoolData) -> FxHashMap<Pubkey, Pubkey> {
    let mut map: FxHashMap<Pubkey, Pubkey> = FxHashMap::default();

    for pool in &pool_data.raydium_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_vault, pool.pool);
        map.insert(pool.sol_vault, pool.pool);
    }
    for pool in &pool_data.raydium_cp_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_vault, pool.pool);
        map.insert(pool.sol_vault, pool.pool);
        map.insert(pool.observation, pool.pool);
    }
    for pool in &pool_data.raydium_clmm_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.x_vault, pool.pool);
        map.insert(pool.y_vault, pool.pool);
        map.insert(pool.bitmap_extension, pool.pool);
        map.insert(pool.observation_state, pool.pool);
        for ta in &pool.tick_arrays { map.insert(*ta, pool.pool); }
    }
    for pool in &pool_data.pump_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_vault, pool.pool);
        map.insert(pool.sol_vault, pool.pool);
    }
    for pool in &pool_data.meteora_damm_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_x_vault, pool.pool);
        map.insert(pool.token_sol_vault, pool.pool);
    }
    for pool in &pool_data.meteora_damm_v2_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_x_vault, pool.pool);
        map.insert(pool.token_sol_vault, pool.pool);
    }
    for pool in &pool_data.dlmm_pairs {
        map.insert(pool.pair, pool.pair);
        map.insert(pool.token_vault, pool.pair);
        map.insert(pool.sol_vault, pool.pair);
        map.insert(pool.oracle, pool.pair);
        for ba in &pool.bin_arrays { map.insert(*ba, pool.pair); }
    }
    for pool in &pool_data.whirlpool_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.x_vault, pool.pool);
        map.insert(pool.y_vault, pool.pool);
        map.insert(pool.oracle, pool.pool);
        for ta in &pool.tick_arrays { map.insert(*ta, pool.pool); }
    }
    for pool in &pool_data.byreal_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.x_vault, pool.pool);
        map.insert(pool.y_vault, pool.pool);
        map.insert(pool.bitmap_extension, pool.pool);
        map.insert(pool.observation_state, pool.pool);
        for ta in &pool.tick_arrays { map.insert(*ta, pool.pool); }
    }
    for pool in &pool_data.pancakeswap_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.x_vault, pool.pool);
        map.insert(pool.y_vault, pool.pool);
        map.insert(pool.bitmap_extension, pool.pool);
        map.insert(pool.observation_state, pool.pool);
        for ta in &pool.tick_arrays { map.insert(*ta, pool.pool); }
    }
    for pool in &pool_data.humidifi_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_x_vault, pool.pool);
        map.insert(pool.token_sol_vault, pool.pool);
    }
    for pool in &pool_data.vertigo_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_x_vault, pool.pool);
        map.insert(pool.token_sol_vault, pool.pool);
    }
    for pool in &pool_data.heaven_pools {
        map.insert(pool.pool, pool.pool);
        map.insert(pool.token_x_vault, pool.pool);
        map.insert(pool.token_base_vault, pool.pool);
    }
    for pool in &pool_data.futarchy_pools {
        // Futarchy identifies its pool by its DAO account, not a pool address.
        map.insert(pool.dao, pool.dao);
        map.insert(pool.token_x_vault, pool.dao);
        map.insert(pool.token_sol_vault, pool.dao);
    }

    map
}

impl ArbitrageGraph {
    pub fn build_with_config(
        pool_data: &crate::mev::pools::MintPoolData,
        config: ArbitrageGraphConfig,
    ) -> Self {
        let mut all_pools: Vec<PoolInfo> = Vec::new();
        let mut pool_to_pairs: FxHashMap<Pubkey, Vec<usize>> = FxHashMap::default();
        let mut pairs: Vec<PoolPair> = Vec::new();

        let account_to_pool = build_account_to_pool_map(pool_data);

        for pool in &pool_data.raydium_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::RaydiumV4,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.raydium_cp_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::RaydiumCpmm,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.raydium_clmm_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::RaydiumClmm,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.pump_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::PumpSwap,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.meteora_damm_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::MeteoraDamm,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.meteora_damm_v2_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::MeteoraDammV2,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.dlmm_pairs {
            all_pools.push(PoolInfo {
                address: pool.pair,
                pool_type: PoolType::MeteoraDlmm,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.whirlpool_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::OrcaWhirlpool,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.byreal_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::Byreal,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.pancakeswap_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::PancakeSwap,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.humidifi_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::Humidifi,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.vertigo_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::Vertigo,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.heaven_pools {
            all_pools.push(PoolInfo {
                address: pool.pool,
                pool_type: PoolType::Heaven,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        for pool in &pool_data.futarchy_pools {
            all_pools.push(PoolInfo {
                address: pool.dao,
                pool_type: PoolType::Futarchy,
                token_x: pool.token_mint,
                token_y: pool.base_mint,
            });
        }

        info!("Total pools collected for graph building: {}", all_pools.len());

        // Group pools by their canonical token-pair key so that only pools
        // containing the same two tokens are compared as potential arb partners.
        // The canonical key sorts the two token pubkeys so that (A, B) and (B, A)
        // produce the same bucket, preventing duplicate pairs.
        let mut token_pair_groups: FxHashMap<(Pubkey, Pubkey), Vec<usize>> = FxHashMap::default();

        for (idx, pool) in all_pools.iter().enumerate() {
            if !pool_has_quote(pool) {
                continue;
            }

            let key = if pool.token_x < pool.token_y {
                (pool.token_x, pool.token_y)
            } else {
                (pool.token_y, pool.token_x)
            };

            token_pair_groups.entry(key).or_default().push(idx);
        }

        for pool_indices in token_pair_groups.values() {
            if pool_indices.len() < 2 {
                continue;
            }

            for i in 0..pool_indices.len() {
                for j in (i + 1)..pool_indices.len() {
                    let pool_a = &all_pools[pool_indices[i]];
                    let pool_b = &all_pools[pool_indices[j]];

                    if !config.allow_same_dex_pairs && pool_a.pool_type == pool_b.pool_type {
                        continue;
                    }

                    if tokens_match(pool_a, pool_b) {
                        let pair_idx = pairs.len();
                        let shared_tokens = get_shared_tokens(pool_a, pool_b);

                        pairs.push(PoolPair {
                            pool_1: pool_a.clone(),
                            pool_2: pool_b.clone(),
                            shared_token_a: shared_tokens.0,
                            shared_token_b: shared_tokens.1,
                        });

                        pool_to_pairs.entry(pool_a.address).or_default().push(pair_idx);
                        pool_to_pairs.entry(pool_b.address).or_default().push(pair_idx);
                    }
                }
            }
        }

        info!(
            "Built arbitrage graph: {} pools total, {} two-hop pairs",
            all_pools.len(),
            pairs.len(),
        );

        Self {
            pool_to_pairs,
            pairs,
            account_to_pool,
            all_pools,
            config,
        }
    }

    /// Add a single newly-created pool to the running graph and immediately
    /// connect it with every existing pool that trades the same token pair.
    ///
    /// This is the graduation fast path.  When the shredstream bridge detects
    /// a pool-creation instruction and the speculative executor confirms the
    /// account exists in the write cache, the engine calls this method to make
    /// the new pool visible to all running ArbitrageExecutor tasks without any
    /// restart or re-registration.
    ///
    /// `new_pool` is the PoolInfo describing the new pool.  `pool_accounts` is
    /// every on-chain pubkey that belongs to this pool — pool state address,
    /// token vaults, tick arrays, oracles, bitmap extensions — anything whose
    /// mutation should trigger re-evaluation of paths through this pool.  Each
    /// account in the slice is inserted into `account_to_pool` mapping to the
    /// new pool's address.
    ///
    /// Pairs are created between the new pool and each existing pool that trades
    /// the same two tokens in any orientation.  The `allow_same_dex_pairs`
    /// config flag is respected: if false, no pair is created between two pools
    /// of the same DEX type.
    ///
    /// Returns the number of new two-hop pairs created.  A return value of zero
    /// means no existing pool trades the same token pair — the new pool is
    /// isolated and no arb paths exist through it yet, though a future pool
    /// registration for the same token pair would pair with it at that time.
    pub fn add_pool(&mut self, new_pool: PoolInfo, pool_accounts: &[Pubkey]) -> usize {
        // A pool with no quote side cannot participate in any two-hop path.
        // Reject it early to avoid polluting all_pools with dead entries that
        // would be matched against — and rejected during — every future add_pool call.
        if !pool_has_quote(&new_pool) {
            return 0;
        }

        // Register every account that belongs to this pool in the reverse index.
        // From this moment, any SpeculativeAccountUpdate that touches one of these
        // accounts will route to this pool's mint via MevEngine::account_to_mint.
        for account in pool_accounts {
            self.account_to_pool.insert(*account, new_pool.address);
        }

        let mut new_pairs_count = 0;

        // Compare the new pool against every existing pool in the graph.
        // tokens_match checks both orientations (A/B and B/A), so the canonical
        // token-pair key sorting used in build_with_config is not needed here.
        for existing_pool in &self.all_pools {
            if !pool_has_quote(existing_pool) {
                continue;
            }

            // Same-DEX pairs are gated by the graph's allow_same_dex_pairs flag.
            // When false, a new Raydium CLMM pool will not pair with an existing
            // Raydium CLMM pool for the same token pair — only cross-DEX pairs fire.
            if !self.config.allow_same_dex_pairs && existing_pool.pool_type == new_pool.pool_type {
                continue;
            }

            if tokens_match(existing_pool, &new_pool) {
                let pair_idx = self.pairs.len();
                let shared_tokens = get_shared_tokens(existing_pool, &new_pool);

                self.pairs.push(PoolPair {
                    pool_1: existing_pool.clone(),
                    pool_2: new_pool.clone(),
                    shared_token_a: shared_tokens.0,
                    shared_token_b: shared_tokens.1,
                });

                // Both the existing pool and the new pool need entries in pool_to_pairs
                // so that either pool's account update routes through get_affected_pairs
                // to the newly created pair index.
                self.pool_to_pairs.entry(existing_pool.address).or_default().push(pair_idx);
                self.pool_to_pairs.entry(new_pool.address).or_default().push(pair_idx);

                new_pairs_count += 1;
            }
        }

        // Append the new pool to all_pools AFTER the comparison loop to avoid
        // pairing it with itself, and so that subsequent add_pool calls for
        // other new pools can find this one as an existing pool to pair against.
        self.all_pools.push(new_pool);

        new_pairs_count
    }

    /// Two-level O(1) lookup: account address → pool address (via account_to_pool),
    /// then pool address → pair indices (via pool_to_pairs). Returns the slice of
    /// pair indices that contain the pool owning the given account. The caller iterates
    /// this slice and re-evaluates each pair.
    pub fn get_affected_pairs(&self, account: &Pubkey) -> &[usize] {
        let pool_addr = match self.account_to_pool.get(account) {
            Some(p) => p,
            None => return &[],
        };
        self.pool_to_pairs
            .get(pool_addr)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn get_pair(&self, idx: usize) -> Option<&PoolPair> {
        self.pairs.get(idx)
    }

    pub fn total_pairs(&self) -> usize {
        self.pairs.len()
    }

    pub fn config(&self) -> &ArbitrageGraphConfig {
        &self.config
    }

    /// Returns every on-chain account pubkey that this graph tracks across all
    /// pools for this mint. The set includes pool state accounts, vaults, tick
    /// arrays, bin arrays, oracle accounts, and bitmap extensions — every address
    /// that, if mutated by a speculative entry batch, could affect an arb pair.
    ///
    /// `MevEngine` stores this list in `MintState::tracked_accounts` so that when
    /// a mint is later de-registered its entries can be bulk-removed from the
    /// global `account_to_mint` reverse index in a single pass.
    pub fn all_tracked_accounts(&self) -> Vec<Pubkey> {
        self.account_to_pool.keys().copied().collect()
    }
}
