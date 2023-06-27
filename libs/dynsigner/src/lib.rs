use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::sync::Arc;

/// Auxiliary data structure to align the types of the solana-clap-utils with anchor-client.
pub struct DynSigner(pub Arc<dyn Signer>);

impl Signer for DynSigner {
    fn pubkey(&self) -> Pubkey {
        self.0.pubkey()
    }

    fn try_pubkey(&self) -> Result<Pubkey, solana_sdk::signer::SignerError> {
        self.0.try_pubkey()
    }

    fn sign_message(&self, message: &[u8]) -> solana_sdk::signature::Signature {
        self.0.sign_message(message)
    }

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        self.0.try_sign_message(message)
    }

    fn is_interactive(&self) -> bool {
        self.0.is_interactive()
    }
}
