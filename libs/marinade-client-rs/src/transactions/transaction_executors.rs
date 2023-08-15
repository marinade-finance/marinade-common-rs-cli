use crate::transactions::prepared_transaction::PreparedTransaction;
use crate::transactions::transaction_builder::TransactionBuilder;
use crate::transactions::transaction_instruction::print_base64;
use anchor_client::RequestBuilder;
use anyhow::bail;
use log::{debug, error, info, warn};
use solana_client::client_error::ClientError as SolanaClientError;
use solana_client::client_error::ClientErrorKind;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_client::rpc_response::{RpcResult, RpcSimulateTransactionResult};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::Signer;
use std::ops::Deref;

pub fn log_execution(
    execution_result: &anyhow::Result<Signature, anchor_client::ClientError>,
) -> anyhow::Result<()> {
    match execution_result {
        Ok(signature) => info!("Transaction {}", signature),
        Err(err) => {
            if let anchor_client::ClientError::SolanaClientError(ce) = &err {
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
                    error!("Solana client error: {}", ce);
                    for log in logs {
                        error!("Log: {}", log);
                    }
                }
            }
            bail!("Transaction error: {:?}", err);
        }
    }
    Ok(())
}

pub trait TransactionSimulator {
    fn simulate(&self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionSimulator for RequestBuilder<'a, C> {
    fn simulate(&self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult> {
        let mut tx = self.transaction().map_err(|err| {
            error!("Cannot build transactions from builder: {:?}", err);
            RpcError::ForUser(format!("Request builder transaction error: {}", err))
        })?;
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        tx.partial_sign::<Vec<&Keypair>>(&vec![], recent_blockhash);
        rpc_client.simulate_transaction(&tx)
    }
}

pub fn log_simulation(
    simulation_result: &RpcResult<RpcSimulateTransactionResult>,
) -> anyhow::Result<()> {
    match simulation_result {
        Ok(result) => {
            if let Some(logs) = &result.value.logs {
                for log in logs {
                    debug!("Log: {}", log);
                }
            }
            if result.value.err.is_some() {
                error!("Transaction ERR {:?}", result);
                bail!("Transaction error: {}", result.value.err.as_ref().unwrap());
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
            bail!("Transaction error: {}", err);
        }
    }
    Ok(())
}

pub fn execute_with_config<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    warn_text_simulate_print_only(simulate, print_only);

    if simulate {
        for builder in anchor_builders {
            if print_only {
                print_base64(&builder.instructions()?)?;
            }
            log_simulation(&builder.simulate(rpc_client))?;
        }
    } else {
        // execute or print_only
        anchor_builders.into_iter().try_for_each(|builder| {
            if print_only {
                print_base64(&builder.instructions()?)
            } else {
                log_execution(&builder.send_with_spinner_and_config(preflight_config))
            }
        })?;
    }

    Ok(())
}

pub fn execute<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    skip_preflight: bool,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    execute_with_config(
        anchor_builders,
        rpc_client,
        RpcSendTransactionConfig {
            skip_preflight,
            ..RpcSendTransactionConfig::default()
        },
        simulate,
        print_only,
    )
}

pub fn execute_single_with_config<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()> {
    warn_text_simulate_print_only(simulate, print_only);

    if print_only {
        print_base64(&anchor_builder.instructions()?)?;
    }

    if simulate {
        log_simulation(&anchor_builder.simulate(rpc_client))?;
    } else if !print_only {
        // !simulate && !print_only
        log_execution(&anchor_builder.send_with_spinner_and_config(preflight_config))?;
    }

    Ok(())
}

pub fn execute_single<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
    skip_preflight: bool,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()> {
    execute_single_with_config(
        anchor_builder,
        rpc_client,
        RpcSendTransactionConfig {
            skip_preflight,
            ..RpcSendTransactionConfig::default()
        },
        simulate,
        print_only,
    )
}

pub fn execute_single_tx_with_config<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()> {
    warn_text_simulate_print_only(simulate, print_only);

    if print_only {
        print_base64(&anchor_builder.instructions()?)?;
    }

    if simulate {
        log_simulation(&anchor_builder.simulate(rpc_client))?;
    } else if !print_only {
        // !simulate && !print_only
        log_execution(&anchor_builder.send_with_spinner_and_config(preflight_config))?;
    }

    Ok(())
}

pub fn execute_transaction_builder(
    transaction_builder: &mut TransactionBuilder,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    blockhash_commitment: CommitmentLevel,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()> {
    warn_text_simulate_print_only(simulate, print_only);

    if print_only {
        print_base64(&transaction_builder.instructions())?;
    }

    if simulate {
        for mut prepared_transaction in transaction_builder.sequence_combined() {
            let simulation_config_default = RpcSimulateTransactionConfig::default();
            let simulation_commitment = if preflight_config.preflight_commitment.is_some() {
                Some(CommitmentConfig {
                    commitment: preflight_config.preflight_commitment.unwrap(),
                })
            } else {
                simulation_config_default.commitment
            };
            let simulation_result = simulate_prepared_transaction(
                &mut prepared_transaction,
                rpc_client,
                RpcSimulateTransactionConfig {
                    sig_verify: true,
                    commitment: simulation_commitment,
                    encoding: preflight_config.encoding,
                    min_context_slot: preflight_config.min_context_slot,
                    ..simulation_config_default
                },
                blockhash_commitment,
            );
            log_simulation(&simulation_result)?;
        }
    } else {
        for mut prepared_transaction in transaction_builder.sequence_combined() {
            let execution_result = execute_prepared_transaction(
                &mut prepared_transaction,
                rpc_client,
                preflight_config,
                blockhash_commitment,
            );
            log_execution(&execution_result)?;
        }
    }

    Ok(())
}

pub fn execute_prepared_transaction(
    prepared_transaction: &mut PreparedTransaction,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    blockhash_commitment: CommitmentLevel,
) -> Result<Signature, anchor_client::ClientError> {
    let rpc_client_blockhash = RpcClient::new_with_commitment(
        rpc_client.url(),
        CommitmentConfig {
            commitment: blockhash_commitment,
        },
    );
    let latest_hash = rpc_client_blockhash.get_latest_blockhash()?;
    let tx = prepared_transaction.sign(latest_hash).map_err(|e| {
        error!(
            "execute_prepared_transaction: error signing transaction with blockhash: {}: {:?}",
            latest_hash, e
        );
        anchor_client::ClientError::SolanaClientError(SolanaClientError::from(e))
    })?;

    rpc_client
        .send_and_confirm_transaction_with_spinner_and_config(
            tx,
            rpc_client.commitment(),
            preflight_config,
        )
        .map_err(|e|{
            error!("execute_prepared_transaction: error send_and_confirm transaction '{:?}', signers: '{:?}': {:?}",
                prepared_transaction.transaction, prepared_transaction.signers.iter().map(|s| s.pubkey()), e);
            e.into()
        })
}

pub fn simulate_prepared_transaction(
    prepared_transaction: &mut PreparedTransaction,
    rpc_client: &RpcClient,
    simulate_config: RpcSimulateTransactionConfig,
    blockhash_commitment: CommitmentLevel,
) -> RpcResult<RpcSimulateTransactionResult> {
    let rpc_client_blockhash = RpcClient::new_with_commitment(
        rpc_client.url(),
        CommitmentConfig {
            commitment: blockhash_commitment,
        },
    );
    let latest_hash = rpc_client_blockhash.get_latest_blockhash()?;
    let tx = prepared_transaction.sign(latest_hash).map_err(|e| {
        error!(
            "simulate_prepared_transaction: error signing transaction with blockhash: {}: {:?}",
            latest_hash, e
        );
        RpcError::ForUser(format!("Signature error: {}", e))
    })?;

    rpc_client.simulate_transaction_with_config(tx, simulate_config)
}

fn warn_text_simulate_print_only(simulate: bool, print_only: bool) {
    if simulate {
        warn!("Simulation mode: transactions will not be executed, only simulated.");
    }
    if print_only {
        warn!("Print only mode: transactions will be printed in base64 format.");
    }
}
