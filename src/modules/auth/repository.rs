use super::model::{NewRefreshToken, RefreshToken};
use crate::{schemas::table::refresh_tokens, services::DBSqlite};
use anyhow::{Result, anyhow};
use diesel::prelude::*;

/// Find a refresh token row by its token string. Returns `None` if not found.
pub async fn find_by_token(
  db: &DBSqlite,
  incoming_token: String,
) -> Result<Option<RefreshToken>> {
  db.execute(move |conn| {
    refresh_tokens::table
      .filter(refresh_tokens::token.eq(&incoming_token))
      .select(RefreshToken::as_select())
      .first(conn)
      .optional()
      .map_err(|e| anyhow!("DB_ERROR: {}", e))
  })
  .await
}

/// Insert a new refresh token row and return the created record.
pub async fn insert(
  db: &DBSqlite,
  new_token: NewRefreshToken,
) -> Result<RefreshToken> {
  let tid = new_token.id.clone();
  db.transaction(move |conn| {
    diesel::insert_into(refresh_tokens::table)
      .values(&new_token)
      .execute(conn)
      .map_err(|e| anyhow!("DB_ERROR_INSERT: {}", e))?;

    refresh_tokens::table
      .filter(refresh_tokens::id.eq(&tid))
      .select(RefreshToken::as_select())
      .first(conn)
      .map_err(|e| anyhow!("DB_ERROR_FETCH: {}", e))
  })
  .await
}

/// Delete the old token and insert a new one atomically. Returns the new record.
pub async fn rotate(
  db: &DBSqlite,
  old_id: String,
  new_token: NewRefreshToken,
) -> Result<RefreshToken> {
  let new_tid = new_token.id.clone();
  db.transaction(move |conn| {
    diesel::delete(refresh_tokens::table.filter(refresh_tokens::id.eq(&old_id)))
      .execute(conn)
      .map_err(|e| anyhow!("DB_ERROR_DELETE: {}", e))?;

    diesel::insert_into(refresh_tokens::table)
      .values(&new_token)
      .execute(conn)
      .map_err(|e| anyhow!("DB_ERROR_INSERT: {}", e))?;

    refresh_tokens::table
      .filter(refresh_tokens::id.eq(&new_tid))
      .select(RefreshToken::as_select())
      .first(conn)
      .map_err(|e| anyhow!("DB_ERROR_FETCH: {}", e))
  })
  .await
}
