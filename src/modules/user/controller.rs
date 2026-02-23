use super::{
  model::{UserQuery, UserResponse},
  service,
};
use crate::{
  extractors::AuthUser,
  models::{AppState, PaginatedResponse},
  utils::{HttpError, HttpResponse},
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
};
use std::sync::Arc;

/// GET /users/me — returns the currently authenticated user's profile.
/// Protected: requires valid Bearer token (applied via auth middleware on the router group).
pub async fn get_me(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
) -> Result<impl IntoResponse, HttpError> {
  let user = service::find_by_id(&state.db, auth.user_id)
    .await
    .map_err(HttpError::from_service_error)?;
  let response: UserResponse = user.into();
  Ok(HttpResponse::ok(response, "OK"))
}

pub async fn list(
  State(state): State<Arc<AppState>>,
  Query(query): Query<UserQuery>,
) -> Result<HttpResponse<PaginatedResponse<UserResponse>>, HttpError> {
  let result = service::find_all(&state.db, query)
    .await
    .map_err(HttpError::from_service_error)?;
  Ok(HttpResponse::ok(result, "OK"))
}
