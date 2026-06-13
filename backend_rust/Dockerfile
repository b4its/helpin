# Multi-stage build for Rust backend
FROM rust:1.94-slim AS builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy everything needed for build
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
COPY migrations/ migrations/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend_rust .
COPY migrations/ migrations/
COPY .env .env

# Default env vars (overridden by docker-compose environment section)
ENV DATABASE_URL=postgres://postgres@postgres:5432/helpin
ENV JWT_SECRET=rahasia_aman_helpin_2026
ENV ML_SERVICE_URL=http://backend_python:8081
ENV BLOCKCHAIN_RPC_URL=http://localhost:8545
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV RUST_LOG=info

EXPOSE 8080

CMD ["./backend_rust"]
