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
    let path = req.uri().path().to_string();

    // Używamy Twojego serwisu do wygenerowania klucza
    let key = state.redis_rate_limiter.make_key("api", &path, &ip);

    // Wywołujemy Twój skrypt Lua (np. 100 żądań na 60 sekund)
    // Możesz te wartości (100, 60) wyciągnąć z state.settings.rate_limit
    let is_allowed = state
        .redis_rate_limiter
        .check(&key, 100, 60)
        .await
        .map_err(|e| {
            tracing::error!("Redis Rate Limiter error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !is_allowed {
        tracing::warn!("Rate limit exceeded in Redis for IP: {}", ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}
