use std::time::Duration;

use serde::Deserialize;

/// Raw configuration from environment
#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    #[serde(default = "default_environment")]
    environment: String,

    #[serde(default = "default_port")]
    port: u16,

    #[serde(default = "default_version")]
    version: String,

    database_url: String,
    redis_url: String,
    jwt_secret: String,

    #[serde(default = "default_access_token_ttl")]
    access_token_ttl: u64,

    #[serde(default = "default_refresh_token_ttl")]
    refresh_token_ttl: u64,

    encryption_key: String,

    #[serde(default = "default_issuer")]
    issuer: String,

    /// Comma-separated list of allowed origins
    #[serde(default = "default_allowed_origins_str")]
    allowed_origins: String,

    #[serde(default = "default_rate_limit")]
    rate_limit: u32,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Environment (development, staging, production)
    pub environment: String,

    /// Server port
    pub port: u16,

    /// Application version
    pub version: String,

    /// PostgreSQL connection URL
    pub database_url: String,

    /// Redis connection URL
    pub redis_url: String,

    /// JWT signing secret (base64 encoded)
    pub jwt_secret: String,

    /// Access token TTL in seconds
    pub access_token_ttl: u64,

    /// Refresh token TTL in seconds
    pub refresh_token_ttl: u64,

    /// AES-256 encryption key (base64 encoded, 32 bytes)
    pub encryption_key: String,

    /// OAuth2 issuer URL
    pub issuer: String,

    /// Allowed CORS origins
    pub allowed_origins: Vec<String>,

    /// Rate limit (requests per minute)
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

fn default_allowed_origins_str() -> String {
    "http://localhost:3000".to_string()
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

        let raw: RawConfig = config.try_deserialize()?;

        // Parse comma-separated origins
        let allowed_origins: Vec<String> = raw
            .allowed_origins
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Config {
            environment: raw.environment,
            port: raw.port,
            version: raw.version,
            database_url: raw.database_url,
            redis_url: raw.redis_url,
            jwt_secret: raw.jwt_secret,
            access_token_ttl: raw.access_token_ttl,
            refresh_token_ttl: raw.refresh_token_ttl,
            encryption_key: raw.encryption_key,
            issuer: raw.issuer,
            allowed_origins,
            rate_limit: raw.rate_limit,
        })
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
