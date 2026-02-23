use crate::{models::AppState, utils::{HttpError, HttpResponse}};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

/// GET /health/live — Kubernetes liveness probe
/// Always returns 200 OK as long as the process is running.
pub async fn liveness() -> impl IntoResponse {
  HttpResponse::ok(serde_json::Value::Null, "OK")
}

/// GET /health/ready — Kubernetes readiness probe
/// Returns 200 if the database is reachable, 503 otherwise.
pub async fn readiness(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
  state
    .db
    .health_check()
    .await
    .map_err(|_| HttpError::new("SERVICE_UNAVAILABLE", axum::http::StatusCode::SERVICE_UNAVAILABLE))?;
  Ok(HttpResponse::ok(serde_json::Value::Null, "READY"))
}
