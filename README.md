# Berry

Berry is a personal finance application composed of a Rust backend and a SvelteKit frontend.

## Project Structure

- `server/` - Axum and SQLx based REST API written in Rust.
- `frontend/` - SvelteKit web application.
- `compose.yaml` - Docker Compose file providing a PostgreSQL database.

## Quick Start

1. Start the database using Docker Compose:
   ```bash
   docker compose up -d
   ```
2. In one terminal run the backend:
   ```bash
   cd server
   cargo run
   ```
3. In another terminal start the frontend:
   ```bash
   cd frontend
   pnpm install
   pnpm dev
   ```

The frontend is available on <http://localhost:5173> and proxies API requests to the backend running on port 8080 by default.

## Running Tests

- Backend tests:
  ```bash
  cd server
  cargo test
  ```
- Frontend tests:
  ```bash
  cd frontend
  pnpm test
  ```

## License

MIT
