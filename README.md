# Learnify — Udemy-like Learning Platform

A full-stack online course platform built with **Rust (Axum)** on the backend and **SvelteKit** on the frontend, running on **PostgreSQL**.

## Tech Stack

| Layer     | Technology                          |
|-----------|-------------------------------------|
| Backend   | Rust, Axum, SQLx, Tokio             |
| Frontend  | SvelteKit, Svelte, TypeScript       |
| Database  | PostgreSQL 16                       |
| Dev Infra | Docker Compose                      |

## Prerequisites

- [Rust](https://rustup.rs/) (1.75+)
- [Node.js](https://nodejs.org/) (18+) and npm
- [Docker](https://www.docker.com/) & Docker Compose

## Quick Start with Docker

```bash
# 1. Copy environment variables
cp .env.example .env

# 2. Start all services
docker compose up --build
```

Services will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- Health check: http://localhost:8080/health

## Local Development (without Docker)

### Backend

```bash
cd backend
cp ../.env.example .env   # edit DATABASE_URL to point to your Postgres
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

## Project Structure

```
├── backend/
│   ├── Cargo.toml           # Rust dependencies
│   ├── src/
│   │   ├── main.rs          # Application entry point & server setup
│   │   ├── routes/          # Route definitions
│   │   ├── handlers/        # Request handlers (controllers)
│   │   ├── models/          # Data models and DB schemas
│   │   ├── middleware/       # Auth, logging middleware
│   │   └── db/              # Database connection and pool
│   └── migrations/          # SQL migration files
├── frontend/
│   ├── package.json
│   ├── svelte.config.js
│   ├── src/
│   │   ├── routes/          # SvelteKit file-based routes
│   │   ├── lib/
│   │   │   ├── components/  # Reusable UI components
│   │   │   └── stores/      # Svelte writable stores
│   │   └── app.html         # HTML shell
│   └── static/              # Static assets (favicon, images)
├── docker-compose.yml
├── .env.example
└── .gitignore
```

## API Endpoints

| Method | Path            | Description          |
|--------|-----------------|----------------------|
| GET    | /health         | Service health check |
| GET    | /api/courses    | List courses         |

## Database

Migrations live in `backend/migrations/`. The initial migration creates:
- `users` — platform users (learners & instructors)
- `courses` — course catalogue
- `enrollments` — user ↔ course relationship