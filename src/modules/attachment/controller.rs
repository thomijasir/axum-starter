use super::{
  model::{AttachmentResponse, AttachmentUploadForm, NewAttachment, UpdateAttachmentRequest},
  service,
};
use crate::{
  constants::ALLOWED_MIME_TYPES,
  extractors::{AuthUser, BodyJson, MultipartForm, PathParam},
  models::{AppState, PaginatedResponse, PaginationQuery},
  services::{HttpError, HttpErrorFormat, HttpResponse, HttpResponseFormat},
  utils::{files, string::slugify_filename},
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
};
use std::{path::Path as FsPath, sync::Arc};

#[utoipa::path(
    post,
    path = "/attachments/upload",
    tag = "attachments",
    security(("bearer_token" = [])),
    request_body(content_type = "multipart/form-data", content = inline(AttachmentUploadForm)),
    responses(
        (status = 201, description = "File uploaded successfully", body = HttpResponseFormat<AttachmentResponse>),
        (status = 400, description = "Invalid file or missing file", body = HttpErrorFormat,
            examples(
                ("NO_FILE_PROVIDED" = (value = json!({"success": false, "message": "ERR024|NO_FILE_PROVIDED"}))),
                ("EMPTY_FILE" = (value = json!({"success": false, "message": "ERR025|EMPTY_FILE"}))),
                ("INVALID_FILE_TYPE" = (value = json!({"success": false, "message": "ERR026|INVALID_FILE_TYPE:allowed=image/jpeg, image/png, image/webp"}))),
                ("INVALID_FILENAME" = (value = json!({"success": false, "message": "ERR027|INVALID_FILENAME"}))),
                ("FILE_TOO_LARGE" = (value = json!({"success": false, "message": "ERR031|FILE_TOO_LARGE:max=10mb"}))),
                ("INVALID_MULTIPART_DATA" = (value = json!({"success": false, "message": "ERR035|INVALID_MULTIPART_DATA:detail"})))
            )
        ),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        ),
        (status = 409, description = "File already exists", body = HttpErrorFormat,
            examples(
                ("FILE_ALREADY_EXISTS" = (value = json!({"success": false, "message": "ERR029|FILE_ALREADY_EXISTS"})))
            )
        )
    )
)]
/// — upload a file (multipart/form-data), validate MIME type, sanitize filename, and persist metadata.
pub async fn upload(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  MultipartForm { fields: _, files }: MultipartForm<AttachmentUploadForm>,
) -> Result<impl IntoResponse, HttpError> {
  let file = files.get("file").ok_or(HttpError::ERR024)?;

  if file.is_empty() {
    return Err(HttpError::ERR025);
  }

  // Validate MIME type against allowlist
  if !ALLOWED_MIME_TYPES.contains(&file.content_type.as_str()) {
    return Err(HttpError::ERR026(format!(
      "allowed={}",
      ALLOWED_MIME_TYPES.join(", ")
    )));
  }

  // Sanitize filename: strip directory components (path traversal) then slugify
  let base_filename = FsPath::new(&file.filename)
    .file_name()
    .and_then(|n| n.to_str())
    .ok_or(HttpError::ERR027)?;
  let sanitized_filename = slugify_filename(base_filename);

  let mime_type = file.content_type.clone();
  let contents = file.bytes.clone();
  let size = file.size as i32;

  let file_path = format!("{}/{}", auth.user_id, sanitized_filename);

  let path = files::save_file_from_bytes(&file_path, &contents, false)
    .await
    .map_err(|e| {
      if e.to_string() == "ERR028|FILE_EXISTS" {
        HttpError::ERR029
      } else {
        HttpError::ERR030
      }
    })?;

  let new_attachment = NewAttachment::new(auth.user_id, sanitized_filename, path, mime_type, size);

  let attachment = service::create(&state.db, new_attachment).await?;

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
        (status = 200, description = "Paginated list of user's attachments", body = HttpResponseFormat<PaginatedResponse<AttachmentResponse>>),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        )
    )
)]
/// — returns a paginated list of attachments belonging to the authenticated user.
pub async fn list(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, HttpError> {
  let result = service::find_by_user(&state.db, auth.user_id, pagination).await?;
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
        (status = 200, description = "Attachment details", body = HttpResponseFormat<AttachmentResponse>),
        (status = 400, description = "Invalid path parameter", body = HttpErrorFormat,
            examples(
                ("INVALID_PATH_PARAM" = (value = json!({"success": false, "message": "ERR032|INVALID_PATH_PARAM:id"})))
            )
        ),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        ),
        (status = 404, description = "Attachment not found", body = HttpErrorFormat,
            examples(
                ("ATTACHMENT_NOT_FOUND" = (value = json!({"success": false, "message": "ERR023|ATTACHMENT_NOT_FOUND"})))
            )
        )
    )
)]
/// — returns a single attachment by ID, enforcing ownership against the authenticated user.
pub async fn get_by_id(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::find_by_id(&state.db, id).await?;

  if attachment.user_id != auth.user_id {
    return Err(HttpError::ERR023);
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
        (status = 200, description = "Attachment updated", body = HttpResponseFormat<AttachmentResponse>),
        (status = 400, description = "Validation error", body = HttpErrorFormat,
            examples(
                ("INVALID_PATH_PARAM" = (value = json!({"success": false, "message": "ERR032|INVALID_PATH_PARAM:id"}))),
                ("INVALID_BODY_REQUEST" = (value = json!({"success": false, "message": "ERR033|INVALID_BODY_REQUEST:detail"}))),
                ("INVALID_VALIDATION" = (value = json!({"success": false, "message": "ERR034|INVALID_VALIDATION:field|rule|message"})))
            )
        ),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        ),
        (status = 404, description = "Attachment not found", body = HttpErrorFormat,
            examples(
                ("ATTACHMENT_NOT_FOUND" = (value = json!({"success": false, "message": "ERR023|ATTACHMENT_NOT_FOUND"})))
            )
        )
    )
)]
/// — updates attachment metadata for the given ID, scoped to the authenticated user.
pub async fn update(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
  BodyJson(body): BodyJson<UpdateAttachmentRequest>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::update(&state.db, id, auth.user_id, body).await?;

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
        (status = 200, description = "Attachment deleted", body = HttpResponseFormat<AttachmentResponse>),
        (status = 400, description = "Invalid path parameter", body = HttpErrorFormat,
            examples(
                ("INVALID_PATH_PARAM" = (value = json!({"success": false, "message": "ERR032|INVALID_PATH_PARAM:id"})))
            )
        ),
        (status = 401, description = "Unauthorized", body = HttpErrorFormat,
            examples(
                ("MISSING_OR_INVALID_AUTHORIZATION_HEADER" = (value = json!({"success": false, "message": "ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER"}))),
                ("EXPIRED_SIGNATURE" = (value = json!({"success": false, "message": "ERR019|EXPIRED_SIGNATURE"}))),
                ("INVALID_TOKEN" = (value = json!({"success": false, "message": "ERR018|INVALID_TOKEN"})))
            )
        ),
        (status = 404, description = "Attachment not found", body = HttpErrorFormat,
            examples(
                ("ATTACHMENT_NOT_FOUND" = (value = json!({"success": false, "message": "ERR023|ATTACHMENT_NOT_FOUND"})))
            )
        )
    )
)]
/// — deletes an attachment record and its file from disk, scoped to the authenticated user.
pub async fn delete(
  State(state): State<Arc<AppState>>,
  auth: AuthUser,
  PathParam(id): PathParam<i32>,
) -> Result<impl IntoResponse, HttpError> {
  let attachment = service::delete(&state.db, id, auth.user_id).await?;

  if let Err(e) = files::delete_file(&attachment.path).await {
    tracing::warn!(error = %e, path = %attachment.path, "Failed to delete file from disk");
  }

  Ok(HttpResponse::ok(
    AttachmentResponse::from(attachment),
    "DELETED",
  ))
}
