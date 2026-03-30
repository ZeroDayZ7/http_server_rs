// src/infrastructure/redis/keys.rs
pub struct RedisKeys;

impl RedisKeys {
    pub fn session(token: &str) -> String {
        format!("auth:session:{}", token)
    }

    pub fn user_profile(id: &str) -> String {
        format!("user:profile:{}", id)
    }

    pub fn rate_limit(ip: &str, path: &str) -> String {
        format!("rl:{}:{}", path, ip)
    }
}
