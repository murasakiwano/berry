# Server

The backend is built with [Axum](https://github.com/tokio-rs/axum) and [SQLx](https://github.com/launchbadge/sqlx). It exposes a small REST API for managing accounts and transactions.

## Requirements

- Rust toolchain
- A PostgreSQL database (``compose.yaml`` provides one for development)

## Running

```bash
cargo run
```

Configuration values can be overridden using environment variables prefixed with `BERRY_` (see `configuration` directory for defaults).

## Tests

Run the server tests with:

```bash
cargo test
```

Tests require Docker as they use `testcontainers` to spin up a PostgreSQL instance.
