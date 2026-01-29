pub mod user;
use crate::AppState;
use crate::docs::api_doc::ApiDoc;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// Define AppRoute
pub struct AppRoute;
impl AppRoute {
    pub fn register() -> Router<Arc<AppState>> {
        // Route Index
        let route_index = Router::new().nest("/users", user::Routes::index());
        // Docs Route
        let openapi = ApiDoc::openapi();
        Router::new()
            .nest("/api/v1", route_index)
            .route("/", get(Self::ping))
            .merge(SwaggerUi::new("/docs").url("/api/openapi.json", openapi))
    }
    async fn ping() -> &'static str {
        "hello world"
    }
}
