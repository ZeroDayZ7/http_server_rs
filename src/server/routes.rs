use crate::handlers::{auth, health};
use crate::server::middleware::{self, RateLimitLayers};
use crate::server::state::AppState;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router(state: AppState) -> Router {
    let cors = middleware::create_cors_layer(&state.settings);
    let security = middleware::create_security_headers_layer().into_inner();

    // wszystkie limity w jednym miejscu
    let rate_limits = RateLimitLayers::new(&state.settings);

    Router::new()
        .route(
            "/health",
            get(health::health).layer(rate_limits.health.clone()),
        )
        .route(
            "/auth/login",
            post(auth::login).layer(rate_limits.auth.clone()),
        )
        .route_layer(rate_limits.global.clone())
        .layer(security)
        .layer(cors)
        .layer(middleware::http_trace_layer())
        .with_state(state)
}
