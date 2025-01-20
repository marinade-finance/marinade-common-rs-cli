use anyhow::bail;
use marinade_finance::State;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;

pub fn verify_manager_authority(
    state: &State,
    validator_manager_authority: &Pubkey,
) -> anyhow::Result<()> {
    if state.validator_system.manager_authority != *validator_manager_authority {
        bail!("verify_manager_authority: validator-manager-authority {} to sign the transaction mismatches Marinade state system manager authority {}",
                validator_manager_authority,
                state.validator_system.manager_authority
            );
    }
    Ok(())
}

pub fn verify_admin_authority(state: &State, admin_authority: &Pubkey) -> anyhow::Result<()> {
    if state.admin_authority != *admin_authority {
        bail!("verify_admin_authority: admin-authority {} signing the transaction mismatches Marinade state admin authority: {}",
                admin_authority,
                state.admin_authority
            );
    }
    Ok(())
}

pub fn verify_rent_payer(rpc_client: &RpcClient, rent_payer: &Pubkey) -> anyhow::Result<()> {
    let rent_account = rpc_client.get_account(rent_payer)?;
    if rent_account.owner != system_program::ID {
        bail!(
            "verify_rent_payer: provided rent payer {} address must be a system account",
            rent_payer
        )
    }
    Ok(())
}

pub fn verify_pause_authority(state: &State, pause_authority: &Pubkey) -> anyhow::Result<()> {
    if &state.pause_authority != pause_authority {
        bail!("verify_pause_authority: pause-authority {} to sign the transaction mismatches Marinade state pause authority {}",
                pause_authority,
                state.pause_authority
            );
    }
    Ok(())
}
