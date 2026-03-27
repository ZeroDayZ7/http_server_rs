use axum::{
    extract::Request,
    extract::{ConnectInfo, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::warn;

use crate::server::state::AppState;

#[derive(Clone)]
pub struct RateLimitConfig {
    pub max: u32,
    pub window: Duration,
}

#[derive(Clone, Default)]
pub struct RateLimiter {
    pub limits: Arc<Mutex<HashMap<String, HashMap<String, (u32, Instant)>>>>,
}

pub type SharedLimiter = Arc<RateLimiter>;

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn presets() -> HashMap<&'static str, RateLimitConfig> {
        let mut m = HashMap::new();
        m.insert(
            "global",
            RateLimitConfig {
                max: 100,
                window: Duration::from_secs(60),
            },
        );
        m.insert(
            "auth",
            RateLimitConfig {
                max: 10,
                window: Duration::from_secs(60),
            },
        );
        m.insert(
            "health",
            RateLimitConfig {
                max: 20,
                window: Duration::from_secs(30),
            },
        );
        m
    }

    pub async fn middleware(
        State(state): State<AppState>, // Zmienione na AppState
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request,
        next: Next,
    ) -> Response {
        let ip = addr.ip().to_string();
        let presets = Self::presets();

        let group = req
            .extensions()
            .get::<&'static str>()
            .copied()
            .unwrap_or("global");
        let cfg = presets.get(group).unwrap_or(presets.get("global").unwrap());

        // Pobieramy limiter z wstrzykniętego stanu
        let mut limits = state.limiter.limits.lock().await;

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
