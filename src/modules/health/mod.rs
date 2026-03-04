pub mod controller;
pub mod doc;

use crate::models::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/health/live", get(controller::liveness))
    .route("/health/ready", get(controller::readiness))
}
