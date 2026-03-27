use crate::config::Settings;
use crate::handlers::{auth, health};
use crate::server::rate_limiter::{RateLimiter, SharedLimiter};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::{
    Router,
    extract::Request,
    middleware::{Next, from_fn, from_fn_with_state},
    response::Response,
    routing::{get, post},
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::info;

// -----------------------------------------------------------------------------
// Middleware globalne (Logging)
// -----------------------------------------------------------------------------
async fn trace_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    info!("started {} {}", method, path);
    let resp = next.run(req).await;
    info!("finished {} {} with status {}", method, path, resp.status());
    resp
}

// -----------------------------------------------------------------------------
// Middleware per route (Grupowanie dla Rate Limitera)
// -----------------------------------------------------------------------------
async fn set_health_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("health");
    next.run(req).await
}

async fn set_auth_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("auth");
    next.run(req).await
}

// -----------------------------------------------------------------------------
// Fabryka warstwy CORS
// -----------------------------------------------------------------------------
fn create_cors_layer(settings: &Settings) -> CorsLayer {
    let origin = settings
        .cors
        .allowed_origin
        .parse::<HeaderValue>()
        .expect("Invalid CORS origin in .env");

    let methods = settings
        .cors
        .allowed_methods
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<Method>()
                .expect("Invalid HTTP method in .env")
        })
        .collect::<Vec<Method>>();

    CorsLayer::new()
        // FIX: allow_origin oczekuje Iterable (np. tablicy) lub konkretnego typu
        .allow_origin(origin)
        .allow_methods(methods)
        .allow_headers(Any)
        .max_age(Duration::from_secs(settings.cors.max_age))
}

fn create_security_headers_layer() -> ServiceBuilder<
    tower::layer::util::Stack<
        SetResponseHeaderLayer<HeaderValue>,
        tower::layer::util::Stack<
            SetResponseHeaderLayer<HeaderValue>,
            tower::layer::util::Stack<
                SetResponseHeaderLayer<HeaderValue>,
                tower::layer::util::Stack<
                    SetResponseHeaderLayer<HeaderValue>,
                    tower::layer::util::Identity,
                >,
            >,
        >,
    >,
> {
    ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; frame-ancestors 'none';"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
}

pub fn router(limiter: SharedLimiter, settings: Settings) -> Router {
    let cors = create_cors_layer(&settings);
    let security_layer = create_security_headers_layer().into_inner();

    Router::new()
        .route(
            "/health",
            get(health::health).layer(from_fn(set_health_group)),
        )
        .route(
            "/auth/login",
            post(auth::login).layer(from_fn(set_auth_group)),
        )
        .route_layer(from_fn_with_state(limiter.clone(), RateLimiter::middleware))
        .layer(security_layer)
        .layer(cors)
        .layer(from_fn(trace_middleware))
        .with_state(limiter)
}
