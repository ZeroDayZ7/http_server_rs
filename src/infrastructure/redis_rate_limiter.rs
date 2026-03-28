use crate::infrastructure::redis::RedisService;
use fred::interfaces::LuaInterface;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisRateLimiter {
    redis: Arc<RedisService>,
    script_content: String,
}

impl RedisRateLimiter {
    pub async fn new(redis: Arc<RedisService>) -> Self {
        let script_content = Self::load_script();
        Self {
            redis,
            script_content,
        }
    }

    fn load_script() -> String {
        std::fs::read_to_string("src/infrastructure/scripts/redis_rate_limit.lua")
            .expect("Failed to load redis_rate_limit.lua")
    }

    pub async fn check(
        &self,
        key: &str,
        limit: u64,
        window_sec: u64,
    ) -> Result<bool, fred::error::Error> {
        let client = self.redis.client();

        // W wersji 10.1.0 eval wymaga 4 generyków: R, S, K, V
        // R: Typ zwracany (i64)
        // S: Typ skryptu (&String)
        // K: Typ kluczy (vec)
        // V: Typ argumentów (vec)
        let result: i64 = client
            .eval::<i64, _, _, _>(
                &self.script_content,
                vec![key],                    // KEYS[1]
                vec![window_sec.to_string()], // ARGV[1]
            )
            .await?;

        Ok(result as u64 <= limit)
    }

    pub fn make_key(&self, prefix: &str, route: &str, ip: &str) -> String {
        format!("rate_limit:{}:{}:{}", prefix, route, ip)
    }
}
