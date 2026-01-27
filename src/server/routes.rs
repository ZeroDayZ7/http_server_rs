// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{routing::get, Router, middleware::from_fn};
use axum::extract::ConnectInfo;
use crate::{handlers, server::rate_limiter::RateLimiter, server::http_logger::http_logger};


// src/server/routes.rs

pub fn router() -> Router {
    let limiter = RateLimiter::new();

    Router::new()
        .route(
            "/health",
            get(handlers::health::health)
                .layer(from_fn({
                    let limiter = limiter.clone();
                    move |conn: ConnectInfo<std::net::SocketAddr>, req, next| {
                        let limiter = limiter.clone();
                        async move {
                            limiter.middleware("health", conn, req, next).await
                        }
                    }
                }))
        )
        .route(
            "/auth/login",
            get(handlers::auth::login)
                .layer(from_fn({
                    let limiter = limiter.clone();
                    move |conn: ConnectInfo<std::net::SocketAddr>, req, next| {
                        let limiter = limiter.clone();
                        async move {
                            limiter.middleware("auth", conn, req, next).await
                        }
                    }
                }))
        )
        .layer(from_fn(http_logger))
}