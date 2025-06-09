# LarpTech Project

This project is a sophisticated data processing pipeline designed to analyze on-chain Solana data. It captures real-time events, enriches them with metadata, performs complex scoring calculations, and exposes the results through a GraphQL API. The system is architected as a series of microservices communicating via Kafka, with infrastructure provisioned on AWS Fargate.

## Default Commands

- **`docker compose up --build`**: Starts the entire local development stack, including all services and backing infrastructure like Kafka, Postgres, and Redis.
- **`cargo test`**: Runs unit tests for Rust-based services (`listener`, `metadata`).
- **`npm test --workspaces`**: Executes tests for all Node.js services (`crawler`, `tester`).
- **`go test ./...`**: Runs all tests for Go-based services (`scorer`, `api`).

These commands are expected to be run in CI pipelines to ensure code quality.

## Environment Variables

The following environment variables are required for the system to operate correctly. A `.env.example` file should be used to track required variables.

- `HELIUS_KEY`: API key for accessing the Helius Solana RPC.
- `REDIS_URL`: Connection string for the Redis instance.
- `PG_URL`: Connection string for the PostgreSQL/TimescaleDB database.
- `AWS_REGION`: The AWS region where infrastructure is deployed.

### Security Note

**Never commit secrets directly to the repository.** Use a `.env` file for local development (which should be in `.gitignore`) and use secret management tools for production environments (e.g., AWS Secrets Manager).

## Style Guide

- **Rust**: `cargo fmt`
- **Go**: `gofumpt`
- **JavaScript/TypeScript**: `eslint --fix`

Consistent code style is enforced in CI.

## Testing

- The `listener` service requires a mocked WebSocket connection that provides a fixture from the Solana devnet to simulate `logsSubscribe` events for its tests.

## Continuous Integration

- A GitHub Actions workflow is used for CI.
- The build will fail if any unit test fails. 