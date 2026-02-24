use crate::{models::PaginationQuery, schemas::table::users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Full user record from the database (includes password hash — do NOT serialize to API responses).
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub id: String,
  pub email: String,
  pub username: String,
  pub password: String,
  pub created_at: String,
  pub updated_at: String,
}

/// New user record for INSERT.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub id: String,
  pub email: String,
  pub username: String,
  pub password: String,
  pub created_at: String,
  pub updated_at: String,
}

/// Public user DTO — password field omitted.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
  pub id: String,
  pub email: String,
  pub username: String,
  pub created_at: String,
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

// To Utilize Query Params
#[derive(Debug, Clone, Deserialize)]
pub struct UserQuery {
  #[serde(flatten)]
  pub pagination: PaginationQuery,
  pub username: Option<String>,
}
