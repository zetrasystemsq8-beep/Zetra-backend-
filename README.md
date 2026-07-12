# Zetra Backend

Production-ready Rust backend for the Zetra ecosystem.

## Stack
- Axum 0.7 + Tokio
- SQLx (PostgreSQL, compile-time-free runtime queries)
- Argon2 password hashing
- JWT access + refresh tokens (jsonwebtoken)
- UUIDs, chrono, tracing, serde, validator

## Features
- Register / Login / Refresh / Me (JWT auth)
- Users table + profile endpoint
- Comments (CRUD, owner-scoped mutations)
- Image and video uploads (multipart, size- and MIME-checked)
- Static serving of uploaded media at `/media/*`
- REST API, JSON only
- Structured logging, CORS, request size limits

## Quick Start

```bash
cp .env.example .env
docker compose up -d db
cargo run --release
```

Migrations run automatically on startup.

## Endpoints

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | /api/auth/register | - | Create account |
| POST | /api/auth/login | - | Get access + refresh tokens |
| POST | /api/auth/refresh | - | Rotate tokens |
| GET | /api/auth/me | JWT | Current user |
| GET | /api/users/:id | - | Public profile |
| GET | /api/comments?target=... | - | List comments for a target |
| POST | /api/comments | JWT | Create comment |
| DELETE | /api/comments/:id | JWT | Delete own comment |
| POST | /api/media/image | JWT | Upload image (jpeg/png/webp/gif) |
| POST | /api/media/video | JWT | Upload video (mp4/webm/quicktime) |
| GET | /media/{filename} | - | Serve uploaded file |
| GET | /healthz | - | Health check |

## Docker

```bash
docker compose up --build
```
