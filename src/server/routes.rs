// src/server/routes.rs
use crate::handlers::{auth, health};
use crate::server::middleware;
use crate::server::rate_limiter::RateLimiter;
use crate::server::state::AppState;
use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
};

pub fn router(state: AppState) -> Router {
    let cors = middleware::create_cors_layer(&state.settings);
    let security = middleware::create_security_headers_layer().into_inner();

    Router::new()
        .route(
            "/health",
            get(health::health).layer(from_fn(middleware::set_health_group)),
        )
        .route(
            "/auth/login",
            post(auth::login).layer(from_fn(middleware::set_auth_group)),
        )
        .route_layer(from_fn_with_state(state.clone(), RateLimiter::middleware))
        .layer(security)
        .layer(cors)
        .layer(from_fn(middleware::trace_middleware))
        .with_state(state)
}
