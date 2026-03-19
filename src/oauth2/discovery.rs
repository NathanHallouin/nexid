use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::AppState;

/// OpenID Connect Discovery document
#[derive(Serialize)]
pub struct OpenIDConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub revocation_endpoint: String,
    pub jwks_uri: String,
    pub response_types_supported: Vec<&'static str>,
    pub grant_types_supported: Vec<&'static str>,
    pub subject_types_supported: Vec<&'static str>,
    pub id_token_signing_alg_values_supported: Vec<&'static str>,
    pub scopes_supported: Vec<&'static str>,
    pub token_endpoint_auth_methods_supported: Vec<&'static str>,
    pub claims_supported: Vec<&'static str>,
    pub code_challenge_methods_supported: Vec<&'static str>,
    pub request_parameter_supported: bool,
    pub request_uri_parameter_supported: bool,
}

/// JSON Web Key Set
#[derive(Serialize)]
pub struct JWKS {
    pub keys: Vec<JWK>,
}

#[derive(Serialize)]
pub struct JWK {
    pub kty: String,
    pub use_: String,
    pub kid: String,
    pub alg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

/// OpenID Connect Discovery endpoint
/// GET /oauth2/.well-known/openid-configuration
pub async fn openid_configuration(
    State(state): State<Arc<AppState>>,
) -> Json<OpenIDConfiguration> {
    let issuer = &state.config.issuer;

    Json(OpenIDConfiguration {
        issuer: issuer.clone(),
        authorization_endpoint: format!("{}/oauth2/authorize", issuer),
        token_endpoint: format!("{}/oauth2/token", issuer),
        userinfo_endpoint: format!("{}/oauth2/userinfo", issuer),
        revocation_endpoint: format!("{}/oauth2/revoke", issuer),
        jwks_uri: format!("{}/.well-known/jwks.json", issuer),
        response_types_supported: vec!["code"],
        grant_types_supported: vec!["authorization_code", "refresh_token"],
        subject_types_supported: vec!["public", "pairwise"],
        id_token_signing_alg_values_supported: vec!["RS256", "ES256"],
        scopes_supported: vec!["openid", "profile", "email", "address", "phone"],
        token_endpoint_auth_methods_supported: vec![
            "client_secret_basic",
            "client_secret_post",
            "private_key_jwt",
        ],
        claims_supported: vec![
            "sub",
            "iss",
            "aud",
            "exp",
            "iat",
            "auth_time",
            "name",
            "given_name",
            "family_name",
            "nickname",
            "preferred_username",
            "email",
            "email_verified",
            "phone_number",
            "phone_number_verified",
            "address",
            "birthdate",
            "locale",
        ],
        code_challenge_methods_supported: vec!["S256"],
        request_parameter_supported: true,
        request_uri_parameter_supported: false,
    })
}

/// JSON Web Key Set endpoint
/// GET /.well-known/jwks.json
pub async fn jwks(State(_state): State<Arc<AppState>>) -> Json<JWKS> {
    // TODO: Fetch active signing keys from database and return as JWKs
    Json(JWKS { keys: vec![] })
}
