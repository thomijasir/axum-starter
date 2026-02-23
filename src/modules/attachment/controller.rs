use super::{
  model::{AttachmentResponse, NewAttachment, UpdateAttachmentRequest},
  service,
};
use crate::{
  extractors::{AuthUser, BodyJson, MultipartForm},
  models::{AppState, PaginatedResponse, PaginationQuery},
  utils::{HttpError, HttpResponse, files},
};
use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UploadForm {}

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
pub async fn upload(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  MultipartForm { fields: _, files }: MultipartForm<UploadForm>,
) -> Result<impl IntoResponse, HttpError> {
  let file = files.get("file").ok_or_else(|| HttpError::bad_request("NO_FILE_PROVIDED"))?;

  if file.is_empty() {
    return Err(HttpError::bad_request("EMPTY_FILE"));
  }

  let filename = file.filename.clone();
  let mime_type = file.content_type.clone();
  let contents = file.bytes.clone();
  let size = file.size as i32;

  let user_id = auth.user_id.clone();
  let file_path = format!("{}/{}", user_id, filename);

  let path = files::save_file_from_bytes(&file_path, &contents, false)
    .await
    .map_err(|e| {
      if e.to_string().contains("FILE_EXISTS") {
        HttpError::unique_constraint_violation("FILE_ALREADY_EXISTS")
      } else {
        HttpError::server_error("FILE_UPLOAD_FAILED")
      }
    })?;

  let new_attachment = NewAttachment::new(user_id, filename, path, mime_type, size);

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
pub async fn get(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::find_by_id(&state.db, id)
    .await
    .map_err(HttpError::from_service_error)?;

  if attachment.user_id != auth.user_id {
    return Err(HttpError::not_found("ATTACHMENT_NOT_FOUND"));
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
  Path(id): Path<i32>,
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
        ("id" = i32, Path, description = "Attachment ID")
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
  Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::delete(&state.db, id, auth.user_id.clone())
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
