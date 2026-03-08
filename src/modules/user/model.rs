use crate::{models::PaginationQuery, schemas::table::users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Full user record queried from the database.
/// Contains the password hash — never serialize this directly to API responses.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  /// Unique user ID (UUID).
  pub id: String,
  /// User's email address, used as the login identifier.
  pub email: String,
  /// User's display name.
  pub username: String,
  /// Bcrypt-hashed password — must not be exposed in API responses.
  pub password: String,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

/// New user record for INSERT into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
  /// Unique user ID (UUID).
  pub id: String,
  /// User's email address.
  pub email: String,
  /// User's display name.
  pub username: String,
  /// Bcrypt-hashed password.
  pub password: String,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

/// Public user DTO returned in API responses — password field omitted.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
  /// Unique user ID (UUID).
  pub id: String,
  /// User's email address.
  pub email: String,
  /// User's display name.
  pub username: String,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

impl From<User> for UserResponse {
  fn from(u: User) -> Self {
    UserResponse {
      id: u.id,
      email: u.email,
      username: u.username,
      created_at: u.created_at,
      updated_at: u.updated_at,
    }
  }
}

/// Query parameters for `GET /users` — pagination plus an optional username filter.
#[derive(Debug, Clone, Deserialize)]
pub struct UserQuery {
  /// Shared pagination fields (`page`, `limit`).
  #[serde(flatten)]
  pub pagination: PaginationQuery,
  /// Filter results to users whose username contains this value.
  pub username: Option<String>,
}
