# Marinade Common Rust CLI Utils

This is a collection of utilities for Rust that enhance the off-chain work with the [Marinade liquid-staking-program](https://github.com/marinade-finance/liquid-staking-program).
It is primarily designed to be used with Marinade's CLI tools in Rust. These utilities provide helpers for utilizing the [clap argument parser](https://docs.rs/clap/latest/clap/),
in conjunction with the [Solana clap utilities](https://github.com/solana-labs/solana/tree/v1.14.19/clap-utils).

The repository is divided into three crates:

1. `dynsigner`: This crate provides a helper class for integrating the Anchor Client with Solana Clap Utils. You can find more details about this integration [here](https://github.com/coral-xyz/anchor/pull/2550).

2. `marinade-client-rs`: This crate offers a set of helper functions for building Marinade instructions using the [Anchor client](https://github.com/coral-xyz/anchor/tree/master/client). It also includes additional utility functions for working with Marinade state, particularly for managing Marinade list accounts.

3. `marinade-common-cli`: This crate provides a command-line parser helper for clap parser library, following the same conventions as the Solana clap utilities.
