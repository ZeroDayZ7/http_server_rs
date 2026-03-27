// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0

use http_server_rs::server::state::AppState;
use http_server_rs::{config, server};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let settings = config::load().expect("Failed to load configuration");

    server::logger::init_logging(&settings.log.level);

    let redis_client = server::redis::io_redis_client(&settings).await;

    let limiter = Arc::new(server::rate_limiter::RateLimiter::new());

    let state = AppState {
        redis: redis_client,
        settings: settings.clone(),
        limiter,
    };

    tracing::info!(
        "Starting application on {}:{}",
        settings.server.host,
        settings.server.port
    );

    let app = server::router(state);

    let addr_str = format!("{}:{}", settings.server.host, settings.server.port);
    let addr: SocketAddr = addr_str.parse().expect("Invalid host or port format");

    server::http::serve(app, addr).await;
}
