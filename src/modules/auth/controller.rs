use super::{
  model::{LoginRequest, RefreshRequest, RegisterRequest},
  service,
};
use crate::{
  extractors::BodyJson,
  models::AppState,
  modules::auth::model::AuthTokensResponse,
  services::{HttpError, HttpResponse},
};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthTokensResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists")
    )
)]
/// POST /auth/register — create a new account and return JWT tokens.
pub async fn register(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<RegisterRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let (user, refresh_token) =
    service::register(&state.db, body.email, body.username, body.password)
      .await
      .map_err(HttpError::from_service_error)?;

  let tokens = service::build_tokens(&user, &refresh_token, state.env.secret.as_bytes())
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::created(tokens, "REGISTERED"))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthTokensResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
/// POST /auth/login — validate credentials and return JWT tokens.
pub async fn login(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<LoginRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let (user, refresh_token) = service::login(&state.db, body.email, body.password)
    .await
    .map_err(HttpError::from_service_error)?;

  let tokens = service::build_tokens(&user, &refresh_token, state.env.secret.as_bytes())
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::ok(tokens, "OK"))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AuthTokensResponse),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
/// POST /auth/refresh — rotate a refresh token and return new JWT tokens.
pub async fn refresh(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<RefreshRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let new_refresh = service::refresh(&state.db, body.refresh_token)
    .await
    .map_err(HttpError::from_service_error)?;

  // Fetch the associated user to build the access token
  let user = crate::modules::user::service::find_by_id(&state.db, new_refresh.user_id.clone())
    .await
    .map_err(HttpError::from_service_error)?;

  let tokens = service::build_tokens(&user, &new_refresh, state.env.secret.as_bytes())
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::ok(tokens, "OK"))
}
