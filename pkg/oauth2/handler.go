package oauth2

import (
	"net/http"

	"github.com/NathanHallouin/nexid/internal/config"
	"github.com/NathanHallouin/nexid/internal/repository"
	"github.com/gin-gonic/gin"
	"github.com/redis/go-redis/v9"
	"go.uber.org/zap"
)

// Handler handles OAuth2/OIDC endpoints
type Handler struct {
	db     *repository.PostgresDB
	redis  *redis.Client
	config *config.Config
	logger *zap.Logger
}

// NewHandler creates a new OAuth2 handler
func NewHandler(db *repository.PostgresDB, redis *redis.Client, cfg *config.Config, logger *zap.Logger) *Handler {
	return &Handler{
		db:     db,
		redis:  redis,
		config: cfg,
		logger: logger,
	}
}

// Authorize handles the authorization endpoint
// GET /oauth2/authorize
func (h *Handler) Authorize(c *gin.Context) {
	// OAuth 2.1 Authorization Code Flow with PKCE
	// Required params: response_type, client_id, redirect_uri, scope, state, code_challenge, code_challenge_method
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not_implemented"})
}

// Token handles the token endpoint
// POST /oauth2/token
func (h *Handler) Token(c *gin.Context) {
	// Exchange authorization code for tokens
	// Supported grant types: authorization_code, refresh_token
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not_implemented"})
}

// Revoke handles token revocation
// POST /oauth2/revoke
func (h *Handler) Revoke(c *gin.Context) {
	// RFC 7009 Token Revocation
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not_implemented"})
}

// UserInfo returns claims about the authenticated user
// GET /oauth2/userinfo
func (h *Handler) UserInfo(c *gin.Context) {
	// OpenID Connect UserInfo endpoint
	c.JSON(http.StatusNotImplemented, gin.H{"error": "not_implemented"})
}

// Discovery returns the OpenID Connect discovery document
// GET /oauth2/.well-known/openid-configuration
func (h *Handler) Discovery(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"issuer":                                h.config.Issuer,
		"authorization_endpoint":               h.config.Issuer + "/oauth2/authorize",
		"token_endpoint":                       h.config.Issuer + "/oauth2/token",
		"userinfo_endpoint":                    h.config.Issuer + "/oauth2/userinfo",
		"revocation_endpoint":                  h.config.Issuer + "/oauth2/revoke",
		"jwks_uri":                             h.config.Issuer + "/oauth2/.well-known/jwks.json",
		"response_types_supported":             []string{"code"},
		"grant_types_supported":                []string{"authorization_code", "refresh_token"},
		"subject_types_supported":              []string{"public", "pairwise"},
		"id_token_signing_alg_values_supported": []string{"RS256", "ES256"},
		"scopes_supported":                     []string{"openid", "profile", "email", "address", "phone"},
		"token_endpoint_auth_methods_supported": []string{"client_secret_basic", "client_secret_post", "private_key_jwt"},
		"claims_supported": []string{
			"sub", "iss", "aud", "exp", "iat", "auth_time",
			"name", "given_name", "family_name", "nickname", "preferred_username",
			"email", "email_verified", "phone_number", "phone_number_verified",
			"address", "birthdate", "locale",
		},
		"code_challenge_methods_supported": []string{"S256"},
		"request_parameter_supported":      true,
		"request_uri_parameter_supported":  false,
	})
}

// JWKS returns the JSON Web Key Set
// GET /oauth2/.well-known/jwks.json
func (h *Handler) JWKS(c *gin.Context) {
	// Return public keys for token verification
	c.JSON(http.StatusOK, gin.H{
		"keys": []gin.H{},
	})
}
