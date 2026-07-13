# ---------- Build Stage ----------
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY . .

# Build application
RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/zetra-backend /app/zetra-backend

# Copy database migrations
COPY migrations ./migrations

# Render provides PORT automatically.
# Your Rust code reads PORT and binds to 0.0.0.0:<PORT>,
# so no BIND_ADDR environment variable is needed.

EXPOSE 8080

CMD ["/app/zetra-backend"]
