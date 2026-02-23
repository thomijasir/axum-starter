use super::model::{Attachment, AttachmentListResponse, NewAttachment, UpdateAttachmentRequest};
use crate::services::DBSqlite;

pub async fn find_by_id(db: &DBSqlite, id: i32) -> anyhow::Result<Attachment> {
    super::repository::find_by_id(db, id).await
}

pub async fn find_by_user(db: &DBSqlite, user_id: String) -> anyhow::Result<AttachmentListResponse> {
    super::repository::find_by_user(db, user_id).await
}

pub async fn find_all(db: &DBSqlite) -> anyhow::Result<AttachmentListResponse> {
    super::repository::find_all(db).await
}

pub async fn create(db: &DBSqlite, new_attachment: NewAttachment) -> anyhow::Result<Attachment> {
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

pub async fn delete(db: &DBSqlite, id: i32, user_id: String) -> anyhow::Result<Attachment> {
    super::repository::delete(db, id, user_id).await
}
