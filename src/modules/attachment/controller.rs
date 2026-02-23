use super::{
  model::{AttachmentResponse, NewAttachment, UpdateAttachmentRequest},
  service,
};
use crate::{
  extractors::{AuthUser, BodyJson},
  models::{AppState, PaginatedResponse, PaginationQuery},
  utils::{HttpError, HttpResponse, files},
};
use axum::{
  extract::{Multipart, Path, Query, State},
  response::IntoResponse,
};
use std::sync::Arc;

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
  mut multipart: Multipart,
) -> Result<impl IntoResponse, HttpError> {
  let mut filename = None;
  let mut mime_type = None;
  let mut contents = None;

  while let Some(field) = multipart.next_field().await.map_err(|e| {
    tracing::error!(error = %e, "Failed to read multipart field");
    HttpError::bad_request("INVALID_MULTIPART_DATA")
  })? {
    let name = field.name().unwrap_or("").to_string();

    if name == "file" {
      filename = field.file_name().map(|s| s.to_string());
      mime_type = field.content_type().map(|s| s.to_string());
      contents = Some(field.bytes().await.map_err(|e| {
        tracing::error!(error = %e, "Failed to read file bytes");
        HttpError::bad_request("FAILED_TO_READ_FILE")
      })?);
      break;
    }
  }

  let filename = filename.unwrap_or_else(|| "unknown".to_string());
  let mime_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
  let contents = contents.ok_or_else(|| HttpError::bad_request("NO_FILE_PROVIDED"))?;

  let size = contents.len() as i32;

  if size == 0 {
    return Err(HttpError::bad_request("EMPTY_FILE"));
  }

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
