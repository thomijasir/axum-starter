use crate::{models::AppState, services::HttpError, utils::token::decode_token};
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

/// Self-contained extractor that reads and validates the `Authorization: Bearer <token>` header.
///
/// Decodes the JWT directly against `AppState.env.secret` — no middleware dependency.
/// Any handler that declares `auth: AuthUser` is automatically protected.
#[derive(Debug, Clone)]
pub struct AuthUser {
  pub user_id: String,
  pub email: String,
}

impl FromRequestParts<Arc<AppState>> for AuthUser {
  type Rejection = HttpError;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &Arc<AppState>,
  ) -> Result<Self, Self::Rejection> {
    let token = parts
      .headers
      .get(axum::http::header::AUTHORIZATION)
      .and_then(|v| v.to_str().ok())
      .and_then(|v| v.strip_prefix("Bearer "))
      .ok_or(HttpError::ERR022)?;

    let (user_id, email) = decode_token(token, state.env.secret.as_bytes())?;

    Ok(AuthUser { user_id, email })
  }
}
