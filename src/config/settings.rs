// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub cors: CorsConfig,
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub shutdown_timeout: u64,
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

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub name: String,
    pub pool_size: u32,
    pub auth_source: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub global_per_second: u64,
    pub global_burst: u32,

    pub health_per_second: u64,
    pub health_burst: u32,

    pub auth_per_second: u64,
    pub auth_burst: u32,
}
