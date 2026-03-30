use crate::config::crypto::CryptoSettings;
use crate::{
    domain::{
        VaultRepository,
        crypto::{CryptoService, EncryptedPayload},
        ports::decoder::Decoder,
        ports::services::VaultServicePort,
        vault::DecryptedCV,
    },
    errors::{AppError, AppResult},
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct VaultService {
    repo: Arc<dyn VaultRepository>,
    crypto: Arc<dyn CryptoService>,
    decoder: Arc<dyn Decoder<DecryptedCV>>,
    #[allow(dead_code)]
    config: CryptoSettings,
}

impl VaultService {
    pub fn new(
        repo: Arc<dyn VaultRepository>,
        crypto: Arc<dyn CryptoService>,
        decoder: Arc<dyn Decoder<DecryptedCV>>,
        config: CryptoSettings,
    ) -> Self {
        Self {
            repo,
            crypto,
            decoder,
            config,
        }
    }
}

#[async_trait]
impl VaultServicePort for VaultService {
    async fn unlock_cv(&self, id: &str, key: &str) -> AppResult<DecryptedCV> {
        let encrypted = self
            .repo
            .get_cv_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("CV {} not found", id)))?;

        let payload = EncryptedPayload {
            ciphertext: encrypted.data,
            salt: encrypted.salt,
            nonce: encrypted.nonce,
        };

        let decrypted_bytes = self.crypto.decrypt(&payload, key)?;
        let cv = self.decoder.decode(&decrypted_bytes)?;

        Ok(cv)
    }
}
