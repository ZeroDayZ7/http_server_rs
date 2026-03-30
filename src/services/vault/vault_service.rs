use std::sync::Arc;

use crate::{
    domain::{
        VaultRepository,
        crypto::{CryptoService, EncryptedPayload},
        ports::decoder::Decoder,
        vault::DecryptedCV,
    },
    errors::{AppError, AppResult},
};

pub struct VaultService {
    repo: Arc<dyn VaultRepository>,
    crypto: Arc<dyn CryptoService>,
    decoder: Arc<dyn Decoder<DecryptedCV> + Send + Sync>,
}

impl VaultService {
    pub fn new(
        repo: Arc<dyn VaultRepository>,
        crypto: Arc<dyn CryptoService>,
        decoder: Arc<dyn Decoder<DecryptedCV> + Send + Sync>,
    ) -> Self {
        Self {
            repo,
            crypto,
            decoder,
        }
    }

    pub async fn unlock_cv(&self, id: &str, key: &str) -> AppResult<DecryptedCV> {
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
