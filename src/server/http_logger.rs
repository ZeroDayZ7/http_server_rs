// src/server/http_logger.rs
use axum::http::{Request, Response};
use tower_http::trace::TraceLayer;
use tracing::Span;
use std::time::Duration;

pub fn http_trace_layer() -> impl tower::Layer<axum::Router> + Clone {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<axum::body::Body>| {
            tracing::info_span!(
                "http-request",
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(|request: &Request<axum::body::Body>, _span: &Span| {
            tracing::info!("started {} {}", request.method(), request.uri().path());
        })
        .on_response(|response: &Response<axum::body::Body>, latency: Duration, _span: &Span| {
            tracing::info!(
                status = %response.status(),
                latency = ?latency,
                "finished processing"
            );
        })
}
