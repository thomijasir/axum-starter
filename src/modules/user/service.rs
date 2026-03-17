use super::{
  model::{NewUser, User},
  repository,
};
use crate::{
  models::PaginatedResponse,
  modules::user::model::{UserQuery, UserResponse},
  services::{DBSqlite, HttpError},
  utils::to_i64,
};

pub async fn find_by_id(
  db: &DBSqlite,
  uid: String,
) -> Result<User, HttpError> {
  repository::find_by_id(db, uid).await.map_err(HttpError::from)
}

pub async fn find_by_email(
  db: &DBSqlite,
  user_email: &str,
) -> Result<Option<User>, HttpError> {
  repository::find_by_email(db, user_email)
    .await
    .map_err(HttpError::from)
}

pub async fn create(
  db: &DBSqlite,
  new_user: NewUser,
) -> Result<User, HttpError> {
  repository::insert(db, new_user).await.map_err(HttpError::from)
}

pub async fn find_all(
  db: &DBSqlite,
  query: UserQuery,
) -> Result<PaginatedResponse<UserResponse>, HttpError> {
  let offset = query.pagination.offset();
  let limit = query.pagination.effective_limit();

  let (results, total) = super::repository::find_all(db, offset, to_i64(limit))
    .await
    .map_err(HttpError::from)?;

  let items: Vec<UserResponse> = results.into_iter().map(Into::into).collect();

  Ok(PaginatedResponse::new(
    items,
    query.pagination.page,
    query.pagination.effective_limit(),
    total,
  ))
}
