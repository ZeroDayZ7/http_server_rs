// Copyright 2026 ZeroDayZ7
use axum::{routing::get, Router, middleware::from_fn, extract::ConnectInfo};
use std::net::SocketAddr;
use crate::{handlers, server::rate_limiter::RateLimiter, server::http_logger::http_logger};

pub fn router() -> Router {
    let limiter = RateLimiter::new();

    Router::new()
        .route(
            "/health",
            get(handlers::health::health)
                .layer(from_fn({
                    let limiter = limiter.clone();
                    move |conn: ConnectInfo<SocketAddr>, req, next| {
                        let limiter = limiter.clone();
                        async move { limiter.middleware("health", conn, req, next).await }
                    }
                })),
        )
        .route(
            "/auth/login",
            get(handlers::auth::login)
                .layer(from_fn({
                    let limiter = limiter.clone();
                    move |conn: ConnectInfo<SocketAddr>, req, next| {
                        let limiter = limiter.clone();
                        async move { limiter.middleware("auth", conn, req, next).await }
                    }
                })),
        )
        // Logger globalny (wykonywany przed limiterami)
        .layer(from_fn(http_logger))
}