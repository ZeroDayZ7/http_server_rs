use crate::errors::{AppError, AppResult};
use aes_gcm::{Aes256Gcm, Nonce, aead::Aead, aes::cipher::KeyInit};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::random;

pub struct CryptoUtils;

impl CryptoUtils {
    pub fn derive_key(password: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let mut key = [0u8; 32];
        let argon2 = Argon2::default();

        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Argon2 error: {}", e)))?;

        Ok(key)
    }

    pub fn encrypt(data: &str, password: &str) -> AppResult<(String, String, String)> {
        let salt_raw: [u8; 16] = random();
        let nonce_raw: [u8; 12] = random();

        let salt_b64 = STANDARD.encode(salt_raw);
        let nonce_b64 = STANDARD.encode(nonce_raw);

        let key_bytes = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new(&key_bytes.into());

        let nonce = Nonce::from_slice(&nonce_raw);
        let ciphertext = cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Encryption error: {}", e)))?;

        Ok((STANDARD.encode(ciphertext), salt_b64, nonce_b64))
    }

    pub fn decrypt(
        encrypted_data_b64: &str,
        password: &str,
        salt_b64: &str,
        nonce_b64: &str,
    ) -> AppResult<String> {
        let ciphertext = STANDARD
            .decode(encrypted_data_b64)
            .map_err(|_| AppError::BadRequest("Błędny format danych (base64)".into()))?;

        let nonce_raw = STANDARD
            .decode(nonce_b64)
            .map_err(|_| AppError::BadRequest("Błędny format nonce".into()))?;

        let salt_raw = STANDARD
            .decode(salt_b64)
            .map_err(|_| AppError::BadRequest("Błędny format salt".into()))?;

        let key_bytes = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new(&key_bytes.into());

        let nonce = Nonce::from_slice(&nonce_raw);
        let decrypted_bytes = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| AppError::BadRequest("Błędny klucz dostępu lub uszkodzone dane".into()))?;

        String::from_utf8(decrypted_bytes)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Błąd dekodowania UTF-8")))
    }
}
