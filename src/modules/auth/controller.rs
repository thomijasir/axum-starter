use super::{
  model::{LoginRequest, RefreshRequest, RegisterRequest},
  service,
};
use crate::{
  extractors::BodyJson,
  models::AppState,
  modules::auth::model::AuthTokensResponse,
  services::{HttpError, HttpErrorFormat, HttpResponse, HttpResponseFormat},
};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = HttpResponseFormat<AuthTokensResponse>),
        (status = 400, description = "Validation error", body = HttpErrorFormat, examples(
        ("INVALID_VALIDATION" = (value = json!({"success": false, "message": "ERR034|INVALID_VALIDATION:password|length|Password must be at least 8 characters|value=\"string\"|min=8"})))
        )),
        (status = 409, description = "Email already exists", body = HttpErrorFormat,
        examples(
        ("EMAIL_ALREADY_EXISTS" = (value = json!({"success": false, "message": "ERR010|EMAIL_ALREADY_EXISTS"})))
        ))
    )
)]
/// — create a new account and return JWT tokens.
pub async fn register(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<RegisterRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let (user, refresh_token) =
    service::register(&state.db, body.email, body.username, body.password).await?;

  let tokens = service::build_tokens(&user, &refresh_token, state.env.secret.as_bytes())?;

  Ok(HttpResponse::created(tokens, "REGISTERED"))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = HttpResponseFormat<AuthTokensResponse>),
        (status = 401, description = "Invalid credentials", body = HttpErrorFormat,
            examples(
                ("INVALID_CREDENTIALS" = (value = json!({"success": false, "message": "ERR013|INVALID_CREDENTIALS"})))
            )
        )
    )
)]
/// — validate credentials and return JWT tokens.
pub async fn login(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<LoginRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let (user, refresh_token) = service::login(&state.db, body.email, body.password).await?;

  let tokens = service::build_tokens(&user, &refresh_token, state.env.secret.as_bytes())?;

  Ok(HttpResponse::ok(tokens, "OK"))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = HttpResponseFormat<AuthTokensResponse>),
        (status = 401, description = "Invalid token", body = HttpErrorFormat,
            examples(
                ("TOKEN_INVALID" = (value = json!({"success": false, "message": "ERR014|INVALID_REFRESH_TOKEN"})))
            )
        )
    )
)]
/// — rotate a refresh token and return new JWT access + refresh tokens.
pub async fn refresh(
  State(state): State<Arc<AppState>>,
  BodyJson(body): BodyJson<RefreshRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let new_refresh = service::refresh(&state.db, body.refresh_token).await?;

  let user =
    crate::modules::user::service::find_by_id(&state.db, new_refresh.user_id.clone()).await?;

  let tokens = service::build_tokens(&user, &new_refresh, state.env.secret.as_bytes())?;

  Ok(HttpResponse::ok(tokens, "OK"))
}
