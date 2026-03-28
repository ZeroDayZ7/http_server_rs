use crate::domain::vault::VaultRepository;
use crate::domain::vault::{DecryptedCV, EncryptedCV};
use crate::errors::{AppError, AppResult};
use crate::infrastructure::mongodb_vault::MongoVaultRepository;
use serde_json;
use std::sync::Arc;

pub struct VaultService {
    repo: Arc<MongoVaultRepository>,
}

impl VaultService {
    pub fn new(repo: Arc<MongoVaultRepository>) -> Self {
        Self { repo }
    }

    pub async fn unlock_cv(&self, id: &str, access_key: &str) -> AppResult<DecryptedCV> {
        let encrypted = self
            .repo
            .get_cv_by_id(id)
            .await?
            .ok_or_else(|| AppError::BadRequest("CV nie istnieje".into()))?;

        let decrypted_json = self.decrypt_logic(&encrypted, access_key)?;

        let cv: DecryptedCV = serde_json::from_str(&decrypted_json)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Błąd deszyfracji danych")))?;

        Ok(cv)
    }

    fn decrypt_logic(&self, _enc: &EncryptedCV, _key: &str) -> AppResult<String> {
        Ok(
            r#"{"name": "ZeroDayZ7", "experience": ["Rust"], "contact": "it@works.pl"}"#
                .to_string(),
        )
    }
}
