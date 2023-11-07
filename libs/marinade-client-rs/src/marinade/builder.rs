#![allow(clippy::too_many_arguments)]
use crate::marinade::instructions::{
    add_liquidity, add_validator, change_authority, claim, config_lp, config_marinade,
    config_validator_system, deactivate_stake, deposit, deposit_stake_account, emergency_pause,
    emergency_resume, emergency_unstake, initialize, liquid_unstake, merge_stakes, order_unstake,
    partial_unstake, redelegate, remove_liquidity, remove_validator, set_validator_score,
    stake_reserve, update_active, update_deactivated, withdraw_stake_account,
};
use crate::marinade::rpc_marinade::RpcMarinade;
use crate::marinade::verifiers::{
    verify_admin_authority, verify_manager_authority, verify_pause_authority,
};
use anchor_client::RequestBuilder;
use dynsigner::PubkeyOrSigner;
use marinade_finance::instructions::{ChangeAuthorityData, ConfigMarinadeParams};
use marinade_finance::state::Fee;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::ops::Deref;
use std::sync::Arc;

pub trait MarinadeRequestBuilder<'a, C> {
    fn add_validator(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        score: u32,
        rent_payer: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn set_validator_score(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        validator_index: u32,
        score: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_validator_system(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        extra_runs: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn emergency_unstake(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn remove_validator(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn add_liquidity(
        &'a self,
        transfer_from: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn change_authority(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        params: ChangeAuthorityData,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deactivate_stake(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deposit(
        &'a self,
        transfer_from: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn deposit_stake_account(
        &'a self,
        stake_account: Pubkey,
        stake_authority: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        validator_index: u32,
        validator_vote: Pubkey,
        rent_payer: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn partial_unstake(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        desired_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn initialize(
        &'a self,
        state: &'a Arc<dyn Signer>,
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
        get_msol_from_authority: &'a PubkeyOrSigner,
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
        burn_from_authority: &'a PubkeyOrSigner,
        transfer_sol_to: Pubkey,
        transfer_msol_to: Pubkey,
        tokens: u64,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_lp(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        min_fee: Option<Fee>,
        max_fee: Option<Fee>,
        liquidity_target: Option<u64>,
        treasury_bp_cut: Option<Fee>,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn config_marinade(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        params: ConfigMarinadeParams,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn stake_reserve(
        &'a self,
        validator_index: u32,
        validator_vote: Pubkey,
        stake_account: &'a PubkeyOrSigner,
        rent_payer: &'a PubkeyOrSigner,
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
        burn_msol_from_authority: &'a PubkeyOrSigner,
        msol_amount: u64,
        ticket_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn claim(
        &'a self,
        ticket_account: Pubkey,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn emergency_pause(
        &'a self,
        pause_authority: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn emergency_resume(
        &'a self,
        pause_authority: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn redelegate(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        dest_validator_account: Pubkey, // dest_validator_vote
        redelegate_stake_account: &'a PubkeyOrSigner,
        stake_index: u32,
        source_validator_index: u32,
        dest_validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>>;

    fn withdraw_stake_account(
        &'a self,
        stake_account: Pubkey,
        burn_msol_from: Pubkey,
        burn_msol_authority: &'a PubkeyOrSigner, // delegated or owner
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        validator_index: u32,
        stake_index: u32,
        msol_amount: u64,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> MarinadeRequestBuilder<'a, C> for RpcMarinade<C> {
    fn add_validator(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        score: u32,
        rent_payer: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = add_validator(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &validator_vote,
            score,
            &rent_payer.pubkey(),
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn set_validator_score(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        validator_index: u32,
        score: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = set_validator_score(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &validator_vote,
            validator_index,
            score,
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn config_validator_system(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        extra_runs: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = config_validator_system(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            extra_runs,
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn emergency_unstake(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = emergency_unstake(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            stake_index,
            validator_index,
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn remove_validator(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        validator_vote: Pubkey,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = remove_validator(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &validator_vote,
            validator_index,
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn add_liquidity(
        &'a self,
        transfer_from: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = add_liquidity(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &transfer_from.pubkey(),
            &mint_to,
            lamports,
        )?;
        if let Some(signer) = transfer_from.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn change_authority(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        params: ChangeAuthorityData,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_admin_authority(&self.state, &admin_authority.pubkey())?;
        let mut builder =
            change_authority(&self.program, &self.instance_pubkey, &self.state, params)?;
        if let Some(signer) = admin_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn deactivate_stake(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = deactivate_stake(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            &split_stake_account.pubkey(),
            &split_stake_rent_payer.pubkey(),
            stake_index,
            validator_index,
        )?;
        if let Some(signer) = split_stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn deposit(
        &'a self,
        transfer_from: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        lamports: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = deposit(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &transfer_from.pubkey(),
            &mint_to,
            lamports,
        )?;
        if let Some(signer) = transfer_from.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn deposit_stake_account(
        &'a self,
        stake_account: Pubkey,
        stake_authority: &'a PubkeyOrSigner,
        mint_to: Pubkey,
        validator_index: u32,
        validator_vote: Pubkey,
        rent_payer: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = deposit_stake_account(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            &stake_authority.pubkey(),
            &mint_to,
            validator_index,
            &validator_vote,
            &rent_payer.pubkey(),
        )?;
        if let Some(signer) = stake_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn partial_unstake(
        &'a self,
        validator_manager_authority: &'a PubkeyOrSigner,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        desired_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_manager_authority(&self.state, &validator_manager_authority.pubkey())?;
        let mut builder = partial_unstake(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            stake_index,
            validator_index,
            &split_stake_account.pubkey(),
            &split_stake_rent_payer.pubkey(),
            desired_amount,
        )?;
        if let Some(signer) = validator_manager_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn initialize(
        &'a self,
        state: &'a Arc<dyn Signer>,
        msol_mint: Pubkey,
        operational_sol_account: Pubkey,
        stake_list: Pubkey,
        validator_list: Pubkey,
        treasury_msol_account: Pubkey,
        lp_mint: Pubkey,
        liq_pool_msol_leg: Pubkey,
        data: marinade_finance::instructions::InitializeData,
    ) -> anyhow::Result<RequestBuilder<C>> {
        Ok(initialize(
            &self.program,
            &state.pubkey(),
            &msol_mint,
            &operational_sol_account,
            &stake_list,
            &validator_list,
            &treasury_msol_account,
            &lp_mint,
            &liq_pool_msol_leg,
            data,
        )?
        .signer(state.as_ref()))
    }

    fn liquid_unstake(
        &'a self,
        get_msol_from: Pubkey,
        get_msol_from_authority: &'a PubkeyOrSigner,
        transfer_sol_to: Pubkey,
        msol_amount: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = liquid_unstake(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &get_msol_from,
            &get_msol_from_authority.pubkey(),
            &transfer_sol_to,
            msol_amount,
        )?;
        if let Some(signer) = get_msol_from_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn merge_stakes(
        &'a self,
        destination_stake: Pubkey,
        destination_stake_index: u32,
        source_stake: Pubkey,
        source_stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let builder = merge_stakes(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &destination_stake,
            destination_stake_index,
            &source_stake,
            source_stake_index,
            validator_index,
        )?;
        Ok(builder)
    }

    fn remove_liquidity(
        &'a self,
        burn_from: Pubkey,
        burn_from_authority: &'a PubkeyOrSigner,
        transfer_sol_to: Pubkey,
        transfer_msol_to: Pubkey,
        tokens: u64,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = remove_liquidity(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &burn_from,
            &burn_from_authority.pubkey(),
            &transfer_sol_to,
            &transfer_msol_to,
            tokens,
        )?;
        if let Some(signer) = burn_from_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn config_lp(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        min_fee: Option<Fee>,
        max_fee: Option<Fee>,
        liquidity_target: Option<u64>,
        treasury_bp_cut: Option<Fee>,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_admin_authority(&self.state, &admin_authority.pubkey())?;
        let mut builder = config_lp(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            min_fee,
            max_fee,
            liquidity_target,
            treasury_bp_cut,
        )?;
        if let Some(signer) = admin_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn config_marinade(
        &'a self,
        admin_authority: &'a PubkeyOrSigner,
        params: ConfigMarinadeParams,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_admin_authority(&self.state, &admin_authority.pubkey())?;
        let mut builder =
            config_marinade(&self.program, &self.instance_pubkey, &self.state, params)?;
        if let Some(signer) = admin_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn stake_reserve(
        &'a self,
        validator_index: u32,
        validator_vote: Pubkey,
        stake_account: &'a PubkeyOrSigner,
        rent_payer: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = stake_reserve(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            validator_index,
            &validator_vote,
            &stake_account.pubkey(),
            &rent_payer.pubkey(),
        )?;
        if let Some(signer) = stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn update_active(
        &'a self,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        update_active(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
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
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            stake_index,
        )
    }

    fn order_unstake(
        &'a self,
        burn_msol_from: Pubkey,
        burn_msol_from_authority: &'a PubkeyOrSigner,
        msol_amount: u64,
        ticket_account: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = order_unstake(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &burn_msol_from,
            &burn_msol_from_authority.pubkey(),
            msol_amount,
            &ticket_account,
        )?;
        if let Some(signer) = burn_msol_from_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn claim(
        &'a self,
        ticket_account: Pubkey,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        claim(
            &self.program,
            &self.instance_pubkey,
            &ticket_account,
            &beneficiary,
        )
    }

    fn emergency_pause(
        &'a self,
        pause_authority: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_pause_authority(&self.state, &pause_authority.pubkey())?;
        let mut builder = emergency_pause(&self.program, &self.instance_pubkey, &self.state)?;
        if let Some(signer) = pause_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn emergency_resume(
        &'a self,
        pause_authority: &'a PubkeyOrSigner,
    ) -> anyhow::Result<RequestBuilder<C>> {
        verify_pause_authority(&self.state, &pause_authority.pubkey())?;
        let mut builder = emergency_resume(&self.program, &self.instance_pubkey, &self.state)?;
        if let Some(signer) = pause_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn redelegate(
        &'a self,
        stake_account: Pubkey,
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        dest_validator_account: Pubkey, // dest_validator_vote
        redelegate_stake_account: &'a PubkeyOrSigner,
        stake_index: u32,
        source_validator_index: u32,
        dest_validator_index: u32,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = redelegate(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            &split_stake_account.pubkey(),
            &split_stake_rent_payer.pubkey(),
            &dest_validator_account,
            &redelegate_stake_account.pubkey(),
            stake_index,
            source_validator_index,
            dest_validator_index,
        )?;
        if let Some(signer) = split_stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = redelegate_stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }

    fn withdraw_stake_account(
        &'a self,
        stake_account: Pubkey,
        burn_msol_from: Pubkey,
        burn_msol_authority: &'a PubkeyOrSigner, // delegated or owner
        split_stake_account: &'a PubkeyOrSigner,
        split_stake_rent_payer: &'a PubkeyOrSigner,
        validator_index: u32,
        stake_index: u32,
        msol_amount: u64,
        beneficiary: Pubkey,
    ) -> anyhow::Result<RequestBuilder<C>> {
        let mut builder = withdraw_stake_account(
            &self.program,
            &self.instance_pubkey,
            &self.state,
            &stake_account,
            &burn_msol_from,
            &burn_msol_authority.pubkey(),
            &split_stake_account.pubkey(),
            &split_stake_rent_payer.pubkey(),
            validator_index,
            stake_index,
            msol_amount,
            &beneficiary,
        )?;
        if let Some(signer) = burn_msol_authority.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_account.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        if let Some(signer) = split_stake_rent_payer.use_signer() {
            builder = builder.signer(signer.as_ref());
        }
        Ok(builder)
    }
}
