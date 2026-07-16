# ---------- Build Stage ----------
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy dependency files first for better Docker layer caching
COPY Cargo.toml Cargo.lock* ./

# Build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy project source
COPY . .

# Build the real application
RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/zetra-backend /app/zetra-backend

# Copy migrations
COPY migrations ./migrations

EXPOSE 8080

CMD ["/app/zetra-backend"]
