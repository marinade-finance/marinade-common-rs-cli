use anchor_client::{Client, Program};
use marinade_finance::state::stake_system::StakeRecord;
use marinade_finance::state::validator_system::ValidatorRecord;
use marinade_finance::state::State;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::ops::Deref;
use crate::marinade::state::{StakeInfo, stakes_info_reversed, stakes_info, validator_list, stake_list};

pub struct RpcMarinade<C> {
    pub client: RpcClient,
    pub program: Program<C>,
    pub program_pubkey: Pubkey,
    pub instance_pubkey: Pubkey,
    pub state: State,
}

impl<C: Deref<Target = impl Signer> + Clone> RpcMarinade<C> {
    pub fn new(
        anchor_client: Client<C>,
        program_pubkey: Pubkey,
        instance_pubkey: Pubkey,
    ) -> anyhow::Result<Self> {
        let program = anchor_client.program(program_pubkey);
        let state: State = program.account(instance_pubkey)?;
        Ok(Self {
            client: program.rpc(),
            program,
            program_pubkey,
            instance_pubkey,
            state,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        self.state = self.program.account(self.instance_pubkey)?;
        Ok(())
    }

    pub fn validator_list(&self) -> anyhow::Result<(Vec<ValidatorRecord>, u32)> {
        validator_list(&self.client, &self.state)
    }

    pub fn stake_list(&self) -> anyhow::Result<(Vec<StakeRecord>, u32)> {
        stake_list(&self.client, &self.state)
    }

    pub fn stakes_info(&self) -> anyhow::Result<(Vec<StakeInfo>, u32)> {
        stakes_info(&self.client, &self.state)
    }

    pub fn stakes_info_reversed(&self) -> anyhow::Result<(Vec<StakeInfo>, u32)> {
        stakes_info_reversed(&self.client, &self.state)
    }
}
