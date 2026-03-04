pub mod controller;
pub mod doc;
pub mod model;
pub mod repository;
pub mod service;

use crate::models::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/users", get(controller::list))
    .route("/users/me", get(controller::get_me))
}
