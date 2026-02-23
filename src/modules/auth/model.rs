use crate::schemas::table::refresh_tokens;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ─── Request bodies ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
  #[validate(email(message = "Must be a valid email address"))]
  pub email: String,
  #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
  pub username: String,
  #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
  pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
  #[validate(email(message = "Must be a valid email address"))]
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshRequest {
  #[validate(length(min = 1, message = "refresh_token must not be empty"))]
  pub refresh_token: String,
}

// ─── Response bodies ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthTokens {
  pub access_token: String,
  pub refresh_token: String,
  /// Seconds until the access token expires (currently 12 hours = 43200)
  pub expires_in: i64,
}

// ─── Database models ─────────────────────────────────────────────────────────

/// Refresh token record from the database.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RefreshToken {
  pub id: String,
  pub user_id: String,
  pub token: String,
  pub expires_at: String,
  pub created_at: String,
}

/// New refresh token record for INSERT.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct NewRefreshToken {
  pub id: String,
  pub user_id: String,
  pub token: String,
  pub expires_at: String,
  pub created_at: String,
}
