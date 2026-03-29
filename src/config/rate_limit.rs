use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub global_per_second: u64,
    pub global_burst: u32,
    pub health_per_second: u64,
    pub health_burst: u32,
    pub auth_per_second: u64,
    pub auth_burst: u32,
}
