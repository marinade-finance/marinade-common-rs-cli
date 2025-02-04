use log::{debug, error};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer, SignerError},
    signers::Signers,
    transaction::Transaction,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct SignatureBuilder {
    pub signers: HashMap<Pubkey, Arc<Keypair>>,
    pub is_check_signers: bool,
}

impl Default for SignatureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SignatureBuilder {
    pub fn new() -> Self {
        Self {
            is_check_signers: true,
            signers: HashMap::new(),
        }
    }

    pub fn new_without_check() -> Self {
        Self {
            is_check_signers: false,
            ..SignatureBuilder::new()
        }
    }

    pub fn add_signer(&mut self, signer: Arc<Keypair>) -> Pubkey {
        let pubkey = signer.pubkey();
        self.signers.insert(pubkey, signer);
        pubkey
    }

    pub fn new_signer(&mut self) -> Pubkey {
        let keypair = Keypair::new();
        let address = keypair.pubkey();
        self.signers.insert(address, Arc::new(keypair));
        address
    }

    pub fn contains_key(&self, key: &Pubkey) -> bool {
        self.signers.contains_key(key)
    }

    pub fn get_signer(&self, key: &Pubkey) -> Option<Arc<Keypair>> {
        self.signers.get(key).cloned()
    }

    pub fn into_signers(self) -> Vec<Arc<Keypair>> {
        self.signers.into_values().collect()
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) -> Result<(), SignerError> {
        let keys = transaction.message().account_keys
            [0..transaction.message().header.num_required_signatures as usize]
            .to_vec();
        let message = transaction.message_data();
        for (pos, key) in keys.into_iter().enumerate() {
            if let Some(keypair) = self.signers.get(&key) {
                transaction.signatures[pos] = keypair.try_sign_message(&message)?;
            } else {
                let error_msg = format!(
                    "sign_transaction: not enough signers, expected key: {}, available keys in builder: {:?}",
                    key, self.signers.keys().collect::<Vec<&Pubkey>>()
                );
                if self.is_check_signers {
                    error!("{}", error_msg);
                    return Err(SignerError::NotEnoughSigners);
                } else {
                    debug!("{}", error_msg);
                }
            }
        }
        Ok(())
    }

    pub fn signers_for_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<Vec<Arc<Keypair>>, Pubkey> {
        transaction.message().account_keys
            [0..transaction.message().header.num_required_signatures as usize]
            .iter()
            .map(|key| self.signers.get(key).cloned().ok_or(*key))
            .collect()
    }
}

impl Signers for SignatureBuilder {
    fn pubkeys(&self) -> Vec<Pubkey> {
        self.signers.keys().cloned().collect()
    }

    fn try_pubkeys(&self) -> Result<Vec<Pubkey>, SignerError> {
        Ok(self.pubkeys())
    }

    fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
        self.signers
            .values()
            .map(|signer| signer.sign_message(message))
            .collect()
    }

    fn try_sign_message(&self, message: &[u8]) -> Result<Vec<Signature>, SignerError> {
        self.signers
            .values()
            .map(|signer| signer.try_sign_message(message))
            .collect()
    }

    fn is_interactive(&self) -> bool {
        false
    }
}
