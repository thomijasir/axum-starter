use utoipa::{OpenApi, openapi};

use super::{controller, model::UserResponse};
use crate::models::PaginatedResponse;

#[derive(OpenApi)]
#[openapi(
    paths(controller::get_me, controller::list),
    components(schemas(UserResponse, PaginatedResponse<UserResponse>)),
    tags((name = "users", description = "User endpoints")),
)]
pub struct UserApiDoc;

pub fn build() -> openapi::OpenApi {
  UserApiDoc::openapi()
}
