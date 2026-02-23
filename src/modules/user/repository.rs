use super::model::{NewUser, User};
use crate::{schemas::table::users, services::DBSqlite};
use anyhow::{Result, anyhow};
use diesel::prelude::*;
/// Find a user by primary key. Returns `NOT_FOUND` if absent.
pub async fn find_by_id(
  db: &DBSqlite,
  uid: String,
) -> Result<User> {
  let user = db
    .execute(move |conn| {
      users::table
        .filter(users::id.eq(&uid))
        .select(User::as_select())
        .first(conn)
        .optional()
        .map_err(|e| anyhow!("DB error: {}", e))
    })
    .await?;

  user.ok_or_else(|| anyhow!("NOT_FOUND"))
}

/// Find a user by email. Returns `None` if not found.
pub async fn find_by_email(
  db: &DBSqlite,
  user_email: String,
) -> Result<Option<User>> {
  db.execute(move |conn| {
    users::table
      .filter(users::email.eq(&user_email))
      .select(User::as_select())
      .first(conn)
      .optional()
      .map_err(|e| anyhow!("DB error: {}", e))
  })
  .await
}

/// Insert a new user row and return the created record.
/// Maps `UniqueViolation` to `UNIQUE_VIOLATION`.
pub async fn insert(
  db: &DBSqlite,
  new_user: NewUser,
) -> Result<User> {
  let uid = new_user.id.clone();
  db.transaction(move |conn| {
    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(conn)
      .map_err(|e| match e {
        diesel::result::Error::DatabaseError(
          diesel::result::DatabaseErrorKind::UniqueViolation,
          _,
        ) => anyhow!("UNIQUE_VIOLATION"),
        other => anyhow!("DB error: {}", other),
      })?;

    users::table
      .filter(users::id.eq(&uid))
      .select(User::as_select())
      .first(conn)
      .map_err(|e| anyhow!("DB error: {}", e))
  })
  .await
}
