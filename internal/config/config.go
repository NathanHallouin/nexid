package config

import (
	"os"
	"strconv"
	"strings"
	"time"
)

// Config holds all application configuration
type Config struct {
	// Server
	Environment string
	Port        int
	Version     string

	// Database
	DatabaseURL string

	// Redis
	RedisURL string

	// Security
	JWTSecret          string
	JWTAccessTokenTTL  time.Duration
	JWTRefreshTokenTTL time.Duration
	EncryptionKey      string // 32 bytes for AES-256

	// Rate Limiting
	RateLimit      int
	RateLimitBurst int

	// CORS
	AllowedOrigins []string

	// OAuth2
	Issuer string
}

// Load reads configuration from environment variables
func Load() (*Config, error) {
	return &Config{
		Environment:        getEnv("ENVIRONMENT", "development"),
		Port:               getEnvInt("PORT", 8080),
		Version:            getEnv("VERSION", "0.1.0"),
		DatabaseURL:        getEnv("DATABASE_URL", "postgres://localhost:5432/nexid?sslmode=disable"),
		RedisURL:           getEnv("REDIS_URL", "redis://localhost:6379"),
		JWTSecret:          getEnv("JWT_SECRET", "change-me-in-production"),
		JWTAccessTokenTTL:  getEnvDuration("JWT_ACCESS_TOKEN_TTL", 15*time.Minute),
		JWTRefreshTokenTTL: getEnvDuration("JWT_REFRESH_TOKEN_TTL", 7*24*time.Hour),
		EncryptionKey:      getEnv("ENCRYPTION_KEY", ""),
		RateLimit:          getEnvInt("RATE_LIMIT", 100),
		RateLimitBurst:     getEnvInt("RATE_LIMIT_BURST", 10),
		AllowedOrigins:     getEnvSlice("ALLOWED_ORIGINS", []string{"http://localhost:3000"}),
		Issuer:             getEnv("ISSUER", "https://nexid.io"),
	}, nil
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func getEnvInt(key string, defaultValue int) int {
	if value := os.Getenv(key); value != "" {
		if intValue, err := strconv.Atoi(value); err == nil {
			return intValue
		}
	}
	return defaultValue
}

func getEnvDuration(key string, defaultValue time.Duration) time.Duration {
	if value := os.Getenv(key); value != "" {
		if duration, err := time.ParseDuration(value); err == nil {
			return duration
		}
	}
	return defaultValue
}

func getEnvSlice(key string, defaultValue []string) []string {
	if value := os.Getenv(key); value != "" {
		return strings.Split(value, ",")
	}
	return defaultValue
}
