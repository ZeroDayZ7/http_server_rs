use crate::errors::{AppError, AppResult};
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::random;

pub struct CryptoUtils;

impl CryptoUtils {
    /// Generuje klucz 32-bajtowy przy użyciu Argon2id
    pub fn derive_key(password: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let mut key = [0u8; 32];
        let argon2 = Argon2::default();

        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::CryptoError(format!("Argon2 failure: {}", e)))?;

        Ok(key)
    }

    /// Szyfruje dane i zwraca (ciphertext_b64, salt_b64, nonce_b64)
    pub fn encrypt(data: &str, password: &str) -> AppResult<(String, String, String)> {
        // Powrót do Twojej działającej logiki
        let salt_raw: [u8; 16] = random();
        let nonce_raw: [u8; 12] = random();

        let salt_b64 = STANDARD.encode(salt_raw);
        let nonce_b64 = STANDARD.encode(nonce_raw);

        let key_bytes = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| AppError::CryptoError(format!("Cipher init error: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_raw);
        let ciphertext = cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| AppError::CryptoError(format!("Encryption failure: {}", e)))?;

        Ok((STANDARD.encode(ciphertext), salt_b64, nonce_b64))
    }

    /// Deszyfruje dane zakodowane w Base64
    pub fn decrypt(
        encrypted_data_b64: &str,
        password: &str,
        salt_b64: &str,
        nonce_b64: &str,
    ) -> AppResult<String> {
        let ciphertext = STANDARD
            .decode(encrypted_data_b64)
            .map_err(|_| AppError::ValidationError("Invalid ciphertext base64".to_string()))?;

        let salt_raw = STANDARD
            .decode(salt_b64)
            .map_err(|_| AppError::ValidationError("Invalid salt base64".to_string()))?;

        let nonce_raw = STANDARD
            .decode(nonce_b64)
            .map_err(|_| AppError::ValidationError("Invalid nonce base64".to_string()))?;

        let key_bytes = Self::derive_key(password, &salt_raw)?;
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| AppError::CryptoError(format!("Cipher init error: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_raw);
        let decrypted_bytes = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|_| {
            AppError::CryptoError("Invalid access key or corrupted data".to_string())
        })?;

        String::from_utf8(decrypted_bytes).map_err(|_| {
            AppError::Internal(anyhow::anyhow!("Data corrupted: Invalid UTF-8 sequence"))
        })
    }
}
