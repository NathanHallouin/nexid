use std::sync::Arc;

use axum::{extract::State, Json};
use validator::Validate;

use crate::crypto::{hash_password, verify_password};
use crate::db::users::NewUser;
use crate::error::{AppError, Result};
use crate::models::{LoginRequest, RegisterRequest, UserResponse};
use crate::AppState;

/// Register a new user
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Check if email already exists
    if state.db.find_user_by_email(&payload.email).await?.is_some() {
        return Err(AppError::BadRequest("Email already registered".to_string()));
    }

    // Check if username already exists
    if let Some(ref username) = payload.username {
        if state.db.find_user_by_username(username).await?.is_some() {
            return Err(AppError::BadRequest("Username already taken".to_string()));
        }
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = state
        .db
        .create_user(NewUser {
            email: payload.email,
            password_hash,
            preferred_username: payload.username,
        })
        .await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        email_verified: user.email_verified,
        username: user.preferred_username,
        created_at: user.created_at,
    }))
}

/// Login user (returns session or token)
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Find user by email
    let user = state
        .db
        .find_user_by_email(&payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    // Update last login
    state.db.update_last_login(user.id).await?;

    // TODO: Generate session/tokens
    Ok(Json(serde_json::json!({
        "message": "Login successful",
        "user_id": user.id.to_string(),
    })))
}

/// Get current user info
pub async fn get_me(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract user from JWT
) -> Result<Json<UserResponse>> {
    // TODO: Implement with JWT extraction
    Err(AppError::Unauthorized)
}

/// Delete current user account
pub async fn delete_me(
    State(_state): State<Arc<AppState>>,
    // TODO: Extract user from JWT
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement with JWT extraction
    Err(AppError::Unauthorized)
}
