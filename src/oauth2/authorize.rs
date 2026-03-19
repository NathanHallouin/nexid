use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};

use crate::error::AppError;
use crate::models::AuthorizationRequest;
use crate::AppState;

/// OAuth2 Authorization endpoint
/// GET /oauth2/authorize
pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthorizationRequest>,
) -> Response {
    // Validate request
    if let Err(e) = validate_authorization_request(&state, &params).await {
        return e.into_response();
    }

    // TODO: Check if user is authenticated
    // If not, redirect to login page with return URL

    // TODO: Check if user has already granted consent for these scopes
    // If yes, generate authorization code and redirect

    // TODO: If not, show consent page

    // For now, return error
    AppError::BadRequest("Authorization endpoint not fully implemented".to_string()).into_response()
}

async fn validate_authorization_request(
    state: &Arc<AppState>,
    params: &AuthorizationRequest,
) -> Result<(), AppError> {
    // Check response_type
    if params.response_type != "code" {
        return Err(AppError::BadRequest("Only response_type=code is supported".to_string()));
    }

    // Check code_challenge_method (PKCE required)
    if params.code_challenge_method != "S256" {
        return Err(AppError::BadRequest(
            "Only code_challenge_method=S256 is supported (PKCE required)".to_string(),
        ));
    }

    // Verify client exists
    let client = state
        .db
        .find_oauth_client(&params.client_id)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid client_id".to_string()))?;

    // Verify redirect_uri matches registered URIs
    if !client.redirect_uris.contains(&params.redirect_uri) {
        return Err(AppError::BadRequest("Invalid redirect_uri".to_string()));
    }

    // Validate scopes
    let requested_scopes: Vec<&str> = params.scope.split_whitespace().collect();
    for scope in &requested_scopes {
        if !client.scopes.iter().any(|s| s == *scope) {
            return Err(AppError::BadRequest(format!("Invalid scope: {}", scope)));
        }
    }

    Ok(())
}
