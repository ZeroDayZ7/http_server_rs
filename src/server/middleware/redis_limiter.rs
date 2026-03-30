use crate::config::rate_limit::RateLimitTier;
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

fn build_rate_limit_headers(limit: u64, current: u64) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let remaining = limit.saturating_sub(current);

    if let Ok(l) = HeaderValue::from_str(&limit.to_string()) {
        headers.insert("X-RateLimit-Limit", l);
    }
    if let Ok(r) = HeaderValue::from_str(&remaining.to_string()) {
        headers.insert("X-RateLimit-Remaining", r);
    }
    headers
}

pub async fn redis_rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = addr.ip().to_string();
    let path = req.uri().path();

    let tier = RateLimitTier::from_path(path);
    let (limit, window) = tier.get_limits(&state.settings.rate_limit);

    let key = state
        .redis_rate_limiter
        .make_key("api", &format!("{:?}:{}", tier, path), &ip);

    let rl_status = match state.redis_rate_limiter.check(&key, limit, window).await {
        Ok(status) => status,
        Err(e) => {
            error!(target: "infra::redis", %e, "Rate Limiter Error");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let headers = build_rate_limit_headers(limit, rl_status.current);

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
