use super::{
  model::{UserQuery, UserResponse},
  service,
};
use crate::{
  extractors::AuthUser,
  models::{AppState, PaginatedResponse},
  services::{HttpError, HttpErrorFormat, HttpResponse, HttpResponseFormat},
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
        (status = 200, description = "Current user profile", body = HttpResponseFormat<UserResponse>),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        )
    )
)]
/// — returns the currently authenticated user's profile.
pub async fn get_me(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
) -> Result<impl IntoResponse, HttpError> {
  let user = service::find_by_id(&state.db, auth.user_id).await?;
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
        (status = 200, description = "Paginated list of users", body = HttpResponseFormat<PaginatedResponse<UserResponse>>),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        )
    )
)]
/// — returns a paginated list of users with optional username filter.
pub async fn list(
  State(state): State<Arc<AppState>>,
  Query(query): Query<UserQuery>,
) -> Result<HttpResponse<PaginatedResponse<UserResponse>>, HttpError> {
  let result = service::find_all(&state.db, query).await?;
  Ok(HttpResponse::ok(result, "OK"))
}
