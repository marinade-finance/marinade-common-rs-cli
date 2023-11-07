use anyhow::anyhow;
use clap::ArgMatches;
use log::debug;
use solana_clap_utils::input_parsers::pubkey_of_signer;
use solana_clap_utils::keypair::signer_from_path;
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::{str::FromStr, sync::Arc};

/// Keypair or Pubkey depending, could be one of that
/// based on parameters of the CLI command. For --print-only
/// we want to permit to pass only pubkey not the keypair in real.
#[derive(Debug)]
pub struct PubkeyOrSigner {
    pubkey: Option<Pubkey>,
    signer: Option<Arc<dyn Signer>>,
}

impl PubkeyOrSigner {
    pub fn new_as_pubkey(pubkey: Pubkey) -> Self {
        Self {
            signer: None,
            pubkey: Some(pubkey),
        }
    }

    pub fn new_as_signer(signer: Arc<dyn Signer>) -> Self {
        Self {
            signer: Some(signer),
            pubkey: None,
        }
    }

    pub fn is_signer(&self) -> bool {
        self.signer.is_some()
    }

    pub fn signer(&self) -> Option<Arc<dyn Signer>> {
        self.signer.clone()
    }

    pub fn pubkey(&self) -> Pubkey {
        if let Some(signer) = &self.signer {
            return signer.pubkey();
        } else if let Some(pubkey) = &self.pubkey {
            return pubkey.clone();
        } else {
            panic!("PubkeyOrSigner is not initialized");
        }
    }
}

// Getting signer from the matched name as the keypair path argument, or returns the default signer
pub fn signer_from_path_or_default(
    matches: &ArgMatches<'_>,
    name: &str,
    default_signer: &Arc<dyn Signer>,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Arc<dyn Signer>> {
    match signer_from_path_or_none(matches, name, wallet_manager)? {
        Some(signer) => Ok(signer),
        None => {
            debug!(
                "failed to load signer {} using default signer {}",
                name,
                default_signer.pubkey()
            );
            Ok(default_signer.clone())
        }
    }
}

// Getting signer from the matched name as the keypair path argument, when not found returns None
pub fn signer_from_path_or_none(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Option<Arc<dyn Signer>>> {
    matches.value_of(name).map_or(Ok(None), |matched_value| {
        Ok(Some(Arc::from(
            signer_from_path(matches, matched_value, name, wallet_manager).map_err(|err| {
                anyhow!(
                    "Failed to load signer from path of parameter {}/{}: {}",
                    name,
                    matched_value,
                    err
                )
            })?,
        )))
    })
}

/// Getting pubkey from the matched name or load it from the signer data, when not provided, return an error
pub fn pubkey_or_of_signer(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Pubkey> {
    pubkey_or_of_signer_optional(matches, name, wallet_manager)
        .map(|pubkey| {
            pubkey.ok_or_else(|| anyhow!("Value for argument '{}' was not provided", name))
        })
        .unwrap_or_else(Err)
}

/// Getting pubkey from the matched name or load it from the signer data, when not provided, option None is returned
pub fn pubkey_or_of_signer_optional(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Option<Pubkey>> {
    matches.value_of(name).map_or(Ok(None), |matched_value| {
        let pubkey = if let Ok(pubkey) = Pubkey::from_str(matched_value) {
            pubkey
        } else {
            pubkey_of_signer(matches, name, wallet_manager)
                .map_err(|err| anyhow!("{}: {}", err, matched_value))?
                .ok_or_else(|| {
                    anyhow!(
                        "Invalid argument '{}' of value '{}' provided",
                        name,
                        matched_value
                    )
                })?
        };
        Ok(Some(pubkey))
    })
}

/// Looking for a set of pubkeys in the matches, and return them as a vector
pub fn process_multiple_pubkeys(
    arg_matches: &ArgMatches,
    arg_name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Vec<Pubkey>> {
    let mut value_pubkeys: Vec<Pubkey> = vec![];
    if let Some(values) = arg_matches.values_of(arg_name) {
        for (i, value) in values.enumerate() {
            let name = format!("{}-{}", arg_name, i.saturating_add(1));
            let value_pubkey = pubkey_or_from_path(arg_matches, &name, value, wallet_manager)?;
            value_pubkeys.push(value_pubkey);
        }
    }
    Ok(value_pubkeys)
}

/// Difference between this and 'pubkey_or_from_signer' method is that this method takes just the `value_or_path`
/// parameter and tries to find it as a pubkey. On the other hand the `pubkey_or_from_signer` matches the name of
/// argument and the value of argument first and then it search for pubkey from the value.
fn pubkey_or_from_path(
    matches: &ArgMatches<'_>,
    name: &str,
    value_or_path: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Pubkey> {
    if let Ok(pubkey) = Pubkey::from_str(value_or_path) {
        Ok(pubkey)
    } else {
        let signer =
            signer_from_path(matches, value_or_path, name, wallet_manager).map_err(|err| {
                anyhow!(
                    "Invalid argument name: {}, value_or_path: {}, err: {}",
                    name,
                    value_or_path,
                    err
                )
            })?;
        Ok(signer.pubkey())
    }
}

/// Returns keypair if the parameter can be parsed as path to a file with keypair,
/// otherwise it parse it as a pubkey. Otherwise it fails.
pub fn pubkey_or_signer(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<PubkeyOrSigner> {
    if let Ok(Some(signer)) = signer_from_path_or_none(matches, name, wallet_manager) {
        Ok(PubkeyOrSigner::new_as_signer(signer))
    } else {
        Ok(PubkeyOrSigner::new_as_pubkey(pubkey_or_of_signer(
            matches,
            name,
            wallet_manager,
        )?))
    }
}

pub fn match_u32(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<u32> {
    match_u32_option(matches, name)?
        .ok_or_else(|| anyhow::Error::msg(format!("argument '{}' missing", name)))
}

pub fn match_u32_option(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<Option<u32>> {
    if let Some(value) = matches.value_of(name) {
        let value = u32::from_str(value).map_err(|e| {
            anyhow!(
                "Failed to convert argument {} of value {} to u32: {:?}",
                name,
                value,
                e
            )
        })?;
        return Ok(Some(value));
    }
    Ok(None)
}

pub fn match_u64(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<u64> {
    match_u64_option(matches, name)?
        .ok_or_else(|| anyhow::Error::msg(format!("argument '{}' missing", name)))
}

pub fn match_u64_option(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<Option<u64>> {
    if let Some(value) = matches.value_of(name) {
        let value = u64::from_str(value).map_err(|e| {
            anyhow!(
                "Failed to convert argument {} of value {} to u64: {:?}",
                name,
                value,
                e
            )
        })?;
        return Ok(Some(value));
    }
    Ok(None)
}

pub fn match_f64(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<f64> {
    match_f64_option(matches, name)?
        .ok_or_else(|| anyhow::Error::msg(format!("argument '{}' missing", name)))
}

pub fn match_f64_option(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<Option<f64>> {
    if let Some(value) = matches.value_of(name) {
        let value = f64::from_str(value).map_err(|e| {
            anyhow!(
                "Failed to convert argument {} of value {} to f64: {:?}",
                name,
                value,
                e
            )
        })?;
        return Ok(Some(value));
    }
    Ok(None)
}
