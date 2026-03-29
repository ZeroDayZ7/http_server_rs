use crate::errors::{AppError, AppResult};
use serde::de::DeserializeOwned;

pub trait VaultDecoder<T>: Send + Sync {
    fn decode(&self, bytes: &[u8]) -> AppResult<T>;
}

pub struct JsonVaultDecoder;

impl<T: DeserializeOwned> VaultDecoder<T> for JsonVaultDecoder {
    fn decode(&self, bytes: &[u8]) -> AppResult<T> {
        serde_json::from_slice(bytes)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON decode error: {}", e)))
    }
}
