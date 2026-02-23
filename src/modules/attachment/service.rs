use super::model::{Attachment, AttachmentResponse, NewAttachment, UpdateAttachmentRequest};
use crate::{models::PaginatedResponse, services::DBSqlite, utils::PaginationQuery};

pub async fn find_by_id(
  db: &DBSqlite,
  id: i32,
) -> anyhow::Result<Attachment> {
  super::repository::find_by_id(db, id).await
}

pub async fn find_by_user(
  db: &DBSqlite,
  user_id: String,
  pagination: PaginationQuery,
) -> anyhow::Result<PaginatedResponse<AttachmentResponse>> {
  let offset = pagination.offset();
  let limit = pagination.effective_limit() as i64;

  let (results, total) = super::repository::find_by_user(db, user_id, offset, limit).await?;

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
) -> anyhow::Result<PaginatedResponse<AttachmentResponse>> {
  let offset = pagination.offset();
  let limit = pagination.effective_limit() as i64;

  let (results, total) = super::repository::find_all(db, offset, limit).await?;

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
) -> anyhow::Result<Attachment> {
  super::repository::insert(db, new_attachment).await
}

pub async fn update(
  db: &DBSqlite,
  id: i32,
  user_id: String,
  req: UpdateAttachmentRequest,
) -> anyhow::Result<Attachment> {
  super::repository::update(db, id, user_id, req).await
}

pub async fn delete(
  db: &DBSqlite,
  id: i32,
  user_id: String,
) -> anyhow::Result<Attachment> {
  super::repository::delete(db, id, user_id).await
}
