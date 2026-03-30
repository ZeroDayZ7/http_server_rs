// src/domain/auth/repository.rs
use super::models::{SessionToken, SessionTtl, UserId};
use crate::errors::AppResult;
use async_trait::async_trait;
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn store_session(
        &self,
        user_id: &UserId,
        token: &SessionToken,
        ttl: SessionTtl,
    ) -> AppResult<()>;

    async fn get_session(&self, user_id: &UserId) -> AppResult<Option<SessionToken>>;

    async fn delete_session(&self, user_id: &UserId) -> AppResult<()>;
}
