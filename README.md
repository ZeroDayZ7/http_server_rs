# http_server_rs

**Production-style Rust backend** built with **Axum + Tokio**, designed as a clean, modular foundation for real-world APIs.

This project demonstrates **enterprise-grade architecture** with strong separation of concerns (domain / services / infrastructure), structured configuration, observability, security middleware, and scalable rate limiting.

---

## Highlights

- **Async HTTP API** powered by **Axum (0.8)** + **Tokio**
- **Clean architecture approach**
  - `domain` defines core contracts (ports)
  - `services` implement business logic
  - `infrastructure` provides MongoDB / Redis adapters
- **MongoDB integration** with repository layer (User + Vault)
- **Redis integration** (Fred client) for caching and distributed rate limiting
- **Two-level rate limiting**
  - In-memory governor-based limits (`tower-governor`)
  - Redis-backed limiter using **Lua script + EVALSHA optimization**
- **Security middleware**
  - CSP, XSS protection, nosniff, frame deny
- **CORS configuration** driven by typed settings
- **Structured logging**
  - console logs + JSON logs to file
  - `tracing` spans + request tracing middleware
- **Graceful shutdown** with configurable timeout

---

## API Endpoints

- `GET /health` — healthcheck
- `POST /auth/login` — authentication entrypoint (stub)
- `POST /vault/unlock` — decrypts stored encrypted payload and returns decrypted CV

---

## Security / Crypto

Vault data is encrypted using:

- **AES-256-GCM**
- key derivation via **Argon2**
- Base64 encoding for payload storage (`ciphertext`, `salt`, `nonce`)

---

## Project Structure

```bash
src/
├── config/          # typed settings + env support
├── domain/          # entities + repository ports + crypto contracts
├── services/        # use-cases (user + vault)
├── infrastructure/  # MongoDB + Redis adapters, Lua scripts, serialization
├── server/          # router, middleware, extractors, graceful shutdown
├── handlers/        # HTTP handlers (auth, vault, health)
└── errors/          # centralized AppError + API error mapping
````

---

## Why this project matters

* strong boundaries between layers,
* production logging,
* defensive error handling,
* rate limiting that works both locally and in distributed environments,
* crypto-ready domain logic.

---

## License

Apache-2.0
