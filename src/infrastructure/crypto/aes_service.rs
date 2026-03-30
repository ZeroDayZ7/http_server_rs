use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::{Algorithm, Argon2, Params, Version};
// base64 nie jest już potrzebne w tym pliku, bo operujemy na bajtach!
use rand::random;

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
        .map_err(|e| AppError::CryptoError(format!("Argon2 params error: {}", e)))?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        argon2
            .hash_password_into(password_with_pepper.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::CryptoError(format!("Argon2 failure: {}", e)))?;

        Ok(key)
    }
}

impl CryptoService for AesCryptoService {
    fn encrypt(&self, data: &[u8], password: &str) -> AppResult<EncryptedPayload> {
        let salt_raw: [u8; 16] = random();
        let nonce_raw: [u8; 12] = random();

        let key = self.derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher init: {}", e)))?;

        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_raw), data)
            .map_err(|e| AppError::CryptoError(format!("Encrypt failure: {}", e)))?;

        Ok(EncryptedPayload {
            ciphertext,
            salt: salt_raw.to_vec(),
            nonce: nonce_raw.to_vec(),
        })
    }

    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> AppResult<Vec<u8>> {
        if payload.salt.len() != 16 {
            return Err(AppError::CryptoError(
                "Nieprawidłowa długość soli (wymagane 16 bajtów)".into(),
            ));
        }

        if payload.nonce.len() != 12 {
            return Err(AppError::CryptoError(
                "Nieprawidłowa długość nonce (wymagane 12 bajtów)".into(),
            ));
        }

        let key = self.derive_key(password, &payload.salt)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Błąd inicjalizacji szyfru: {}", e)))?;

        let nonce = Nonce::from_slice(&payload.nonce);

        let decrypted = cipher
            .decrypt(nonce, payload.ciphertext.as_ref())
            .map_err(|_| {
                AppError::CryptoError("Błędny klucz lub uszkodzone dane (MAC mismatch)".into())
            })?;

        Ok(decrypted)
    }
}
