use crate::{
  models::PaginatedResponse,
  modules::attachment::model::{AttachmentResponse, UpdateAttachmentRequest},
};

#[utoipa::path(
    post,
    path = "/attachments/upload",
    tag = "attachments",
    security(("bearer_token" = [])),
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "File uploaded successfully", body = AttachmentResponse),
        (status = 400, description = "Invalid file or missing file"),
        (status = 401, description = "Unauthorized")
    )
)]
#[allow(dead_code)]
pub fn attachments_upload() {}

#[utoipa::path(
    get,
    path = "/attachments",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1, example: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 20, max: 100, example: 20)")
    ),
    responses(
        (status = 200, description = "Paginated list of user's attachments", body = PaginatedResponse<AttachmentResponse>, example = json!({
            "page": 1,
            "perPage": 20,
            "totalItems": 45,
            "totalPages": 3,
            "items": []
        })),
        (status = 401, description = "Unauthorized")
    )
)]
#[allow(dead_code)]
pub fn attachments_list() {}

#[utoipa::path(
    get,
    path = "/attachments/{id}",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("id" = i32, Path, description = "Attachment ID")
    ),
    responses(
        (status = 200, description = "Attachment details", body = AttachmentResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found")
    )
)]
#[allow(dead_code)]
pub fn attachments_get() {}

#[utoipa::path(
    patch,
    path = "/attachments/{id}",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("id" = i32, Path, description = "Attachment ID")
    ),
    request_body = UpdateAttachmentRequest,
    responses(
        (status = 200, description = "Attachment updated", body = AttachmentResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found")
    )
)]
#[allow(dead_code)]
pub fn attachments_update() {}

#[utoipa::path(
    delete,
    path = "/attachments/{id}",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("id" = i32, Path, description = "Attachment ID")
    ),
    responses(
        (status = 200, description = "Attachment deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found")
    )
)]
#[allow(dead_code)]
pub fn attachments_delete() {}
