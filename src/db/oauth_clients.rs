use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::Database;
use crate::error::Result;

/// OAuth2 client record
#[derive(Debug, Clone, FromRow)]
pub struct OAuthClient {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub client_id: String,
    pub client_secret_hash: String,
    pub client_name: String,
    pub client_description: Option<String>,
    pub client_uri: Option<String>,
    pub logo_uri: Option<String>,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub response_types: Vec<String>,
    pub scopes: Vec<String>,
    pub access_token_ttl: i32,
    pub refresh_token_ttl: i32,
    pub token_endpoint_auth_method: String,
    pub require_pkce: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

/// New OAuth client creation data
#[derive(Debug)]
pub struct NewOAuthClient {
    pub owner_id: Uuid,
    pub client_id: String,
    pub client_secret_hash: String,
    pub client_name: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}

impl Database {
    /// Create a new OAuth client
    pub async fn create_oauth_client(&self, client: NewOAuthClient) -> Result<OAuthClient> {
        let result = sqlx::query_as::<_, OAuthClient>(
            r#"
            INSERT INTO oauth_clients (owner_id, client_id, client_secret_hash, client_name, redirect_uris, scopes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&client.owner_id)
        .bind(&client.client_id)
        .bind(&client.client_secret_hash)
        .bind(&client.client_name)
        .bind(&client.redirect_uris)
        .bind(&client.scopes)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Find OAuth client by client_id
    pub async fn find_oauth_client(&self, client_id: &str) -> Result<Option<OAuthClient>> {
        let client = sqlx::query_as::<_, OAuthClient>(
            "SELECT * FROM oauth_clients WHERE client_id = $1 AND is_active = true",
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(client)
    }

    /// List OAuth clients by owner
    pub async fn list_oauth_clients_by_owner(&self, owner_id: Uuid) -> Result<Vec<OAuthClient>> {
        let clients = sqlx::query_as::<_, OAuthClient>(
            "SELECT * FROM oauth_clients WHERE owner_id = $1 ORDER BY created_at DESC",
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    /// Deactivate an OAuth client
    pub async fn deactivate_oauth_client(&self, client_id: &str, owner_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE oauth_clients SET is_active = false, updated_at = NOW() WHERE client_id = $1 AND owner_id = $2",
        )
        .bind(client_id)
        .bind(owner_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
