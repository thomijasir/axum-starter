use crate::services::HttpError;
use axum::{
  extract::{FromRequestParts, Path, rejection::PathRejection},
  http::request::Parts,
};
use serde::de::DeserializeOwned;
use std::ops::Deref;

/// Custom path extractor that converts Axum's [`PathRejection`] into a
/// structured [`HttpError`] instead of Axum's default plain-text response.
///
/// Drop-in replacement for `axum::extract::Path`:
///
/// ```text
/// // before
/// Path(id): Path<i32>
///
/// // after
/// PathParam(id): PathParam<i32>
/// ```
pub struct PathParam<T>(pub T);

impl<T> Deref for PathParam<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S, T> FromRequestParts<S> for PathParam<T>
where
  T: DeserializeOwned + Send,
  S: Send + Sync,
{
  type Rejection = HttpError;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    Path::<T>::from_request_parts(parts, state)
      .await
      .map(|Path(value)| PathParam(value))
      .map_err(|rejection| match rejection {
        PathRejection::FailedToDeserializePathParams(e) => {
          HttpError::bad_request(format!("INVALID_PATH_PARAM|{}", e))
        }
        _ => HttpError::bad_request("INVALID_PATH_PARAM"),
      })
  }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
  use super::*;
  use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::get,
  };
  use tower::ServiceExt;

  async fn handler_i32(PathParam(id): PathParam<i32>) -> StatusCode {
    let _ = id;
    StatusCode::OK
  }

  fn test_app() -> Router {
    Router::new().route("/items/{id}", get(handler_i32))
  }

  #[tokio::test]
  async fn valid_integer_path_param() {
    let response = test_app()
      .oneshot(
        Request::builder()
          .uri("/items/42")
          .body(Body::empty())
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  }

  #[tokio::test]
  async fn invalid_path_param_returns_bad_request() {
    let response = test_app()
      .oneshot(
        Request::builder()
          .uri("/items/not-a-number")
          .body(Body::empty())
          .unwrap(),
      )
      .await
      .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }
}
