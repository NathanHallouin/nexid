use std::sync::Arc;

use axum::{extract::State, Form, Json};

use crate::models::{TokenRequest, TokenResponse, OAuth2Error};
use crate::AppState;

/// OAuth2 Token endpoint
/// POST /oauth2/token
pub async fn token(
    State(state): State<Arc<AppState>>,
    Form(params): Form<TokenRequest>,
) -> Result<Json<TokenResponse>, Json<OAuth2Error>> {
    match params.grant_type.as_str() {
        "authorization_code" => handle_authorization_code(&state, &params).await,
        "refresh_token" => handle_refresh_token(&state, &params).await,
        _ => Err(Json(OAuth2Error {
            error: "unsupported_grant_type".to_string(),
            error_description: Some("Only authorization_code and refresh_token are supported".to_string()),
            error_uri: None,
        })),
    }
}

async fn handle_authorization_code(
    _state: &Arc<AppState>,
    params: &TokenRequest,
) -> Result<Json<TokenResponse>, Json<OAuth2Error>> {
    // Validate required parameters
    let _code = params.code.as_ref().ok_or_else(|| {
        Json(OAuth2Error {
            error: "invalid_request".to_string(),
            error_description: Some("Missing code parameter".to_string()),
            error_uri: None,
        })
    })?;

    let _redirect_uri = params.redirect_uri.as_ref().ok_or_else(|| {
        Json(OAuth2Error {
            error: "invalid_request".to_string(),
            error_description: Some("Missing redirect_uri parameter".to_string()),
            error_uri: None,
        })
    })?;

    let _code_verifier = params.code_verifier.as_ref().ok_or_else(|| {
        Json(OAuth2Error {
            error: "invalid_request".to_string(),
            error_description: Some("Missing code_verifier parameter (PKCE required)".to_string()),
            error_uri: None,
        })
    })?;

    // TODO: Implement authorization code exchange
    // 1. Find authorization code in database
    // 2. Verify code_verifier against stored code_challenge
    // 3. Verify redirect_uri matches
    // 4. Verify code hasn't expired
    // 5. Mark code as used
    // 6. Generate access token, refresh token, and optionally id_token
    // 7. Return token response

    Err(Json(OAuth2Error {
        error: "server_error".to_string(),
        error_description: Some("Token endpoint not fully implemented".to_string()),
        error_uri: None,
    }))
}

async fn handle_refresh_token(
    _state: &Arc<AppState>,
    params: &TokenRequest,
) -> Result<Json<TokenResponse>, Json<OAuth2Error>> {
    let _refresh_token = params.refresh_token.as_ref().ok_or_else(|| {
        Json(OAuth2Error {
            error: "invalid_request".to_string(),
            error_description: Some("Missing refresh_token parameter".to_string()),
            error_uri: None,
        })
    })?;

    // TODO: Implement refresh token exchange
    // 1. Find refresh token in database
    // 2. Verify token hasn't expired or been revoked
    // 3. Generate new access token
    // 4. Optionally rotate refresh token
    // 5. Return token response

    Err(Json(OAuth2Error {
        error: "server_error".to_string(),
        error_description: Some("Refresh token exchange not fully implemented".to_string()),
        error_uri: None,
    }))
}
