# ---------- Build Stage ----------
FROM rust:1.87-bookworm AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

RUN cargo build --release


# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zetra-backend /app/zetra-backend

COPY migrations ./migrations

EXPOSE 8080

CMD ["/app/zetra-backend"]
