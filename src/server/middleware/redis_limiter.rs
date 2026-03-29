use crate::server::state::AppState;
use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request, StatusCode},
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

    // 1. Dobór limitów na podstawie ścieżki
    let (limit, window) = match path {
        p if p.starts_with("/auth") => (rl.auth_burst as u64, 1),
        p if p.starts_with("/health") => (rl.health_burst as u64, 1),
        _ => (rl.global_burst as u64, 1),
    };

    let key = state.redis_rate_limiter.make_key("api", path, &ip);

    // 2. Sprawdzenie limitu w Redis
    let rl_status = match state.redis_rate_limiter.check(&key, limit, window).await {
        Ok(status) => status,
        Err(e) => {
            error!("Redis Rate Limiter Error: {}", e);
            // Jeśli Redis padnie, Senior zazwyczaj pozwala na przejście (fail-open)
            // lub zwraca 500. Tutaj zwracamy 500 dla bezpieczeństwa.
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // 3. Przygotowanie nagłówków informacyjnych
    let mut headers = HeaderMap::new();
    headers.insert("X-RateLimit-Limit", limit.into());
    let remaining = limit.saturating_sub(rl_status.current);
    headers.insert("X-RateLimit-Remaining", remaining.into());

    // 4. Obsługa blokady
    if !rl_status.allowed {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            headers,
            "Rate limit exceeded. Please try again later.",
        )
            .into_response();
    }

    // 5. Kontynuacja i doklejenie nagłówków do odpowiedzi sukcesu
    let mut response = next.run(req).await;
    response.headers_mut().extend(headers);

    response
}
