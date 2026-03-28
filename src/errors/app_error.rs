use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Brak autoryzacji")]
    Unauthorized,

    #[error("Błędne zapytanie: {0}")]
    BadRequest(String),

    #[error("Błąd wewnętrzny serwera")]
    Internal(#[from] anyhow::Error),

    #[error("Błąd bazy danych")]
    DatabaseError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(err) => {
                tracing::error!(%err, "Internal Server Error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::DatabaseError(err) => {
                tracing::error!(target: "database", %err, "Database failure");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}

impl From<fred::error::Error> for AppError {
    fn from(err: fred::error::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(err))
    }
}
