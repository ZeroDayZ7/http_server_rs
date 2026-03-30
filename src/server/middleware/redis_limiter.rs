use crate::server::state::AppState;
use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderMap, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use tracing::error;

pub async fn redis_rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = addr.ip().to_string();
    let path = req.uri().path();
    let rl = &state.settings.rate_limit;

    let (limit, window) = match path {
        p if p.starts_with("/auth") => (rl.auth_burst as u64, 1),
        p if p.starts_with("/health") => (rl.health_burst as u64, 1),
        _ => (rl.global_burst as u64, 1),
    };

    let key = state.redis_rate_limiter.make_key("api", path, &ip);

    let rl_status = match state.redis_rate_limiter.check(&key, limit, window).await {
        Ok(status) => status,
        Err(e) => {
            error!("Redis Rate Limiter Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut headers = HeaderMap::new();
    let remaining = limit.saturating_sub(rl_status.current);

    if let Ok(limit_val) = HeaderValue::from_str(&limit.to_string()) {
        headers.insert("X-RateLimit-Limit", limit_val);
    }
    if let Ok(rem_val) = HeaderValue::from_str(&remaining.to_string()) {
        headers.insert("X-RateLimit-Remaining", rem_val);
    }

    if !rl_status.allowed {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            headers,
            "Rate limit exceeded. Please try again later.",
        )
            .into_response();
    }

    let mut response = next.run(req).await;
    response.headers_mut().extend(headers);
    response
}
