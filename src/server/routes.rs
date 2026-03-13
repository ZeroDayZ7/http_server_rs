use axum::{
    Router,
    routing::{get, post},
    middleware::{from_fn, from_fn_with_state, Next},
    response::Response,
    extract::Request,
};
use crate::server::rate_limiter::{RateLimiter, SharedLimiter};
use crate::handlers::{health, auth};
use tracing::info;

// ------------------------
// Middleware globalne
// ------------------------
async fn trace_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    info!("started {} {}", method, path);

    let resp = next.run(req).await;

    info!("finished {} {} with status {}", method, path, resp.status());
    resp
}

// ------------------------
// Middleware per route
// ------------------------
async fn set_health_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("health");
    next.run(req).await
}

async fn set_auth_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("auth");
    next.run(req).await
}

// ------------------------
// Router
// ------------------------
pub fn router(limiter: SharedLimiter) -> Router {
    Router::new()
        .route("/health", get(health::health).layer(from_fn(set_health_group)))
        .route("/auth/login", post(auth::login).layer(from_fn(set_auth_group)))
        .route_layer(from_fn_with_state(limiter.clone(), RateLimiter::middleware))
        .layer(from_fn(trace_middleware))
        .with_state(limiter)
}
