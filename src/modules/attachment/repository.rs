use super::model::{Attachment, NewAttachment, UpdateAttachmentRequest};
use crate::{schemas::table::attachments, services::DBSqlite};
use diesel::prelude::*;

pub async fn find_by_id(
  db: &DBSqlite,
  id: i32,
) -> anyhow::Result<Attachment> {
  let attachment = db
    .execute(move |conn| {
      attachments::table
        .filter(attachments::id.eq(id))
        .select(Attachment::as_select())
        .first(conn)
        .optional()
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))
    })
    .await?;

  attachment.ok_or_else(|| anyhow::anyhow!("ATTACHMENT_NOT_FOUND"))
}

pub async fn find_by_user(
  db: &DBSqlite,
  user_id: String,
  offset: i64,
  limit: i64,
) -> anyhow::Result<(Vec<Attachment>, i64)> {
  let user_id_clone = user_id.clone();
  let count = db
    .execute(move |conn| {
      attachments::table
        .filter(attachments::user_id.eq(&user_id_clone))
        .count()
        .first::<i64>(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))
    })
    .await?;

  let results: Vec<Attachment> = db
    .execute(move |conn| {
      attachments::table
        .filter(attachments::user_id.eq(&user_id))
        .order(attachments::created_at.desc())
        .offset(offset)
        .limit(limit)
        .select(Attachment::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))
    })
    .await?;

  Ok((results, count))
}

pub async fn find_all(
  db: &DBSqlite,
  offset: i64,
  limit: i64,
) -> anyhow::Result<(Vec<Attachment>, i64)> {
  let count = db
    .execute(move |conn| {
      attachments::table
        .count()
        .first::<i64>(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))
    })
    .await?;

  let results: Vec<Attachment> = db
    .execute(move |conn| {
      attachments::table
        .order(attachments::created_at.desc())
        .offset(offset)
        .limit(limit)
        .select(Attachment::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))
    })
    .await?;

  Ok((results, count))
}

pub async fn insert(
  db: &DBSqlite,
  new_attachment: NewAttachment,
) -> anyhow::Result<Attachment> {
  db.transaction(move |conn| {
    diesel::insert_into(attachments::table)
      .values(&new_attachment)
      .execute(conn)
      .map_err(|e| anyhow::anyhow!("DB error: {}", e))?;

    attachments::table
      .order(attachments::id.desc())
      .select(Attachment::as_select())
      .first(conn)
      .map_err(|e| anyhow::anyhow!("DB error: {}", e))
  })
  .await
}

pub async fn update(
  db: &DBSqlite,
  id: i32,
  user_id: String,
  req: UpdateAttachmentRequest,
) -> anyhow::Result<Attachment> {
  let now = chrono::Utc::now().to_rfc3339();

  db.transaction(move |conn| {
    let target = attachments::table
      .filter(attachments::id.eq(id))
      .filter(attachments::user_id.eq(&user_id));

    if let Some(filename) = &req.filename {
      diesel::update(target)
        .set((
          attachments::filename.eq(filename),
          attachments::updated_at.eq(&now),
        ))
        .execute(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))?;
    } else {
      diesel::update(target)
        .set(attachments::updated_at.eq(&now))
        .execute(conn)
        .map_err(|e| anyhow::anyhow!("DB error: {}", e))?;
    }

    attachments::table
      .filter(attachments::id.eq(id))
      .select(Attachment::as_select())
      .first(conn)
      .map_err(|e| anyhow::anyhow!("DB error: {}", e))
  })
  .await
}

pub async fn delete(
  db: &DBSqlite,
  id: i32,
  user_id: String,
) -> anyhow::Result<Attachment> {
  db.transaction(move |conn| {
    let attachment = attachments::table
      .filter(attachments::id.eq(id))
      .filter(attachments::user_id.eq(&user_id))
      .select(Attachment::as_select())
      .first(conn)
      .optional()
      .map_err(|e| anyhow::anyhow!("DB error: {}", e))?
      .ok_or_else(|| anyhow::anyhow!("ATTACHMENT_NOT_FOUND"))?;

    diesel::delete(attachments::table.filter(attachments::id.eq(id)))
      .execute(conn)
      .map_err(|e| anyhow::anyhow!("DB error: {}", e))?;

    Ok(attachment)
  })
  .await
}
