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
│   └── environment.rs   # Environment configuration struct, AppState
├── modules/             # Feature modules (vertical slices)
│   ├── doc.rs           # ApiDoc aggregator, SecurityAddon, swagger_router()
│   ├── health/          # Health check endpoints
│   ├── user/            # User management
│   ├── auth/            # Authentication (register, login, refresh)
│   └── attachment/      # File upload/management
├── extractors/          # Custom Axum extractors
│   ├── auth.rs          # AuthUser (JWT validation)
│   ├── body.rs          # JSON body extractor with validation
│   └── formdata.rs      # Multipart form extractor with file validation
├── services/            # Infrastructure services
│   ├── http_error.rs    # HttpError type, from_service_error mapper
│   ├── http_response.rs # HttpResponse type
│   └── sqlite.rs        # DBSqlite connection pool wrapper
├── schemas/             # Diesel table definitions
│   └── table.rs         # table! macros
└── utils/               # Shared utilities
    ├── validation.rs    # Shared format_validation_errors helper
    ├── token.rs         # JWT creation/verification
    ├── encrypt.rs       # Password hashing (argon2)
    ├── files.rs         # File upload/delete utilities
    ├── generator.rs     # Snowflake ID generator
    └── integer.rs       # Numeric conversion utilities
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
- Imports `HttpError` and `HttpResponse` from `crate::services`

### Service Layer

- Contains business logic
- No HTTP dependencies (no `axum`, no `StatusCode`)
- Returns `anyhow::Result<T>`
- Uses `bail!("ERROR_CODE")` / `anyhow!("ERROR_CODE")` for expected failures

### Repository Layer

- All database operations (Diesel queries)
- Returns `anyhow::Result<T>`
- Uses `anyhow!("ERROR_CODE")` strings that match entries in `HttpError::from_service_error()`

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
        │ Matches e.to_string() against known error code strings
        │ Maps to HTTP status codes
        │
        ▼
HTTP Response: { "success": false, "message": "NOT_FOUND" }
```

`HttpError::from_service_error()` lives in `src/services/http_error.rs`. Add new error code strings there when introducing new domain errors.

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
- **Migrations**: Single file at `migrations/2024-01-01-000001_initial_schema/`
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

OpenAPI documentation lives **inline with the real handlers** — there are no separate stub files.

- `src/modules/doc.rs` — `ApiDoc` aggregator, `SecurityAddon` modifier, `swagger_router()`
- Each `controller.rs` — `#[utoipa::path]` macro directly above the handler function

### How It Works

1. Each handler is annotated with `#[utoipa::path]` directly in its `controller.rs`
2. `ApiDoc` in `src/modules/doc.rs` aggregates all paths by referencing the actual functions
3. Swagger UI is mounted at `/spec` (development only)
4. OpenAPI JSON is available at `/api-docs/openapi.json`

### Documentation Pattern

```rust
// src/modules/user/controller.rs
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
pub async fn get_me(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
) -> Result<impl IntoResponse, HttpError> {
    // ...
}
```

### ApiDoc Aggregation

```rust
// src/modules/doc.rs
#[derive(OpenApi)]
#[openapi(
    paths(
        health_controller::liveness,
        auth_controller::register,
        user_controller::get_me,
        attachment_controller::upload,
        // ...
    ),
    // ...
)]
pub struct ApiDoc;
```

### Adding New Endpoints

1. Add `#[utoipa::path]` directly above the handler in `controller.rs`
2. Add the function path to `ApiDoc` `paths()` in `src/modules/doc.rs`
3. Add any new schemas to `components(schemas())`
4. Add tag if introducing a new module
