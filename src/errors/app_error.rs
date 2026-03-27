// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    BadRequest(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };

        let body = Json(ErrorResponse { error: msg });
        (status, body).into_response()
    }
}
