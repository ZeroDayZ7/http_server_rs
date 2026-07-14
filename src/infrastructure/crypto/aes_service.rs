use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use argon2::{Algorithm, Argon2, Params, Version};

use crate::{
    config::crypto::CryptoSettings,
    domain::crypto::{CryptoService, EncryptedPayload},
    errors::{AppError, AppResult},
};

pub struct AesCryptoService {
    settings: CryptoSettings,
}

impl AesCryptoService {
    pub fn new(settings: CryptoSettings) -> Self {
        Self { settings }
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let mut key = [0u8; 32];
        let password_with_pepper = format!("{}{}", password, self.settings.secret_key);

        let params = Params::new(
            self.settings.argon2_m_cost,
            self.settings.argon2_t_cost,
            self.settings.argon2_p_cost,
            None,
        )
        .map_err(|e| AppError::CryptoError(format!("Argon2 params invalid: {e}")))?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        argon2
            .hash_password_into(password_with_pepper.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::CryptoError(format!("Argon2 derivation failed: {e}")))?;

        Ok(key)
    }
}

impl CryptoService for AesCryptoService {
    fn encrypt(&self, data: &[u8], password: &str) -> AppResult<EncryptedPayload> {
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];

        let mut rng = OsRng;
        rng.try_fill_bytes(&mut salt)
            .map_err(|e| AppError::CryptoError(format!("RNG error: {e}")))?;

        rng.fill_bytes(&mut nonce_bytes);

        let key = self.derive_key(password, &salt)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| AppError::CryptoError(format!("Encryption failed: {e}")))?;

        Ok(EncryptedPayload {
            ciphertext,
            salt: salt.to_vec(),
            nonce: nonce_bytes.to_vec(),
        })
    }

    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> AppResult<Vec<u8>> {
        let salt: [u8; 16] =
            payload.salt.as_slice().try_into().map_err(|_| {
                AppError::CryptoError("Invalid salt length: expected 16 bytes".into())
            })?;

        let nonce_raw: [u8; 12] =
            payload.nonce.as_slice().try_into().map_err(|_| {
                AppError::CryptoError("Invalid nonce length: expected 12 bytes".into())
            })?;

        let key = self.derive_key(password, &salt)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&nonce_raw);

        let decrypted = cipher
            .decrypt(nonce, payload.ciphertext.as_ref())
            .map_err(|_| {
                AppError::CryptoError("Decryption failed: check password or data integrity".into())
            })?;

        Ok(decrypted)
    }
}