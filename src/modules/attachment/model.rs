use crate::schemas::table::attachments;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Attachment record queried from the database.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = attachments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
  /// Auto-incremented attachment ID.
  pub id: i32,
  /// ID of the user who uploaded the file.
  pub user_id: String,
  /// Sanitized filename (no directory components).
  pub filename: String,
  /// Relative path on disk where the file is stored.
  pub path: String,
  /// MIME type of the uploaded file (e.g. `"image/png"`).
  pub mime_type: String,
  /// File size in bytes.
  pub size: i32,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

#[derive(Debug, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct AttachmentUploadForm {
  /// The file to upload
  #[serde(default)]
  #[schema(format = Binary, required = true)]
  pub file: String,
}

/// New attachment record for INSERT into the database.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = attachments)]
pub struct NewAttachment {
  /// ID of the user who uploaded the file.
  pub user_id: String,
  /// Sanitized filename.
  pub filename: String,
  /// Relative path on disk where the file was saved.
  pub path: String,
  /// MIME type of the uploaded file.
  pub mime_type: String,
  /// File size in bytes.
  pub size: i32,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

impl NewAttachment {
  pub fn new(
    user_id: String,
    filename: String,
    path: String,
    mime_type: String,
    size: i32,
  ) -> Self {
    let now = Utc::now().to_rfc3339();
    Self {
      user_id,
      filename,
      path,
      mime_type,
      size,
      created_at: now.clone(),
      updated_at: now,
    }
  }
}

/// Public attachment DTO returned in API responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentResponse {
  /// Auto-incremented attachment ID.
  pub id: i32,
  /// ID of the user who owns the file.
  pub user_id: String,
  /// Sanitized filename.
  pub filename: String,
  /// Relative path on disk where the file is stored.
  pub path: String,
  /// MIME type of the file (e.g. `"image/png"`).
  pub mime_type: String,
  /// File size in bytes.
  pub size: i32,
  /// ISO-8601 creation timestamp.
  pub created_at: String,
  /// ISO-8601 last-updated timestamp.
  pub updated_at: String,
}

impl From<Attachment> for AttachmentResponse {
  fn from(a: Attachment) -> Self {
    AttachmentResponse {
      id: a.id,
      user_id: a.user_id,
      filename: a.filename,
      path: a.path,
      mime_type: a.mime_type,
      size: a.size,
      created_at: a.created_at,
      updated_at: a.updated_at,
    }
  }
}

/// Request body for `PATCH /attachments/{id}` — all fields are optional.
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAttachmentRequest {
  /// New filename to assign; must not be empty if provided.
  #[validate(length(min = 1, message = "Filename cannot be empty"))]
  pub filename: Option<String>,
}
