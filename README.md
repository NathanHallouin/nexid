# NexID

**Universal Digital Identity Platform**

NexID is a secure, GDPR-compliant identity provider that enables seamless authentication across services. Users create one account with all their personal data (name, address, payment info, etc.) and can authorize third-party applications to access this data via standard OAuth 2.1 / OpenID Connect protocols.

## Features

- **OAuth 2.1 / OpenID Connect** compliant identity provider
- **PKCE** required for all authorization flows
- **WebAuthn/FIDO2** passwordless authentication support (planned)
- **AES-256-GCM** encryption for sensitive data at rest
- **Argon2id** password hashing (OWASP recommended)
- **GDPR compliant** with data export and deletion
- **Audit logging** for all data access
- **Rate limiting** to prevent abuse
- **Horizontal scaling** ready
- **Memory-safe** by design (Rust)

## Tech Stack

- **Language**: Rust 1.84+
- **Framework**: Axum
- **Database**: PostgreSQL 16
- **Cache/Sessions**: Redis 7
- **Authentication**: OAuth 2.1, OpenID Connect

## Quick Start

### Prerequisites

- Rust 1.84+
- Docker & Docker Compose
- Make

### Development

```bash
# Clone the repository
git clone https://github.com/NathanHallouin/nexid.git
cd nexid

# Copy environment variables
cp .env.example .env

# Start dependencies (PostgreSQL, Redis)
make docker-up

# Run the application
cargo run
```

### Docker

```bash
# Build and run all services
docker compose up -d

# View logs
docker compose logs -f api
```

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/oauth2/authorize` | GET | OAuth2 authorization |
| `/oauth2/token` | POST | Token exchange |
| `/oauth2/userinfo` | GET | User information |
| `/oauth2/.well-known/openid-configuration` | GET | OIDC discovery |
| `/.well-known/jwks.json` | GET | JSON Web Key Set |
| `/api/v1/users` | POST | Register user |
| `/api/v1/auth/login` | POST | User login |
| `/api/v1/users/me` | GET | Get current user |
| `/api/v1/consents` | GET | List granted consents |
| `/api/v1/gdpr/export` | POST | Export all user data |
| `/api/v1/gdpr/delete` | POST | Request data deletion |

## Architecture

```
src/
├── main.rs           # Entry point, server setup
├── config/           # Configuration
├── crypto/           # Encryption & hashing
│   ├── encryption.rs # AES-256-GCM
│   └── password.rs   # Argon2id
├── db/               # Database layer
│   ├── users.rs      # User repository
│   ├── oauth_clients.rs
│   └── consents.rs
├── handlers/         # HTTP handlers
├── middleware/       # Security headers, etc.
├── models/           # DTOs and domain types
├── oauth2/           # OAuth2/OIDC implementation
│   ├── authorize.rs
│   ├── token.rs
│   ├── userinfo.rs
│   └── discovery.rs
├── error.rs          # Error types
└── routes.rs         # Route definitions
```

## Security

- All sensitive data is encrypted at rest using AES-256-GCM
- Passwords are hashed with Argon2id (OWASP recommended parameters)
- OAuth 2.1 with PKCE required (no implicit flow)
- Constant-time comparison for security-sensitive operations
- Rate limiting on all endpoints
- Security headers (HSTS, X-Frame-Options, X-Content-Type-Options, etc.)
- Memory-safe by design (no buffer overflows, use-after-free, etc.)

## GDPR Compliance

- **Data Export**: Users can export all their data in JSON format
- **Right to Delete**: Users can request complete account deletion
- **Consent Management**: Granular control over data sharing
- **Audit Logging**: All data access is logged
- **Data Minimization**: Only collect what's necessary

## Why Rust?

- **Memory Safety**: No buffer overflows, use-after-free, or data races
- **Performance**: Comparable to C/C++, no garbage collector pauses
- **Predictable Latency**: Critical for authentication endpoints
- **Security**: Type system prevents many classes of vulnerabilities
- **Concurrency**: Safe, fearless concurrency with async/await

## License

MIT License - see [LICENSE](LICENSE) for details.
