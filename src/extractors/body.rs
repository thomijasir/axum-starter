use crate::{constants::error::*, services::HttpError, utils::validation::format_validation_errors};
use axum::{
  Json,
  body::Body,
  extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use validator::Validate;

// Define the extractor struct
pub struct BodyJson<T>(pub T);

// Implement Deref for easy access to the inner value
impl<T> Deref for BodyJson<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S, T> FromRequest<S> for BodyJson<T>
where
  S: Send + Sync,
  T: DeserializeOwned + Validate + Send,
{
  type Rejection = HttpError;

  async fn from_request(
    req: Request<Body>,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state)
      .await
      .map_err(|e| HttpError::bad_request(format!("{} {}", ERR033, e)))?;

    value.validate().map_err(|e| {
      HttpError::bad_request(format!(
        "{} {}",
        ERR034,
        format_validation_errors(&e)
      ))
    })?;

    Ok(BodyJson(value))
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
    routing::post,
  };
  use serde::Deserialize;
  use tower::ServiceExt;
  use validator::Validate;

  #[derive(Deserialize, Validate, Debug)]
  struct TestPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    username: String,
    #[validate(range(min = 18, message = "Must be 18 or older"))]
    age: u32,
  }

  async fn test_handler(BodyJson(payload): BodyJson<TestPayload>) -> StatusCode {
    println!("Received valid payload: {:?}", payload);
    StatusCode::OK
  }

  fn test_app() -> Router {
    Router::new().route("/", post(test_handler))
  }

  #[tokio::test]
  async fn valid_request() {
    let app = test_app();
    let request_body = r#"{"username": "testuser", "age": 25}"#;
    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header("content-type", "application/json")
      .body(Body::from(request_body))
      .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  }

  #[tokio::test]
  async fn invalid_data_validation_failed() {
    let app = test_app();
    let request_body = r#"{"username": "a", "age": 30}"#;
    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header("content-type", "application/json")
      .body(Body::from(request_body))
      .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }

  #[tokio::test]
  async fn invalid_json_format() {
    let app = test_app();
    let request_body = r#"{"username": "testuser", "age": 25"#;
    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header("content-type", "application/json")
      .body(Body::from(request_body))
      .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }

  #[tokio::test]
  async fn invalid_json_missing_field() {
    let app = test_app();
    let request_body = r#"{"username": "testuser"}"#;
    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header("content-type", "application/json")
      .body(Body::from(request_body))
      .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
  }
}
