# Coding Conventions

This document outlines the coding conventions used in this project.

## Error Handling

### Pattern: anyhow Everywhere, Mapper at Boundary

**Repository and Service layers** use `anyhow::Result<T>`:

```rust
// repository.rs
pub async fn find_by_id(db: &DBSqlite, uid: String) -> anyhow::Result<User> {
    let user = db.execute(/* ... */).await?;
    user.ok_or_else(|| anyhow::anyhow!("NOT_FOUND"))
}

// service.rs
pub async fn register(db: &DBSqlite, email: String) -> anyhow::Result<User> {
    if find_by_email(db, &email).await?.is_some() {
        bail!("EMAIL_ALREADY_EXISTS");
    }
    // ...
}
```

**Controller layer** maps to HTTP errors:

```rust
// controller.rs
pub async fn get_me(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    let user = service::find_by_id(&state.db, uid)
        .await
        .map_err(HttpError::from_service_error)?;
    Ok(HttpResponse::ok(user, "OK"))
}
```

### Error Codes

Use short, uppercase strings as error codes:

| Code                    | HTTP Status | When to Use                     |
| ----------------------- | ----------- | ------------------------------- |
| `NOT_FOUND`             | 404         | Resource not found              |
| `UNIQUE_VIOLATION`      | 409         | Database unique constraint      |
| `EMAIL_ALREADY_EXISTS`  | 409         | Email taken during registration |
| `INVALID_CREDENTIALS`   | 401         | Wrong email/password            |
| `INVALID_REFRESH_TOKEN` | 401         | Token not in database           |
| `REFRESH_TOKEN_EXPIRED` | 401         | Token past expiry               |
| `TOKEN_CREATE_FAILED`   | 500         | JWT signing failure             |
| `PASSWORD_HASH_FAILED`  | 500         | Argon2 hashing failure          |
| `ATTACHMENT_NOT_FOUND`  | 404         | Attachment not found            |

Add new codes to `HttpError::from_service_error()` in `src/services/http_error.rs`.

## Module Structure

Each feature module follows this structure:

```
module_name/
├── mod.rs           # Route definitions, public exports
├── model.rs         # Domain models, DTOs
├── repository.rs    # Database operations
├── service.rs       # Business logic
└── controller.rs    # HTTP handlers
```

### mod.rs

Exports routes and wires the module:

```rust
use axum::Router;
use std::sync::Arc;
use crate::models::AppState;

pub struct ModuleRoutes;

impl ModuleRoutes {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new()
            .route("/endpoint", get(controller::handler))
    }
}
```

### model.rs — Type Naming Convention

| Category      | Pattern                   | Examples                                                   |
| ------------- | ------------------------- | ---------------------------------------------------------- |
| DB entity     | `{Entity}`                | `User`, `Attachment`, `RefreshToken`                       |
| Insertable    | `New{Entity}`             | `NewUser`, `NewAttachment`                                 |
| Response DTO  | `{Entity}Response`        | `UserResponse`, `AttachmentResponse`, `AuthTokensResponse` |
| Request DTO   | `{Action}{Entity}Request` | `RegisterRequest`, `LoginRequest`, `UpdateAttachmentRequest` |
| Query params  | `{Entity}Query`           | `UserQuery`                                                |

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schemas::table::users)]
pub struct User {
    pub id: String,
    pub email: String,
    // ...
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRequest {
    pub email: String,
    // ...
}
```

### controller.rs — Handler Naming Convention

Handlers use HTTP-action-oriented names:

| Operation           | Convention  | Example                        |
| ------------------- | ----------- | ------------------------------ |
| List resources      | `list`      | `list`                         |
| Get single resource | `get_by_id` | `get_by_id`                    |
| Get current user    | `get_me`    | `get_me`                       |
| Create resource     | `create`    | `create`                       |
| Upload file         | `upload`    | `upload`                       |
| Update resource     | `update`    | `update`                       |
| Delete resource     | `delete`    | `delete`                       |
| Auth actions        | verb        | `register`, `login`, `refresh` |

Import `HttpError` and `HttpResponse` from `crate::services`:

```rust
use super::{model::*, service};
use crate::{
    extractors::BodyJson,
    models::AppState,
    services::{HttpError, HttpResponse},
};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    BodyJson(body): BodyJson<Request>,
) -> Result<impl IntoResponse, HttpError> {
    let result = service::do_something(&state.db, body.field)
        .await
        .map_err(HttpError::from_service_error)?;
    Ok(HttpResponse::ok(result, "SUCCESS"))
}
```

### service.rs — Business Logic Naming Convention

Services use domain-oriented names (no HTTP concepts):

| Operation            | Convention        | Example                         |
| -------------------- | ----------------- | ------------------------------- |
| Find by ID           | `find_by_id`      | `find_by_id`                    |
| Find by field        | `find_by_{field}` | `find_by_email`, `find_by_user` |
| Find all (paginated) | `find_all`        | `find_all`                      |
| Create               | `create`          | `create`                        |
| Update               | `update`          | `update`                        |
| Delete               | `delete`          | `delete`                        |
| Auth actions         | descriptive verb  | `register`, `login`, `refresh`  |

### repository.rs — Database Operation Naming Convention

Repositories use database-oriented terms:

| Operation            | Convention        | Example                          |
| -------------------- | ----------------- | -------------------------------- |
| Find by ID           | `find_by_id`      | `find_by_id`                     |
| Find by field        | `find_by_{field}` | `find_by_email`, `find_by_token` |
| Find all (paginated) | `find_all`        | `find_all`                       |
| Insert record        | `insert`          | `insert`                         |
| Rotate token         | `rotate`          | `rotate`                         |
| Update record        | `update`          | `update`                         |
| Delete record        | `delete`          | `delete`                         |

```rust
use super::model::User;
use crate::{schemas::table::users, services::DBSqlite};
use diesel::prelude::*;

pub async fn find_by_id(db: &DBSqlite, uid: String) -> anyhow::Result<User> {
    // Diesel queries only
}
```

### mod.rs — Route Struct Naming Convention

```rust
pub struct {Module}Routes;

impl {Module}Routes {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new()
            // ...
    }
}
```

Current modules: `UserRoutes`, `AuthRoutes`, `AttachmentRoutes`, `HealthRoutes`.

## API Routing Conventions

### Route Structure

Each module defines its routes in `mod.rs` using a `{Module}Routes` struct:

```rust
pub struct UserRoutes;

impl UserRoutes {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new()
            .route("/users/me", get(controller::get_me))
            .route("/users/{id}", get(controller::get_by_id))
    }
}
```

Routes are merged into the main router in `src/modules/mod.rs`:

```rust
Router::new()
    .merge(UserRoutes::build())
    .merge(AuthRoutes::build())
```

### Route Naming Pattern

| Module Type   | Pattern                         | Example Routes                     |
| ------------- | ------------------------------- | ---------------------------------- |
| Resource      | `/{resource}`                   | `/users`, `/attachments`           |
| Resource ID   | `/{resource}/{id}`              | `/users/{id}`, `/attachments/{id}` |
| Sub-resource  | `/{resource}/{id}/{sub}`        | `/users/{id}/posts`                |
| Actions       | `/{resource}/{action}`          | `/users/me`, `/auth/login`         |
| Nested        | `/{module}/{resource}/{action}` | `/health/live`, `/health/ready`    |

### Correlation with Modules

| Module       | Route Prefix   | Example Routes                                    |
| ------------ | -------------- | ------------------------------------------------- |
| `auth`       | `/auth`        | `/auth/register`, `/auth/login`, `/auth/refresh`  |
| `user`       | `/users`       | `/users/me`, `/users/{id}`                        |
| `health`     | `/health`      | `/health/live`, `/health/ready`                   |
| `attachment` | `/attachments` | `/attachments`, `/attachments/{id}`               |

### Naming Rules

1. **Resource routes** use plural nouns: `/users`, `/posts`, `/attachments`
2. **Action routes** use verbs after the resource: `/auth/login`, `/auth/register`
3. **Special endpoints** use descriptive names: `/users/me` (current user)
4. **Health/monitoring** routes are outside `/api`: `/health/live`, `/health/ready`
5. **CRUD operations** follow REST conventions:
   - `GET /users` — list all
   - `GET /users/{id}` — get one
   - `POST /users` — create
   - `PATCH /users/{id}` — update
   - `DELETE /users/{id}` — delete

## Naming Conventions

| Element     | Convention      | Example                             |
| ----------- | --------------- | ----------------------------------- |
| Modules     | snake_case      | `user`, `auth`                      |
| Structs     | PascalCase      | `UserResponse`, `AuthTokensResponse` |
| Functions   | snake_case      | `find_by_id`, `build_tokens`        |
| Constants   | SCREAMING_SNAKE | `ACCESS_TOKEN_EXPIRES_IN`           |
| Error codes | SCREAMING_SNAKE | `NOT_FOUND`, `EMAIL_ALREADY_EXISTS` |
| Routes      | kebab-case      | `/user-profiles`, `/auth/callback`  |

## Async Patterns

- Use `async fn` for all handlers and service functions
- Diesel operations wrapped in `db.execute()` and `db.transaction()`
- No `.unwrap()` in production code — use `?` or `map_err`

## JSON Responses

All responses use `HttpResponse` (from `crate::services`):

```rust
// Success
HttpResponse::ok(data, "SUCCESS_MESSAGE")
HttpResponse::created(data, "CREATED")

// Error (via HttpError)
HttpError::not_found("NOT_FOUND")
HttpError::bad_request("VALIDATION_ERROR")
```

Response format:

```json
{
    "success": true,
    "message": "SUCCESS_MESSAGE",
    "data": { ... }
}
```

## Testing

- Test files mirror module structure: `auth_test.rs`, `user_test.rs`
- Use `TestApp::new()` for isolated test environment
- Test both success and error cases

## Swagger Documentation

### Structure

Documentation lives **inline with the real handlers** — no separate stub files.

- `src/modules/doc.rs` — `ApiDoc` aggregator, `SecurityAddon`, `swagger_router()`
- Each `controller.rs` — `#[utoipa::path]` macro directly above its handler

### Documentation Pattern

Place `#[utoipa::path]` immediately before the `pub async fn` it documents:

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
) -> Result<impl IntoResponse, HttpError> { /* ... */ }
```

### Integration with ApiDoc

```rust
// src/modules/doc.rs
use crate::modules::{
    auth::controller as auth_controller,
    user::controller as user_controller,
    // ...
};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_controller::register,
        auth_controller::login,
        user_controller::get_me,
        user_controller::list,
        // ...
    ),
    components(schemas(
        RegisterRequest,
        UserResponse,
        // ...
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
    )
)]
pub struct ApiDoc;
```

### Swagger UI Access

- **URL**: `/spec` (development only)
- **OpenAPI JSON**: `/api-docs/openapi.json`
- **Disabled**: Production environment

### Adding New Endpoint Documentation

1. Add `#[utoipa::path]` directly above the handler in `controller.rs`
2. Add the function to `ApiDoc` paths in `src/modules/doc.rs`
3. Add any new request/response schemas to `components(schemas(...))`
4. Add a new tag entry if creating a new module
