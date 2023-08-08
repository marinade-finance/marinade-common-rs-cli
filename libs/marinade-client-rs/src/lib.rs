#![cfg_attr(not(debug_assertions), deny(warnings))]

pub mod marinade;
pub mod transactions;

pub use solana_sdk;
pub use spl_associated_token_account;
