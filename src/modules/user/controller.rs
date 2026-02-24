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

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    security(("bearer_token" = [])),
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
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

#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    security(("bearer_token" = [])),
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10, max: 100)"),
        ("username" = Option<String>, Query, description = "Filter by username")
    ),
    responses(
        (status = 200, description = "Paginated list of users", body = PaginatedResponse<UserResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list(
  State(state): State<Arc<AppState>>,
  Query(query): Query<UserQuery>,
) -> Result<HttpResponse<PaginatedResponse<UserResponse>>, HttpError> {
  let result = service::find_all(&state.db, query)
    .await
    .map_err(HttpError::from_service_error)?;
  Ok(HttpResponse::ok(result, "OK"))
}
