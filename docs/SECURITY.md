# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.x.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability within NexID, please send an email to security@nexid.io. All security vulnerabilities will be promptly addressed.

Please include the following information in your report:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Security Measures

### Data Protection

- **Encryption at Rest**: All sensitive user data (names, addresses, phone numbers, birthdates) is encrypted using AES-256-GCM before storage
- **Password Hashing**: Passwords are hashed using Argon2id with OWASP-recommended parameters (64MB memory, 3 iterations, 4 threads)
- **Key Management**: Encryption keys are stored separately from encrypted data (HashiCorp Vault in production)

### Authentication & Authorization

- **OAuth 2.1**: Latest OAuth specification with mandatory PKCE
- **No Implicit Flow**: Implicit grant type is not supported
- **Token Rotation**: Refresh tokens are rotated on each use
- **WebAuthn/FIDO2**: Passwordless authentication support

### Transport Security

- **TLS 1.3**: All connections require TLS 1.3
- **HSTS**: Strict Transport Security with 1-year max-age
- **Certificate Pinning**: Recommended for mobile clients

### Rate Limiting

- Authentication endpoints: 5 requests/minute/IP
- API endpoints: 100 requests/minute/user
- Token endpoint: 20 requests/minute/client

### Security Headers

```
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Referrer-Policy: strict-origin-when-cross-origin
Content-Security-Policy: default-src 'self'
Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### Audit Logging

All security-relevant events are logged:
- Authentication attempts (success/failure)
- Token issuance and revocation
- Consent grants and revocations
- Data access by third-party applications
- GDPR data export/deletion requests

### GDPR Compliance

- **Data Minimization**: Only collect necessary data
- **Purpose Limitation**: Data used only for stated purposes
- **Consent Management**: Granular per-client consent
- **Right to Access**: Data export functionality
- **Right to Erasure**: Account deletion with cascade
- **Data Portability**: JSON export format

## Security Checklist for Deployment

- [ ] Generate strong JWT secret (64 bytes)
- [ ] Generate unique encryption key (32 bytes)
- [ ] Configure TLS termination
- [ ] Set up rate limiting in reverse proxy
- [ ] Enable audit logging
- [ ] Configure backup encryption
- [ ] Set up monitoring and alerting
- [ ] Review CORS allowed origins
- [ ] Enable database connection encryption
- [ ] Configure Redis TLS
