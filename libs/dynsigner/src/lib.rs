use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
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

impl From<Arc<dyn Signer>> for PubkeyOrSigner {
    fn from(val: Arc<dyn Signer>) -> Self {
        PubkeyOrSigner::Signer(val)
    }
}

impl From<&Arc<dyn Signer>> for PubkeyOrSigner {
    fn from(val: &Arc<dyn Signer>) -> Self {
        PubkeyOrSigner::Signer(val.clone())
    }
}

impl From<PubkeyOrSigner> for Pubkey {
    fn from(value: PubkeyOrSigner) -> Self {
        value.pubkey()
    }
}

impl From<Pubkey> for PubkeyOrSigner {
    fn from(val: Pubkey) -> Self {
        PubkeyOrSigner::Pubkey(val)
    }
}

#[derive(Debug, Clone)]
pub enum PubkeyOrKeypair {
    Pubkey(Pubkey),
    Keypair(Arc<Keypair>),
}

impl PubkeyOrKeypair {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            PubkeyOrKeypair::Pubkey(pubkey) => *pubkey,
            PubkeyOrKeypair::Keypair(keypair) => keypair.pubkey(),
        }
    }

    pub fn try_as_keypair(&self) -> Option<Arc<Keypair>> {
        match self {
            PubkeyOrKeypair::Pubkey(_) => None,
            PubkeyOrKeypair::Keypair(keypair) => Some(keypair.clone()),
        }
    }

    pub fn use_keypair(&self) -> Option<&Arc<Keypair>> {
        match self {
            PubkeyOrKeypair::Pubkey(_) => None,
            PubkeyOrKeypair::Keypair(keypair) => Some(keypair),
        }
    }
}

impl From<PubkeyOrKeypair> for Arc<Keypair> {
    fn from(value: PubkeyOrKeypair) -> Self {
        match value {
            PubkeyOrKeypair::Pubkey(_) => panic!("Cannot convert PubkeyOrSigner::Pubkey to Signer"),
            PubkeyOrKeypair::Keypair(keypair) => keypair,
        }
    }
}

impl From<Arc<Keypair>> for PubkeyOrKeypair {
    fn from(val: Arc<Keypair>) -> Self {
        PubkeyOrKeypair::Keypair(val)
    }
}

impl From<&Arc<Keypair>> for PubkeyOrKeypair {
    fn from(val: &Arc<Keypair>) -> Self {
        PubkeyOrKeypair::Keypair(val.clone())
    }
}

impl From<PubkeyOrKeypair> for Pubkey {
    fn from(value: PubkeyOrKeypair) -> Self {
        value.pubkey()
    }
}

impl From<Pubkey> for PubkeyOrKeypair {
    fn from(val: Pubkey) -> Self {
        PubkeyOrKeypair::Pubkey(val)
    }
}
