use super::model::{NewUser, User};
use crate::{schemas::table::users, services::DBSqlite, utils::to_u32};
use crate::constants::error::*;
use anyhow::{Result, anyhow};
use diesel::prelude::*;

pub async fn find_all(
  db: &DBSqlite,
  offset: i64,
  limit: i64,
) -> Result<(Vec<User>, u32)> {
  let count = db
    .execute(move |conn| {
      users::table
        .count()
        .first::<i64>(conn)
        .map_err(|e| anyhow!("{} {}", ERR006, e))
    })
    .await?;

  let results: Vec<User> = db
    .execute(move |conn| {
      users::table
        .order(users::created_at.desc())
        .offset(offset)
        .limit(limit)
        .select(User::as_select())
        .load(conn)
        .map_err(|e| anyhow!("{} {}", ERR006, e))
    })
    .await?;

  Ok((results, to_u32(count)))
}

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
        .map_err(|e| anyhow!("{} {}", ERR006, e))
    })
    .await?;

  user.ok_or_else(|| anyhow!(ERR004))
}

/// Find a user by email. Returns `None` if not found.
pub async fn find_by_email(
  db: &DBSqlite,
  user_email: &str,
) -> Result<Option<User>> {
  let user_email = user_email.to_string();
  db.execute(move |conn| {
    users::table
      .filter(users::email.eq(&user_email))
      .select(User::as_select())
      .first(conn)
      .optional()
      .map_err(|e| anyhow!("{} {}", ERR006, e))
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
        ) => anyhow!(ERR005),
        other => anyhow!("DB error: {}", other),
      })?;

    users::table
      .filter(users::id.eq(&uid))
      .select(User::as_select())
      .first(conn)
      .map_err(|e| anyhow!("{} {}", ERR006, e))
  })
  .await
}
