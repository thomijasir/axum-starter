use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use std::sync::Arc;

use crate::models::AppState;

pub struct AppRoutes;

impl AppRoutes {
  pub fn build() -> Router<Arc<AppState>> {
    // Build the main API router
    let api_routes = Router::new().route("/", get(Self::ping));

    // Build the root router
    Router::new()
      .nest("/api", api_routes)
  }

  /// Returns 200 OK to indicate the service is running
  pub async fn ping() -> Response {
    (StatusCode::OK, "Ping!").into_response()
  }
}
