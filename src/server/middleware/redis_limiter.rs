use crate::server::state::AppState;
use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

pub async fn redis_rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip().to_string();
    let path = req.uri().path();
    let rl = &state.settings.rate_limit;

    let (limit, window) = match path {
        p if p.starts_with("/auth") => (rl.auth_burst as u64, 1),
        p if p.starts_with("/health") => (rl.health_burst as u64, 1),
        _ => (rl.global_burst as u64, 1),
    };

    let key = state.redis_rate_limiter.make_key("api", path, &ip);

    let is_allowed = state
        .redis_rate_limiter
        .check(&key, limit, window)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_allowed {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}
