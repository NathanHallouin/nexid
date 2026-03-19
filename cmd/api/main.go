package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/NathanHallouin/nexid/internal/config"
	"github.com/NathanHallouin/nexid/internal/middleware"
	"github.com/NathanHallouin/nexid/internal/repository"
	"github.com/NathanHallouin/nexid/pkg/oauth2"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

func main() {
	// Initialize logger
	logger, _ := zap.NewProduction()
	defer logger.Sync()

	// Load configuration
	cfg, err := config.Load()
	if err != nil {
		logger.Fatal("Failed to load configuration", zap.Error(err))
	}

	// Set Gin mode
	if cfg.Environment == "production" {
		gin.SetMode(gin.ReleaseMode)
	}

	// Initialize database
	db, err := repository.NewPostgresDB(cfg.DatabaseURL)
	if err != nil {
		logger.Fatal("Failed to connect to database", zap.Error(err))
	}
	defer db.Close()

	// Initialize Redis
	redis, err := repository.NewRedisClient(cfg.RedisURL)
	if err != nil {
		logger.Fatal("Failed to connect to Redis", zap.Error(err))
	}
	defer redis.Close()

	// Initialize router
	router := gin.New()
	router.Use(gin.Recovery())
	router.Use(middleware.Logger(logger))
	router.Use(middleware.CORS(cfg.AllowedOrigins))
	router.Use(middleware.RateLimit(redis, cfg.RateLimit))
	router.Use(middleware.SecurityHeaders())

	// Health check
	router.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":    "ok",
			"timestamp": time.Now().UTC().Format(time.RFC3339),
			"version":   cfg.Version,
		})
	})

	// OAuth2/OIDC routes
	oauth2Handler := oauth2.NewHandler(db, redis, cfg, logger)
	authGroup := router.Group("/oauth2")
	{
		authGroup.GET("/authorize", oauth2Handler.Authorize)
		authGroup.POST("/token", oauth2Handler.Token)
		authGroup.POST("/revoke", oauth2Handler.Revoke)
		authGroup.GET("/userinfo", oauth2Handler.UserInfo)
		authGroup.GET("/.well-known/openid-configuration", oauth2Handler.Discovery)
		authGroup.GET("/.well-known/jwks.json", oauth2Handler.JWKS)
	}

	// API v1 routes
	v1 := router.Group("/api/v1")
	{
		// User management
		v1.POST("/users", nil)          // Register
		v1.GET("/users/me", nil)        // Get current user
		v1.PATCH("/users/me", nil)      // Update user
		v1.DELETE("/users/me", nil)     // Delete account (GDPR)

		// Consent management
		v1.GET("/consents", nil)        // List consents
		v1.POST("/consents", nil)       // Grant consent
		v1.DELETE("/consents/:id", nil) // Revoke consent

		// Data portability (GDPR)
		v1.POST("/gdpr/export", nil)    // Export all data
		v1.POST("/gdpr/delete", nil)    // Request deletion

		// Client applications (for developers)
		v1.POST("/clients", nil)        // Register OAuth client
		v1.GET("/clients", nil)         // List user's clients
		v1.PATCH("/clients/:id", nil)   // Update client
		v1.DELETE("/clients/:id", nil)  // Delete client
	}

	// Start server
	srv := &http.Server{
		Addr:         fmt.Sprintf(":%d", cfg.Port),
		Handler:      router,
		ReadTimeout:  15 * time.Second,
		WriteTimeout: 15 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Graceful shutdown
	go func() {
		logger.Info("Starting server", zap.Int("port", cfg.Port))
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			logger.Fatal("Server failed to start", zap.Error(err))
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	logger.Info("Shutting down server...")
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		logger.Fatal("Server forced to shutdown", zap.Error(err))
	}

	logger.Info("Server stopped")
}
