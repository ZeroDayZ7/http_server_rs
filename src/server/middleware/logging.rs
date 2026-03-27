use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn trace_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    info!("started {} {}", method, path);

    let resp = next.run(req).await;

    info!("finished {} {} with status {}", method, path, resp.status());
    resp
}
