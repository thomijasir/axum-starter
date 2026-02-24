use super::{
  model::{NewUser, User},
  repository,
};
use crate::{
  models::PaginatedResponse,
  modules::user::model::{UserQuery, UserResponse},
  services::DBSqlite,
  utils::to_i64,
};
use anyhow::Result;

pub async fn find_by_id(
  db: &DBSqlite,
  uid: String,
) -> Result<User> {
  repository::find_by_id(db, uid).await
}

pub async fn find_by_email(
  db: &DBSqlite,
  user_email: &str,
) -> Result<Option<User>> {
  repository::find_by_email(db, user_email).await
}

pub async fn create(
  db: &DBSqlite,
  new_user: NewUser,
) -> Result<User> {
  repository::insert(db, new_user).await
}

pub async fn find_all(
  db: &DBSqlite,
  query: UserQuery,
) -> Result<PaginatedResponse<UserResponse>> {
  let offset = query.pagination.offset();
  let limit = query.pagination.effective_limit();

  let (results, total) = super::repository::find_all(db, offset, to_i64(limit)).await?;

  let items: Vec<UserResponse> = results.into_iter().map(Into::into).collect();

  Ok(PaginatedResponse::new(
    items,
    query.pagination.page,
    query.pagination.effective_limit(),
    total,
  ))
}
