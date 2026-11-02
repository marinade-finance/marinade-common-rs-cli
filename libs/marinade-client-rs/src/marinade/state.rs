use anchor_lang::AnchorDeserialize;
use anyhow::bail;
use marinade_finance::state::stake_system::StakeRecord;
use marinade_finance::state::validator_system::ValidatorRecord;
use marinade_finance::state::State;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::clock::Clock;
use solana_stake_interface::state::StakeStateV2;

pub async fn validator_list(
    rpc_client: &RpcClient,
    state: &State,
) -> anyhow::Result<(Vec<ValidatorRecord>, u32)> {
    let validator_list_account_data = rpc_client
        .get_account_data(state.validator_system.validator_list_address())
        .await?;
    let validator_record_size = state.validator_system.validator_record_size() as usize;

    Ok((
        (0..state.validator_system.validator_count())
            .map(|index| {
                let start = 8 + index as usize * validator_record_size;
                ValidatorRecord::deserialize(
                    &mut &validator_list_account_data[start..(start + validator_record_size)],
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
        state
            .validator_system
            .validator_list_capacity(validator_list_account_data.len())?,
    ))
}

pub async fn stake_list(
    rpc_client: &RpcClient,
    state: &State,
) -> anyhow::Result<(Vec<StakeRecord>, u32)> {
    let stake_list_account_data = rpc_client
        .get_account_data(state.stake_system.stake_list_address())
        .await?;
    let stake_record_size = state.stake_system.stake_record_size() as usize;
    Ok((
        (0..state.stake_system.stake_count())
            .map(|index| {
                let start = 8 + index as usize * stake_record_size;
                StakeRecord::deserialize(
                    &mut &stake_list_account_data[start..(start + stake_record_size)],
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
        state
            .stake_system
            .stake_list_capacity(stake_list_account_data.len())?,
    ))
}

/// composes a Vec<StakeInfo> from each account in stake_list
/// StakeInfo includes {index, account data, stake & current balance }
pub async fn stakes_info(
    rpc_client: &RpcClient,
    state: &State,
) -> anyhow::Result<(Vec<StakeInfo>, u32)> {
    let (stake_list, stakes_max_capacity) = stake_list(rpc_client, state).await?;

    let mut result_vec: Vec<StakeInfo> = Vec::new();

    let to_process = stake_list.len();
    let mut processed = 0;
    // rpc.get_multiple_accounts() has a max of 100 accounts
    const BATCH_SIZE: usize = 100;
    while processed < to_process {
        let accounts = rpc_client
            .get_multiple_accounts(
                &stake_list
                    .iter()
                    .map(|record| record.stake_account)
                    .skip(processed)
                    .take(BATCH_SIZE)
                    .collect::<Vec<_>>(),
            )
            .await?;
        result_vec.append(
            &mut accounts
                .into_iter()
                .enumerate()
                .map(|(index, maybe_account)| {
                    if let Some(account) = maybe_account {
                        let stake = bincode::deserialize(&account.data)?;
                        Ok(StakeInfo {
                            index: processed as u32 + index as u32,
                            record: stake_list[processed + index],
                            stake,
                            balance: account.lamports,
                        })
                    } else {
                        bail!(
                            "Can not find account {} from stake list",
                            stake_list[processed + index].stake_account
                        );
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        processed += BATCH_SIZE;
    }
    Ok((result_vec, stakes_max_capacity))
}

/// The vec is returned **reversed** meaning the last index is the first item.
/// This is because when merging or deleting an account, the account record
/// on the list on-chain is "removed". Removal is made by a "replace with last & list.count-=1"
/// so in order to not invalidate already computed indexes (not processed yet),
/// the account list must be processed from last to first, ergo, reversed.
pub async fn stakes_info_reversed(
    rpc_client: &RpcClient,
    state: &State,
) -> anyhow::Result<(Vec<StakeInfo>, u32)> {
    let (mut vec, stakes_capacity) = stakes_info(rpc_client, state).await?;
    // reverse vector (last indexes should be processed first)
    vec.reverse();
    Ok((vec, stakes_capacity))
}

pub async fn get_clock(rpc_client: &RpcClient) -> anyhow::Result<Clock> {
    Ok(bincode::deserialize(
        &rpc_client
            .get_account_data(&solana_sdk::sysvar::clock::ID)
            .await?,
    )?)
}

pub struct StakeInfo {
    pub index: u32,
    pub record: StakeRecord,
    pub stake: StakeStateV2,
    pub balance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // default #[tokio::test] is a current-thread runtime, where the blocking RpcClient panics
    #[tokio::test]
    async fn rpc_call_on_current_thread_runtime() {
        let rpc_client = RpcClient::new("http://127.0.0.1:1".to_string());
        assert!(get_clock(&rpc_client).await.is_err());
    }
}
