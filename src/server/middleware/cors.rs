use crate::config::Settings;
use axum::http::{HeaderValue, Method};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer(settings: &Settings) -> CorsLayer {
    let origin = settings
        .cors
        .allowed_origin
        .parse::<HeaderValue>()
        .expect("Invalid CORS origin");

    let methods = settings
        .cors
        .allowed_methods
        .split(',')
        .map(|s| s.trim().parse::<Method>().expect("Invalid HTTP method"))
        .collect::<Vec<Method>>();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(methods)
        .allow_headers(Any)
        .max_age(Duration::from_secs(settings.cors.max_age))
}
