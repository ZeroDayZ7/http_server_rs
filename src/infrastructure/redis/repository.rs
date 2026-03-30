use crate::domain::auth::repository::AuthRepository;
use crate::domain::value_objects::session_token::SessionToken;
use crate::domain::value_objects::session_ttl::SessionTtl;
use crate::domain::value_objects::user_id::UserId;
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
    async fn store_session(
        &self,
        user_id: &UserId,
        token: &SessionToken,
        ttl: SessionTtl,
    ) -> AppResult<()> {
        let key = RedisKeys::session(token.as_str());

        self.redis
            .set_ex(&key, &user_id.to_string(), ttl.as_secs())
            .await
    }

    async fn get_session(&self, user_id: &UserId) -> AppResult<Option<SessionToken>> {
        let key = RedisKeys::session(&user_id.to_string());

        let result: Option<String> = self.redis.get(&key).await?;

        result.map(|t| SessionToken::new(t)).transpose()
    }

    async fn delete_session(&self, user_id: &UserId) -> AppResult<()> {
        let key = RedisKeys::session(&user_id.to_string());
        self.redis.del(&key).await
    }
}
