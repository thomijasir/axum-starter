# Axum Starter

A production-ready Rust API starter kit built with Axum, featuring JWT authentication, structured error handling, and clean architecture.

## Features

- **Axum 0.8** - Modern async web framework
- **JWT Authentication** - Self-contained extractor pattern (no middleware needed)
- **Diesel ORM** - SQLite (dev/test) and PostgreSQL (production)
- **Swagger UI** - Auto-generated API docs (non-production only)
- **Structured Logging** - Tracing with JSON output
- **Clean Architecture** - Repository → Service → Controller layers

## Quick Start

```bash
# Development
./run.sh dev

# Run tests
cargo test

# View API docs (development only)
open http://localhost:3000/swagger-ui
```

## Task Runner (`run.sh`)

This project uses `run.sh` as a task runner (similar to `npm run` in Node.js). No `package.json` needed.

### Available Commands

| Command | Description |
|---------|-------------|
| `./run.sh dev` | Load `.env.local`, run cargo run |
| `./run.sh dev:staging` | Load `.env.staging`, run cargo run |
| `./run.sh dev:production` | Load `.env.production`, run cargo run --release |
| `./run.sh start` | Production start (alias for dev:production) |
| `./run.sh build` | cargo build --release |
| `./run.sh lint` | cargo clippy -D warnings |
| `./run.sh lint:fix` | cargo clippy --fix |
| `./run.sh format` | cargo fmt |
| `./run.sh docker:up` | docker compose up -d --build |
| `./run.sh docker:down` | docker compose down |

### Database Commands

| Command | Description |
|---------|-------------|
| `./run.sh db:migration:create NAME` | Create new migration |
| `./run.sh db:migration:run` | Run pending migrations |
| `./run.sh db:migration:revert` | Revert last migration |
| `./run.sh db:migration:status` | List migration status |

### Passing Extra Args

```bash
# Pass args to cargo after --
./run.sh dev -- --bin myapp --features tracing

# Example: run with specific binary
./run.sh dev -- --bin api
```

### Environment Files

| File | Environment |
|------|-------------|
| `.env.local` | Development (default for `dev`) |
| `.env.staging` | Staging environment |
| `.env.production` | Production environment |

## API Endpoints

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| GET | `/health/live` | Liveness probe | No |
| GET | `/health/ready` | Readiness probe (DB check) | No |
| POST | `/auth/register` | Create new account | No |
| POST | `/auth/login` | Login with credentials | No |
| POST | `/auth/refresh` | Refresh access token | No |
| GET | `/users/me` | Get current user | Yes |

## Project Structure

```
src/
├── main.rs              # Entry point
├── modules/             # Feature modules
│   ├── auth/            # Authentication
│   ├── user/            # User management
│   └── health/          # Health checks
├── extractors/          # Custom Axum extractors
├── utils/               # Shared utilities
└── schemas/             # Diesel table definitions

guide/
├── ARCHITECTURE.md      # System architecture
├── CONVENTIONS.md       # Coding conventions
└── RULES.md             # Do's and don'ts

migrations/              # Database migrations
tests/                   # Integration tests
```

## Documentation

- **[Architecture Guide](guide/ARCHITECTURE.md)** - System design, layered architecture, data flow
- **[Coding Conventions](guide/CONVENTIONS.md)** - Module structure, naming, error handling patterns
- **[Development Rules](guide/RULES.md)** - Do's and don'ts for the codebase

## Environment Variables

```bash
# Required
APP_ENV=development
APP_SECRET=your-secret-key-min-32-chars
DATABASE_URL=sqlite://dev.db

# Optional
CORS_ORIGINS=http://localhost:3000,http://localhost:5173
```

## Docker

```bash
# Start all services
./run.sh docker:up

# Stop all services
./run.sh docker:down
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_register_new_user

# Run with output
cargo test -- --nocapture
```

## License

MIT
