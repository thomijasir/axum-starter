use crate::{
  models::AppState,
  services::{HttpError, HttpResponse},
};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/health/live",
    tag = "health",
    responses(
        (status = 200, description = "Service is alive")
    )
)]
/// — Kubernetes liveness probe. Always returns 200 OK as long as the process is running.
pub async fn liveness() -> impl IntoResponse {
  HttpResponse::ok(serde_json::Value::Null, "OK")
}

#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready (DB reachable)"),
        (status = 503, description = "Service unavailable (DB unreachable)")
    )
)]
/// — Kubernetes readiness probe. Returns 200 if the database is reachable, 503 otherwise.
pub async fn readiness(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
  state
    .db
    .health_check()
    .await
    .map_err(|_| HttpError::ERR503)?;
  Ok(HttpResponse::ok(serde_json::Value::Null, "READY"))
}
