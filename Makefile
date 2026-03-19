.PHONY: help build run test lint clean docker-build docker-up docker-down migrate

# Variables
BINARY_NAME=nexid
VERSION?=$(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
LDFLAGS=-ldflags "-w -s -X main.Version=$(VERSION)"

# Colors
GREEN=\033[0;32m
NC=\033[0m

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-15s$(NC) %s\n", $$1, $$2}'

## Development

build: ## Build the binary
	@echo "Building $(BINARY_NAME)..."
	go build $(LDFLAGS) -o bin/$(BINARY_NAME) ./cmd/api

run: ## Run the application
	@echo "Running $(BINARY_NAME)..."
	go run ./cmd/api

dev: ## Run with hot reload (requires air)
	air

## Testing

test: ## Run tests
	go test -v -race -cover ./...

test-coverage: ## Run tests with coverage report
	go test -v -race -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html

## Linting

lint: ## Run linters
	golangci-lint run ./...

fmt: ## Format code
	go fmt ./...
	goimports -w .

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

generate-key: ## Generate a new encryption key
	@openssl rand -base64 32

generate-secret: ## Generate a new JWT secret
	@openssl rand -base64 64

## Misc

clean: ## Clean build artifacts
	rm -rf bin/
	rm -f coverage.out coverage.html

deps: ## Install dependencies
	go mod download
	go mod tidy

update-deps: ## Update dependencies
	go get -u ./...
	go mod tidy

install-tools: ## Install development tools
	go install github.com/air-verse/air@latest
	go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
	go install golang.org/x/tools/cmd/goimports@latest
