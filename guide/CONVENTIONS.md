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
    if find_by_email(db, email.clone()).await?.is_some() {
        anyhow::bail!("EMAIL_ALREADY_EXISTS");
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

Add new codes to `HttpError::from_service_error()` in `src/utils/http_error.rs`.

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
pub use self::{controller::*, model::*, service::*};

use axum::Router;
use std::sync::Arc;
use crate::models::AppState;

pub struct ModuleRoutes;

impl ModuleRoutes {
    pub fn build(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/endpoint", get(controller::handler))
            .with_state(state)
    }
}
```

### model.rs

Domain models and DTOs:

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

### repository.rs

Database operations only. No business logic:

```rust
use super::model::User;
use crate::{schemas::table::users, services::DBSqlite};
use diesel::prelude::*;

pub async fn find_by_id(db: &DBSqlite, uid: String) -> anyhow::Result<User> {
    // Diesel queries only
}
```

### service.rs

Business logic. No HTTP dependencies:

```rust
use super::{model::*, repository};
use crate::services::DBSqlite;

pub async fn do_something(db: &DBSqlite, param: String) -> anyhow::Result<Output> {
    // Validation, computation, calling repository
}
```

### controller.rs

HTTP handlers. Thin orchestration:

```rust
use super::{model::*, service};
use crate::{extractors::BodyJson, models::AppState, utils::{HttpError, HttpResponse}};
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

## Naming Conventions

| Element     | Convention      | Example                             |
| ----------- | --------------- | ----------------------------------- |
| Modules     | snake_case      | `user`, `auth`                      |
| Structs     | PascalCase      | `UserResponse`, `AuthService`       |
| Functions   | snake_case      | `find_by_id`, `build_tokens`        |
| Constants   | SCREAMING_SNAKE | `ACCESS_TOKEN_EXPIRES_IN`           |
| Error codes | SCREAMING_SNAKE | `NOT_FOUND`, `EMAIL_ALREADY_EXISTS` |

## Async Patterns

- Use `async fn` for all handlers and service functions
- Diesel operations wrapped in `db.execute()` and `db.transaction()`
- No `.unwrap()` in production code—use `?` or `map_err`

## JSON Responses

All responses use `HttpResponse`:

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
