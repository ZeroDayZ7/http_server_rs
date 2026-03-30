use crate::domain::auth::repository::AuthRepository;
use crate::errors::AppResult;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::keys::RedisKeys;
use async_trait::async_trait;
use std::sync::Arc;

pub struct RedisAuthRepository {
    redis: Arc<RedisManager>,
}

impl RedisAuthRepository {
    pub fn new(redis: Arc<RedisManager>) -> Self {
        Self { redis }
    }
}

#[async_trait]
impl AuthRepository for RedisAuthRepository {
    async fn store_session(&self, user_id: &str, token: &str, ttl_sec: u64) -> AppResult<()> {
        let key = RedisKeys::session(token);
        self.redis.set_ex(&key, user_id, ttl_sec).await
    }

    async fn get_session(&self, token: &str) -> AppResult<Option<String>> {
        let key = RedisKeys::session(token);
        self.redis.get(&key).await
    }

    async fn delete_session(&self, token: &str) -> AppResult<()> {
        let key = RedisKeys::session(token);
        self.redis.del(&key).await
    }
}
