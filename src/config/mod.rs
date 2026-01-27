// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use dotenvy::dotenv;

pub use settings::Settings;

mod settings;

pub fn load() -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    Config::builder()
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}
