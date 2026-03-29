use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::Argon2;
use base64::{Engine, engine::general_purpose::STANDARD};
use rand::random;

use crate::{
    domain::crypto::{CryptoService, EncryptedPayload},
    errors::{AppError, AppResult},
};

pub struct AesCryptoService;

impl AesCryptoService {
    fn derive_key(password: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::CryptoError(format!("Argon2 failure: {}", e)))?;
        Ok(key)
    }
}

impl CryptoService for AesCryptoService {
    fn encrypt(&self, data: &[u8], password: &str) -> AppResult<EncryptedPayload> {
        let salt_raw: [u8; 16] = random();
        let nonce_raw: [u8; 12] = random();

        let key = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher init error: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_raw);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| AppError::CryptoError(format!("Encryption failure: {}", e)))?;

        Ok(EncryptedPayload {
            ciphertext: STANDARD.encode(ciphertext),
            salt: STANDARD.encode(salt_raw),
            nonce: STANDARD.encode(nonce_raw),
        })
    }

    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> AppResult<Vec<u8>> {
        let ciphertext = STANDARD
            .decode(&payload.ciphertext)
            .map_err(|_| AppError::ValidationError("Invalid ciphertext base64".into()))?;

        let salt_raw = STANDARD
            .decode(&payload.salt)
            .map_err(|_| AppError::ValidationError("Invalid salt base64".into()))?;

        let nonce_raw = STANDARD
            .decode(&payload.nonce)
            .map_err(|_| AppError::ValidationError("Invalid nonce base64".into()))?;

        let key = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher init error: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_raw);

        let decrypted = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| AppError::CryptoError("Invalid key or corrupted data".into()))?;

        Ok(decrypted)
    }
}
