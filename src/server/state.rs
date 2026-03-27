use crate::config::Settings;
use crate::server::rate_limiter::SharedLimiter;
use fred::prelude::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis: Client,
    pub settings: Settings,
    pub limiter: SharedLimiter,
}
