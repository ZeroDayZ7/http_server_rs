// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use config::{Config, Environment, File};
use dotenvy::dotenv;

mod cors;
mod database;
mod log;
mod rate_limit;
mod redis;
mod server;
mod settings;

pub use cors::HttpMethod;
pub use log::LogConfig;
pub use log::LogLevel;
pub use settings::Settings;

pub fn load() -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    Config::builder()
        .add_source(File::with_name("config/settings").required(true))
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}
