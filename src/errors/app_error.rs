use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Autoryzacja nie powiodła się")]
    Unauthorized,

    #[error("Nie znaleziono zasobu: {0}")]
    NotFound(String),

    #[error("Błędne dane wejściowe: {0}")]
    ValidationError(String),

    #[error("Błąd kryptograficzny: {0}")]
    CryptoError(String),

    #[error("Błąd bazy danych")]
    DatabaseError(#[source] mongodb::error::Error),

    #[error("Błąd usługi Redis")]
    RedisError(#[source] fred::error::Error),

    // "Catch-all" dla błędów, których nie przewidzieliśmy
    #[error("Wystąpił nieoczekiwany błąd serwera")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl AppError {
    // Mapowanie błędów na unikalne kody dla Front-endu
    fn error_code(&self) -> &'static str {
        match self {
            Self::Unauthorized => "AUTH_FAILED",
            Self::NotFound(_) => "RESOURCE_NOT_FOUND",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::CryptoError(_) => "CRYPTO_FAILURE",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::RedisError(_) => "CACHE_ERROR",
            Self::Internal(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = self.error_code();
        let message = self.to_string();

        let status = match &self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::CryptoError(_) => StatusCode::UNPROCESSABLE_ENTITY, // 422 przy błędach deszyfracji
            Self::DatabaseError(err) => {
                tracing::error!(target: "infra::db", %err, "MongoDB Error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::RedisError(err) => {
                tracing::error!(target: "infra::redis", %err, "Redis Error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Internal(err) => {
                tracing::error!(%err, "Unexpected Internal Error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        // W środowisku produkcyjnym nie chcemy wysyłać szczegółów błędów bazy do klienta
        let body = Json(ErrorResponse {
            code,
            message: message.into(),
            // Szczegóły wysyłamy TYLKO przy błędach walidacji/not found
            details: match &self {
                Self::ValidationError(d) | Self::NotFound(d) | Self::CryptoError(d) => {
                    Some(d.clone())
                }
                _ => None,
            },
        });

        (status, body).into_response()
    }
}

// Mapowania automatyczne (To usuwa potrzebę .map_err(|e| AppError::Internal(e.into())) w wielu miejscach)
impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::DatabaseError(err)
    }
}

impl From<fred::error::Error> for AppError {
    fn from(err: fred::error::Error) -> Self {
        Self::RedisError(err)
    }
}
