// src/domain/crypto.rs
use crate::errors::AppResult;

pub struct EncryptedPayload {
    pub ciphertext: String,
    pub salt: String,
    pub nonce: String,
}

pub trait CryptoService: Send + Sync {
    fn encrypt(&self, data: &[u8], password: &str) -> AppResult<EncryptedPayload>;
    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> AppResult<Vec<u8>>;
}
