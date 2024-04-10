use anyhow::anyhow;
use clap::ArgMatches;
use marinade_solana_common::PubkeyOrSigner;
use log::debug;
use solana_clap_utils::input_parsers::pubkey_of_signer;
use solana_clap_utils::keypair::signer_from_path;
use solana_remote_wallet::remote_wallet::RemoteWalletManager;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::rc::Rc;
use std::{str::FromStr, sync::Arc};

// Getting signer from the matched name as the keypair path argument, or returns the default signer
pub fn signer_from_path_or_default(
    matches: &ArgMatches<'_>,
    name: &str,
    default_signer: &Arc<dyn Signer>,
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
) -> anyhow::Result<Arc<dyn Signer>> {
    if let Some(location) = matches.value_of(name) {
        Ok(Arc::from(
            signer_from_path(matches, location, name, wallet_manager)
                .map_err(|e| {
                    debug!("signer_from_path_or_default failed: location {}, keypair name: {}, matches: {:?}: {:?}",
                        location, name, matches, e);
                    anyhow!("{}: arg name: {}, location: {}", e, name, location)
                })?,
        ))
    } else {
        debug!(
            "failed to load signer {} using default signer {}",
            name,
            default_signer.pubkey()
        );
        Ok(default_signer.clone())
    }
}

/// Getting pubkey from the matched name or load it from the signer data, when not provided, return an error
pub fn pubkey_or_of_signer(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
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
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
) -> anyhow::Result<Option<Pubkey>> {
    matches.value_of(name).map_or(Ok(None), |matched_value| {
        let pubkey = Pubkey::from_str(matched_value).or_else(|e| {
            debug!("pubkey_or_of_signer_optional failed to load as pubkey {:?}, trying pubkey of signer: {:?}",
                matched_value, e);
            pubkey_of_signer(matches, name, wallet_manager)
                .map_err(|err| anyhow!("{}: {}", err, matched_value))?
                .ok_or_else(|| {
                    anyhow!(
                        "Invalid argument '{}' of value '{}' provided",
                        name,
                        matched_value
                    )
                })
        })?;
        Ok(Some(pubkey))
    })
}

/// Looking for a set of pubkeys in the matches, and return them as a vector
pub fn process_multiple_pubkeys(
    arg_matches: &ArgMatches,
    arg_name: &str,
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
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
/// argument and the value of argument first, and then it search for pubkey from the value.
fn pubkey_or_from_path(
    matches: &ArgMatches<'_>,
    name: &str,
    value_or_path: &str,
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
) -> anyhow::Result<Pubkey> {
    Pubkey::from_str(value_or_path).or_else(|e| {
        debug!(
            "pubkey_or_from_path failed to load as pubkey {:?}, trying signer: {:?}",
            value_or_path, e
        );
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
    })
}

/// Returns keypair if the parameter can be parsed as path to a file with keypair,
/// otherwise it parse it as a pubkey. Otherwise, it fails.
pub fn pubkey_or_signer(
    matches: &ArgMatches<'_>,
    name: &str,
    wallet_manager: &mut Option<Rc<RemoteWalletManager>>,
) -> anyhow::Result<Option<PubkeyOrSigner>> {
    // when the argument provides no value then returns None
    // when the argument provides a value then we parse and parsing error is returned as an error, not as None
    matches.value_of(name).map_or(Ok(None), |matched_value| {
        let parsed_signer = signer_from_path(matches, matched_value, name, wallet_manager);
        match parsed_signer {
            Ok(signer) => Ok(Some(PubkeyOrSigner::Signer(Arc::from(signer)))),
            Err(_) => {
                let parsed_pubkey = Pubkey::from_str(matched_value).map_err(|e| {
                    anyhow!(
                        "Failed to parse argument {:?}/{} as pubkey: {}",
                        matches,
                        name,
                        e
                    )
                })?;
                Ok(Some(PubkeyOrSigner::Pubkey(parsed_pubkey)))
            }
        }
    })
}

pub fn match_u16(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<u16> {
    match_u16_option(matches, name)?
        .ok_or_else(|| anyhow::Error::msg(format!("match_u16: argument '{}' missing", name)))
}

pub fn match_u16_option(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<Option<u16>> {
    if let Some(value) = matches.value_of(name) {
        let value = u16::from_str(value).map_err(|e| {
            anyhow!(
                "Failed to convert argument {} of value {} to u16: {:?}",
                name,
                value,
                e
            )
        })?;
        return Ok(Some(value));
    }
    Ok(None)
}

pub fn match_u32(matches: &ArgMatches<'_>, name: &str) -> anyhow::Result<u32> {
    match_u32_option(matches, name)?
        .ok_or_else(|| anyhow::Error::msg(format!("match_u32: argument '{}' missing", name)))
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
        .ok_or_else(|| anyhow::Error::msg(format!("match_u64: argument '{}' missing", name)))
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
        .ok_or_else(|| anyhow::Error::msg(format!("match_f64: argument '{}' missing", name)))
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
