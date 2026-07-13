# ---------- Build Stage ----------
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml only (no Cargo.lock required)
COPY Cargo.toml ./

# Cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/zetra-backend /app/zetra-backend

# Copy migrations
COPY migrations ./migrations

# Render provides PORT automatically.
EXPOSE 8080

# Start the application
CMD ["/app/zetra-backend"]
