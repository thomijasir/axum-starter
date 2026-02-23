pub mod attachment;
pub mod auth;
pub mod health;
pub mod user;

use crate::{docs, models::AppState};
use attachment::AttachmentRoutes;
use auth::AuthRoutes;
use axum::{
  Router,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::get,
};
use health::HealthRoutes;
use std::sync::Arc;
use user::UserRoutes;

pub struct AppRoutes;

impl AppRoutes {
  /// Build and seal the router with the given AppState.
  /// Returns a plain `Router` (state already applied) ready to pass to `axum::serve`.
  pub fn build(state: Arc<AppState>) -> Router {
    let api_routes = Router::new().route("/", get(Self::ping));

    let mut router: Router<Arc<AppState>> = Router::new()
      .nest("/api", api_routes)
      .merge(HealthRoutes::build())
      .merge(AuthRoutes::build())
      .merge(UserRoutes::build())
      .merge(AttachmentRoutes::build());

    // Swagger UI only in non-production environments
    if let Some(swagger) = docs::swagger_router(&state) {
      router = router.merge(swagger);
    }

    router.with_state(state)
  }

  pub async fn ping() -> Response {
    (StatusCode::OK, "Ping!").into_response()
  }
}
