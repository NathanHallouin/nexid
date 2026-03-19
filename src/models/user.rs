use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Validate username format (alphanumeric + underscore only)
fn validate_username(username: &str) -> Result<(), validator::ValidationError> {
    if username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_username"))
    }
}

/// User registration request
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(
        min = 12,
        max = 128,
        message = "Password must be between 12 and 128 characters"
    ))]
    pub password: String,

    #[validate(length(
        min = 3,
        max = 30,
        message = "Username must be between 3 and 30 characters"
    ))]
    #[validate(custom(function = "validate_username"))]
    pub username: Option<String>,
}

/// User login request
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// User profile update request
#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(max = 100))]
    pub given_name: Option<String>,

    #[validate(length(max = 100))]
    pub family_name: Option<String>,

    #[validate(length(max = 100))]
    pub nickname: Option<String>,

    #[validate(url)]
    pub picture_url: Option<String>,
}

/// User response (public representation)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub username: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// User info response (OpenID Connect UserInfo)
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
