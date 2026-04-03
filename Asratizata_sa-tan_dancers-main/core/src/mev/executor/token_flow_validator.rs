use anyhow::{anyhow, Result};
use solana_pubkey::Pubkey;
use crate::mev::arbitrage::{ArbitragePath, PoolInfo};
// All four quote-token mints come from the canonical constant module.
// Importing from crate::mev::constants rather than re-declaring locally means
// that any future address correction propagates to this module automatically —
// the exact drift risk that caused the original USD1 address mismatch between
// this file and smb_instruction_builder.rs before constants.rs was introduced.
use crate::mev::constants::{SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT};

/// One step in the two-hop token flow. Passed to `SmbInstructionBuilder` which
/// reads `pool`, `base_mint`, `input_token`, and `output_token` to determine
/// which account indices to write into the instruction's account list.
#[derive(Clone, Debug)]
pub struct TokenFlowStep {
    /// The pool that executes this swap leg.
    pub pool: PoolInfo,
    /// Token the executor deposits into this pool.
    pub input_token: Pubkey,
    /// Token the executor receives from this pool.
    pub output_token: Pubkey,
    /// The global denomination for the entire two-hop path (SOL or USDC).
    /// All steps share the same `base_mint` regardless of per-pool denomination.
    pub base_mint: Pubkey,
    /// True when the executor is buying the speculative token (base → token).
    /// False when selling (token → base).
    pub is_buy: bool,
}

pub struct TokenFlowValidator;

impl TokenFlowValidator {
    pub fn validate_and_build_flow(path: &ArbitragePath) -> Result<Vec<TokenFlowStep>> {
        let base_mint = Self::determine_base_mint(path)?;
        let flow = Self::build_token_flow(path, &base_mint)?;
        Self::validate_flow(&flow)?;
        Ok(flow)
    }

    /// Determine the arb denomination by inspecting which quote currency the
    /// first pool contains. SOL takes precedence over USDC, which takes
    /// precedence over USDT and USD1, matching the priority order used by
    /// the on-chain executor when selecting the capital source.
    fn determine_base_mint(path: &ArbitragePath) -> Result<Pubkey> {
        let pools = path.pools();
        let first_pool = pools[0];

        if first_pool.token_x == SOL_MINT || first_pool.token_y == SOL_MINT {
            return Ok(SOL_MINT);
        }
        if first_pool.token_x == USDC_MINT || first_pool.token_y == USDC_MINT {
            return Ok(USDC_MINT);
        }
        if first_pool.token_x == USDT_MINT || first_pool.token_y == USDT_MINT {
            return Ok(USDT_MINT);
        }
        if first_pool.token_x == USD1_MINT || first_pool.token_y == USD1_MINT {
            return Ok(USD1_MINT);
        }

        Err(anyhow!(
            "Cannot determine base mint for pool {} with tokens {}/{}",
            first_pool.address,
            first_pool.token_x,
            first_pool.token_y
        ))
    }

    fn build_token_flow(path: &ArbitragePath, base_mint: &Pubkey) -> Result<Vec<TokenFlowStep>> {
        let ArbitragePath::TwoHop { pool_1, pool_2, intermediate_token } = path;
        Self::build_2hop_flow(pool_1, pool_2, intermediate_token, base_mint)
    }

    fn build_2hop_flow(
        pool_1: &PoolInfo,
        pool_2: &PoolInfo,
        intermediate_token: &Pubkey,
        base_mint: &Pubkey,
    ) -> Result<Vec<TokenFlowStep>> {
        let is_buy_1 = pool_1.token_y == *base_mint || pool_1.token_x == *intermediate_token;

        let step_1 = TokenFlowStep {
            pool: pool_1.clone(),
            input_token: *base_mint,
            output_token: *intermediate_token,
            base_mint: *base_mint,
            is_buy: is_buy_1,
        };

        let is_buy_2 = pool_2.token_x == *base_mint || pool_2.token_y == *intermediate_token;

        let step_2 = TokenFlowStep {
            pool: pool_2.clone(),
            input_token: *intermediate_token,
            output_token: *base_mint,
            base_mint: *base_mint,
            is_buy: is_buy_2,
        };

        Ok(vec![step_1, step_2])
    }

    fn validate_flow(flow: &[TokenFlowStep]) -> Result<()> {
        Self::validate_flow_structure(flow)?;
        Self::validate_token_connectivity(flow)?;
        Self::validate_base_token_consistency(flow)?;
        Self::validate_pool_token_membership(flow)?;
        Self::validate_no_stable_to_stable(flow)?;
        Self::validate_no_duplicate_pools(flow)?;
        Self::validate_token_uniqueness(flow)?;
        Self::validate_2hop_requirements(flow)?;
        Ok(())
    }

    fn validate_flow_structure(flow: &[TokenFlowStep]) -> Result<()> {
        if flow.is_empty() {
            return Err(anyhow!("Token flow is empty"));
        }
        if flow.len() != 2 {
            return Err(anyhow!("Token flow must have exactly 2 steps for 2-hop, got {}", flow.len()));
        }
        Ok(())
    }

    fn validate_token_connectivity(flow: &[TokenFlowStep]) -> Result<()> {
        for i in 1..flow.len() {
            if flow[i - 1].output_token != flow[i].input_token {
                return Err(anyhow!(
                    "Token flow broken at step {}: output {} != input {}",
                    i,
                    flow[i - 1].output_token,
                    flow[i].input_token
                ));
            }
        }
        Ok(())
    }

    fn validate_base_token_consistency(flow: &[TokenFlowStep]) -> Result<()> {
        let base_mint = flow[0].base_mint;
        for (i, step) in flow.iter().enumerate() {
            if step.base_mint != base_mint {
                return Err(anyhow!(
                    "Base mint inconsistency at step {}: expected {}, got {}",
                    i, base_mint, step.base_mint
                ));
            }
        }

        if flow[0].input_token != base_mint {
            return Err(anyhow!(
                "First step must start with base token {}, got {}",
                base_mint, flow[0].input_token
            ));
        }

        if flow[flow.len() - 1].output_token != base_mint {
            return Err(anyhow!(
                "Last step must end with base token {}, got {}",
                base_mint, flow[flow.len() - 1].output_token
            ));
        }

        Ok(())
    }

    fn pool_contains_token(pool: &PoolInfo, token: &Pubkey) -> bool {
        pool.token_x == *token || pool.token_y == *token
    }

    fn validate_pool_token_membership(flow: &[TokenFlowStep]) -> Result<()> {
        for (i, step) in flow.iter().enumerate() {
            if !Self::pool_contains_token(&step.pool, &step.input_token) {
                return Err(anyhow!(
                    "Step {}: pool {} does not contain input token {}. Pool has: {}/{}",
                    i, step.pool.address, step.input_token,
                    step.pool.token_x, step.pool.token_y
                ));
            }
            if !Self::pool_contains_token(&step.pool, &step.output_token) {
                return Err(anyhow!(
                    "Step {}: pool {} does not contain output token {}. Pool has: {}/{}",
                    i, step.pool.address, step.output_token,
                    step.pool.token_x, step.pool.token_y
                ));
            }
        }
        Ok(())
    }

    fn is_stable_token(token: &Pubkey) -> bool {
        *token == USDC_MINT || *token == USDT_MINT || *token == USD1_MINT
    }

    fn validate_no_stable_to_stable(flow: &[TokenFlowStep]) -> Result<()> {
        for (i, step) in flow.iter().enumerate() {
            if Self::is_stable_token(&step.input_token) && Self::is_stable_token(&step.output_token) {
                return Err(anyhow!(
                    "Step {}: direct stable-to-stable swap detected: {} -> {}",
                    i, step.input_token, step.output_token
                ));
            }
        }
        Ok(())
    }

    fn validate_no_duplicate_pools(flow: &[TokenFlowStep]) -> Result<()> {
        for i in 0..flow.len() {
            for j in (i + 1)..flow.len() {
                if flow[i].pool.address == flow[j].pool.address {
                    return Err(anyhow!(
                        "Duplicate pool detected: pool {} used in steps {} and {}",
                        flow[i].pool.address, i, j
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_token_uniqueness(flow: &[TokenFlowStep]) -> Result<()> {
        let mut all_tokens: Vec<Pubkey> = Vec::new();
        all_tokens.push(flow[0].input_token);
        for step in flow {
            all_tokens.push(step.output_token);
        }

        for i in 1..(all_tokens.len() - 1) {
            let token = all_tokens[i];
            if token == flow[0].base_mint {
                return Err(anyhow!(
                    "Base token {} appears as intermediate token at position {}",
                    token, i
                ));
            }
            for j in (i + 1)..(all_tokens.len() - 1) {
                if token == all_tokens[j] {
                    return Err(anyhow!(
                        "Intermediate token {} appears multiple times (positions {} and {})",
                        token, i, j
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_2hop_requirements(flow: &[TokenFlowStep]) -> Result<()> {
        if flow.len() != 2 {
            return Ok(());
        }

        let p1x = flow[0].pool.token_x;
        let p1y = flow[0].pool.token_y;
        let p2x = flow[1].pool.token_x;
        let p2y = flow[1].pool.token_y;

        let same_pair = (p1x == p2x && p1y == p2y) || (p1x == p2y && p1y == p2x);

        if !same_pair {
            return Err(anyhow!(
                "2-hop: pools must trade the same token pair. Pool 1: {}/{}, Pool 2: {}/{}",
                p1x, p1y, p2x, p2y
            ));
        }

        Ok(())
    }
}
