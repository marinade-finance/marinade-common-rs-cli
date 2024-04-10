use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::sync::Arc;

/// Keypair or Pubkey depending, could be one of that based on parameters of the CLI command.
/// When --print and --simulate are set, a pubkey instead of a valid keypair can be passed.
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

    pub fn use_signer(&self) -> Option<&Arc<dyn Signer>> {
        match self {
            PubkeyOrSigner::Pubkey(_) => None,
            PubkeyOrSigner::Signer(keypair) => Some(keypair),
        }
    }
}

impl From<PubkeyOrSigner> for Arc<dyn Signer> {
    fn from(value: PubkeyOrSigner) -> Self {
        match value {
            PubkeyOrSigner::Pubkey(_) => panic!("Cannot convert PubkeyOrSigner::Pubkey to Signer"),
            PubkeyOrSigner::Signer(keypair) => keypair,
        }
    }
}

impl Into<PubkeyOrSigner> for Arc<dyn Signer> {
    fn into(self) -> PubkeyOrSigner {
        PubkeyOrSigner::Signer(self)
    }
}

impl Into<PubkeyOrSigner> for &Arc<dyn Signer> {
    fn into(self) -> PubkeyOrSigner {
        PubkeyOrSigner::Signer(self.clone())
    }
}

impl From<PubkeyOrSigner> for Pubkey {
    fn from(value: PubkeyOrSigner) -> Self {
        value.pubkey()
    }
}

impl Into<PubkeyOrSigner> for Pubkey {
    fn into(self) -> PubkeyOrSigner {
        PubkeyOrSigner::Pubkey(self)
    }
}
