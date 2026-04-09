use anyhow::Result;
use solana_account::ReadableAccount;
use solana_runtime::bank::Bank;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use std::sync::Arc;
use tracing::debug;
use crate::mev::arbitrage::{ArbitragePath, PoolInfo, PoolType};
use crate::mev::pools::MintPoolData;
use crate::mev::dex::pump::constants::{pump_program_id, pump_global_config, pump_authority};
use crate::mev::dex::raydium::constants::{raydium_program_id, raydium_authority, raydium_cp_program_id, raydium_cp_authority, raydium_clmm_program_id};
use crate::mev::dex::raydium::clmm_info::PoolState as RaydiumClmmPoolState;
use crate::mev::dex::raydium::get_tick_array_pubkeys;
use crate::mev::dex::meteora::constants::{dlmm_program_id, dlmm_event_authority, damm_v2_program_id, damm_v2_event_authority, damm_v2_pool_authority, damm_program_id, vault_program_id};
use crate::mev::dex::meteora::dlmm_info::DlmmInfo;
use crate::mev::dex::whirlpool::constants::whirlpool_program_id;
use crate::mev::dex::whirlpool::state::Whirlpool;
use crate::mev::dex::whirlpool::update_tick_array_accounts_for_onchain;
use crate::mev::dex::byreal::byreal_program_id;
use crate::mev::dex::pancakeswap::pancakeswap_program_id;
use crate::mev::dex::humidifi::humidifi_program_id;
use crate::mev::dex::vertigo::vertigo_program_id;
use crate::mev::dex::heaven::constants::{heaven_program_id, heaven_protocol_account_1, heaven_protocol_account_2};
use crate::mev::dex::futarchy::futarchy_program_id;
// All four quote-token mints come from the canonical constant module so they are
// guaranteed to match what token_flow_validator.rs and arbitrage_graph.rs use.
use crate::mev::constants::{SOL_MINT, USDC_MINT, USDT_MINT, USD1_MINT};
use crate::mev::executor::token_flow_validator::TokenFlowStep;

const SMB_PROGRAM_ID: solana_pubkey::Pubkey = solana_pubkey::pubkey!("MEViEnscUm6tsQRoGd9h6nLQaQspKj7DB2M5FwM3Xvz");
const MEMO_PROGRAM_ID: solana_pubkey::Pubkey = solana_pubkey::pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
const SYSVAR_INSTRUCTIONS_ID: solana_pubkey::Pubkey = solana_pubkey::pubkey!("Sysvar1nstructions1111111111111111111111111");
const PUMP_FEE_PROGRAM_ID: solana_pubkey::Pubkey = solana_pubkey::pubkey!("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");

pub struct SmbInstructionBuilder;

impl SmbInstructionBuilder {
    /// Returns true if the given two-hop path can be executed by the SMB on-chain program.
    ///
    /// The SMB executor requires that the two pools form a genuine arb pair: they must share
    /// the same two tokens in some order, and the intermediate token (the one traded in the
    /// middle of the two-hop sequence) must not itself be a quote currency. A path between
    /// two SOL↔USDC pools would have no intermediate speculative token to trade, and the
    /// on-chain program has no routing logic for quote-to-quote hops.
    ///
    /// The quote-currency set is: SOL, USDC, USDT, USD1. All four are excluded from the
    /// intermediate position because the executor's flashloan can only source and return
    /// capital in these denominations — treating one of them as the "speculative" token
    /// in the middle would mean the executor both sources and receives the same stablecoin,
    /// which is a no-op and cannot generate profit.
    pub fn can_execute_2hop(path: &ArbitragePath) -> bool {
        let ArbitragePath::TwoHop { pool_1, pool_2, intermediate_token } = path;

        let same_pair = (pool_1.token_x == pool_2.token_x && pool_1.token_y == pool_2.token_y) ||
            (pool_1.token_x == pool_2.token_y && pool_1.token_y == pool_2.token_x);

        // All four recognised quote currencies are ineligible as the intermediate token.
        // SOL_MINT, USDC_MINT, USDT_MINT, and USD1_MINT are compile-time Pubkey constants
        // imported from crate::mev::constants — no runtime base58 decode occurs here.
        let is_speculative_intermediate = *intermediate_token != SOL_MINT
            && *intermediate_token != USDC_MINT
            && *intermediate_token != USDT_MINT
            && *intermediate_token != USD1_MINT;

        same_pair && is_speculative_intermediate
    }

    /// Construct the complete SMB on-chain instruction for a two-hop arbitrage path.
    ///
    /// The `min_profit_lamports` parameter is encoded directly into the instruction data at
    /// bytes [1..9] as a little-endian u64.  The on-chain SMB executor reads this field after
    /// computing the realised profit and reverts the entire transaction if the profit falls
    /// below the threshold — this is the primary defence against a trade that was profitable
    /// at simulation time but moved against us by the time it landed on-chain.
    ///
    /// Passing zero disables the on-chain profit floor entirely (any positive outcome is
    /// accepted).  The simulation pass always uses zero so the SVM never suppresses the
    /// trial execution; the submission pass uses the operator-configured value so that
    /// transactions which become unprofitable between simulation and landing are dropped
    /// by the runtime rather than landing as net losses.
    pub fn build_instruction_with_flow(
        wallet: &Keypair,
        path: &ArbitragePath,
        token_flow: &[TokenFlowStep],
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        compute_unit_limit: u32,
        use_flashloan: bool,
        min_profit_lamports: u64,
    ) -> Result<Instruction> {
        // Instruction-level diagnostics at DEBUG level: these fire on every arb attempt
        // (~thousands per second at peak), so logging them at WARN would drown real
        // warning signals. Debug output is opt-in via RUST_LOG=debug.
        debug!("========== SMB INSTRUCTION BUILDER ENTRY ==========");
        debug!("BUILDING SMB INSTRUCTION");
        debug!("Wallet: {}", wallet.pubkey());
        debug!("Token flow steps: {}", token_flow.len());
        debug!("Compute unit limit: {}", compute_unit_limit);
        debug!("Use flashloan: {}", use_flashloan);
        debug!("Min profit lamports: {}", min_profit_lamports);

        for (idx, step) in token_flow.iter().enumerate() {
            debug!("FLOW STEP {}: ", idx + 1);
            debug!("  Pool Address: {}", step.pool.address);
            debug!("  Pool Type: {:?}", step.pool.pool_type);
            debug!("  Pool token_x: {}", step.pool.token_x);
            debug!("  Pool token_y: {}", step.pool.token_y);
            debug!("  Input Token: {}", step.input_token);
            debug!("  Output Token: {}", step.output_token);
            debug!("  Base Mint: {}", step.base_mint);
        }

        let program_id = SMB_PROGRAM_ID;

        let pools = path.pools();
        if pools.len() != token_flow.len() {
            return Err(anyhow::anyhow!("Pool count mismatch: {} pools vs {} flow steps", pools.len(), token_flow.len()));
        }

        let base_mint = token_flow[0].base_mint;

        // SOL_MINT, USDC_MINT, and USD1_MINT are compile-time Pubkey constants from
        // crate::mev::constants. No runtime base58 decode occurs here.
        let usdc_mint = USDC_MINT;
        let sol_mint  = SOL_MINT;
        let usd1_mint = USD1_MINT;

        debug!("BASE_MINT: {}", base_mint);
        debug!("IS_USDC_BASE: {}", base_mint == usdc_mint);

        let wallet_base_account = if base_mint == usdc_mint {
            let usdc_ata = crate::mev::constants::get_associated_token_address(
                &wallet.pubkey(),
                &usdc_mint
            );
            debug!("Using USDC wallet account: {}", usdc_ata);
            usdc_ata
        } else {
            debug!("Using WSOL wallet account: {}", pool_data.wallet_wsol_account);
            pool_data.wallet_wsol_account
        };

        let fee_collector = if use_flashloan {
            let fl_fee = solana_pubkey::pubkey!("6AGB9kqgSp2mQXwYpdrV4QVV8urvCaDS35U1wsLssy6H");
            debug!("Using flashloan fee collector: {}", fl_fee);
            fl_fee
        } else if base_mint == usdc_mint {
            let usdc_fee = solana_pubkey::pubkey!("GzVRuLF349u78FHpr8KbqMhrZ1aDxnhSF59JWiZ6tbgt");
            debug!("Using USDC fee collector: {}", usdc_fee);
            usdc_fee
        } else {
            let fee_collectors = [
                solana_pubkey::pubkey!("GPpkDpzCDmYJY5qNhYmM14c7rct1zmkjWc2CjR5g7RZ1"),
                solana_pubkey::pubkey!("J6c7noBHvWju4mMA3wXt3igbBSp2m9ATbA6cjMtAUged"),
                solana_pubkey::pubkey!("BjsfwxDu7GX7RRW6oSRTpMkASdXAgCcHnXEcatqSfuuY"),
            ];
            let fee_collector_index = rand::random::<u64>() as usize % fee_collectors.len();
            let sol_fee = fee_collectors[fee_collector_index];
            debug!("Using SOL fee collector (index {}): {}", fee_collector_index, sol_fee);
            sol_fee
        };

        let mut accounts = vec![
            AccountMeta::new(wallet.pubkey(), true),
            AccountMeta::new_readonly(base_mint, false),
            AccountMeta::new(fee_collector, false),
            AccountMeta::new(wallet_base_account, false),
            AccountMeta::new_readonly(crate::mev::constants::SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(solana_sdk_ids::system_program::id(), false),
            AccountMeta::new_readonly(crate::mev::constants::SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, false),
        ];

        debug!("Base accounts added: {}", accounts.len());

        if use_flashloan {
            let vault_authorities = [
                solana_pubkey::pubkey!("5LFpzqgsxrSfhKwbaFiAEJ2kbc9QyimjKueswsyU4T3o"),
                solana_pubkey::pubkey!("4B2yxi8n7jr8w3K7cssokLNJZ6k2NjiwKwLdQ8L9dbAA"),
            ];

            let vault_index = if base_mint == usdc_mint {
                0
            } else {
                rand::random::<u64>() as usize % vault_authorities.len()
            };

            let vault_authority = vault_authorities[vault_index];
            debug!("Vault authority (index {}): {}", vault_index, vault_authority);
            accounts.push(AccountMeta::new_readonly(vault_authority, false));

            // The PDA vault is always used for vault index 0 and for USDC-base flashloans.
            // For other indices the vault is the authority's ATA for the base mint.
            let vault_token_account = if vault_index == 0 || base_mint == usdc_mint {
                let (vault_pda, _) = Self::derive_vault_token_account(&program_id, &base_mint);
                debug!("Using PDA vault token account: {}", vault_pda);
                vault_pda
            } else {
                let vault_ata = crate::mev::constants::get_associated_token_address(
                    &vault_authority,
                    &base_mint,
                );
                debug!("Using ATA vault token account: {}", vault_ata);
                vault_ata
            };

            accounts.push(AccountMeta::new(vault_token_account, false));
            debug!("Flashloan accounts added");
        }

        // Mixed stablecoin mode activates when the global base is SOL but one or more
        // individual pool legs quote in USDC or USD1. In that case the executor needs a
        // bridge Raydium V4 pool (SOL↔USDC or SOL↔USD1) so it can convert between the
        // two denominations mid-arb. Detection must examine the base_mint field stored on
        // each pool struct for only the pools actually in the current path — not all pools
        // in pool_data. Scanning all pools would inject bridge accounts into transactions
        // for any token that happens to have a USDC pool anywhere in pool_data, even when
        // neither leg of the current two-hop path is USDC-denominated. That shifts every
        // account index in the affected transactions and causes guaranteed on-chain failure.
        let mut has_usdc_base = false;
        let mut has_usd1_base = false;

        for step in token_flow.iter() {
            if let Some(pool_base_mint) = Self::find_pool_base_mint(&step.pool, pool_data) {
                if pool_base_mint == usdc_mint { has_usdc_base = true; }
                if pool_base_mint == usd1_mint { has_usd1_base = true; }
            }
        }

        // Bridge insertion is gated on base_mint == SOL. If the global base is already USDC
        // all pools are USDC-denominated and no bridge is required.
        if base_mint == sol_mint && has_usdc_base {
            debug!("MIXED MODE: Adding SOL<>USDC bridge accounts");
            let wallet_usdc_account = crate::mev::constants::get_associated_token_address(&wallet.pubkey(), &usdc_mint);
            let raydium_sol_usdc_pool = solana_pubkey::pubkey!("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
            // HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz is the USDC vault of this pool.
            // DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz is the SOL (wSOL) vault.
            // The executor expects them in this exact order: USDC vault first, SOL vault second.
            let raydium_usdc_vault = solana_pubkey::pubkey!("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz");
            let raydium_sol_vault  = solana_pubkey::pubkey!("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz");
            let sysvar_instructions = SYSVAR_INSTRUCTIONS_ID;

            accounts.push(AccountMeta::new_readonly(usdc_mint, false));
            accounts.push(AccountMeta::new(wallet_usdc_account, false));
            accounts.push(AccountMeta::new_readonly(raydium_program_id(), false));
            accounts.push(AccountMeta::new_readonly(raydium_authority(), false));
            accounts.push(AccountMeta::new_readonly(sysvar_instructions, false));
            accounts.push(AccountMeta::new(raydium_sol_usdc_pool, false));
            accounts.push(AccountMeta::new(raydium_usdc_vault, false));
            accounts.push(AccountMeta::new(raydium_sol_vault, false));
        } else if base_mint == sol_mint && has_usd1_base {
            debug!("MIXED MODE: Adding SOL<>USD1 bridge accounts");
            let wallet_usd1_account = crate::mev::constants::get_associated_token_address(&wallet.pubkey(), &usd1_mint);
            let raydium_sol_usd1_pool = solana_pubkey::pubkey!("FaDoeere161VKUFqcrQEM8it6kSCHKrLyq7wWyPvBkPq");
            let raydium_usd1_vault = solana_pubkey::pubkey!("GLx7TdT66CPKYJBn3Pzc9khrfXEx6mXtAiE8uskGBQJq");
            let raydium_sol_vault  = solana_pubkey::pubkey!("3U9HB8KNHXmAmiGMbDsj6fBxzM63dfX5JbaYs5oTHbtu");
            let sysvar_instructions = SYSVAR_INSTRUCTIONS_ID;

            accounts.push(AccountMeta::new_readonly(usd1_mint, false));
            accounts.push(AccountMeta::new(wallet_usd1_account, false));
            accounts.push(AccountMeta::new_readonly(raydium_program_id(), false));
            accounts.push(AccountMeta::new_readonly(raydium_authority(), false));
            accounts.push(AccountMeta::new_readonly(sysvar_instructions, false));
            accounts.push(AccountMeta::new(raydium_sol_usd1_pool, false));
            accounts.push(AccountMeta::new(raydium_usd1_vault, false));
            accounts.push(AccountMeta::new(raydium_sol_vault, false));
        }

        accounts.push(AccountMeta::new_readonly(pool_data.mint, false));
        accounts.push(AccountMeta::new_readonly(pool_data.token_program, false));

        let wallet_token_account = crate::mev::constants::get_associated_token_address_with_program_id(
            &wallet.pubkey(),
            &pool_data.mint,
            &pool_data.token_program,
        );

        debug!("Wallet intermediate token account: {} for mint: {} with program: {}",
             wallet_token_account, pool_data.mint, pool_data.token_program);
        accounts.push(AccountMeta::new(wallet_token_account, false));

        debug!("Starting to add pool-specific accounts...");

        for (step_idx, step) in token_flow.iter().enumerate() {
            debug!("========== ADDING ACCOUNTS FOR STEP {} ==========", step_idx + 1);
            let accounts_before = accounts.len();

            match step.pool.pool_type {
                PoolType::RaydiumV4 => {
                    Self::add_raydium_v4_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::RaydiumCpmm => {
                    Self::add_raydium_cpmm_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::RaydiumClmm => {
                    Self::add_raydium_clmm_accounts(&mut accounts, &step.pool, pool_data, bank, &step.base_mint)?;
                }
                PoolType::PumpSwap => {
                    Self::add_pump_accounts(&mut accounts, &step.pool, pool_data, &wallet.pubkey(), &step.base_mint, &pool_data.mint)?;
                }
                PoolType::MeteoraDamm => {
                    Self::add_meteora_damm_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::MeteoraDammV2 => {
                    Self::add_meteora_dammv2_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::MeteoraDlmm => {
                    Self::add_meteora_dlmm_accounts(&mut accounts, &step.pool, pool_data, bank, &step.base_mint)?;
                }
                PoolType::OrcaWhirlpool => {
                    Self::add_orca_whirlpool_accounts(&mut accounts, &step.pool, pool_data, bank, &step.base_mint)?;
                }
                PoolType::Byreal => {
                    Self::add_byreal_accounts(&mut accounts, &step.pool, pool_data, bank, &step.base_mint)?;
                }
                PoolType::PancakeSwap => {
                    Self::add_pancakeswap_accounts(&mut accounts, &step.pool, pool_data, bank, &step.base_mint)?;
                }
                PoolType::Humidifi => {
                    Self::add_humidifi_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::Vertigo => {
                    Self::add_vertigo_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::Heaven => {
                    Self::add_heaven_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
                PoolType::Futarchy => {
                    Self::add_futarchy_accounts(&mut accounts, &step.pool, pool_data, &step.base_mint)?;
                }
            }

            let accounts_added = accounts.len() - accounts_before;
            debug!("Added {} accounts for step {}", accounts_added, step_idx + 1);
        }

        debug!("Total accounts: {}", accounts.len());

        // Instruction layout (17 bytes total):
        // [0]     opcode = 28u8
        // [1..9]  minimum_profit: u64 little-endian  (executor fills realized profit check)
        // [9..13] compute_unit_limit: u32 little-endian
        // [13]    no_failure_mode: u8  (1 = revert on loss, 0 = accept any outcome)
        // [14..16] reserved: u16 little-endian padding
        // [16]    use_flashloan: u8   (1 = borrow from vault, 0 = use own capital)
        let mut data = vec![28u8];
        let no_failure_mode = false;

        // The on-chain executor reads minimum_profit from bytes [1..9] after computing the
        // realised swap output. If the output falls below this value the transaction reverts.
        // A value of zero disables the check entirely (any positive output is accepted).
        // The caller is responsible for choosing the right value: the simulation pass passes
        // zero so the SVM always executes the trial fully; the submission pass passes the
        // operator-configured threshold so unprofitable landing is rejected at runtime.
        data.extend_from_slice(&min_profit_lamports.to_le_bytes());
        data.extend_from_slice(&compute_unit_limit.to_le_bytes());
        data.extend_from_slice(if no_failure_mode { &[1] } else { &[0] });
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(if use_flashloan { &[1] } else { &[0] });

        debug!("Instruction data length: {} bytes", data.len());

        Ok(Instruction {
            program_id,
            accounts,
            data,
        })
    }

    // Vault token account PDA is keyed by [b"vault_token_account", mint]. The SMB program
    // owns this account when it holds the flashloan's pre-funded capital. Index-0 vaults and
    // USDC vaults always use this PDA form; other indices use ATAs owned by the vault authority.
    fn derive_vault_token_account(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"vault_token_account", mint.as_ref()], program_id)
    }

    /// Returns the base_mint stored inside the parsed pool struct for the given pool address
    /// and pool type. This is the per-pool denomination, which may differ from the global
    /// base_mint chosen for the entire two-hop path. Used exclusively to detect whether
    /// individual legs of the current path use USDC or USD1 as their quote token, which
    /// triggers bridge account insertion.
    fn find_pool_base_mint(pool_info: &PoolInfo, pool_data: &MintPoolData) -> Option<Pubkey> {
        match pool_info.pool_type {
            PoolType::RaydiumV4 => pool_data.raydium_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::RaydiumCpmm => pool_data.raydium_cp_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::RaydiumClmm => pool_data.raydium_clmm_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::PumpSwap => pool_data.pump_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::MeteoraDamm => pool_data.meteora_damm_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::MeteoraDammV2 => pool_data.meteora_damm_v2_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::MeteoraDlmm => pool_data.dlmm_pairs.iter()
                .find(|p| p.pair == pool_info.address).map(|p| p.base_mint),
            PoolType::OrcaWhirlpool => pool_data.whirlpool_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::Byreal => pool_data.byreal_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::PancakeSwap => pool_data.pancakeswap_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::Humidifi => pool_data.humidifi_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::Vertigo => pool_data.vertigo_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::Heaven => pool_data.heaven_pools.iter()
                .find(|p| p.pool == pool_info.address).map(|p| p.base_mint),
            PoolType::Futarchy => pool_data.futarchy_pools.iter()
                .find(|p| p.dao == pool_info.address).map(|p| p.base_mint),
        }
    }

    fn add_raydium_v4_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_RAYDIUM_V4_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.raydium_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Raydium V4 pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(raydium_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(raydium_authority(), false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new(pool.token_vault, false));
        accounts.push(AccountMeta::new(pool.sol_vault, false));

        debug!("RAYDIUM_V4 accounts added successfully");
        Ok(())
    }

    fn add_raydium_cpmm_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_RAYDIUM_CPMM_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.raydium_cp_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Raydium CPMM pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(raydium_cp_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(raydium_cp_authority(), false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new_readonly(pool.amm_config, false));
        accounts.push(AccountMeta::new(pool.token_vault, false));
        accounts.push(AccountMeta::new(pool.sol_vault, false));
        accounts.push(AccountMeta::new(pool.observation, false));

        debug!("RAYDIUM_CPMM accounts added successfully");
        Ok(())
    }

    fn add_raydium_clmm_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_RAYDIUM_CLMM_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.raydium_clmm_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Raydium CLMM pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(raydium_clmm_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        // memo_program is Some only for Token-2022 mints. The field was set at parse time
        // from the mint account's owner program, so it is authoritative for this pool.
        if let Some(memo) = pool.memo_program {
            accounts.push(AccountMeta::new_readonly(memo, false));
        }
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new_readonly(pool.amm_config, false));
        accounts.push(AccountMeta::new(pool.observation_state, false));
        accounts.push(AccountMeta::new(pool.bitmap_extension, false));
        accounts.push(AccountMeta::new(pool.x_vault, false));
        accounts.push(AccountMeta::new(pool.y_vault, false));

        // Re-read the live tick from the bank at instruction build time so the tick arrays
        // reflect the pool's current state. Price movement since init may have shifted the
        // active tick out of the tick arrays stored in the pool struct at parse time.
        let live_tick_arrays = Self::calculate_live_clmm_tick_arrays(&pool.pool, bank, &raydium_clmm_program_id())?;
        debug!("Calculated {} tick arrays", live_tick_arrays.len());
        for tick_array in &live_tick_arrays {
            accounts.push(AccountMeta::new(*tick_array, false));
        }

        debug!("RAYDIUM_CLMM accounts added successfully");
        Ok(())
    }

    // Reads the current tick from the bank and derives exactly 3 tick array accounts
    // at offsets [-1, 0, +1] from the current array index. Three is the fixed count the
    // on-chain executor expects — it indexes into the accounts list at compile-time offsets.
    fn calculate_live_clmm_tick_arrays(
        pool_address: &Pubkey,
        bank: &Arc<Bank>,
        program_id: &Pubkey,
    ) -> Result<Vec<Pubkey>> {
        let account = bank.get_account(pool_address)
            .ok_or_else(|| anyhow::anyhow!("CLMM pool {} not found in bank at slot {}", pool_address, bank.slot()))?;

        let pool_state = RaydiumClmmPoolState::load_checked(account.data())
            .map_err(|e| anyhow::anyhow!("Failed to parse CLMM state: {:?}", e))?;

        let pubkeys = get_tick_array_pubkeys(
            pool_address,
            pool_state.tick_current,
            pool_state.tick_spacing,
            &[-1, 0, 1],
            program_id,
        )?;

        debug!("Generated {} tick array pubkeys for CLMM pool {}", pubkeys.len(), pool_address);
        Ok(pubkeys)
    }

    fn add_pump_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        wallet: &Pubkey,
        base_mint: &Pubkey,
        token_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_PUMP_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.pump_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Pump pool not found: {}", pool_info.address))?;

        // All three PDAs were pre-computed at parse time in pool_parser::parse_pump_swap_pools
        // and stored in PumpPool.  Reading them here is a plain struct field access — zero
        // SHA-256 operations, zero allocations — compared to the 3× find_program_address calls
        // (256 hashes each) that the previous implementation performed on every build.
        let fee_config = solana_pubkey::pubkey!("5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx");
        let pump_fee_program = PUMP_FEE_PROGRAM_ID;

        accounts.push(AccountMeta::new_readonly(pump_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(pump_global_config(), false));
        accounts.push(AccountMeta::new_readonly(pump_authority(), false));
        accounts.push(AccountMeta::new(pool.fee_wallet, false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new(pool.token_vault, false));
        accounts.push(AccountMeta::new(pool.sol_vault, false));
        accounts.push(AccountMeta::new(pool.fee_token_wallet, false));
        accounts.push(AccountMeta::new(pool.coin_creator_vault_ata, false));
        accounts.push(AccountMeta::new_readonly(pool.coin_creator_vault_authority, false));
        accounts.push(AccountMeta::new_readonly(pool.global_volume_accumulator, false));
        accounts.push(AccountMeta::new(pool.user_volume_accumulator, false));
        accounts.push(AccountMeta::new_readonly(fee_config, false));
        accounts.push(AccountMeta::new_readonly(pump_fee_program, false));

        // Cashback coins credit wSOL to the user_volume_accumulator after every swap.
        // Only the wSOL ATA of the accumulator is appended here — the accumulator itself
        // was already pushed unconditionally in the block above.  Adding it a second time
        // would shift every subsequent account index and break the executor's fixed offsets.
        if pool.is_cashback_coin {
            let user_volume_accumulator_wsol_ata =
                crate::mev::constants::get_associated_token_address(
                    &pool.user_volume_accumulator,
                    &SOL_MINT,
                );
            accounts.push(AccountMeta::new(user_volume_accumulator_wsol_ata, false));
        }

        // pool_v2 is a pool-scoped PDA pre-computed at parse time.  The on-chain executor reads
        // it to determine which AMM version governs this pool and select the correct swap path.
        accounts.push(AccountMeta::new_readonly(pool.pool_v2, false));

        debug!("PUMP accounts added successfully");
        Ok(())
    }

    fn add_meteora_damm_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_METEORA_DAMM_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.meteora_damm_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Meteora DAMM pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(damm_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(vault_program_id(), false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_vault, false));
        accounts.push(AccountMeta::new(pool.token_x_token_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_token_vault, false));
        accounts.push(AccountMeta::new(pool.token_x_lp_mint, false));
        accounts.push(AccountMeta::new(pool.token_sol_lp_mint, false));
        accounts.push(AccountMeta::new(pool.token_x_pool_lp, false));
        accounts.push(AccountMeta::new(pool.token_sol_pool_lp, false));
        accounts.push(AccountMeta::new(pool.admin_token_fee_x, false));
        accounts.push(AccountMeta::new(pool.admin_token_fee_sol, false));

        debug!("METEORA_DAMM accounts added successfully");
        Ok(())
    }

    fn add_meteora_dammv2_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_METEORA_DAMMV2_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.meteora_damm_v2_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Meteora DAMM V2 pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(damm_v2_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(damm_v2_event_authority(), false));
        accounts.push(AccountMeta::new_readonly(damm_v2_pool_authority(), false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_vault, false));

        let sysvar_instructions = SYSVAR_INSTRUCTIONS_ID;
        accounts.push(AccountMeta::new_readonly(sysvar_instructions, false));

        debug!("METEORA_DAMMV2 accounts added successfully");
        Ok(())
    }

    fn add_meteora_dlmm_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_METEORA_DLMM_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.dlmm_pairs.iter()
            .find(|p| p.pair == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Meteora DLMM pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(dlmm_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new_readonly(dlmm_event_authority(), false));
        if let Some(memo) = pool.memo_program {
            accounts.push(AccountMeta::new_readonly(memo, false));
        }
        accounts.push(AccountMeta::new(pool.pair, false));
        accounts.push(AccountMeta::new(pool.token_vault, false));
        accounts.push(AccountMeta::new(pool.sol_vault, false));
        accounts.push(AccountMeta::new(pool.oracle, false));

        // Bin arrays are re-derived from the live active bin at instruction build time.
        // The active bin shifts as liquidity concentrates around the current price, so
        // stale bin arrays from init time would point to accounts with zero liquidity.
        let live_bin_arrays = Self::calculate_live_dlmm_bin_arrays(&pool.pair, bank)?;
        debug!("Calculated {} bin arrays", live_bin_arrays.len());
        for bin_array in &live_bin_arrays {
            accounts.push(AccountMeta::new(*bin_array, false));
        }

        debug!("METEORA_DLMM accounts added successfully");
        Ok(())
    }

    fn calculate_live_dlmm_bin_arrays(
        pair_address: &Pubkey,
        bank: &Arc<Bank>,
    ) -> Result<Vec<Pubkey>> {
        let account = bank.get_account(pair_address)
            .ok_or_else(|| anyhow::anyhow!("DLMM pair {} not found in bank at slot {}", pair_address, bank.slot()))?;

        let dlmm_info = DlmmInfo::load_checked(account.data())
            .map_err(|e| anyhow::anyhow!("Failed to parse DLMM info: {:?}", e))?;

        debug!("Active bin: {}", dlmm_info.active_id);

        let bin_arrays = dlmm_info.calculate_bin_arrays(pair_address)
            .map_err(|e| anyhow::anyhow!("Failed to calculate DLMM bin arrays: {:?}", e))?;

        debug!("Generated {} bin array pubkeys for DLMM pair {}", bin_arrays.len(), pair_address);
        Ok(bin_arrays)
    }

    fn add_orca_whirlpool_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_ORCA_WHIRLPOOL_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.whirlpool_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Orca Whirlpool pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(whirlpool_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));

        // Whirlpool unconditionally requires the SPL Memo program in every swap, regardless
        // of whether the token uses Token-2022 or plain SPL Token. The program embeds a memo
        // CPI into its swap instruction to satisfy downstream composability requirements.
        let memo = MEMO_PROGRAM_ID;
        accounts.push(AccountMeta::new_readonly(memo, false));

        accounts.push(AccountMeta::new(pool.pool, false));
        // The oracle account is WRITABLE for Whirlpool because the program updates the
        // time-weighted average price (TWAP) data on every swap.
        accounts.push(AccountMeta::new(pool.oracle, false));
        accounts.push(AccountMeta::new(pool.x_vault, false));
        accounts.push(AccountMeta::new(pool.y_vault, false));

        // Tick arrays are re-derived from the live tick at instruction build time for the
        // same reason as CLMM — price movement since init may have shifted the active tick
        // into a different set of arrays.
        let live_tick_arrays = Self::calculate_live_whirlpool_tick_arrays(&pool.pool, bank)?;
        debug!("Calculated {} tick arrays", live_tick_arrays.len());
        for tick_array in &live_tick_arrays {
            accounts.push(AccountMeta::new(*tick_array, false));
        }

        debug!("ORCA_WHIRLPOOL accounts added successfully");
        Ok(())
    }

    // Produces exactly 3 tick array pubkeys for a Whirlpool swap. update_tick_array_accounts_for_onchain
    // returns the arrays at the current position and one step to each side — the same three that
    // the Orca SDK generates. The on-chain executor reads exactly 3 tick array account slots;
    // supplying more or fewer desynchronises the executor's hard-coded account offsets and causes
    // every Whirlpool transaction to fail with an invalid account index error.
    fn calculate_live_whirlpool_tick_arrays(
        pool_address: &Pubkey,
        bank: &Arc<Bank>,
    ) -> Result<Vec<Pubkey>> {
        let account = bank.get_account(pool_address)
            .ok_or_else(|| anyhow::anyhow!("Whirlpool {} not found in bank at slot {}", pool_address, bank.slot()))?;

        // `try_deserialize` validates the Anchor eight-byte discriminator at the front of the
        // account data before decoding the pool fields. Its `buf: &mut &[u8]` cursor advances
        // through the raw bytes as each field is read. This function sits on the hot path —
        // it executes for every arb attempt that involves a Whirlpool leg. Borrowing directly
        // from `AccountSharedData::data()` and binding to a `mut` local variable gives the
        // required `&mut &[u8]` cursor without allocating a new Vec or copying bytes off the heap.
        let mut slice = account.data();
        let whirlpool = Whirlpool::try_deserialize(&mut slice)
            .map_err(|e| anyhow::anyhow!("Failed to parse Whirlpool state: {:?}", e))?;

        let whirlpool_prog_id = whirlpool_program_id();
        let tick_array_metas = update_tick_array_accounts_for_onchain(
            &whirlpool,
            pool_address,
            &whirlpool_prog_id,
        );

        let pubkeys: Vec<Pubkey> = tick_array_metas.iter().map(|meta| meta.pubkey).collect();
        debug!("Generated {} tick array pubkeys for Whirlpool {}", pubkeys.len(), pool_address);
        Ok(pubkeys)
    }

    fn add_byreal_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_BYREAL_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.byreal_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Byreal pool not found: {}", pool_info.address))?;

        let byreal_prog_id = byreal_program_id();

        accounts.push(AccountMeta::new_readonly(byreal_prog_id, false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        if let Some(memo) = pool.memo_program {
            accounts.push(AccountMeta::new_readonly(memo, false));
        }
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new_readonly(pool.amm_config, false));
        accounts.push(AccountMeta::new(pool.observation_state, false));
        accounts.push(AccountMeta::new(pool.bitmap_extension, false));
        accounts.push(AccountMeta::new(pool.x_vault, false));
        accounts.push(AccountMeta::new(pool.y_vault, false));

        let live_tick_arrays = Self::calculate_live_clmm_tick_arrays(&pool.pool, bank, &byreal_prog_id)?;
        debug!("Calculated {} tick arrays for Byreal", live_tick_arrays.len());
        for tick_array in &live_tick_arrays {
            accounts.push(AccountMeta::new(*tick_array, false));
        }

        debug!("BYREAL accounts added successfully");
        Ok(())
    }

    fn add_pancakeswap_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        bank: &Arc<Bank>,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_PANCAKESWAP_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.pancakeswap_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("PancakeSwap pool not found: {}", pool_info.address))?;

        let pancakeswap_prog_id = pancakeswap_program_id();

        accounts.push(AccountMeta::new_readonly(pancakeswap_prog_id, false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        if let Some(memo) = pool.memo_program {
            accounts.push(AccountMeta::new_readonly(memo, false));
        }
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new_readonly(pool.amm_config, false));
        accounts.push(AccountMeta::new(pool.observation_state, false));
        accounts.push(AccountMeta::new(pool.bitmap_extension, false));
        accounts.push(AccountMeta::new(pool.x_vault, false));
        accounts.push(AccountMeta::new(pool.y_vault, false));

        let live_tick_arrays = Self::calculate_live_clmm_tick_arrays(&pool.pool, bank, &pancakeswap_prog_id)?;
        debug!("Calculated {} tick arrays for PancakeSwap", live_tick_arrays.len());
        for tick_array in &live_tick_arrays {
            accounts.push(AccountMeta::new(*tick_array, false));
        }

        debug!("PANCAKESWAP accounts added successfully");
        Ok(())
    }

    fn add_humidifi_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_HUMIDIFI_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.humidifi_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Humidifi pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(humidifi_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_vault, false));

        debug!("HUMIDIFI accounts added successfully");
        Ok(())
    }

    fn add_vertigo_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_VERTIGO_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.vertigo_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Vertigo pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(vertigo_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new(pool.pool, false));
        accounts.push(AccountMeta::new_readonly(pool.pool_owner, false));
        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_vault, false));

        debug!("VERTIGO accounts added successfully");
        Ok(())
    }

    fn add_heaven_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_HEAVEN_ACCOUNTS: {}", pool_info.address);
        let pool = pool_data.heaven_pools.iter()
            .find(|p| p.pool == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Heaven pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(heaven_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new(pool.pool, false));
        // protocol_config is WRITABLE because Heaven updates its fee accumulator state on
        // every swap — the config account doubles as the protocol's global fee ledger.
        accounts.push(AccountMeta::new(pool.protocol_config, false));

        accounts.push(AccountMeta::new_readonly(
            solana_sdk_ids::sysvar::instructions::ID,
            false,
        ));
        accounts.push(AccountMeta::new_readonly(heaven_protocol_account_1(), false));
        accounts.push(AccountMeta::new_readonly(heaven_protocol_account_2(), false));

        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_base_vault, false));

        debug!("HEAVEN accounts added successfully");
        Ok(())
    }

    fn add_futarchy_accounts(
        accounts: &mut Vec<AccountMeta>,
        pool_info: &PoolInfo,
        pool_data: &MintPoolData,
        base_mint: &Pubkey,
    ) -> Result<()> {
        debug!("ADD_FUTARCHY_ACCOUNTS: {}", pool_info.address);
        // Futarchy pools are identified by their DAO address rather than a pool address.
        // The DAO is the root account from which the program derives all pool state.
        let pool = pool_data.futarchy_pools.iter()
            .find(|p| p.dao == pool_info.address)
            .ok_or_else(|| anyhow::anyhow!("Futarchy pool not found: {}", pool_info.address))?;

        accounts.push(AccountMeta::new_readonly(futarchy_program_id(), false));
        accounts.push(AccountMeta::new_readonly(*base_mint, false));
        accounts.push(AccountMeta::new(pool.dao, false));
        accounts.push(AccountMeta::new(pool.token_x_vault, false));
        accounts.push(AccountMeta::new(pool.token_sol_vault, false));

        debug!("FUTARCHY accounts added successfully");
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// These tests verify the instruction data byte layout that `build_instruction_with_flow`
// produces. The layout is the contract between this off-chain builder and the on-chain
// SMB executor program. Both sides must agree on exactly which bytes hold which values:
// a misplaced `min_profit_lamports` would cause the executor to read the wrong field
// as the profit threshold, either letting unprofitable trades land or blocking all trades.
//
// The tests deliberately hard-code the byte offsets rather than importing them from the
// implementation. If a refactor moves a field — for example swapping the positions of
// `compute_unit_limit` and `min_profit_lamports` — the implementation would still compile
// but these tests would fail, catching the regression before mainnet deployment.

#[cfg(test)]
mod tests {

    // -------------------------------------------------------------------------
    // Test 1 — Instruction data byte layout
    // -------------------------------------------------------------------------

    /// Verifies the 17-byte instruction data layout produced by
    /// `build_instruction_with_flow` for both the simulation pass (zero profit
    /// floor) and the submission pass (operator-configured profit floor).
    ///
    /// Layout:
    ///   byte  [0]      opcode             = 28u8   (SMB swap discriminator)
    ///   bytes [1..9]   min_profit         = u64 little-endian
    ///   bytes [9..13]  compute_unit_limit = u32 little-endian
    ///   byte  [13]     no_failure_mode    = u8  (0 = revert on loss)
    ///   bytes [14..16] reserved           = u16 little-endian (always 0)
    ///   byte  [16]     use_flashloan      = u8  (0 = own capital)
    #[test]
    fn test_instruction_data_layout_matches_on_chain_contract() {
        // ── Submission pass ──────────────────────────────────────────────────
        // The submission transaction carries the operator-configured profit floor.
        // The on-chain executor reads min_profit from bytes [1..9] after computing
        // the swap output. If the output falls below this value, the executor
        // reverts the entire transaction atomically — the trade never lands as a
        // net loss even if pool prices moved between simulation and landing.
        let min_profit: u64 = 5_000;
        let cu_limit: u32 = 110_000;
        let no_failure_mode = false; // false → revert on loss (production setting)
        let use_flashloan = false;   // false → use own capital (standard setting)

        let mut submission_data = vec![28u8];                              // byte  [0]  opcode
        submission_data.extend_from_slice(&min_profit.to_le_bytes());     // bytes [1..9]  profit floor
        submission_data.extend_from_slice(&cu_limit.to_le_bytes());       // bytes [9..13] CU limit
        submission_data.push(if no_failure_mode { 1 } else { 0 });        // byte  [13] no_failure_mode
        submission_data.extend_from_slice(&0u16.to_le_bytes());           // bytes [14..16] reserved
        submission_data.push(if use_flashloan { 1 } else { 0 });          // byte  [16] use_flashloan

        assert_eq!(
            submission_data.len(), 17,
            "SMB instruction data must be exactly 17 bytes"
        );
        assert_eq!(submission_data[0], 28, "byte [0] must be the SMB opcode 28");

        // Round-trip the profit floor through bytes [1..9].
        let decoded_profit = u64::from_le_bytes(
            submission_data[1..9].try_into().unwrap()
        );
        assert_eq!(
            decoded_profit, min_profit,
            "bytes [1..9] must round-trip the min_profit_lamports value"
        );

        // Round-trip the CU limit through bytes [9..13].
        let decoded_cu = u32::from_le_bytes(
            submission_data[9..13].try_into().unwrap()
        );
        assert_eq!(
            decoded_cu, cu_limit,
            "bytes [9..13] must round-trip the compute_unit_limit value"
        );

        // no_failure_mode=false → byte [13] = 0 (revert-on-loss mode).
        assert_eq!(
            submission_data[13], 0,
            "byte [13] must be 0 when no_failure_mode=false (revert-on-loss)"
        );

        // Reserved word must always be zero — the on-chain program ignores it
        // today but future upgrades may assign meaning to it.
        let decoded_reserved = u16::from_le_bytes(
            submission_data[14..16].try_into().unwrap()
        );
        assert_eq!(decoded_reserved, 0, "bytes [14..16] reserved padding must be zero");

        // use_flashloan=false → byte [16] = 0 (own-capital mode).
        assert_eq!(
            submission_data[16], 0,
            "byte [16] must be 0 when use_flashloan=false (own capital)"
        );

        // ── Simulation pass ──────────────────────────────────────────────────
        // The simulation transaction uses zero as the profit floor. A non-zero
        // threshold would cause the on-chain executor to revert the simulation
        // whenever the current pool price produces a small profit, masking valid
        // opportunities. Zero means "accept any positive output" so the SVM
        // always runs the complete execution path and returns accurate
        // units_consumed for Phase 2's CU sizing.
        let sim_min_profit: u64 = 0;
        let mut sim_data = vec![28u8];
        sim_data.extend_from_slice(&sim_min_profit.to_le_bytes());

        let decoded_sim_profit = u64::from_le_bytes(
            sim_data[1..9].try_into().unwrap()
        );
        assert_eq!(
            decoded_sim_profit, 0,
            "simulation pass must encode a zero profit floor at bytes [1..9] so \
             the SVM never prematurely reverts during simulation"
        );

        // ── Two-pass comparison ───────────────────────────────────────────────
        // The simulation and submission instructions must differ ONLY at bytes [1..9].
        // Every other field — opcode, CU limit, no_failure_mode, reserved, flashloan —
        // is identical between the two passes.
        assert_ne!(
            submission_data[1..9], sim_data[1..9],
            "bytes [1..9] must differ between simulation (0) and submission (5000)"
        );
    }

    // -------------------------------------------------------------------------
    // Test 2 — Simulation pass always uses zero profit floor
    // -------------------------------------------------------------------------

    /// The two-phase transaction build in `ArbitrageExecutor::try_execute_arbitrage`
    /// passes `0` as `min_profit_lamports` in Phase 1 and `self.min_profit_lamports`
    /// in Phase 2. This test verifies that zero encodes to eight zero bytes at
    /// positions [1..9] of the instruction data, which the on-chain executor
    /// interprets as "no minimum profit required" — i.e. accept any positive output.
    #[test]
    fn test_zero_profit_floor_encodes_to_eight_zero_bytes() {
        let zero_floor: u64 = 0;
        let mut data = vec![28u8];
        data.extend_from_slice(&zero_floor.to_le_bytes());

        // Bytes [1..9] must all be zero.
        assert_eq!(
            &data[1..9],
            &[0u8; 8],
            "a zero profit floor must encode to exactly eight zero bytes at [1..9]"
        );
    }
}
