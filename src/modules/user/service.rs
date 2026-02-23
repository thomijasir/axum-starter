use super::{
  model::{NewUser, User},
  repository,
};
use crate::services::DBSqlite;
use anyhow::Result;

pub async fn find_by_id(
  db: &DBSqlite,
  uid: String,
) -> Result<User> {
  repository::find_by_id(db, uid).await
}

pub async fn find_by_email(
  db: &DBSqlite,
  user_email: String,
) -> Result<Option<User>> {
  repository::find_by_email(db, user_email).await
}

pub async fn create(
  db: &DBSqlite,
  new_user: NewUser,
) -> Result<User> {
  repository::insert(db, new_user).await
}
