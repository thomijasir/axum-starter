use crate::{
  models::{AppEnv, AppState},
  modules::{
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

/// Health check response schema (empty data payload)
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
        auth_register,
        auth_login,
        auth_refresh,
        users_me,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            AuthTokens,
            UserResponse,
            HealthResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
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

// ─── Dummy path handlers (utoipa path annotations only — real handlers are elsewhere) ─

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

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthTokens),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists")
    )
)]
#[allow(dead_code)]
fn auth_register() {}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthTokens),
        (status = 401, description = "Invalid credentials")
    )
)]
#[allow(dead_code)]
fn auth_login() {}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AuthTokens),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
#[allow(dead_code)]
fn auth_refresh() {}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    security(("bearer_token" = [])),
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
#[allow(dead_code)]
fn users_me() {}

// ─── Router builder ──────────────────────────────────────────────────────────

/// Mount Swagger UI at `/swagger-ui/` and the raw OpenAPI JSON at `/api-docs/openapi.json`.
/// **Only mounted in non-production environments.**
pub fn swagger_router(state: &Arc<AppState>) -> Option<Router<Arc<AppState>>> {
  if matches!(state.env.mode, AppEnv::Production) {
    return None;
  }

  Some(
    SwaggerUi::new("/swagger-ui")
      .url("/api-docs/openapi.json", ApiDoc::openapi())
      .into(),
  )
}
