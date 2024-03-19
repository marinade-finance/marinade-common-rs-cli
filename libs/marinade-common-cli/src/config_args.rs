use clap::Arg;
use solana_clap_utils::input_validators::is_url_or_moniker;
use solana_clap_utils::{input_validators, ArgConstant};

pub const CONFIG_FILE_ARG: ArgConstant<'static> = ArgConstant {
    name: "config_file",
    long: "config-file",
    help: "Configuration file to use [default: ~/.config/solana/cli/config.yml]",
};
pub fn config_file_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(CONFIG_FILE_ARG.name)
        .long(CONFIG_FILE_ARG.long)
        .short("c")
        .value_name("PATH")
        .takes_value(true)
        .help(CONFIG_FILE_ARG.help)
}

pub const VERBOSE_ARG: ArgConstant<'static> = ArgConstant {
    name: "verbose",
    long: "verbose",
    help: "Show additional information",
};
pub fn verbose_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(VERBOSE_ARG.name)
        .long(VERBOSE_ARG.long)
        .short("v")
        .takes_value(false)
        .help(VERBOSE_ARG.help)
}

pub const SIMULATE_ARG: ArgConstant<'static> = ArgConstant {
    name: "simulate",
    long: "simulate",
    help: "Transactions are not executed against the cluster, only simulation is executed.",
};
pub fn simulate_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(SIMULATE_ARG.name)
        .long(SIMULATE_ARG.long)
        .short("s")
        .takes_value(false)
        .help(SIMULATE_ARG.help)
}

pub const SKIP_PREFLIGHT_ARG: ArgConstant<'static> = ArgConstant {
    name: "skip_preflight",
    long: "skip-preflight",
    help:
        "Skip transactions simulation at RPC node before sending to cluster (by default simulated).",
};
pub fn skip_preflight_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(SKIP_PREFLIGHT_ARG.name)
        .long(SKIP_PREFLIGHT_ARG.long)
        .takes_value(false)
        .help(SKIP_PREFLIGHT_ARG.help)
}

pub const RPC_URL_ARG: ArgConstant<'static> = ArgConstant {
    name: "rpc_url",
    long: "url",
    help: "URL for Solana's JSON RPC or moniker (or their first letter): \
           [mainnet-beta, testnet, devnet, localhost] \
           Default from the --config-file.",
};
pub fn rpc_url_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(RPC_URL_ARG.name)
        .long(RPC_URL_ARG.long)
        .short("u")
        .value_name("URL_OR_MONIKER")
        .takes_value(true)
        .validator(is_url_or_moniker)
        .help(RPC_URL_ARG.help)
}

pub const RENT_PAYER_ARG: ArgConstant<'static> = ArgConstant {
    name: "rent_payer",
    long: "rent-payer",
    help: "Specify the rent-payer signer. When not provided, the fee-payer is used.",
};
pub fn rent_payer_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(RENT_PAYER_ARG.name)
        .long(RENT_PAYER_ARG.long)
        .short("r")
        .value_name("KEYPAIR")
        .env("RENT_PAYER")
        .takes_value(true)
        .validator(input_validators::is_valid_signer)
        .help(RENT_PAYER_ARG.help)
}

pub const VALIDATOR_MANAGER_ARG: ArgConstant<'static> = ArgConstant {
    name: "validator_manager_authority",
    long: "validator-manager-authority",
    help:
        "Specify the validator manager authority signer. When not provided, the fee-payer is used.",
};
pub fn validator_manager_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(VALIDATOR_MANAGER_ARG.name)
        .long(VALIDATOR_MANAGER_ARG.long)
        .short("m")
        .value_name("KEYPAIR")
        .takes_value(true)
        .validator(input_validators::is_valid_signer)
        .help(VALIDATOR_MANAGER_ARG.help)
}

pub const PROGRAM_ARG: ArgConstant<'static> = ArgConstant {
    name: "program",
    long: "program",
    help: "Marinade Liquid Staking Program id.",
};
pub fn program_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(PROGRAM_ARG.name)
        .long(PROGRAM_ARG.long)
        .short("p")
        .value_name("MARINADE_PROGRAM")
        .takes_value(true)
        .env("MARINADE_PROGRAM")
        .default_value("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD")
        .help(INSTANCE_ARG.help)
}

pub const INSTANCE_ARG: ArgConstant<'static> = ArgConstant {
    name: "instance",
    long: "instance",
    help: "Marinade instance pubkey.",
};
pub fn instance_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(INSTANCE_ARG.name)
        .long(INSTANCE_ARG.long)
        .short("i")
        .value_name("MARINADE_INSTANCE")
        .takes_value(true)
        .env("MARINADE_INSTANCE")
        .default_value("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC")
        .help(INSTANCE_ARG.help)
}

pub const PRINT_ONLY_ARG: ArgConstant<'static> = ArgConstant {
    name: "print_only",
    long: "print-only",
    help: "Transactions are not executed against the cluster, the transaction content is only printed in base64 format for multisig-use in SPL Gov.",
};
pub fn print_only_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(PRINT_ONLY_ARG.name)
        .long(PRINT_ONLY_ARG.long)
        .takes_value(false)
        .help(PRINT_ONLY_ARG.help)
}

pub const WITH_COMPUTE_UNIT_PRICE_ARG: ArgConstant<'static> = ArgConstant {
    name: "with_compute_unit_price",
    long: "with-compute-unit-price",
    help: "Set compute unit price for transaction, in increments of 0.000001 lamports per compute unit.",
};
pub fn with_compute_unit_price<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(WITH_COMPUTE_UNIT_PRICE_ARG.name)
        .value_name("COMPUTE-UNIT-PRICE")
        .takes_value(true)
        .long(WITH_COMPUTE_UNIT_PRICE_ARG.long)
        .help(WITH_COMPUTE_UNIT_PRICE_ARG.help)
        .default_value("0")
}

pub const BLOCKHASH_NOT_FOUND_RETRIES_ARG: ArgConstant<'static> = ArgConstant {
    name: "blockhash_not_found_retries",
    long: "blockhash-not-found-retries",
    help: "Number of retries to get a blockhash when exception BlockhashNotFound is thrown on execution.",
};

pub fn blockhash_not_found_retries_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(BLOCKHASH_NOT_FOUND_RETRIES_ARG.name)
        .long(BLOCKHASH_NOT_FOUND_RETRIES_ARG.long)
        .value_name("NUMBER")
        .takes_value(true)
        .help(BLOCKHASH_NOT_FOUND_RETRIES_ARG.help)
        .default_value("0")
}
