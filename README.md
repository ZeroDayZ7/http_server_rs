# http_server_rs

A lightweight and extendable Rust HTTP server demonstrating **modular architecture**, **async support with Axum**, **structured configuration**, and **logging with tracing**.

---

## Features

- Async HTTP server built with [Axum](https://docs.rs/axum)
- Enterprise-style project structure
- Typed configuration via `config` and `.env` support
- Structured logging using `tracing` and `tracing-subscriber`
- Healthcheck endpoint (`GET /health`)
- Ready for extension: services, handlers, domain, repository layers

---

## Project Structure

```

src/
├── main.rs             # Entry point (binary)
├── lib.rs              # Library for core modules
├── config/             # Configuration loading
├── server/             # HTTP server and routing
├── handlers/           # Request handlers
├── domain/             # Business logic
├── services/           # Use-case orchestration
├── repository/         # Data access
├── errors/             # Application errors
└── utils/              # Helpers

tests/                  # Integration tests

```

---

## Getting Started

### Prerequisites

- Rust 1.76+ and Cargo
- Optional: `curl` to test endpoints

### Clone and Run

```bash
git clone https://github.com/ZeroDayZ7/http_server_rs.git
cd http_server_rs
cargo run
````

The server will start at the host and port defined in `.env` (default `127.0.0.1:8080`).

### Test Healthcheck

```bash
curl http://127.0.0.1:8080/health
```

Expected response:

```json
{"status":"ok"}
```

---

## Configuration

Use `.env` file for local configuration:

```env
SERVER__HOST=127.0.0.1
SERVER__PORT=8080
LOG__LEVEL=debug
```

Environment variables override defaults.

---

## Logging

* Uses `tracing` and `tracing-subscriber`
* Log level can be set via `LOG__LEVEL` in `.env` (e.g., `debug`, `info`)

---

## Next Steps

* Add database support
* Implement AppError handling and HTTP error mapping
* Graceful shutdown handling
* More endpoints and services

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
