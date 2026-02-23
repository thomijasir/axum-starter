use super::{model::UserResponse, service};
use crate::{
  extractors::AuthUser,
  models::{AppState, PaginatedResponse},
  utils::{HttpError, HttpResponse},
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
};
use std::{collections::HashMap, sync::Arc};

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
  Query(query): Query<HashMap<String, String>>,
) -> Result<HttpResponse<PaginatedResponse<UserResponse>>, HttpError> {
  let offset = query
    .get("page")
    .and_then(|s| s.parse::<i64>().ok())
    .unwrap_or(1);

  let limit = query
    .get("limit")
    .and_then(|s| s.parse::<i64>().ok())
    .unwrap_or(10);

  let (items, total) = super::repository::find_all(&state.db, offset, limit)
    .await
    .map_err(HttpError::from_service_error)?;

  let results: Vec<UserResponse> = items.into_iter().map(Into::into).collect();
  let paginate_result = PaginatedResponse::new(results, offset as u32, limit as u32, total as u32);
  Ok(HttpResponse::ok(paginate_result, "OK"))
}
