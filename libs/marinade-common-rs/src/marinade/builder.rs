use crate::marinade::instructions::{
    add_liquidity, add_validator, change_authority, claim, config_lp, config_marinade,
    config_validator_system, deactivate_stake, deposit, deposit_stake_account, emergency_unstake,
    initialize, liquid_unstake, merge_stakes, order_unstake, partial_unstake, remove_liquidity,
    remove_validator, set_validator_score, stake_reserve, update_active, update_deactivated,
};
use crate::marinade::rpc_marinade::RpcMarinade;
use anchor_client::RequestBuilder;
use marinade_finance::instructions::{ChangeAuthorityData, ConfigMarinadeParams};
use marinade_finance::state::Fee;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::ops::Deref;
use std::sync::Arc;

/// Utilizing the TransactionBuilder from transaction_builder.rs
/// to work with marinade transactions
pub trait TransactionBuilderMarinade<'a, C> {
    fn add_validator(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        score: u32,
        rent_payer: &'a Arc<dyn Signer>,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn set_validator_score(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        validator_index: u32,
        score: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_validator_system(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        extra_runs: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn emergency_unstake(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn remove_validator(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn add_liquidity(
        &'a self,
        transfer_from: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn change_authority(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        params: ChangeAuthorityData,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deactivate_stake(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a Arc<dyn Signer>,
        split_stake_rent_payer: &'a Arc<dyn Signer>,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deposit(
        &'a self,
        transfer_from: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deposit_stake_account(
        &'a self,
        stake_account: Pubkey,
        stake_authority: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        validator_index: u32,
        validator_vote: Pubkey,
        rent_payer: &'a Arc<dyn Signer>,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn partial_unstake(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
        split_stake_account: &'a Arc<dyn Signer>,
        split_stake_rent_payer: &'a Arc<dyn Signer>,
        desired_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn initialize(
        &'a self,
        state: &'a Arc<dyn Signer>,
        creator_authority: &'a Arc<dyn Signer>,
        msol_mint: Pubkey,
        operational_sol_account: Pubkey,
        stake_list: Pubkey,
        validator_list: Pubkey,
        treasury_msol_account: Pubkey,
        lp_mint: Pubkey,
        liq_pool_msol_leg: Pubkey,
        data: marinade_finance::instructions::InitializeData,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn liquid_unstake(
        &'a self,
        get_msol_from: Pubkey,
        get_msol_from_authority: &'a Arc<dyn Signer>,
        transfer_sol_to: Pubkey,
        msol_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn merge_stakes(
        &'a self,
        destination_stake: Pubkey,
        destination_stake_index: u32,
        source_stake: Pubkey,
        source_stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn remove_liquidity(
        &'a self,
        burn_from: Pubkey,
        burn_from_authority: &'a Arc<dyn Signer>,
        transfer_sol_to: Pubkey,
        transfer_msol_to: Pubkey,
        tokens: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_lp(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        min_fee: Option<Fee>,
        max_fee: Option<Fee>,
        liquidity_target: Option<u64>,
        treasury_bp_cut: Option<Fee>,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_marinade(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        params: ConfigMarinadeParams,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn stake_reserve(
        &'a self,
        validator_index: u32,
        validator_vote: Pubkey,
        stake_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn update_active(
        &'a self,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn update_deactivated(
        &'a self,
        stake_account: Pubkey,
        stake_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn order_unstake(
        &'a self,
        burn_msol_from: Pubkey,
        burn_msol_from_authority: &'a Arc<dyn Signer>,
        msol_amount: u64,
        ticket_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn claim(
        &'a self,
        ticket_account: Pubkey,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionBuilderMarinade<'a, C>
    for RpcMarinade<C>
{
    fn add_validator(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        score: u32,
        rent_payer: &'a Arc<dyn Signer>,
    ) -> anyhow::Result<RequestBuilder<C>> {
        add_validator(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            validator_vote,
            score,
            rent_payer,
        )
    }

    fn set_validator_score(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        validator_index: u32,
        score: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        set_validator_score(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            validator_vote,
            validator_index,
            score,
        )
    }

    fn config_validator_system(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        extra_runs: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        config_validator_system(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            extra_runs,
        )
    }

    fn emergency_unstake(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        emergency_unstake(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            stake_account,
            stake_index,
            validator_index,
        )
    }

    fn remove_validator(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        validator_vote: Pubkey,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        remove_validator(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            validator_vote,
            validator_index,
        )
    }

    fn add_liquidity(
        &'a self,
        transfer_from: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        add_liquidity(
            &self.program,
            self.instance_pubkey,
            &self.state,
            transfer_from,
            mint_to,
            lamports,
        )
    }

    fn change_authority(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        params: ChangeAuthorityData,
    ) -> anyhow::Result<RequestBuilder<C>> {
        change_authority(
            &self.program,
            self.instance_pubkey,
            &self.state,
            admin_authority,
            params,
        )
    }

    fn deactivate_stake(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a Arc<dyn Signer>,
        split_stake_rent_payer: &'a Arc<dyn Signer>,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        deactivate_stake(
            &self.program,
            self.instance_pubkey,
            &self.state,
            stake_account,
            split_stake_account,
            split_stake_rent_payer,
            stake_index,
            validator_index,
        )
    }

    fn deposit(
        &'a self,
        transfer_from: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        deposit(
            &self.program,
            self.instance_pubkey,
            &self.state,
            transfer_from,
            mint_to,
            lamports,
        )
    }

    fn deposit_stake_account(
        &'a self,
        stake_account: Pubkey,
        stake_authority: &'a Arc<dyn Signer>,
        mint_to: Pubkey,
        validator_index: u32,
        validator_vote: Pubkey,
        rent_payer: &'a Arc<dyn Signer>,
    ) -> anyhow::Result<RequestBuilder<C>> {
        deposit_stake_account(
            &self.program,
            self.instance_pubkey,
            &self.state,
            stake_account,
            stake_authority,
            mint_to,
            validator_index,
            validator_vote,
            rent_payer,
        )
    }

    fn partial_unstake(
        &'a self,
        validator_manager_authority: &'a Arc<dyn Signer>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
        split_stake_account: &'a Arc<dyn Signer>,
        split_stake_rent_payer: &'a Arc<dyn Signer>,
        desired_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        partial_unstake(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_manager_authority,
            stake_account,
            stake_index,
            validator_index,
            split_stake_account,
            split_stake_rent_payer,
            desired_amount,
        )
    }

    fn initialize(
        &'a self,
        state: &'a Arc<dyn Signer>,
        creator_authority: &'a Arc<dyn Signer>,
        msol_mint: Pubkey,
        operational_sol_account: Pubkey,
        stake_list: Pubkey,
        validator_list: Pubkey,
        treasury_msol_account: Pubkey,
        lp_mint: Pubkey,
        liq_pool_msol_leg: Pubkey,
        data: marinade_finance::instructions::InitializeData,
    ) -> anyhow::Result<RequestBuilder<C>> {
        initialize(
            &self.program,
            state,
            msol_mint,
            operational_sol_account,
            stake_list,
            validator_list,
            treasury_msol_account,
            lp_mint,
            liq_pool_msol_leg,
            creator_authority,
            data,
        )
    }

    fn liquid_unstake(
        &'a self,
        get_msol_from: Pubkey,
        get_msol_from_authority: &'a Arc<dyn Signer>,
        transfer_sol_to: Pubkey,
        msol_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        liquid_unstake(
            &self.program,
            self.instance_pubkey,
            &self.state,
            get_msol_from,
            get_msol_from_authority,
            transfer_sol_to,
            msol_amount,
        )
    }

    fn merge_stakes(
        &'a self,
        destination_stake: Pubkey,
        destination_stake_index: u32,
        source_stake: Pubkey,
        source_stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        merge_stakes(
            &self.program,
            self.instance_pubkey,
            &self.state,
            destination_stake,
            destination_stake_index,
            source_stake,
            source_stake_index,
            validator_index,
        )
    }

    fn remove_liquidity(
        &'a self,
        burn_from: Pubkey,
        burn_from_authority: &'a Arc<dyn Signer>,
        transfer_sol_to: Pubkey,
        transfer_msol_to: Pubkey,
        tokens: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        remove_liquidity(
            &self.program,
            self.instance_pubkey,
            &self.state,
            burn_from,
            burn_from_authority,
            transfer_sol_to,
            transfer_msol_to,
            tokens,
        )
    }

    fn config_lp(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        min_fee: Option<Fee>,
        max_fee: Option<Fee>,
        liquidity_target: Option<u64>,
        treasury_bp_cut: Option<Fee>,
    ) -> anyhow::Result<RequestBuilder<C>> {
        config_lp(
            &self.program,
            self.instance_pubkey,
            &self.state,
            admin_authority,
            min_fee,
            max_fee,
            liquidity_target,
            treasury_bp_cut,
        )
    }

    fn config_marinade(
        &'a self,
        admin_authority: &'a Arc<dyn Signer>,
        params: ConfigMarinadeParams,
    ) -> anyhow::Result<RequestBuilder<C>> {
        config_marinade(
            &self.program,
            self.instance_pubkey,
            &self.state,
            admin_authority,
            params,
        )
    }

    fn stake_reserve(
        &'a self,
        validator_index: u32,
        validator_vote: Pubkey,
        stake_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        stake_reserve(
            &self.program,
            self.instance_pubkey,
            &self.state,
            validator_index,
            validator_vote,
            stake_account,
        )
    }

    fn update_active(
        &'a self,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        update_active(
            &self.program,
            self.instance_pubkey,
            &self.state,
            stake_account,
            stake_index,
            validator_index,
        )
    }

    fn update_deactivated(
        &'a self,
        stake_account: Pubkey,
        stake_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        update_deactivated(
            &self.program,
            self.instance_pubkey,
            &self.state,
            stake_account,
            stake_index,
        )
    }

    fn order_unstake(
        &'a self,
        burn_msol_from: Pubkey,
        burn_msol_from_authority: &'a Arc<dyn Signer>,
        msol_amount: u64,
        ticket_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        order_unstake(
            &self.program,
            self.instance_pubkey,
            &self.state,
            burn_msol_from,
            burn_msol_from_authority,
            msol_amount,
            ticket_account,
        )
    }

    fn claim(
        &'a self,
        ticket_account: Pubkey,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        claim(
            &self.program,
            self.instance_pubkey,
            ticket_account,
            beneficiary,
        )
    }
}
