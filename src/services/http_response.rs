use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt;
use std::fmt::Display;

/// Standard success message variants used in response bodies.
#[derive(Debug, Clone)]
pub enum ResponsesMessage {
  /// Generic success — maps to `"OK"`.
  OK,
  /// Resource created — maps to `"Resource successfully created"`.
  CREATED,
}

impl ResponsesMessage {
  pub fn to_str(&self) -> String {
    match self {
      ResponsesMessage::OK => "OK".to_string(),
      ResponsesMessage::CREATED => "Resource successfully created".to_string(),
    }
  }
}

impl Display for ResponsesMessage {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.to_str())
  }
}

/// JSON envelope serialized into every successful HTTP response body.
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(bound = "T: utoipa::ToSchema")]
#[serde(rename_all = "camelCase")]
pub struct HttpResponseFormat<T = serde_json::Value>
where
  T: serde::Serialize,
{
  /// Always `true` for successful responses.
  pub success: bool,
  /// Human-readable status message (e.g. `"OK"`, `"REGISTERED"`).
  pub message: String,
  /// Response payload; omitted from JSON when `None`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

use serde::Serialize;

/// Builder for HTTP responses. Converts into an Axum `Response` via `IntoResponse`.
#[derive(Debug, Clone)]
pub struct HttpResponse<T = serde_json::Value>
where
  T: Serialize,
{
  /// Status message forwarded into [`HttpResponseFormat`].
  pub message: String,
  /// HTTP status code for the response.
  pub status: StatusCode,
  /// Optional response payload.
  pub data: Option<T>,
}

impl<T: Serialize> HttpResponse<T> {
  pub fn new(
    message: impl Into<String>,
    status: StatusCode,
    data: Option<T>,
  ) -> Self {
    HttpResponse {
      message: message.into(),
      status,
      data,
    }
  }

  pub fn ok(
    data: T,
    msg: &str,
  ) -> Self {
    HttpResponse {
      status: StatusCode::OK,
      message: msg.to_string(),
      data: Some(data),
    }
  }

  pub fn created(
    data: T,
    msg: &str,
  ) -> Self {
    HttpResponse {
      status: StatusCode::CREATED,
      message: msg.to_string(),
      data: Some(data),
    }
  }

  pub fn delete(id: String) -> Self {
    HttpResponse {
      status: StatusCode::OK,
      message: format!("DELETED:{id}"),
      data: None,
    }
  }

  pub fn into_http_response(self) -> Response {
    let format = HttpResponseFormat {
      success: self.status.is_success(),
      message: self.message,
      data: self.data,
    };
    (self.status, Json(format)).into_response()
  }
}

impl<T: Serialize> fmt::Display for HttpResponse<T> {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    write!(
      f,
      "HttpResponse: message: {}, status: {}",
      self.message, self.status
    )
  }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
  fn into_response(self) -> Response {
    self.into_http_response()
  }
}
