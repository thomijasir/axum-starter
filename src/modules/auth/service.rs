use super::{
  model::{AuthTokens, NewRefreshToken, RefreshToken},
  repository,
};
use crate::{
  modules::user::{
    model::{NewUser, User},
    service as user_service,
  },
  services::DBSqlite,
  utils::{encrypt, generate_id, generator::uuid, token::create_token},
};
use anyhow::{Result, anyhow, bail};
use chrono::{Duration, Utc};

/// Access token expiry in seconds (12 hours).
pub const ACCESS_TOKEN_EXPIRES_IN: i64 = 43200;

// ─── Internal helpers ────────────────────────────────────────────────────────

fn new_refresh_token_record(uid: &str) -> NewRefreshToken {
  let now = Utc::now();
  NewRefreshToken {
    id: generate_id().to_string(),
    user_id: uid.to_string(),
    token: uuid(),
    expires_at: (now + Duration::days(30)).to_rfc3339(),
    created_at: now.to_rfc3339(),
  }
}

// ─── Public service functions ────────────────────────────────────────────────

/// Register a new user: check uniqueness, hash password, persist, issue refresh token.
pub async fn register(
  db: &DBSqlite,
  user_email: String,
  username: String,
  password: String,
) -> Result<(User, RefreshToken)> {
  if user_service::find_by_email(db, user_email.clone())
    .await?
    .is_some()
  {
    bail!("EMAIL_ALREADY_EXISTS");
  }

  let hashed = encrypt::hash(&password).map_err(|_| anyhow!("PASSWORD_HASH_FAILED"))?;

  let now = Utc::now().to_rfc3339();
  let new_user = NewUser {
    id: generate_id().to_string(),
    email: user_email,
    username,
    password: hashed,
    created_at: now.clone(),
    updated_at: now,
  };

  let user = user_service::create(db, new_user).await?;
  let refresh = repository::insert_token(db, new_refresh_token_record(&user.id)).await?;

  Ok((user, refresh))
}

/// Validate credentials and issue a new refresh token.
pub async fn login(
  db: &DBSqlite,
  user_email: String,
  password: String,
) -> Result<(User, RefreshToken)> {
  let user = user_service::find_by_email(db, user_email)
    .await?
    .ok_or_else(|| anyhow!("INVALID_CREDENTIALS"))?;

  let valid =
    encrypt::verify(&password, &user.password).map_err(|_| anyhow!("PASSWORD_VERIFY_FAILED"))?;

  if !valid {
    bail!("INVALID_CREDENTIALS");
  }

  let refresh = repository::insert_token(db, new_refresh_token_record(&user.id)).await?;

  Ok((user, refresh))
}

/// Validate a refresh token, check expiry, and rotate it.
pub async fn refresh(
  db: &DBSqlite,
  incoming_token: String,
) -> Result<RefreshToken> {
  let existing = repository::find_token(db, incoming_token)
    .await?
    .ok_or_else(|| anyhow!("INVALID_REFRESH_TOKEN"))?;

  let expires = chrono::DateTime::parse_from_rfc3339(&existing.expires_at)
    .map_err(|_| anyhow!("INVALID_TOKEN_EXPIRY_FORMAT"))?;

  if expires < Utc::now() {
    bail!("REFRESH_TOKEN_EXPIRED");
  }

  repository::rotate_token(db, existing.id, new_refresh_token_record(&existing.user_id)).await
}

/// Build the `AuthTokens` response payload from a User + RefreshToken.
pub fn build_tokens(
  user: &User,
  refresh_token: &RefreshToken,
  secret: &[u8],
) -> Result<AuthTokens> {
  let access_token = create_token(format!("{}|{}", user.id, user.email), secret)
    .map_err(|_| anyhow!("TOKEN_CREATE_FAILED"))?;

  Ok(AuthTokens {
    access_token,
    refresh_token: refresh_token.token.clone(),
    expires_in: ACCESS_TOKEN_EXPIRES_IN,
  })
}
