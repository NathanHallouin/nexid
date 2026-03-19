use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::AppState;

#[derive(Serialize)]
pub struct ExportResponse {
    pub request_id: Uuid,
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub request_id: Uuid,
    pub status: &'static str,
    pub message: &'static str,
}

/// Export all user data (GDPR Article 20 - Data Portability)
pub async fn export_data(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract user from JWT
) -> Result<Json<ExportResponse>> {
    // TODO: Implement with JWT extraction
    // This should:
    // 1. Create a GDPR export request in the database
    // 2. Queue a background job to collect all user data
    // 3. Encrypt the export file
    // 4. Send email with download link when ready
    Err(AppError::Unauthorized)
}

/// Request account deletion (GDPR Article 17 - Right to Erasure)
pub async fn request_deletion(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract user from JWT
) -> Result<Json<DeleteResponse>> {
    // TODO: Implement with JWT extraction
    // This should:
    // 1. Create a GDPR delete request in the database
    // 2. Send confirmation email
    // 3. After confirmation, soft-delete user and revoke all consents
    // 4. Queue hard deletion after retention period (e.g., 30 days)
    Err(AppError::Unauthorized)
}
