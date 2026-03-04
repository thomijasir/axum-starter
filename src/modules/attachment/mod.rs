pub mod controller;
pub mod doc;
pub mod model;
pub mod repository;
pub mod service;

use crate::models::AppState;
use axum::{
  Router,
  routing::{delete, get, patch, post},
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
  Router::new()
    .route("/attachments/upload", post(controller::upload))
    .route("/attachments", get(controller::list))
    .route("/attachments/{id}", get(controller::get_by_id))
    .route("/attachments/{id}", patch(controller::update))
    .route("/attachments/{id}", delete(controller::delete))
}
