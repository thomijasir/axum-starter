# Development Rules

This document defines the do's and don'ts for working with this codebase.

## Must Do

### Error Handling

- DO use `anyhow::Result<T>` in repository and service layers
- DO use `anyhow::bail!("ERROR_CODE")` for expected failures
- DO use `.map_err(HttpError::from_service_error)?` in controllers
- DO add new error codes to `HttpError::from_service_error()` mapper

### Architecture

- DO keep database operations in repository files
- DO keep business logic in service files
- DO keep HTTP handling in controller files
- DO return `anyhow::Result` from service functions

### Authentication

- DO use `AuthUser` extractor for protected endpoints
- DO declare auth requirement in handler signature:
  ```rust
  pub async fn protected(auth: AuthUser) -> Result<impl IntoResponse, HttpError>
  ```

### Database

- DO use `db.execute()` for single queries
- DO use `db.transaction()` for multi-operation transactions
- DO handle `UniqueViolation` in repository:
  ```rust
  .map_err(|e| match e {
      diesel::result::Error::DatabaseError(
          diesel::result::DatabaseErrorKind::UniqueViolation, _
      ) => anyhow::anyhow!("UNIQUE_VIOLATION"),
      other => anyhow::anyhow!("DB error: {}", other),
  })
  ```

### Testing

- DO create isolated tests using `TestApp::new()`
- DO test both success and error paths
- DO use descriptive test names: `test_register_with_existing_email_returns_conflict`

## Must Not Do

### Error Handling

- DO NOT return `HttpError` from service or repository layers
- DO NOT use `StatusCode` in service or repository layers
- DO NOT use `.unwrap()` or `.expect()` in production code
- DO NOT create new error types—use error codes

### Architecture

- DO NOT skip layers (controller → repository directly)
- DO NOT put business logic in controllers
- DO NOT put HTTP concerns in services (no `axum` imports)
- DO NOT put database queries in controllers

### Authentication

- DO NOT create a separate middleware for auth
- DO NOT manually validate JWT in handlers—use `AuthUser` extractor

### Database

- DO NOT use raw SQL—use Diesel queries
- DO NOT share database connections—use the pool

### Security

- DO NOT log secrets, tokens, or passwords
- DO NOT commit `.env` files
- DO NOT expose internal error details to clients

### Code Quality

- DO NOT leave TODO comments without tracking them
- DO NOT add dependencies without updating Cargo.toml properly
- DO NOT use `unwrap()` outside of tests

## Conditional Rules

### Environment-Specific

| Feature | Development | Production |
|---------|-------------|------------|
| Swagger UI | Enabled at `/spec` | Disabled |
| Database | SQLite | PostgreSQL |
| CORS | Permissive | Strict origins from config |
| Error details | Verbose | Minimal |

### When Adding New Features

1. Create module directory in `src/modules/`
2. Add module to `src/lib.rs`
3. Create model, repository, service, controller files
4. Define routes in `mod.rs`
5. Register routes in `src/modules/mod.rs`
6. Add error codes to `HttpError::from_service_error()`
7. Create documentation in `src/docs/{module}.rs`
8. Add paths and schemas to `ApiDoc` in `src/docs/mod.rs`
9. Write integration tests

### When Adding New Endpoints

1. Define request/response models in `model.rs`
2. Add `#[derive(ToSchema)]` for Swagger
3. Create service function with `anyhow::Result`
4. Create controller handler with `HttpError` return
5. Add route to module's `Routes::build()`
6. Add `#[utoipa::path]` documentation in `src/docs/{module}.rs`
7. Register path and schemas in `src/docs/mod.rs`
8. Test with authenticated and unauthenticated requests

## Common Patterns

### Adding a New Error Code

1. Decide on error code: `RESOURCE_EXHAUSTED`
2. Choose HTTP status: 429 Too Many Requests
3. Add to mapper in `src/utils/http_error.rs`:
   ```rust
   "RESOURCE_EXHAUSTED" => Self::new("RESOURCE_EXHAUSTED", StatusCode::TOO_MANY_REQUESTS),
   ```
4. Use in service: `anyhow::bail!("RESOURCE_EXHAUSTED")`

### Adding a Protected Endpoint

1. Add `AuthUser` parameter to handler
2. Access user ID via `auth.user_id`
3. Service validates ownership if needed

### Adding a Database Query

1. Add function to repository file
2. Return `anyhow::Result<T>`
3. Map Diesel errors to error codes
4. Call from service layer
