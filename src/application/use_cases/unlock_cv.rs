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

pub struct UnlockCvUseCase {
    repo: Arc<dyn VaultRepository>,
    crypto: Arc<dyn CryptoService>,
    decoder: Arc<dyn Decoder<DecryptedCV>>,
}

impl UnlockCvUseCase {
    pub fn new(
        repo: Arc<dyn VaultRepository>,
        crypto: Arc<dyn CryptoService>,
        decoder: Arc<dyn Decoder<DecryptedCV>>,
    ) -> Self {
        Self {
            repo,
            crypto,
            decoder,
        }
    }

    pub async fn execute(&self, id: &str, key: &str) -> AppResult<DecryptedCV> {
        // 1. Pobierz zaszyfrowane CV
        let encrypted = self
            .repo
            .get_cv_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("CV {} not found", id)))?;

        // 2. Zbuduj payload do decrypta
        let payload = EncryptedPayload {
            ciphertext: encrypted.data,
            salt: encrypted.salt,
            nonce: encrypted.nonce,
        };

        // 3. Odszyfruj
        let decrypted_bytes = self.crypto.decrypt(&payload, key)?;

        // 4. Zdekoduj JSON → domain
        let cv = self.decoder.decode(&decrypted_bytes)?;

        Ok(cv)
    }
}
