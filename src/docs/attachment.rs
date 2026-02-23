use crate::modules::attachment::model::{
  AttachmentListResponse, AttachmentResponse, UpdateAttachmentRequest,
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
    responses(
        (status = 200, description = "List of user's attachments", body = AttachmentListResponse),
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
