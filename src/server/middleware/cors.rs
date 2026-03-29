use crate::config::{HttpMethod, Settings};
use axum::http::{HeaderValue, Method};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer(settings: &Settings) -> CorsLayer {
    let origin = settings
        .cors
        .allowed_origin
        .parse::<HeaderValue>()
        .expect("Invalid CORS origin");

    let methods: Vec<Method> = settings
        .cors
        .allowed_methods
        .iter()
        .map(|m| match m {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Options => Method::OPTIONS,
        })
        .collect();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(methods)
        .allow_headers(Any)
        .max_age(Duration::from_secs(settings.cors.max_age))
}
