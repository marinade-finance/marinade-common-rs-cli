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
use solana_client::rpc_request::RpcError::ForUser;
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_client::rpc_response::{RpcResult, RpcSimulateTransactionResult};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::TransactionError;
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
    fn simulate(
        &self,
        rpc_client: &RpcClient,
        sig_verify: bool,
    ) -> RpcResult<RpcSimulateTransactionResult>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionSimulator for RequestBuilder<'a, C> {
    fn simulate(
        &self,
        rpc_client: &RpcClient,
        sig_verify: bool,
    ) -> RpcResult<RpcSimulateTransactionResult> {
        let tx = self.signed_transaction().map_err(|err| {
            error!(
                "RequestBuilder#simulate: cannot build transactions from builder: {:?}",
                err
            );
            ForUser(format!("Building transaction error: {}", err))
        })?;
        rpc_client.simulate_transaction_with_config(
            &tx,
            RpcSimulateTransactionConfig {
                sig_verify,
                ..RpcSimulateTransactionConfig::default()
            },
        )
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

pub fn execute_anchor_builders_with_config<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    simulate: bool,
    print: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    warn_text_simulate_print(simulate, print);

    if simulate {
        for builder in anchor_builders {
            if print {
                print_base64(&builder.instructions()?)?;
            }
            log_simulation(&builder.simulate(rpc_client, !print))?;
        }
    } else {
        anchor_builders.into_iter().try_for_each(|builder| {
            if print {
                print_base64(&builder.instructions()?)?;
            }
            log_execution(&builder.send_with_spinner_and_config(preflight_config))
        })?;
    }

    Ok(())
}

pub fn execute_anchor_builders<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    skip_preflight: bool,
    simulate: bool,
    print: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    execute_anchor_builders_with_config(
        anchor_builders,
        rpc_client,
        RpcSendTransactionConfig {
            skip_preflight,
            ..RpcSendTransactionConfig::default()
        },
        simulate,
        print,
    )
}

pub fn execute_anchor_builder_with_config<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    simulate: bool,
    print: bool,
) -> anyhow::Result<()> {
    execute_anchor_builders_with_config(
        std::iter::once(anchor_builder),
        rpc_client,
        preflight_config,
        simulate,
        print,
    )
}

pub fn execute_anchor_builder<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
    skip_preflight: bool,
    simulate: bool,
    print: bool,
) -> anyhow::Result<()> {
    execute_anchor_builders(
        std::iter::once(anchor_builder),
        rpc_client,
        skip_preflight,
        simulate,
        print,
    )
}

pub fn execute_transaction_builder(
    transaction_builder: &mut TransactionBuilder,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    blockhash_commitment: CommitmentLevel,
    simulate: bool,
    print: bool,
    blockhash_failure_retries: Option<u16>,
) -> anyhow::Result<()> {
    warn_text_simulate_print(simulate, print);

    if print {
        print_base64(&transaction_builder.instructions())?;
    }

    if simulate {
        // expecting the instructions are dependent one to each other
        // the result of the first can be used in the next one, for that simulation is run only for the fist bunch
        let mut number_of_transactions = 0_u32;
        let is_checked_signers = transaction_builder.is_check_signers();
        for mut prepared_transaction in transaction_builder.sequence_combined() {
            number_of_transactions += 1;
            if number_of_transactions > 1 {
                // only the first bunch is simulated
                // need to drain whole sequence to find the number of transaction bunches
                continue;
            }
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
                    sig_verify: !print && is_checked_signers,
                    commitment: simulation_commitment,
                    encoding: preflight_config.encoding,
                    min_context_slot: preflight_config.min_context_slot,
                    ..simulation_config_default
                },
                blockhash_commitment,
            );
            log_simulation(&simulation_result)?;
        }
        if number_of_transactions > 1 {
            warn!("Simulation mode: only the first bunch of transactions was simulated, the rest was not simulated.");
        }
    } else {
        for mut prepared_transaction in transaction_builder.sequence_combined() {
            let execution_result = execute_prepared_transaction_blockhash_retry(
                &mut prepared_transaction,
                rpc_client,
                preflight_config,
                blockhash_commitment,
                blockhash_failure_retries,
            );
            log_execution(&execution_result)?;
        }
    }

    Ok(())
}

fn execute_prepared_transaction_internal(
    prepared_transaction: &mut PreparedTransaction,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
) -> Result<Signature, solana_client::client_error::ClientError> {
    let latest_hash = rpc_client.get_latest_blockhash()?;
    let tx = prepared_transaction.sign(latest_hash).map_err(|e| {
        error!(
            "execute_prepared_transaction: error signing transaction with blockhash: {}: {:?}",
            latest_hash, e
        );
        SolanaClientError::from(e)
    })?;

    rpc_client.send_and_confirm_transaction_with_spinner_and_config(
        tx,
        rpc_client.commitment(),
        preflight_config,
    )
}

fn execute_prepared_transaction_retry_blockhash_internal(
    prepared_transaction: &mut PreparedTransaction,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    blockhash_failure_retries: Option<u16>,
) -> Result<Signature, anchor_client::ClientError> {
    let mut retry_count: u16 = 0;
    let blockhash_failure_retries = blockhash_failure_retries.unwrap_or(0);
    let mut last_error = anchor_client::ClientError::SolanaClientError(SolanaClientError::from(
        RpcError::RpcRequestError("send_transaction: unknown retry failure".to_string()),
    ));
    while retry_count <= blockhash_failure_retries {
        let send_result = execute_prepared_transaction_internal(
            prepared_transaction,
            rpc_client,
            preflight_config,
        );
        match send_result {
            Ok(signature) => {
                return Ok(signature);
            }
            Err(err) => {
                last_error =
                    anchor_client::ClientError::SolanaClientError(SolanaClientError::from(err));
                if let anchor_client::ClientError::SolanaClientError(ce) = &last_error {
                    let to_check_err: Option<&TransactionError> = match ce.kind() {
                        ClientErrorKind::RpcError(RpcError::RpcResponseError {
                            data:
                                RpcResponseErrorData::SendTransactionPreflightFailure(
                                    RpcSimulateTransactionResult {
                                        err: transaction_error,
                                        logs,
                                        accounts,
                                        ..
                                    },
                                ),
                            ..
                        }) => {
                            debug!(
                                "Failed to send transaction: {:?}, logs: {:?}, accounts: {:?}",
                                transaction_error, logs, accounts
                            );
                            transaction_error.as_ref()
                        }
                        ClientErrorKind::RpcError(ForUser(message)) => {
                            // unable to confirm transaction. This can happen in situations such as transaction expiration and insufficient fee-payer funds
                            if message
                                .to_lowercase()
                                .contains("unable to confirm transaction")
                            {
                                Some(&TransactionError::BlockhashNotFound)
                            } else {
                                None
                            }
                        }
                        ClientErrorKind::TransactionError(te) => Some(te),
                        _ => None,
                    };

                    if let Some(tx_err) = to_check_err {
                        if *tx_err == TransactionError::BlockhashNotFound {
                            debug!(
                                "Retried attempt #{}/{} to send transaction with error: {:?} ",
                                retry_count, blockhash_failure_retries, tx_err
                            );
                            // retry
                            retry_count += 1;
                            continue;
                        }
                    }
                    // No Error to retry, let's break the loop and use the last error
                    break;
                }
            }
        }
    }
    error!("Transaction ERR send_transaction: {:?}", last_error);
    Err(last_error)
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
    execute_prepared_transaction_internal(
        prepared_transaction,
        &rpc_client_blockhash,
        preflight_config,
    ).map_err(|e|{
        error!("execute_prepared_transaction: error send_and_confirm transaction '{:?}', signers: '{:?}': {:?}",
                prepared_transaction.transaction, prepared_transaction.signers.iter().map(|s| s.pubkey()), e);
        e.into()
    })
}

pub fn execute_prepared_transaction_blockhash_retry(
    prepared_transaction: &mut PreparedTransaction,
    rpc_client: &RpcClient,
    preflight_config: RpcSendTransactionConfig,
    blockhash_commitment: CommitmentLevel,
    blockhash_failure_retries: Option<u16>,
) -> Result<Signature, anchor_client::ClientError> {
    let rpc_client_blockhash = RpcClient::new_with_commitment(
        rpc_client.url(),
        CommitmentConfig {
            commitment: blockhash_commitment,
        },
    );
    execute_prepared_transaction_retry_blockhash_internal(
        prepared_transaction,
        &rpc_client_blockhash,
        preflight_config,
        blockhash_failure_retries,
    )
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
    let latest_blockhash = rpc_client_blockhash.get_latest_blockhash()?;
    let tx = if simulate_config.sig_verify {
        prepared_transaction.sign(latest_blockhash).map_err(|e| {
            error!(
                "simulate_prepared_transaction: error signing transaction with blockhash: {}: {:?}",
                latest_blockhash, e
            );
            ForUser(format!("Signing transaction error: {}", e))
        })?
    } else {
        prepared_transaction.partial_sign(latest_blockhash)
    };

    rpc_client.simulate_transaction_with_config(tx, simulate_config)
}

fn warn_text_simulate_print(simulate: bool, print: bool) {
    if simulate {
        warn!("Simulation mode: transactions will not be executed, only simulated.");
    }
    if print {
        warn!("Print mode: transactions will also be printed in base64 format.");
    }
}
