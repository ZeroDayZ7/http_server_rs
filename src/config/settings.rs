// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origin: String,
    pub allowed_methods: String,
    pub max_age: u64,
}
