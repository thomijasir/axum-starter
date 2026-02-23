mod attachment;
mod auth;
mod user;

use crate::{
  models::{AppEnv, AppState, PaginatedResponse},
  modules::{
    attachment::model::{AttachmentResponse, UpdateAttachmentRequest},
    auth::model::{AuthTokens, LoginRequest, RefreshRequest, RegisterRequest},
    user::model::UserResponse,
  },
};
use axum::Router;
use std::sync::Arc;
use utoipa::{
  openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
  OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(utoipa::ToSchema)]
pub struct HealthResponse {
  pub success: bool,
  pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "axum-starter API",
        version = "0.1.0",
        description = "A JWT-authenticated REST API starter built with Axum + Diesel"
    ),
    paths(
        health_live,
        health_ready,
        auth::auth_register,
        auth::auth_login,
        auth::auth_refresh,
        user::users_me,
        attachment::attachments_upload,
        attachment::attachments_list,
        attachment::attachments_get,
        attachment::attachments_update,
        attachment::attachments_delete,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            AuthTokens,
            UserResponse,
            HealthResponse,
            AttachmentResponse,
            PaginatedResponse<AttachmentResponse>,
            UpdateAttachmentRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
        (name = "attachments", description = "File attachment endpoints"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
  fn modify(
    &self,
    openapi: &mut utoipa::openapi::OpenApi,
  ) {
    if let Some(components) = openapi.components.as_mut() {
      components.add_security_scheme(
        "bearer_token",
        SecurityScheme::Http(
          HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .build(),
        ),
      );
    }
  }
}

#[utoipa::path(
    get,
    path = "/health/live",
    tag = "health",
    responses(
        (status = 200, description = "Service is alive", body = HealthResponse)
    )
)]
#[allow(dead_code)]
fn health_live() {}

#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready (DB reachable)", body = HealthResponse),
        (status = 503, description = "Service unavailable (DB unreachable)")
    )
)]
#[allow(dead_code)]
fn health_ready() {}

pub fn swagger_router(state: &Arc<AppState>) -> Option<Router<Arc<AppState>>> {
  if matches!(state.env.mode, AppEnv::Production) {
    return None;
  }

  Some(
    SwaggerUi::new("/spec")
      .url("/api-docs/openapi.json", ApiDoc::openapi())
      .into(),
  )
}
