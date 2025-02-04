#![allow(clippy::too_many_arguments)]
use anchor_client::{Program, RequestBuilder};
use marinade_finance::state::liq_pool::LiqPool;
use marinade_finance::state::stake_system::StakeSystem;
use marinade_finance::state::validator_system::ValidatorRecord;
use marinade_finance::state::{Fee, State};
use marinade_finance::{
    accounts as marinade_finance_accounts, instruction as marinade_finance_instruction,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{stake, system_program, sysvar};
use std::ops::Deref;

pub fn add_validator<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &'a State,
    validator_vote: &Pubkey,
    score: u32,
    rent_payer: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::AddValidator {
            state: *state_pubkey,
            manager_authority: state.validator_system.manager_authority,
            validator_list: *state.validator_system.validator_list_address(),
            validator_vote: *validator_vote,
            duplication_flag: ValidatorRecord::find_duplication_flag(
                &state_pubkey.clone(),
                validator_vote,
            )
            .0,
            rent_payer: *rent_payer,
            clock: sysvar::clock::id(),
            rent: sysvar::rent::id(),
            system_program: system_program::ID,
        })
        .args(marinade_finance_instruction::AddValidator { score }))
}

pub fn config_validator_system<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &'a State,
    extra_runs: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::ConfigValidatorSystem {
            state: *state_pubkey,
            manager_authority: state.validator_system.manager_authority,
        })
        .args(marinade_finance_instruction::ConfigValidatorSystem { extra_runs }))
}

pub fn set_validator_score<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    validator_vote: &Pubkey,
    validator_index: u32,
    score: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::SetValidatorScore {
            state: *state_pubkey,
            manager_authority: state.validator_system.manager_authority,
            validator_list: *state.validator_system.validator_list_address(),
        })
        .args(marinade_finance_instruction::SetValidatorScore {
            score,
            index: validator_index,
            validator_vote: *validator_vote,
        }))
}

pub fn remove_validator<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    validator_vote: &Pubkey,
    index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::RemoveValidator {
            state: *state_pubkey,
            manager_authority: state.validator_system.manager_authority,
            validator_list: *state.validator_system.validator_list_address(),
            duplication_flag: ValidatorRecord::find_duplication_flag(
                &state_pubkey.clone(),
                validator_vote,
            )
            .0,
            operational_sol_account: state.operational_sol_account,
        })
        .args(marinade_finance_instruction::RemoveValidator {
            index,
            validator_vote: *validator_vote,
        }))
}

pub fn emergency_unstake<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    stake_index: u32,
    validator_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::EmergencyUnstake {
            state: *state_pubkey,
            validator_manager_authority: state.validator_system.manager_authority,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_account: *stake_account,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            clock: sysvar::clock::id(),
            stake_program: stake::program::id(),
        })
        .args(marinade_finance_instruction::EmergencyUnstake {
            stake_index,
            validator_index,
        }))
}

pub fn add_liquidity<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    transfer_from: &Pubkey,
    mint_to: &Pubkey,
    lamports: u64,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::AddLiquidity {
            state: *state_pubkey,
            lp_mint: state.liq_pool.lp_mint,
            lp_mint_authority: LiqPool::find_lp_mint_authority(state_pubkey).0,
            liq_pool_msol_leg: state.liq_pool.msol_leg,
            liq_pool_sol_leg_pda: LiqPool::find_sol_leg_address(state_pubkey).0,
            transfer_from: *transfer_from,
            mint_to: *mint_to,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(marinade_finance_instruction::AddLiquidity { lamports }))
}

pub fn change_authority<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    data: marinade_finance::instructions::ChangeAuthorityData,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::ChangeAuthority {
            state: *state_pubkey,
            admin_authority: state.admin_authority,
        })
        .args(marinade_finance_instruction::ChangeAuthority { data }))
}

pub fn deactivate_stake<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    split_stake_account: &Pubkey,
    split_stake_rent_payer: &Pubkey,
    stake_index: u32,
    validator_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::DeactivateStake {
            state: *state_pubkey,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_account: *stake_account,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            split_stake_account: *split_stake_account,
            split_stake_rent_payer: *split_stake_rent_payer,
            clock: sysvar::clock::id(),
            rent: sysvar::rent::id(),
            epoch_schedule: sysvar::epoch_schedule::id(),
            stake_history: sysvar::stake_history::id(),
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::DeactivateStake {
            stake_index,
            validator_index,
        }))
}

pub fn deposit<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    transfer_from: &Pubkey,
    mint_to: &Pubkey,
    lamports: u64,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::Deposit {
            state: *state_pubkey,
            msol_mint: state.msol_mint,
            liq_pool_sol_leg_pda: LiqPool::find_sol_leg_address(state_pubkey).0,
            liq_pool_msol_leg: state.liq_pool.msol_leg,
            liq_pool_msol_leg_authority: LiqPool::find_msol_leg_authority(state_pubkey).0,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            transfer_from: *transfer_from,
            mint_to: *mint_to,
            msol_mint_authority: State::find_msol_mint_authority(state_pubkey).0,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(marinade_finance_instruction::Deposit { lamports }))
}

pub fn deposit_stake_account<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    stake_authority: &Pubkey,
    mint_to: &Pubkey,
    validator_index: u32,
    validator_vote: &Pubkey,
    rent_payer: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::DepositStakeAccount {
            state: *state_pubkey,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_account: *stake_account,
            stake_authority: *stake_authority,
            duplication_flag: ValidatorRecord::find_duplication_flag(
                &state_pubkey.clone(),
                validator_vote,
            )
            .0,
            rent_payer: *rent_payer,
            msol_mint: state.msol_mint,
            mint_to: *mint_to,
            msol_mint_authority: State::find_msol_mint_authority(state_pubkey).0,
            clock: sysvar::clock::id(),
            rent: sysvar::rent::id(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::DepositStakeAccount { validator_index }))
}

pub fn withdraw_stake_account<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    burn_msol_from: &Pubkey,
    burn_msol_authority: &Pubkey, // delegated or owner
    split_stake_account: &Pubkey,
    split_stake_rent_payer: &Pubkey,
    validator_index: u32,
    stake_index: u32,
    msol_amount: u64,
    beneficiary: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::WithdrawStakeAccount {
            state: *state_pubkey,
            msol_mint: state.msol_mint,
            burn_msol_from: *burn_msol_from,
            burn_msol_authority: *burn_msol_authority,
            treasury_msol_account: state.treasury_msol_account,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_withdraw_authority: StakeSystem::find_stake_withdraw_authority(state_pubkey).0,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            stake_account: *stake_account,
            split_stake_account: *split_stake_account,
            split_stake_rent_payer: *split_stake_rent_payer,
            clock: sysvar::clock::id(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::WithdrawStakeAccount {
            stake_index,
            validator_index,
            msol_amount,
            beneficiary: *beneficiary,
        }))
}

pub fn partial_unstake<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    stake_index: u32,
    validator_index: u32,
    split_stake_account: &Pubkey,
    split_stake_rent_payer: &Pubkey,
    desired_unstake_amount: u64,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::PartialUnstake {
            state: *state_pubkey,
            validator_manager_authority: state.validator_system.manager_authority,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_account: *stake_account,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            split_stake_account: *split_stake_account,
            split_stake_rent_payer: *split_stake_rent_payer,
            clock: sysvar::clock::id(),
            rent: sysvar::rent::id(),
            stake_history: sysvar::stake_history::id(),
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::PartialUnstake {
            stake_index,
            validator_index,
            desired_unstake_amount,
        }))
}

pub fn initialize<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state: &Pubkey,
    msol_mint: &Pubkey,
    operational_sol_account: &Pubkey,
    stake_list: &Pubkey,
    validator_list: &Pubkey,
    treasury_msol_account: &Pubkey,
    lp_mint: &Pubkey,
    liq_pool_msol_leg: &Pubkey,
    data: marinade_finance::instructions::InitializeData,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::Initialize {
            state: *state,
            reserve_pda: State::find_reserve_address(state).0,
            stake_list: *stake_list,
            validator_list: *validator_list,
            msol_mint: *msol_mint,
            operational_sol_account: *operational_sol_account,
            treasury_msol_account: *treasury_msol_account,
            clock: sysvar::clock::id(),
            rent: sysvar::rent::id(),
            liq_pool: marinade_finance_accounts::LiqPoolInitialize {
                lp_mint: *lp_mint,
                sol_leg_pda: LiqPool::find_sol_leg_address(state).0,
                msol_leg: *liq_pool_msol_leg,
            },
        })
        .args(marinade_finance_instruction::Initialize { data }))
}

pub fn liquid_unstake<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    get_msol_from: &Pubkey,
    get_msol_from_authority: &Pubkey,
    transfer_sol_to: &Pubkey,
    msol_amount: u64,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::LiquidUnstake {
            state: *state_pubkey,
            msol_mint: state.msol_mint,
            liq_pool_sol_leg_pda: LiqPool::find_sol_leg_address(state_pubkey).0,
            liq_pool_msol_leg: state.liq_pool.msol_leg,
            get_msol_from: *get_msol_from,
            get_msol_from_authority: *get_msol_from_authority,
            transfer_sol_to: *transfer_sol_to,
            treasury_msol_account: state.treasury_msol_account,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(marinade_finance_instruction::LiquidUnstake { msol_amount }))
}

pub fn merge_stakes<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    destination_stake: &Pubkey,
    destination_stake_index: u32,
    source_stake: &Pubkey,
    source_stake_index: u32,
    validator_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::MergeStakes {
            state: *state_pubkey,
            stake_list: *state.stake_system.stake_list_address(),
            validator_list: *state.validator_system.validator_list_address(),
            destination_stake: *destination_stake,
            source_stake: *source_stake,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            stake_withdraw_authority: StakeSystem::find_stake_withdraw_authority(state_pubkey).0,
            operational_sol_account: state.operational_sol_account,
            clock: sysvar::clock::id(),
            stake_history: sysvar::stake_history::id(),
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::MergeStakes {
            destination_stake_index,
            source_stake_index,
            validator_index,
        }))
}

pub fn redelegate<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    split_stake_account: &Pubkey,
    split_stake_rent_payer: &Pubkey,
    dest_validator_account: &Pubkey, // dest_validator_vote
    redelegate_stake_account: &Pubkey,
    stake_index: u32,
    source_validator_index: u32,
    dest_validator_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::ReDelegate {
            state: *state_pubkey,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            stake_account: *stake_account,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            split_stake_account: *split_stake_account,
            split_stake_rent_payer: *split_stake_rent_payer,
            dest_validator_account: *dest_validator_account,
            redelegate_stake_account: *redelegate_stake_account,
            clock: sysvar::clock::id(),
            stake_history: sysvar::stake_history::id(),
            stake_program: stake::program::ID,
            system_program: system_program::ID,
            stake_config: stake::config::ID,
        })
        .args(marinade_finance_instruction::Redelegate {
            stake_index,
            source_validator_index,
            dest_validator_index,
        }))
}

pub fn remove_liquidity<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    burn_from: &Pubkey,
    burn_from_authority: &Pubkey,
    transfer_sol_to: &Pubkey,
    transfer_msol_to: &Pubkey,
    tokens: u64,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::RemoveLiquidity {
            state: *state_pubkey,
            lp_mint: state.liq_pool.lp_mint,
            burn_from: *burn_from,
            burn_from_authority: *burn_from_authority, //owner acc is also token owner
            transfer_sol_to: *transfer_sol_to,
            transfer_msol_to: *transfer_msol_to,
            liq_pool_sol_leg_pda: LiqPool::find_sol_leg_address(state_pubkey).0,
            liq_pool_msol_leg: state.liq_pool.msol_leg,
            liq_pool_msol_leg_authority: LiqPool::find_msol_leg_authority(state_pubkey).0,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(marinade_finance_instruction::RemoveLiquidity { tokens }))
}

pub fn config_lp<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    min_fee: Option<Fee>,
    max_fee: Option<Fee>,
    liquidity_target: Option<u64>,
    treasury_bp_cut: Option<Fee>,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::ConfigLp {
            state: *state_pubkey,
            admin_authority: state.admin_authority,
        })
        .args(marinade_finance_instruction::ConfigLp {
            params: marinade_finance::instructions::ConfigLpParams {
                min_fee,
                max_fee,
                liquidity_target,
                treasury_cut: treasury_bp_cut,
            },
        }))
}

pub fn config_marinade<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    params: marinade_finance::instructions::ConfigMarinadeParams,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::ConfigMarinade {
            state: *state_pubkey,
            admin_authority: state.admin_authority,
        })
        .args(marinade_finance_instruction::ConfigMarinade { params }))
}

pub fn stake_reserve<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    validator_index: u32,
    validator_vote: &Pubkey,
    stake_account: &Pubkey,
    rent_payer: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::StakeReserve {
            state: *state_pubkey,
            validator_list: *state.validator_system.validator_list_address(),
            stake_list: *state.stake_system.stake_list_address(),
            validator_vote: *validator_vote,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            stake_account: *stake_account,
            stake_deposit_authority: StakeSystem::find_stake_deposit_authority(state_pubkey).0,
            rent_payer: *rent_payer,
            clock: sysvar::clock::id(),
            epoch_schedule: sysvar::epoch_schedule::ID,
            rent: sysvar::rent::id(),
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
        .args(marinade_finance_instruction::StakeReserve { validator_index }))
}

pub fn update_active<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    stake_index: u32,
    validator_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::UpdateActive {
            common: marinade_finance_accounts::UpdateCommon {
                state: *state_pubkey,
                stake_list: *state.stake_system.stake_list_address(),
                stake_account: *stake_account,
                stake_withdraw_authority: StakeSystem::find_stake_withdraw_authority(state_pubkey)
                    .0,
                reserve_pda: State::find_reserve_address(state_pubkey).0,
                msol_mint: state.msol_mint,
                clock: sysvar::clock::id(),
                stake_history: sysvar::stake_history::ID,
                msol_mint_authority: State::find_msol_mint_authority(state_pubkey).0,
                treasury_msol_account: state.treasury_msol_account,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            validator_list: *state.validator_system.validator_list_address(),
        })
        .args(marinade_finance_instruction::UpdateActive {
            stake_index,
            validator_index,
        }))
}

pub fn update_deactivated<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    stake_account: &Pubkey,
    stake_index: u32,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::UpdateDeactivated {
            common: marinade_finance_accounts::UpdateCommon {
                state: *state_pubkey,
                stake_list: *state.stake_system.stake_list_address(),
                stake_account: *stake_account,
                stake_withdraw_authority: StakeSystem::find_stake_withdraw_authority(state_pubkey)
                    .0,
                reserve_pda: State::find_reserve_address(state_pubkey).0,
                msol_mint: state.msol_mint,
                clock: sysvar::clock::id(),
                stake_history: sysvar::stake_history::ID,
                msol_mint_authority: State::find_msol_mint_authority(state_pubkey).0,
                treasury_msol_account: state.treasury_msol_account,
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
            operational_sol_account: state.operational_sol_account,
            system_program: system_program::ID,
        })
        .args(marinade_finance_instruction::UpdateDeactivated { stake_index }))
}

pub fn claim<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    ticket_account: &Pubkey,
    transfer_sol_to: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::Claim {
            state: *state_pubkey,
            reserve_pda: State::find_reserve_address(state_pubkey).0,
            ticket_account: *ticket_account,
            transfer_sol_to: *transfer_sol_to,
            system_program: system_program::ID,
            clock: sysvar::clock::ID,
        })
        .args(marinade_finance_instruction::Claim {}))
}

pub fn order_unstake<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
    burn_msol_from: &Pubkey,
    burn_msol_from_authority: &Pubkey, // delegated or owner
    msol_amount: u64,
    new_ticket_account: &Pubkey,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::OrderUnstake {
            state: *state_pubkey,
            msol_mint: state.msol_mint,
            burn_msol_from: *burn_msol_from,
            burn_msol_authority: *burn_msol_from_authority,
            new_ticket_account: *new_ticket_account,
            token_program: spl_token::ID,
            clock: sysvar::clock::ID,
            rent: sysvar::rent::ID,
        })
        .args(marinade_finance_instruction::OrderUnstake { msol_amount }))
}

pub fn emergency_pause<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::EmergencyPause {
            state: *state_pubkey,
            pause_authority: state.pause_authority,
        })
        .args(marinade_finance_instruction::Pause {}))
}

pub fn emergency_resume<'a, C: Deref<Target = impl Signer> + Clone>(
    program: &'a Program<C>,
    state_pubkey: &Pubkey,
    state: &State,
) -> anyhow::Result<RequestBuilder<'a, C>> {
    Ok(program
        .request()
        .accounts(marinade_finance_accounts::EmergencyPause {
            state: *state_pubkey,
            pause_authority: state.pause_authority,
        })
        .args(marinade_finance_instruction::Resume {}))
}
