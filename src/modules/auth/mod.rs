pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use crate::models::AppState;
use axum::{Router, routing::post};
use std::sync::Arc;

pub struct AuthRoutes;
impl AuthRoutes {
  pub fn build() -> Router<Arc<AppState>> {
    Router::new()
      .route("/auth/register", post(controller::register))
      .route("/auth/login", post(controller::login))
      .route("/auth/refresh", post(controller::refresh))
  }
}
