use crate::config::Settings;
use crate::infrastructure::redis::RedisService;
use crate::infrastructure::redis_rate_limiter::RedisRateLimiter;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisService,
    pub settings: Arc<Settings>,
    pub redis_rate_limiter: RedisRateLimiter,
}
