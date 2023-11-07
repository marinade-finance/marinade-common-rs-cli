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



/// Keypair or Pubkey depending, could be one of that based on parameters of the CLI command.
/// For --print-only we want to permit to pass only pubkey not the keypair in real.
#[derive(Debug, Clone)]
pub enum PubkeyOrSigner {
    Pubkey(Pubkey),
    Signer(Arc<dyn Signer>),
}

impl PubkeyOrSigner {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            PubkeyOrSigner::Pubkey(pubkey) => *pubkey,
            PubkeyOrSigner::Signer(keypair) => keypair.pubkey(),
        }
    }

    pub fn try_as_signer(&self) -> Option<Arc<dyn Signer>> {
        match self {
            PubkeyOrSigner::Pubkey(_) => None,
            PubkeyOrSigner::Signer(keypair) => Some(keypair.clone()),
        }
    }
}