use std::sync::Arc;

use axum::{extract::State, Json};

use crate::error::{AppError, Result};
use crate::models::UserInfoResponse;
use crate::AppState;

/// OpenID Connect UserInfo endpoint
/// GET /oauth2/userinfo
pub async fn userinfo(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract access token from Authorization header
) -> Result<Json<UserInfoResponse>> {
    // TODO: Implement UserInfo endpoint
    // 1. Extract and validate access token from Authorization header
    // 2. Get user from token claims
    // 3. Filter response based on granted scopes
    // 4. Decrypt encrypted fields as needed
    // 5. Return UserInfo response

    Err(AppError::Unauthorized)
}
