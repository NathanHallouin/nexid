use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::Database;
use crate::error::Result;

/// User consent record
#[derive(Debug, Clone, FromRow)]
pub struct Consent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub scopes: Vec<String>,
    pub granted_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// Consent with client details
#[derive(Debug, Clone, FromRow)]
pub struct ConsentWithClient {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub scopes: Vec<String>,
    pub granted_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub client_name: String,
    pub client_uri: Option<String>,
    pub logo_uri: Option<String>,
}

impl Database {
    /// Grant or update consent
    pub async fn upsert_consent(
        &self,
        user_id: Uuid,
        client_id: Uuid,
        scopes: Vec<String>,
    ) -> Result<Consent> {
        let consent = sqlx::query_as::<_, Consent>(
            r#"
            INSERT INTO consents (user_id, client_id, scopes)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, client_id)
            DO UPDATE SET scopes = $3, granted_at = NOW(), revoked_at = NULL
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(client_id)
        .bind(&scopes)
        .fetch_one(&self.pool)
        .await?;

        Ok(consent)
    }

    /// Check if user has granted consent for specific scopes
    pub async fn has_consent(
        &self,
        user_id: Uuid,
        client_id: Uuid,
        required_scopes: &[String],
    ) -> Result<bool> {
        let consent = sqlx::query_as::<_, Consent>(
            "SELECT * FROM consents WHERE user_id = $1 AND client_id = $2 AND revoked_at IS NULL",
        )
        .bind(user_id)
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await?;

        match consent {
            Some(c) => Ok(required_scopes.iter().all(|s| c.scopes.contains(s))),
            None => Ok(false),
        }
    }

    /// List all active consents for a user
    pub async fn list_user_consents(&self, user_id: Uuid) -> Result<Vec<ConsentWithClient>> {
        let consents = sqlx::query_as::<_, ConsentWithClient>(
            r#"
            SELECT c.*, oc.client_name, oc.client_uri, oc.logo_uri
            FROM consents c
            JOIN oauth_clients oc ON c.client_id = oc.id
            WHERE c.user_id = $1 AND c.revoked_at IS NULL
            ORDER BY c.granted_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(consents)
    }

    /// Revoke a consent
    pub async fn revoke_consent(&self, consent_id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE consents SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL",
        )
        .bind(consent_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Revoke all consents for a user (used during account deletion)
    pub async fn revoke_all_user_consents(&self, user_id: Uuid) -> Result<u64> {
        let result = sqlx::query(
            "UPDATE consents SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
