use std::time::Duration;

use serde::Deserialize;

/// Application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Environment (development, staging, production)
    #[serde(default = "default_environment")]
    pub environment: String,

    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Application version
    #[serde(default = "default_version")]
    pub version: String,

    /// PostgreSQL connection URL
    pub database_url: String,

    /// Redis connection URL
    pub redis_url: String,

    /// JWT signing secret (base64 encoded)
    pub jwt_secret: String,

    /// Access token TTL in seconds
    #[serde(default = "default_access_token_ttl")]
    pub access_token_ttl: u64,

    /// Refresh token TTL in seconds
    #[serde(default = "default_refresh_token_ttl")]
    pub refresh_token_ttl: u64,

    /// AES-256 encryption key (base64 encoded, 32 bytes)
    pub encryption_key: String,

    /// OAuth2 issuer URL
    #[serde(default = "default_issuer")]
    pub issuer: String,

    /// Allowed CORS origins
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,

    /// Rate limit (requests per minute)
    #[serde(default = "default_rate_limit")]
    pub rate_limit: u32,
}

fn default_environment() -> String {
    "development".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn default_access_token_ttl() -> u64 {
    900 // 15 minutes
}

fn default_refresh_token_ttl() -> u64 {
    604800 // 7 days
}

fn default_issuer() -> String {
    "http://localhost:8080".to_string()
}

fn default_allowed_origins() -> Vec<String> {
    vec!["http://localhost:3000".to_string()]
}

fn default_rate_limit() -> u32 {
    100
}

impl Config {
    /// Load configuration from environment variables
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        let settings: Config = config.try_deserialize()?;
        Ok(settings)
    }

    /// Get access token TTL as Duration
    pub fn access_token_duration(&self) -> Duration {
        Duration::from_secs(self.access_token_ttl)
    }

    /// Get refresh token TTL as Duration
    pub fn refresh_token_duration(&self) -> Duration {
        Duration::from_secs(self.refresh_token_ttl)
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
