pub mod controller;
pub mod doc;
pub mod model;
pub mod repository;
pub mod service;

use crate::models::AppState;
use axum::{Router, routing::post};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/auth/register", post(controller::register))
    .route("/auth/login", post(controller::login))
    .route("/auth/refresh", post(controller::refresh))
}
