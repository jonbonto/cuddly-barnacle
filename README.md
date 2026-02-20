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

## Issues

> Note: The list below is a static snapshot of selected open issues and may become outdated. For the latest list, see the live GitHub Issues page: https://github.com/jonbonto/cuddly-barnacle/issues

```yaml type="issue-tree"
data:
- tag: 'jonbonto/cuddly-barnacle#4'
  title: 'Course progress tracking and completion certificates'
  repository: 'jonbonto/cuddly-barnacle'
  number: 4
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/4'
- tag: 'jonbonto/cuddly-barnacle#5'
  title: 'Implement shopping cart and course purchase flow'
  repository: 'jonbonto/cuddly-barnacle'
  number: 5
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/5'
- tag: 'jonbonto/cuddly-barnacle#6'
  title: 'Implement video upload and playback'
  repository: 'jonbonto/cuddly-barnacle'
  number: 6
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/6'
- tag: 'jonbonto/cuddly-barnacle#7'
  title: 'Implement user authentication and authorization (backend and frontend)'
  repository: 'jonbonto/cuddly-barnacle'
  number: 7
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/7'
- tag: 'jonbonto/cuddly-barnacle#8'
  title: 'Course creation and management (CRUD, publish, instructor UI)'
  repository: 'jonbonto/cuddly-barnacle'
  number: 8
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/8'
- tag: 'jonbonto/cuddly-barnacle#9'
  title: 'Reviews, ratings, search, and filtering'
  repository: 'jonbonto/cuddly-barnacle'
  number: 9
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/9'
- tag: 'jonbonto/cuddly-barnacle#10'
  title: 'Course progress tracking and completion certificates'
  repository: 'jonbonto/cuddly-barnacle'
  number: 10
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/10'
- tag: 'jonbonto/cuddly-barnacle#11'
  title: 'Instructor dashboard: stats and analytics'
  repository: 'jonbonto/cuddly-barnacle'
  number: 11
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/11'
- tag: 'jonbonto/cuddly-barnacle#12'
  title: 'Responsive UI & Udemy-like design theme'
  repository: 'jonbonto/cuddly-barnacle'
  number: 12
  state: 'open'
  url: 'https://github.com/jonbonto/cuddly-barnacle/issues/12'
```