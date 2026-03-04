use super::{
  model::{AttachmentResponse, NewAttachment, UpdateAttachmentRequest},
  service,
};
use crate::{
  constants::error::*,
  extractors::{AuthUser, BodyJson, MultipartForm, PathParam},
  models::{AppState, PaginatedResponse, PaginationQuery},
  services::{HttpError, HttpResponse},
  utils::{files, string::slugify_filename},
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
};
use serde::Deserialize;
use std::{path::Path as FsPath, sync::Arc};

/// MIME types accepted for file uploads.
const ALLOWED_MIME_TYPES: &[&str] = &[
  "image/jpeg",
  "image/png",
  "image/gif",
  "image/webp",
  "application/pdf",
  "text/plain",
  "text/csv",
];

#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct UploadForm {
  /// The file to upload
  #[serde(default)]
  #[schema(format = Binary, required = true)]
  pub file: String,
}

#[utoipa::path(
    post,
    path = "/attachments/upload",
    tag = "attachments",
    security(("bearer_token" = [])),
    request_body(content_type = "multipart/form-data", content = inline(UploadForm)),
    responses(
        (status = 201, description = "File uploaded successfully", body = AttachmentResponse),
        (status = 400, description = "Invalid file or missing file"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn upload(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  MultipartForm { fields: _, files }: MultipartForm<UploadForm>,
) -> Result<impl IntoResponse, HttpError> {
  let file = files
    .get("file")
    .ok_or_else(|| HttpError::bad_request(ERR024))?;

  if file.is_empty() {
    return Err(HttpError::bad_request(ERR025));
  }

  // Validate MIME type against allowlist
  if !ALLOWED_MIME_TYPES.contains(&file.content_type.as_str()) {
    return Err(HttpError::bad_request(format!(
      "{} allowed={}",
      ERR026,
      ALLOWED_MIME_TYPES.join(", ")
    )));
  }

  // Sanitize filename: strip directory components (path traversal) then slugify
  let base_filename = FsPath::new(&file.filename)
    .file_name()
    .and_then(|n| n.to_str())
    .ok_or_else(|| HttpError::bad_request(ERR027))?;
  let sanitized_filename = slugify_filename(base_filename);

  let mime_type = file.content_type.clone();
  let contents = file.bytes.clone();
  let size = file.size as i32;

  let file_path = format!("{}/{}", auth.user_id, sanitized_filename);

  let path = files::save_file_from_bytes(&file_path, &contents, false)
    .await
    .map_err(|e| {
      if e.to_string() == ERR028 {
        HttpError::unique_constraint_violation(ERR029)
      } else {
        HttpError::server_error(ERR030)
      }
    })?;

  let new_attachment = NewAttachment::new(auth.user_id, sanitized_filename, path, mime_type, size);

  let attachment = service::create(&state.db, new_attachment)
    .await
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::created(
    AttachmentResponse::from(attachment),
    "FILE_UPLOADED",
  ))
}

#[utoipa::path(
    get,
    path = "/attachments",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 20, max: 100)")
    ),
    responses(
        (status = 200, description = "Paginated list of user's attachments", body = PaginatedResponse<AttachmentResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, HttpError> {
  let result = service::find_by_user(&state.db, auth.user_id, pagination)
    .await
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::ok(result, "OK"))
}

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
pub async fn get_by_id(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::find_by_id(&state.db, id)
    .await
    .map_err(HttpError::from_service_error)?;

  if attachment.user_id != auth.user_id {
    return Err(HttpError::not_found(ERR023));
  }

  Ok(HttpResponse::ok(AttachmentResponse::from(attachment), "OK"))
}

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
pub async fn update(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
  BodyJson(body): BodyJson<UpdateAttachmentRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::update(&state.db, id, auth.user_id, body)
    .await
    .map_err(HttpError::from_service_error)?;

  Ok(HttpResponse::ok(
    AttachmentResponse::from(attachment),
    "UPDATED",
  ))
}

#[utoipa::path(
    delete,
    path = "/attachments/{id}",
    tag = "attachments",
    security(("bearer_token" = [])),
    params(
        ("id" = String, Path, description = "Attachment ID")
    ),
    responses(
        (status = 200, description = "Attachment deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found")
    )
)]
pub async fn delete(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::delete(&state.db, id, auth.user_id)
    .await
    .map_err(HttpError::from_service_error)?;

  if let Err(e) = files::delete_file(&attachment.path).await {
    tracing::warn!(error = %e, path = %attachment.path, "Failed to delete file from disk");
  }

  Ok(HttpResponse::ok(
    AttachmentResponse::from(attachment),
    "DELETED",
  ))
}
