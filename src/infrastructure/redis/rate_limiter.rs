use crate::errors::{AppError, AppResult};
use crate::infrastructure::redis::client::RedisManager;
use fred::interfaces::LuaInterface;
use std::sync::Arc;
use tracing::warn;

const LUA_SCRIPT: &str = include_str!("../scripts/redis_rate_limit.lua");

pub struct RateLimitResult {
    pub allowed: bool,
    pub current: u64,
}

#[derive(Clone)]
pub struct RedisRateLimiter {
    redis: Arc<RedisManager>,
    script_hash: String,
}

impl RedisRateLimiter {
    pub async fn new(redis: Arc<RedisManager>) -> Self {
        let client = redis.client();

        let script_hash = match client.script_load(LUA_SCRIPT).await {
            Ok(hash) => hash,
            Err(e) => {
                warn!(
                    "⚠️ Nie udało się załadować skryptu: {}. Fallback do EVAL.",
                    e
                );
                String::new()
            }
        };

        Self { redis, script_hash }
    }

    pub fn make_key(&self, prefix: &str, route: &str, ip: &str) -> String {
        format!("rl:{}:{}:{}", prefix, route, ip)
    }

    pub async fn check(
        &self,
        key: &str,
        limit: u64,
        window_sec: u64,
    ) -> AppResult<RateLimitResult> {
        let client = self.redis.client();

        let result: i64 = match client
            .evalsha::<i64, _, _, _>(&self.script_hash, vec![key], vec![window_sec.to_string()])
            .await
        {
            Ok(res) => res,
            Err(_) => client
                .eval::<i64, _, _, _>(LUA_SCRIPT, vec![key], vec![window_sec.to_string()])
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis Eval Error: {}", e)))?,
        };

        Ok(RateLimitResult {
            allowed: result as u64 <= limit,
            current: result as u64,
        })
    }
}
