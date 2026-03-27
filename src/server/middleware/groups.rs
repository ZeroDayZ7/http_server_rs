use axum::{extract::Request, middleware::Next, response::Response};

pub async fn set_health_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("health");
    next.run(req).await
}

pub async fn set_auth_group(mut req: Request, next: Next) -> Response {
    req.extensions_mut().insert("auth");
    next.run(req).await
}
