use crate::infrastructure::redis::RedisService;
use fred::interfaces::LuaInterface;
use std::sync::Arc;
use tracing::warn;

// Kompilujemy skrypt do binarki
const LUA_SCRIPT: &str = include_str!("scripts/redis_rate_limit.lua");

pub struct RateLimitResult {
    pub allowed: bool,
    pub current: u64,
}

#[derive(Clone)]
pub struct RedisRateLimiter {
    redis: Arc<RedisService>,
    // Przechowujemy hash, aby Redis nie musiał parsować skryptu za każdym razem
    script_hash: String,
}

impl RedisRateLimiter {
    pub async fn new(redis: Arc<RedisService>) -> Self {
        let client = redis.client();

        // Senior Move: Ładujemy skrypt do pamięci Redisa raz, przy starcie
        // Jeśli Redis zrestartuje, EVALSHA rzuci błąd, a my zrobimy fallback do EVAL
        let script_hash = match client.script_load(LUA_SCRIPT).await {
            Ok(hash) => hash,
            Err(e) => {
                warn!(
                    "⚠️ Nie udało się załadować skryptu do Redisa (load): {}. Używam pustego hasha.",
                    e
                );
                String::new()
            }
        };

        Self { redis, script_hash }
    }

    pub async fn check(
        &self,
        key: &str,
        limit: u64,
        window_sec: u64,
    ) -> Result<RateLimitResult, fred::error::Error> {
        let client = self.redis.client();

        // Architect Move: Używamy EVALSHA.
        // Wysyłamy do Redisa 40 bajtów (hash) zamiast całego kodu Lua.
        // Oszczędność pasma przy 10k req/s jest ogromna.
        let result: i64 = match client
            .evalsha::<i64, _, _, _>(&self.script_hash, vec![key], vec![window_sec.to_string()])
            .await
        {
            Ok(res) => res,
            Err(_) => {
                // Fallback: Jeśli skrypt wyleciał z cache Redisa, użyj pełnego skryptu (EVAL)
                client
                    .eval(LUA_SCRIPT, vec![key], vec![window_sec.to_string()])
                    .await?
            }
        };

        Ok(RateLimitResult {
            allowed: result as u64 <= limit,
            current: result as u64,
        })
    }

    /// Generuje klucz w sposób wydajny (Zero-allocation dla małych stringów w przyszłości)
    pub fn make_key(&self, prefix: &str, route: &str, ip: &str) -> String {
        let mut s = String::with_capacity(32 + prefix.len() + route.len() + ip.len());
        s.push_str("rl:");
        s.push_str(prefix);
        s.push(':');
        s.push_str(route);
        s.push(':');
        s.push_str(ip);
        s
    }
}
