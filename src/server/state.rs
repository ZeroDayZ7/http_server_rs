use crate::config::Settings;
use crate::infrastructure::redis::RedisService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisService,
    pub settings: Arc<Settings>,
}
