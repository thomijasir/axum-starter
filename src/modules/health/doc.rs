use utoipa::{OpenApi, openapi};

use super::controller;

#[derive(utoipa::ToSchema)]
pub struct HealthResponse {
  pub success: bool,
  pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(controller::liveness, controller::readiness),
    components(schemas(HealthResponse)),
    tags((name = "health", description = "Health check endpoints")),
)]
pub struct HealthApiDoc;

pub fn build() -> openapi::OpenApi {
  HealthApiDoc::openapi()
}
