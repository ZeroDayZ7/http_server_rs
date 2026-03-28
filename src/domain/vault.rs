use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedCV {
    pub id: String,
    pub data: String,  // Base64
    pub salt: String,  // Base64
    pub nonce: String, // Base64
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
