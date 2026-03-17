use super::model::{Attachment, AttachmentResponse, NewAttachment, UpdateAttachmentRequest};
use crate::{
  models::{PaginatedResponse, PaginationQuery},
  services::{DBSqlite, HttpError},
};

pub async fn find_by_id(
  db: &DBSqlite,
  id: i32,
) -> Result<Attachment, HttpError> {
  super::repository::find_by_id(db, id).await.map_err(HttpError::from)
}

pub async fn find_by_user(
  db: &DBSqlite,
  user_id: String,
  pagination: PaginationQuery,
) -> Result<PaginatedResponse<AttachmentResponse>, HttpError> {
  let offset = pagination.offset();
  let limit = pagination.effective_limit() as i64;

  let (results, total) = super::repository::find_by_user(db, user_id, offset, limit)
    .await
    .map_err(HttpError::from)?;

  let items: Vec<AttachmentResponse> = results.into_iter().map(Into::into).collect();

  Ok(PaginatedResponse::new(
    items,
    pagination.page,
    pagination.effective_limit(),
    total as u32,
  ))
}

pub async fn find_all(
  db: &DBSqlite,
  pagination: PaginationQuery,
) -> Result<PaginatedResponse<AttachmentResponse>, HttpError> {
  let offset = pagination.offset();
  let limit = pagination.effective_limit() as i64;

  let (results, total) = super::repository::find_all(db, offset, limit)
    .await
    .map_err(HttpError::from)?;

  let items: Vec<AttachmentResponse> = results.into_iter().map(Into::into).collect();

  Ok(PaginatedResponse::new(
    items,
    pagination.page,
    pagination.effective_limit(),
    total as u32,
  ))
}

pub async fn create(
  db: &DBSqlite,
  new_attachment: NewAttachment,
) -> Result<Attachment, HttpError> {
  super::repository::insert(db, new_attachment)
    .await
    .map_err(HttpError::from)
}

pub async fn update(
  db: &DBSqlite,
  id: i32,
  user_id: String,
  req: UpdateAttachmentRequest,
) -> Result<Attachment, HttpError> {
  super::repository::update(db, id, user_id, req)
    .await
    .map_err(HttpError::from)
}

pub async fn delete(
  db: &DBSqlite,
  id: i32,
  user_id: String,
) -> Result<Attachment, HttpError> {
  super::repository::delete(db, id, user_id).await.map_err(HttpError::from)
}
