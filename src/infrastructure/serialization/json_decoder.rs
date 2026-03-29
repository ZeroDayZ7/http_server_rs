// src/infrastructure/serialization/json_decoder.rs
use crate::errors::{AppError, AppResult};
use crate::services::vault::vault_decoder::VaultDecoder;

pub struct JsonVaultDecoder;

impl<T: for<'a> serde::Deserialize<'a>> VaultDecoder<T> for JsonVaultDecoder {
    fn decode(&self, bytes: &[u8]) -> AppResult<T> {
        serde_json::from_slice(bytes)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON decode error: {}", e)))
    }
}
