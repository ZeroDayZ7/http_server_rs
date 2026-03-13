// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use http_server_rs::{config, server};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let settings = config::load().expect("Failed to load configuration");

    server::logger::init_logging(&settings.log.level);

    let limiter = Arc::new(server::rate_limiter::RateLimiter::new());

    tracing::info!(
        "Starting application on {}:{}",
        settings.server.host,
        settings.server.port
    );

    let app = server::router(limiter);

    let addr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .expect("Invalid host or port");

    server::http::serve(app, addr).await;
}
