use crate::config::Settings;
use fred::prelude::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis: Client,
    pub settings: Settings,
}
