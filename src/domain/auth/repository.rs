// src/domain/auth/repository.rs
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn store_session(&self, user_id: &str, token: &str, ttl_sec: u64) -> AppResult<()>;
    async fn get_session(&self, user_id: &str) -> AppResult<Option<String>>;
    async fn delete_session(&self, user_id: &str) -> AppResult<()>;
}
