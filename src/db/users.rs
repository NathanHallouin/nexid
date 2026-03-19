use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::Database;
use crate::error::{AppError, Result};

/// User record from database
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,

    // Encrypted fields
    pub given_name_encrypted: Option<String>,
    pub family_name_encrypted: Option<String>,
    pub nickname: Option<String>,
    pub preferred_username: Option<String>,
    pub picture_url: Option<String>,
    pub locale: String,
    pub birthdate_encrypted: Option<String>,
    pub phone_number_encrypted: Option<String>,
    pub phone_number_verified: bool,
    pub address_encrypted: Option<serde_json::Value>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// New user creation data
#[derive(Debug)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub preferred_username: Option<String>,
}

impl Database {
    /// Create a new user
    pub async fn create_user(&self, new_user: NewUser) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, preferred_username)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(&new_user.email)
        .bind(&new_user.password_hash)
        .bind(&new_user.preferred_username)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by username
    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE preferred_username = $1 AND deleted_at IS NULL",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update user's last login timestamp
    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Soft delete a user (GDPR compliance)
    pub async fn soft_delete_user(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users SET
                deleted_at = NOW(),
                email = CONCAT('deleted_', id::text, '@deleted.nexid.io'),
                password_hash = 'DELETED',
                given_name_encrypted = NULL,
                family_name_encrypted = NULL,
                nickname = NULL,
                preferred_username = NULL,
                picture_url = NULL,
                birthdate_encrypted = NULL,
                phone_number_encrypted = NULL,
                address_encrypted = NULL
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update user profile fields
    pub async fn update_user_profile(
        &self,
        user_id: Uuid,
        given_name_encrypted: Option<String>,
        family_name_encrypted: Option<String>,
        nickname: Option<String>,
        picture_url: Option<String>,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                given_name_encrypted = COALESCE($2, given_name_encrypted),
                family_name_encrypted = COALESCE($3, family_name_encrypted),
                nickname = COALESCE($4, nickname),
                picture_url = COALESCE($5, picture_url),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(given_name_encrypted)
        .bind(family_name_encrypted)
        .bind(nickname)
        .bind(picture_url)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
