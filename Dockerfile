# Build stage
FROM rust:1.84-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Create a dummy project to cache dependencies
RUN cargo new --bin nexid
WORKDIR /app/nexid

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (cached layer)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations

# Build the actual binary
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.21

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1001 -S nexid && \
    adduser -u 1001 -S nexid -G nexid

# Copy binary from builder
COPY --from=builder /app/nexid/target/release/nexid /app/nexid
COPY --from=builder /app/nexid/migrations /app/migrations

# Set ownership
RUN chown -R nexid:nexid /app

# Switch to non-root user
USER nexid

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Run binary
ENTRYPOINT ["/app/nexid"]
