.PHONY: help build run test lint clean docker-build docker-up docker-down migrate

# Variables
BINARY_NAME=nexid
VERSION?=$(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")

# Colors
GREEN=\033[0;32m
NC=\033[0m

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-15s$(NC) %s\n", $$1, $$2}'

## Development

build: ## Build the binary
	@echo "Building $(BINARY_NAME)..."
	cargo build --release

run: ## Run the application
	@echo "Running $(BINARY_NAME)..."
	cargo run

dev: ## Run with hot reload (requires cargo-watch)
	cargo watch -x run

## Testing

test: ## Run tests
	cargo test

test-coverage: ## Run tests with coverage (requires cargo-tarpaulin)
	cargo tarpaulin --out Html

## Linting

lint: ## Run clippy
	cargo clippy -- -D warnings

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

## Database

migrate: ## Run database migrations
	@echo "Running migrations..."
	psql $(DATABASE_URL) -f migrations/001_initial_schema.sql

migrate-down: ## Rollback migrations (destructive!)
	@echo "Rolling back migrations..."
	psql $(DATABASE_URL) -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

## Docker

docker-build: ## Build Docker image
	docker build -t nexid:$(VERSION) -t nexid:latest .

docker-up: ## Start all services with Docker Compose
	docker compose up -d

docker-up-dev: ## Start all services including dev tools
	docker compose --profile dev up -d

docker-down: ## Stop all services
	docker compose down

docker-logs: ## View logs
	docker compose logs -f

docker-clean: ## Remove all containers and volumes
	docker compose down -v --remove-orphans

## Security

generate-key: ## Generate a new 32-byte encryption key (base64)
	@openssl rand -base64 32

generate-secret: ## Generate a new JWT secret (base64)
	@openssl rand -base64 64

## Misc

clean: ## Clean build artifacts
	cargo clean

deps: ## Install dependencies
	cargo fetch

update-deps: ## Update dependencies
	cargo update

install-tools: ## Install development tools
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install sqlx-cli

audit: ## Run security audit
	cargo audit
