// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{
    http::StatusCode,
    response::IntoResponse,
    middleware::Next,
    extract::ConnectInfo,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::warn;

/// Presets for different endpoints
#[derive(Clone)]
pub struct RateLimitConfig {
    pub max: u32,
    pub window: Duration,
}

/// Shared state for limiter
#[derive(Clone, Default)]
pub struct RateLimiter {
    pub limits: Arc<Mutex<HashMap<String, HashMap<String, (u32, Instant)>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn presets() -> HashMap<&'static str, RateLimitConfig> {
        let mut m = HashMap::new();
        m.insert("global", RateLimitConfig { max: 100, window: Duration::from_secs(60) });
        m.insert("auth", RateLimitConfig { max: 10, window: Duration::from_secs(60) });
        m.insert("reset", RateLimitConfig { max: 3, window: Duration::from_secs(60) });
        m.insert("notifications", RateLimitConfig { max: 30, window: Duration::from_secs(60) });
        m.insert("users", RateLimitConfig { max: 5, window: Duration::from_secs(60) });
        m.insert("health", RateLimitConfig { max: 20, window: Duration::from_secs(30) });
        m
    }


pub async fn middleware(
    self,
    group: &'static str,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    req: axum::extract::Request,
    next: Next,                
) -> impl IntoResponse {
    let ip = addr.ip().to_string();
    let presets = Self::presets();
    let cfg = presets.get(group).unwrap_or(presets.get("global").unwrap());

    let mut limits = self.limits.lock().await;
    let endpoint = group.to_string();
    let user_entry = limits.entry(endpoint.clone()).or_default();
    let counter = user_entry.entry(ip.clone()).or_insert((0, Instant::now()));

    if counter.1.elapsed() > cfg.window {
        *counter = (0, Instant::now());
    }

    if counter.0 >= cfg.max {
        warn!("Rate limit exceeded: ip={} endpoint={}", ip, endpoint);
        return (StatusCode::TOO_MANY_REQUESTS, "Too many requests").into_response();
    }

    counter.0 += 1;

    drop(limits);
    next.run(req).await
}
}
