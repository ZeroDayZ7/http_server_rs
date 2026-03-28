use crate::domain::vault::DecryptedCV;
use crate::errors::AppResult;
use crate::server::state::AppState;
use crate::services::vault_service::VaultService;
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
    let service = VaultService::new(state.db_repo.clone());

    let cv = service
        .unlock_cv(&payload.cv_id, &payload.access_key)
        .await?;

    Ok(Json(cv))
}
