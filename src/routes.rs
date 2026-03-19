use std::sync::Arc;

use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::handlers::{consents, gdpr, health, users};
use crate::oauth2;
use crate::AppState;

/// Health check routes
pub fn health_routes() -> Router {
    Router::new().route("/health", get(health::health_check))
}

/// OAuth2 / OpenID Connect routes
pub fn oauth2_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/oauth2/authorize", get(oauth2::authorize))
        .route("/oauth2/token", post(oauth2::token))
        .route("/oauth2/userinfo", get(oauth2::userinfo))
        .route(
            "/oauth2/.well-known/openid-configuration",
            get(oauth2::openid_configuration),
        )
        .route("/.well-known/jwks.json", get(oauth2::jwks))
        .with_state(state)
}

/// API v1 routes
pub fn api_v1_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Users
        .route("/api/v1/users", post(users::register))
        .route("/api/v1/auth/login", post(users::login))
        .route("/api/v1/users/me", get(users::get_me))
        .route("/api/v1/users/me", delete(users::delete_me))
        // Consents
        .route("/api/v1/consents", get(consents::list_consents))
        .route("/api/v1/consents/:id", delete(consents::revoke_consent))
        // GDPR
        .route("/api/v1/gdpr/export", post(gdpr::export_data))
        .route("/api/v1/gdpr/delete", post(gdpr::request_deletion))
        .with_state(state)
}
