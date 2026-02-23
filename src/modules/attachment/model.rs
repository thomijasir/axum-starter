use crate::schemas::table::attachments;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = attachments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attachment {
  pub id: i32,
  pub user_id: String,
  pub filename: String,
  pub path: String,
  pub mime_type: String,
  pub size: i32,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = attachments)]
pub struct NewAttachment {
  pub user_id: String,
  pub filename: String,
  pub path: String,
  pub mime_type: String,
  pub size: i32,
  pub created_at: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttachmentResponse {
  pub id: i32,
  pub user_id: String,
  pub filename: String,
  pub path: String,
  pub mime_type: String,
  pub size: i32,
  pub created_at: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttachmentListResponse {
  pub attachments: Vec<AttachmentResponse>,
  pub total: usize,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UpdateAttachmentRequest {
  #[validate(length(min = 1, message = "Filename cannot be empty"))]
  pub filename: Option<String>,
}
