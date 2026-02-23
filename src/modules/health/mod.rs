pub mod controller;

use crate::models::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub struct HealthRoutes;
impl HealthRoutes {
  pub fn build() -> Router<Arc<AppState>> {
    Router::new()
      .route("/health/live", get(controller::liveness))
      .route("/health/ready", get(controller::readiness))
  }
}
