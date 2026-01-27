// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use axum::{Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login() -> Json<LoginResponse> {
    Json(LoginResponse {
        message: "Login successful".to_string(),
    })
}
