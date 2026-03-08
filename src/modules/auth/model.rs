use crate::schemas::table::refresh_tokens;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ─── Request bodies ──────────────────────────────────────────────────────────

/// Request body for `POST /auth/register`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
  /// Valid email address — used as the login identifier.
  #[validate(email(message = "Must be a valid email address"))]
  pub email: String,
  /// Display name, minimum 3 characters.
  #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
  pub username: String,
  /// Plain-text password, minimum 8 characters. Hashed before storage.
  #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
  pub password: String,
}

/// Request body for `POST /auth/login`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
  /// Registered email address.
  #[validate(email(message = "Must be a valid email address"))]
  pub email: String,
  /// Plain-text password to verify against the stored hash.
  pub password: String,
}

/// Request body for `POST /auth/refresh`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
  /// A valid, non-expired refresh token issued by a previous login or refresh call.
  #[validate(length(min = 1, message = "refresh_token must not be empty"))]
  pub refresh_token: String,
}

// ─── Response bodies ─────────────────────────────────────────────────────────

/// Response body returned after successful register, login, or token refresh.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokensResponse {
  /// Short-lived JWT used to authenticate subsequent requests.
  pub access_token: String,
  /// Opaque token used to obtain a new access token via `POST /auth/refresh`.
  pub refresh_token: String,
  /// Seconds until the access token expires (currently 12 hours = 43200).
  pub expires_in: i64,
}

// ─── Database models ─────────────────────────────────────────────────────────

/// Refresh token record queried from the database.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RefreshToken {
  /// Unique token ID (UUID).
  pub id: String,
  /// ID of the user this token belongs to.
  pub user_id: String,
  /// Opaque token string stored and matched on refresh.
  pub token: String,
  /// ISO-8601 expiry timestamp.
  pub expires_at: String,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
}

/// New refresh token record for INSERT into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct NewRefreshToken {
  /// Unique token ID (UUID).
  pub id: String,
  /// ID of the user this token belongs to.
  pub user_id: String,
  /// Opaque token string to store.
  pub token: String,
  /// ISO-8601 expiry timestamp.
  pub expires_at: String,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
}
