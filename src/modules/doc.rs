use crate::{
  models::{AppEnv, AppState, PaginatedResponse},
  modules::{
    attachment::controller::{self as attachment_controller, UploadForm},
    attachment::model::{AttachmentResponse, UpdateAttachmentRequest},
    auth::{
      controller as auth_controller,
      model::{AuthTokensResponse, LoginRequest, RefreshRequest, RegisterRequest},
    },
    health::controller as health_controller,
    user::{controller as user_controller, model::UserResponse},
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
        health_controller::liveness,
        health_controller::readiness,
        auth_controller::register,
        auth_controller::login,
        auth_controller::refresh,
        user_controller::get_me,
        user_controller::list,
        attachment_controller::upload,
        attachment_controller::list,
        attachment_controller::get_by_id,
        attachment_controller::update,
        attachment_controller::delete,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            AuthTokensResponse,
            UserResponse,
            HealthResponse,
            AttachmentResponse,
            PaginatedResponse<AttachmentResponse>,
            PaginatedResponse<UserResponse>,
            UpdateAttachmentRequest,
            UploadForm,
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
