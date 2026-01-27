// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::info;
use axum::body::Body;

pub async fn http_logger(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();

    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Wykonanie requestu
    let response = next.run(req).await;

    let status = response.status().as_u16();
    let latency_ms = start.elapsed().as_millis();

    info!(%ip, %method, %path, status, latency_ms, "HTTP request");

    response
}
