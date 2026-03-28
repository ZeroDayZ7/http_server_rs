use crate::config::Settings;
use fred::prelude::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub redis: Client,
    pub settings: Arc<Settings>,
}
