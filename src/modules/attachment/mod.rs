pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use crate::models::AppState;
use axum::{routing::{delete, get, patch, post}, Router};
use std::sync::Arc;

pub struct AttachmentRoutes;

impl AttachmentRoutes {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new()
            .route("/attachments/upload", post(controller::upload))
            .route("/attachments", get(controller::list))
            .route("/attachments/{id}", get(controller::get_by_id))
            .route("/attachments/{id}", patch(controller::update))
            .route("/attachments/{id}", delete(controller::delete))
    }
}
