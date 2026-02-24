# axum-starter Project Memory

## Project Overview
JWT-authenticated REST API built with Axum, Diesel (SQLite), Tokio. Feature modules in `src/modules/`, custom extractors in `src/extractors/`.

## Key Architecture

- **HttpError / HttpResponse**: live in `src/services/http_error.rs` and `src/services/http_response.rs`. Exported via `crate::services::{HttpError, HttpResponse}`. All controllers, extractors, and server.rs import from `crate::services`, not `crate::utils`.
- **Error handling**: String-based `bail!()` / `anyhow!()` in service/repo layers. `HttpError::from_service_error()` matches on `e.to_string().as_str()`. Flexible — no typed ServiceError enum.
- **Validation**: Shared `format_validation_errors` in `src/utils/validation.rs`, imported by both `body.rs` and `formdata.rs`.
- **AppState**: `{ env: Environment, db: DBSqlite }` — no cache field.
- **Files**: `src/utils/files.rs` returns `anyhow::Result` (was `Box<dyn Error>`).

## Naming Conventions (enforced)
- Response DTOs: `{Entity}Response` (e.g., `AuthTokensResponse`, not `AuthTokens`)
- Single-resource handler: `get_by_id` (not `get` — shadows std)
- Repo auth functions: `find_by_token`, `insert`, `rotate` (not `find_token`, `insert_token`, `rotate_token`)
- Request DTOs: `{Action}{Entity}Request`

## Security
- Upload filenames sanitized via `Path::file_name()` in `attachment/controller.rs`
- MIME type allowlist in `ALLOWED_MIME_TYPES` constant in `attachment/controller.rs`

## Removed
- `pub mod middlewares` from `lib.rs` (empty logger module)
- `pub mod view` from `schemas/mod.rs` (empty file)
- `pub mod cache/nric/generator_account/string` from `utils/mod.rs`
- `Cache` from `AppState` and `main.rs`
- `pub use integer::*` wildcard re-export (now explicit: `to_i64`, `to_u32`)
- 225 lines of commented-out code in `encrypt.rs`
- TODO comments from `main.rs`
- Redundant `exp < iat` check in `token.rs` (jsonwebtoken handles expiry)
- `src/utils/service_error.rs` (not used — string-based errors are kept flexible)
- `http_error.rs` / `http_response.rs` from `src/utils/` (moved to `src/services/`)

## Migration
- `migrations/2024-01-01-000004_attachment_index/` — index on `attachments.user_id`

## Build
- `cargo build` → clean (0 errors)
- `cargo clippy` → 3 pre-existing warnings only (integer.rs doc, formdata.rs collapsible_if, pagination.rs div_ceil)
