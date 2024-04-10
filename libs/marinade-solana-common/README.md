# DynSigner

When using Solana labs CLI utilities
(`solana-clap-utils`, https://github.com/solana-labs/solana/tree/master/clap-utils
-> https://github.com/solana-labs/solana/blob/6b013f46ebd30c82f7fa9c50c5a0e9ae32df3c44/clap-utils/src/keypair.rs#L357)
the loaded signer of type `Box<dyn Signer>` is not aligned to the expected `Client<C>` type of `<C: Clone + Deref<Target = impl Signer>>`.
Using the dyn Signer fails with error:

```
the size for values of type `dyn solana_sdk::signature::Signer` cannot be known at compilation time [E0277] doesn't have a size known at compile-time Help: the trait `Sized` is not implemented for `dyn solana_sdk::signature::Signer` Note: required by a bound in `anchor_client::Client::<C>::new_with_options`
```

Adding the helper DynSigner makes possible to match those types and use the Signer loded from the Solana utils with the Anchor client.

```
let fee_payer: Arc<dyn Signer> = ...
let anchor_client: Client<Arc<DynSigner>> = Client::new_with_options(
    anchor_cluster,
    Arc::new(DynSigner(fee_payer.clone())),
    CommitmentConfig::confirmed(),
);
```