use crate::transaction_instruction::{TransactionAccount, TransactionInstruction};
use anchor_client::RequestBuilder;
use anyhow::bail;
use borsh::BorshSerialize;
use log::{debug, error, info, warn};
use solana_client::client_error::ClientErrorKind;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcSendTransactionConfig};
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_client::rpc_response::{RpcResult, RpcSimulateTransactionResult};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::Signer;
use spl_token::solana_program::instruction::Instruction;
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

pub fn log_execution(
    execution_result: &Result<Signature, anchor_client::ClientError>,
) -> anyhow::Result<()> {
    match execution_result {
        Ok(signature) => debug!("Transaction {}", signature),
        Err(err) => {
            error!("Transaction error: {}", err);
            if let anchor_client::ClientError::SolanaClientError(ce) = &err {
                error!("Solana client error: {}", ce);
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
                }
            }
            bail!("Transaction error: {}", err);
        }
    }
    Ok(())
}

pub trait TransactionSimulator {
    fn simulate(&self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult>;
}

impl<'a, C: Deref<Target = impl Signer> + Clone> TransactionSimulator for RequestBuilder<'a, C> {
    fn simulate(&self, rpc_client: &RpcClient) -> RpcResult<RpcSimulateTransactionResult> {
        let mut tx = self
            .transaction()
            .map_err(|err| RpcError::RpcRequestError(format!("Transaction error: {}", err)))?;
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

pub fn print_base64(instructions: &Vec<Instruction>) -> anyhow::Result<()> {
    for instruction in instructions {
        let transaction_instruction = TransactionInstruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(TransactionAccount::from)
                .collect(),
            data: instruction.data.clone(),
        };
        println!("base64 instruction of program {}:", instruction.program_id);
        println!(
            " {}",
            anchor_lang::__private::base64::encode(transaction_instruction.try_to_vec()?)
        );
    }
    Ok(())
}

pub fn execute<'a, I, C>(
    anchor_builders: I,
    rpc_client: &RpcClient,
    simulate: bool,
    print_only: bool,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = RequestBuilder<'a, C>>,
    C: Deref<Target = dynsigner::DynSigner> + Clone,
{
    warn_text_simulate_print_only(simulate, print_only);

    if simulate {
        let mut count = 0u32;
        for builder in anchor_builders {
            if print_only {
                print_base64(&builder.instructions()?)?;
                continue;
            }
            log_simulation(&builder.simulate(rpc_client))?;
            count += 1;
        }
        if count > 1 {
            warn!(
                "Simulation mode: only the first transaction was simulated. The rest are ignored."
            );
        }
    } else {
        // execute or print_only
        let commitment_level = rpc_client.commitment().commitment;
        anchor_builders.into_iter().try_for_each(|builder| {
            if print_only {
                print_base64(&builder.instructions()?)
            } else {
                log_execution(&builder.execute(commitment_level))
            }
        })?;
    }

    Ok(())
}

pub fn execute_single<C: Deref<Target = dynsigner::DynSigner> + Clone>(
    anchor_builder: RequestBuilder<C>,
    rpc_client: &RpcClient,
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
        let commitment_level = rpc_client.commitment().commitment;
        log_execution(&anchor_builder.execute(commitment_level))?;
    }

    Ok(())
}

fn warn_text_simulate_print_only(simulate: bool, print_only: bool) {
    if simulate {
        warn!("Simulation mode: transactions will not be executed, only simulated.");
    }
    if print_only {
        warn!("Print only mode: transactions will only be printed in base64 format.");
    }
}
