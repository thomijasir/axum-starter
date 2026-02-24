use crate::{services::HttpError, utils::validation::format_validation_errors};
use axum::{
  body::{Body, Bytes},
  extract::{FromRequest, Multipart, Request},
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::ops::Deref;
use validator::{Validate, ValidationErrors};

const DEFAULT_MAX_SIZE: usize = 10 * 1024 * 1024;
const DEFAULT_MAX_FILES: usize = 5;

#[derive(Debug, Clone)]
pub struct FileValidationConfig {
  pub max_size: usize,
  pub allowed_mime_types: Option<Vec<String>>,
  pub max_files: usize,
}

impl Default for FileValidationConfig {
  fn default() -> Self {
    Self {
      max_size: DEFAULT_MAX_SIZE,
      allowed_mime_types: None,
      max_files: DEFAULT_MAX_FILES,
    }
  }
}

impl FileValidationConfig {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn max_size(
    mut self,
    size: usize,
  ) -> Self {
    self.max_size = size;
    self
  }

  pub fn allowed_mime_types(
    mut self,
    types: Vec<String>,
  ) -> Self {
    self.allowed_mime_types = Some(types);
    self
  }

  pub fn max_files(
    mut self,
    count: usize,
  ) -> Self {
    self.max_files = count;
    self
  }
}

#[derive(Debug, Clone)]
pub struct MultipartFile {
  pub filename: String,
  pub content_type: String,
  pub bytes: Bytes,
  pub size: usize,
}

impl MultipartFile {
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  pub fn validate(
    &self,
    config: &FileValidationConfig,
  ) -> Result<(), HttpError> {
    if self.size > config.max_size {
      return Err(HttpError::bad_request(format!(
        "FILE_TOO_LARGE|max_size={}bytes, actual={}bytes",
        config.max_size, self.size
      )));
    }

    if let Some(allowed) = &config.allowed_mime_types {
      if !allowed.contains(&self.content_type) {
        return Err(HttpError::bad_request(format!(
          "INVALID_FILE_TYPE|allowed={}",
          allowed.join(", ")
        )));
      }
    }

    Ok(())
  }
}

pub struct MultipartForm<T> {
  pub fields: T,
  pub files: HashMap<String, MultipartFile>,
}

impl<T> Deref for MultipartForm<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.fields
  }
}

impl<S, T> FromRequest<S> for MultipartForm<T>
where
  S: Send + Sync,
  T: DeserializeOwned + Validate + Send,
{
  type Rejection = HttpError;

  async fn from_request(
    req: Request<Body>,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let config = FileValidationConfig::default();

    let multipart = Multipart::from_request(req, state)
      .await
      .map_err(|e| HttpError::bad_request(format!("INVALID_MULTIPART_DATA|{}", e)))?;

    parse_multipart(multipart, &config).await
  }
}

async fn parse_multipart<T>(
  mut multipart: Multipart,
  config: &FileValidationConfig,
) -> Result<MultipartForm<T>, HttpError>
where
  T: DeserializeOwned + Validate + Send,
{
  let mut text_fields: HashMap<String, String> = HashMap::new();
  let mut files: HashMap<String, MultipartFile> = HashMap::new();

  while let Some(field) = multipart
    .next_field()
    .await
    .map_err(|e| HttpError::bad_request(format!("INVALID_MULTIPART_FIELD|{}", e)))?
  {
    let field_name = field.name().unwrap_or("").to_string();

    if field.content_type().is_some() && field.file_name().is_some() {
      let filename = field.file_name().unwrap_or("unknown").to_string();
      let content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

      let bytes = field
        .bytes()
        .await
        .map_err(|e| HttpError::bad_request(format!("FAILED_TO_READ_FILE|{}", e)))?;

      let size = bytes.len();

      let file = MultipartFile {
        filename,
        content_type,
        bytes,
        size,
      };

      file.validate(config)?;

      files.insert(field_name, file);

      if files.len() > config.max_files {
        return Err(HttpError::bad_request(format!(
          "TOO_MANY_FILES|max={}",
          config.max_files
        )));
      }
    } else {
      let text = field
        .text()
        .await
        .map_err(|e| HttpError::bad_request(format!("FAILED_TO_READ_FIELD|{}", e)))?;

      text_fields.insert(field_name, text);
    }
  }

  let fields: T = serde_json::from_value(
    serde_json::to_value(&text_fields)
      .map_err(|e| HttpError::bad_request(format!("INVALID_FIELD_SERIALIZATION|{}", e)))?,
  )
  .map_err(|e| HttpError::bad_request(format!("INVALID_FIELD_FORMAT|{}", e)))?;

  fields.validate().map_err(|e: ValidationErrors| {
    HttpError::bad_request(format!(
      "INVALID_VALIDATION|{}",
      format_validation_errors(&e)
    ))
  })?;

  Ok(MultipartForm { fields, files })
}

pub struct MultipartFormWithConfig<T> {
  pub fields: T,
  pub files: HashMap<String, MultipartFile>,
  pub config: FileValidationConfig,
}

impl<T> Deref for MultipartFormWithConfig<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.fields
  }
}

impl<S, T> FromRequest<S> for MultipartFormWithConfig<T>
where
  S: Send + Sync,
  T: DeserializeOwned + Validate + Send,
{
  type Rejection = HttpError;

  async fn from_request(
    req: Request<Body>,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let config = req
      .extensions()
      .get::<FileValidationConfig>()
      .cloned()
      .unwrap_or_default();

    let multipart = Multipart::from_request(req, state)
      .await
      .map_err(|e| HttpError::bad_request(format!("INVALID_MULTIPART_DATA|{}", e)))?;

    let MultipartForm { fields, files } = parse_multipart(multipart, &config).await?;

    Ok(MultipartFormWithConfig {
      fields,
      files,
      config,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{
    Router,
    http::{Request, header::CONTENT_TYPE},
    routing::post,
  };
  use serde::Deserialize;
  use tower::ServiceExt;

  #[derive(Debug, Deserialize, Validate)]
  struct TestForm {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    name: String,
  }

  async fn test_handler(MultipartForm { fields, files }: MultipartForm<TestForm>) -> &'static str {
    assert!(!files.is_empty() || fields.name.len() >= 3);
    "OK"
  }

  fn test_app() -> Router {
    Router::new().route("/", post(test_handler))
  }

  #[tokio::test]
  async fn valid_multipart_form() {
    let body = "--boundary\r\n\
      Content-Disposition: form-data; name=\"name\"\r\n\r\n\
      testname\r\n\
      --boundary--\r\n";

    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header(CONTENT_TYPE, "multipart/form-data; boundary=boundary")
      .body(Body::from(body))
      .unwrap();

    let response = test_app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
  }

  #[tokio::test]
  async fn validation_fails() {
    let body = "--boundary\r\n\
      Content-Disposition: form-data; name=\"name\"\r\n\r\n\
      ab\r\n\
      --boundary--\r\n";

    let request = Request::builder()
      .method("POST")
      .uri("/")
      .header(CONTENT_TYPE, "multipart/form-data; boundary=boundary")
      .body(Body::from(body))
      .unwrap();

    let response = test_app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
  }
}
