use utoipa::{OpenApi, openapi};

use super::{
  controller::{self, UploadForm},
  model::{AttachmentResponse, UpdateAttachmentRequest},
};
use crate::models::PaginatedResponse;

#[derive(OpenApi)]
#[openapi(
    paths(controller::upload, controller::list, controller::get_by_id, controller::update, controller::delete),
    components(schemas(AttachmentResponse, PaginatedResponse<AttachmentResponse>, UpdateAttachmentRequest, UploadForm)),
    tags((name = "attachments", description = "File attachment endpoints")),
)]
pub struct AttachmentApiDoc;

pub fn build() -> openapi::OpenApi {
  AttachmentApiDoc::openapi()
}
