# Architecture Overview

This document describes the high-level architecture of the axum-starter project.

## Project Structure

```
src/
├── main.rs              # Entry point, tracing init, AppState creation
├── lib.rs               # Module declarations
├── config.rs            # Environment loading, CORS origins parsing
├── server.rs            # AppServer, middleware layers, graceful shutdown
├── models/              # Shared domain models
│   └── environment.rs   # Environment configuration struct
├── modules/             # Feature modules (vertical slices)
│   ├── health/          # Health check endpoints
│   ├── user/            # User management
│   ├── auth/            # Authentication (register, login, refresh)
│   └── attachment/      # File upload/management
├── extractors/          # Custom Axum extractors
│   ├── auth.rs          # AuthUser (JWT validation)
│   ├── body.rs          # JSON body extractor
│   └── formdata.rs      # Multipart form extractor
├── middlewares/         # Tower middleware (currently unused)
├── schemas/             # Diesel table definitions
├── docs/                # OpenAPI/Swagger documentation
│   ├── mod.rs           # ApiDoc aggregation, swagger router
│   ├── auth.rs          # Auth endpoint documentation
│   ├── user.rs          # User endpoint documentation
│   └── attachment.rs    # Attachment endpoint documentation
└── utils/               # Shared utilities
    ├── http_error.rs    # HTTP error handling
    ├── http_response.rs # Standardized HTTP responses
    ├── token.rs         # JWT creation/verification
    ├── encrypt.rs       # Password hashing (argon2)
    ├── files.rs         # File upload/delete utilities
    ├── generator.rs     # Snowflake ID generator
    └── pagination.rs    # Pagination query params
```

## Layered Architecture

The project follows a strict layered architecture with unidirectional dependencies:

```
┌─────────────────────────────────────────────────────┐
│                    Controller                        │
│         (HTTP handlers, request/response)           │
└───────────────────────┬─────────────────────────────┘
                        │ calls
                        ▼
┌─────────────────────────────────────────────────────┐
│                     Service                          │
│          (Business logic, validation)               │
└───────────────────────┬─────────────────────────────┘
                        │ calls
                        ▼
┌─────────────────────────────────────────────────────┐
│                   Repository                         │
│              (Database operations)                  │
└─────────────────────────────────────────────────────┘
```

### Controller Layer

- Handles HTTP request/response
- Uses extractors to parse input
- Calls service functions
- Maps service errors to HTTP errors via `HttpError::from_service_error()`
- Returns `HttpResponse` or `HttpError`

### Service Layer

- Contains business logic
- No HTTP dependencies (no `axum`, no `StatusCode`)
- Returns `anyhow::Result<T>`
- Uses `anyhow::bail!()` with error codes for expected failures

### Repository Layer

- All database operations (Diesel queries)
- Returns `anyhow::Result<T>`
- Maps database errors to meaningful error codes

## Error Handling Flow

```
Repository/Service: anyhow::Result<T>
        │
        │ bail!("NOT_FOUND")
        │ bail!("EMAIL_ALREADY_EXISTS")
        │
        ▼
Controller: .map_err(HttpError::from_service_error)?
        │
        │ Maps error codes to HTTP status codes
        │
        ▼
HTTP Response: { "success": false, "message": "NOT_FOUND" }
```

## Authentication Flow

1. **Self-contained Extractor**: `AuthUser` implements `FromRequestParts`
2. No separate middleware layer needed
3. Handlers declare auth requirement via parameter type:
   ```rust
   pub async fn get_me(auth: AuthUser) -> Result<impl IntoResponse, HttpError>
   ```
4. Invalid/missing token returns 401 automatically

## Database

- **ORM**: Diesel 2.3 with SQLite (dev/test) and PostgreSQL (production)
- **Connection Pool**: r2d2
- **Migrations**: Located in `migrations/` directory
- **Schema**: Defined in `src/schemas/table.rs` using `table!` macros

## Request Lifecycle

1. Request hits Axum router
2. Tower middleware layers (CORS, RequestId, tracing)
3. Route handler invoked
4. Extractors run (AuthUser, BodyJson, etc.)
5. Controller calls service
6. Service calls repository
7. Response flows back through layers
8. `IntoResponse` implementations convert to HTTP response

## Testing Strategy

- Integration tests in `tests/` directory
- Each test gets an isolated SQLite database
- `TestApp` helper handles setup/teardown
- Tests use `reqwest` to hit actual HTTP endpoints

## Swagger/OpenAPI Documentation

The project uses `utoipa` for automatic OpenAPI documentation generation.

### Structure

```
src/docs/
├── mod.rs           # ApiDoc aggregation, swagger router, health docs
├── auth.rs          # Auth endpoint documentation
├── user.rs          # User endpoint documentation
└── attachment.rs    # Attachment endpoint documentation
```

### How It Works

1. Each module's endpoints are documented in dedicated files under `src/docs/`
2. The `ApiDoc` struct in `mod.rs` aggregates all paths and schemas
3. Swagger UI is mounted at `/spec` (development only)
4. OpenAPI JSON is available at `/api-docs/openapi.json`

### Documentation Pattern

Each endpoint is documented with a dummy function containing only the `#[utoipa::path]` attribute:

```rust
// src/docs/user.rs
#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    security(("bearer_token" = [])),
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
#[allow(dead_code)]
pub fn users_me() {}
```

The actual handler logic remains in `src/modules/*/controller.rs`.

### Adding New Endpoints

1. Create documentation function in appropriate `src/docs/{module}.rs`
2. Add path to `ApiDoc` `paths()` macro
3. Add any new schemas to `components(schemas())`
4. Add tag if introducing a new module
