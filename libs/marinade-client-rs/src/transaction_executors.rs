use anchor_client::RequestBuilder;
use anyhow::anyhow;
use log::{debug, error, info, warn};
use solana_client::client_error::ClientErrorKind;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_client::rpc_response::{RpcResult, RpcSimulateTransactionResult};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use std::ops::Deref;

pub trait TransactionExecutor {
    fn execute(self, commitment: CommitmentLevel) -> Result<Signature, anchor_client::ClientError>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionExecutor for RequestBuilder<'a, C> {
    fn execute(self, commitment: CommitmentLevel) -> Result<Signature, anchor_client::ClientError> {
        self.send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: false,
            preflight_commitment: Some(commitment),
            ..RpcSendTransactionConfig::default()
        })
    }
}

pub fn log_execution(execution_result: Result<Signature, anchor_client::ClientError>) {
    match execution_result {
        Ok(signature) => debug!("Transaction {}", signature),
        Err(err) => {
            error!("Transaction error: {}", err);
            match &err {
                anchor_client::ClientError::SolanaClientError(ce) => {
                    error!("Transaction error: {}", err);
                    if let ClientErrorKind::RpcError(RpcError::RpcResponseError {
                        data:
                            RpcResponseErrorData::SendTransactionPreflightFailure(
                                RpcSimulateTransactionResult {
                                    err: _,
                                    logs: Some(logs),
                                    accounts: _,
                                    return_data: _,
                                    units_consumed: _,
                                },
                            ),
                        ..
                    }) = ce.kind()
                    {
                        for log in logs {
                            error!("Log: {}", log);
                        }
                        error!("Transaction ERR {:?}", err);
                    }
                }
                _ => {
                    error!("Transaction ERR {:?}", err);
                }
            }
        }
    }
}

pub trait TransactionSimulator {
    fn simulate(self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionSimulator for RequestBuilder<'a, C> {
    fn simulate(self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult> {
        let tx = &self
            .transaction()
            .map_err(|err| RpcError::RpcRequestError(format!("Transaction error: {}", err)))?;
        rpc_client.simulate_transaction(tx)
    }
}

pub fn log_simulation(simulation_result: RpcResult<RpcSimulateTransactionResult>) {
    match simulation_result {
        Ok(result) => {
            if let Some(logs) = &result.value.logs {
                for log in logs {
                    debug!("Log: {}", log);
                }
            }
            if result.value.err.is_some() {
                error!("Transaction ERR {:?}", result);
            } else {
                info!("Transaction simulation Ok");
            }
        }
        Err(err) => {
            error!("Transaction error: {}", err);
            if let ClientErrorKind::RpcError(RpcError::RpcResponseError {
                data:
                    RpcResponseErrorData::SendTransactionPreflightFailure(
                        RpcSimulateTransactionResult {
                            err: _,
                            logs: Some(logs),
                            accounts: _,
                            units_consumed: _,
                            return_data: _,
                        },
                    ),
                ..
            }) = err.kind()
            {
                for log in logs {
                    error!("Log: {}", log);
                }
                error!("Transaction ERR {:?}", err);
            }
        }
    }
}

pub fn execute<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    simulate: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    if !simulate {
        let commitment_level = rpc_client.commitment().commitment;
        anchor_builders
            .into_iter()
            .for_each(|builder| log_execution(builder.execute(commitment_level)));
    } else {
        let mut builders_iterator = anchor_builders.into_iter();
        log_simulation(
            builders_iterator
                .next()
                .ok_or(anyhow!("No transactions to simulate"))?
                .simulate(rpc_client),
        );
        if builders_iterator.next().is_some() {
            warn!(
                "Simulation mode: only the first transaction was simulated. The rest are ignored."
            );
        }
    }
    Ok(())
}
