use utoipa::{OpenApi, openapi};

use super::{
  controller,
  model::{AuthTokensResponse, LoginRequest, RefreshRequest, RegisterRequest},
};

#[derive(OpenApi)]
#[openapi(
    paths(controller::register, controller::login, controller::refresh),
    components(schemas(RegisterRequest, LoginRequest, RefreshRequest, AuthTokensResponse)),
    tags((name = "auth", description = "Authentication endpoints")),
)]
pub struct AuthApiDoc;

pub fn build() -> openapi::OpenApi {
  AuthApiDoc::openapi()
}
