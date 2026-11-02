#![cfg_attr(not(debug_assertions), deny(warnings))]

pub mod marinade;
pub mod transactions;

pub type RequestBuilder<'a, C> =
    anchor_client::RequestBuilder<'a, C, std::sync::Arc<dyn anchor_client::ThreadSafeSigner>>;

pub use solana_sdk;
pub use spl_associated_token_account;
