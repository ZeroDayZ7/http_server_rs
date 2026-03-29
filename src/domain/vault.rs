use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedCV {
    pub id: String,
    pub data: String,
    pub salt: String,
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedCV {
    pub name: String,
    pub experience: Vec<String>,
    pub contact: String,
}

#[async_trait]
pub trait VaultRepository: Send + Sync + 'static {
    async fn get_cv_by_id(&self, id: &str) -> AppResult<Option<EncryptedCV>>;
}
