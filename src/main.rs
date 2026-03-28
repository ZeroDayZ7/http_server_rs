// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0

use http_server_rs::server::state::AppState;
use http_server_rs::{config, server};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    // 1. Load config
    let settings = Arc::new(config::load().expect("Failed to load config"));

    // 2. Init logging
    let _guards = server::logger::init_logging(&settings.log.level);

    // 3. Build State (cały syf schowany w AppState::new)
    let state = AppState::new(settings.clone()).await;

    info!(
        "Starting server on {}:{}",
        settings.server.host, settings.server.port
    );

    // 4. Run Server
    let app = server::router(state);
    let addr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .expect("Invalid address");

    server::http::serve(app, addr, settings.server.shutdown_timeout).await;
}
