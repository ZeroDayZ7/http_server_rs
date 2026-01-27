// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{routing::get, Router};
use crate::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(handlers::health::health))
}
