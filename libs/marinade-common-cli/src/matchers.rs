use anyhow::anyhow;
use clap::ArgMatches;
use solana_clap_utils::input_parsers::pubkey_of_signer;
use solana_clap_utils::keypair::signer_from_path;
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use std::{str::FromStr, sync::Arc};

// Getting signer from the matched name as the keypair path argument, or returns the default signer
pub fn signer_from_path_or_default(
    matches: &ArgMatches<'_>,
    name: &str,
    default_signer: &Arc<dyn Signer>,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Arc<dyn Signer>> {
    if let Some(location) = matches.value_of(name) {
        Ok(Arc::from(
            signer_from_path(matches, location, name, wallet_manager)
                .map_err(|err| anyhow!("{}: {}", err, location))?,
        ))
    } else {
        Ok(default_signer.clone())
    }
}

/// Getting pubkey from the matched name or load it from the signer data
pub fn pubkey_or_of_signer(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
) -> anyhow::Result<Pubkey> {
    if let Some(matched_value) = matches.value_of(name) {
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
        Ok(pubkey)
    } else {
        Err(anyhow!("Value for argument '{}' was not provided", name))
    }
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
