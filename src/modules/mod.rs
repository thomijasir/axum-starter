pub mod attachment;
pub mod auth;
pub mod health;
pub mod user;

use crate::models::{AppEnv, AppState};
use axum::{
  Router,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::get,
};
use std::sync::Arc;
use utoipa::{
  OpenApi,
  openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
  fn modify(
    &self,
    openapi: &mut utoipa::openapi::OpenApi,
  ) {
    let components = openapi.components.get_or_insert(Default::default());
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

#[derive(OpenApi)]
#[openapi(
    info(
        title = "axum-starter API",
        version = "0.1.0",
        description = "A JWT-authenticated REST API starter built with Axum + Diesel"
    ),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

pub struct AppRoutes;

impl AppRoutes {
  /// Build and seal the router with the given AppState.
  /// Returns a plain `Router` (state already applied) ready to pass to `axum::serve`.
  pub fn build(state: Arc<AppState>) -> Router {
    let api_routes = Router::new().route("/", get(Self::ping));

    let mut router: Router<Arc<AppState>> = Router::new()
      .nest("/api", api_routes)
      .merge(health::routes())
      .merge(auth::routes())
      .merge(user::routes())
      .merge(attachment::routes());

    // Swagger UI only in non-production environments
    if let Some(swagger) = Self::swagger(&state) {
      router = router.merge(swagger);
    }

    router.with_state(state)
  }

  fn swagger(state: &Arc<AppState>) -> Option<Router<Arc<AppState>>> {
    if matches!(state.env.mode, AppEnv::Production) {
      return None;
    }

    let mut doc = ApiDoc::openapi();
    doc.merge(health::doc::build());
    doc.merge(auth::doc::build());
    doc.merge(user::doc::build());
    doc.merge(attachment::doc::build());

    Some(
      SwaggerUi::new("/spec")
        .url("/spec/openapi.json", doc)
        .into(),
    )
  }

  pub async fn ping() -> Response {
    (StatusCode::OK, "Ping!").into_response()
  }
}
