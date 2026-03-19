use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::AppState;

#[derive(Serialize)]
pub struct ConsentResponse {
    pub id: Uuid,
    pub client_name: String,
    pub client_uri: Option<String>,
    pub logo_uri: Option<String>,
    pub scopes: Vec<String>,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

/// List all active consents for the current user
pub async fn list_consents(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract user from JWT
) -> Result<Json<Vec<ConsentResponse>>> {
    // TODO: Implement with JWT extraction
    Err(AppError::Unauthorized)
}

/// Revoke a specific consent
pub async fn revoke_consent(
    State(_state): State<Arc<AppState>>,
    Path(_consent_id): Path<Uuid>,
    // TODO: Extract user from JWT
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement with JWT extraction
    Err(AppError::Unauthorized)
}
