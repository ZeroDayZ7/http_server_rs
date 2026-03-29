use crate::domain::vault::DecryptedCV;
use crate::errors::AppResult;
use crate::server::state::AppState;
use axum::{Json, extract::State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UnlockRequest {
    pub cv_id: String,
    pub access_key: String,
}

pub async fn unlock_cv(
    State(state): State<AppState>,
    Json(payload): Json<UnlockRequest>,
) -> AppResult<Json<DecryptedCV>> {
    // Używamy gotowego serwisu ze stanu aplikacji
    let cv = state
        .vault_service
        .unlock_cv(&payload.cv_id, &payload.access_key)
        .await?;

    Ok(Json(cv))
}
