use crate::constants::error::*;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone)]
pub struct HttpError {
  pub message: String,
  pub status: StatusCode,
}

impl HttpError {
  pub fn new(
    message: impl Into<String>,
    status: StatusCode,
  ) -> Self {
    HttpError {
      message: message.into(),
      status,
    }
  }
  pub fn server_error(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
  pub fn bad_request(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::BAD_REQUEST,
    }
  }
  pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::CONFLICT,
    }
  }
  pub fn unauthorized(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::UNAUTHORIZED,
    }
  }
  pub fn timeout(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::REQUEST_TIMEOUT,
    }
  }
  pub fn not_found(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::NOT_FOUND,
    }
  }
  pub fn forbidden(message: impl Into<String>) -> Self {
    HttpError {
      message: message.into(),
      status: StatusCode::FORBIDDEN,
    }
  }

  pub fn from_service_error(e: anyhow::Error) -> Self {
    let msg = e.to_string();
    let s = msg.as_str();
    match s {
      ERR004 => Self::not_found(ERR004),
      ERR005 | ERR010 => Self::unique_constraint_violation(s),
      ERR013 | ERR014 | ERR016 => Self::unauthorized(s),
      ERR017 | ERR011 => Self::server_error(s),
      ERR023 => Self::not_found(ERR023),
      _ => {
        tracing::error!(error = %e, "unhandled service error");
        Self::server_error(ERR046)
      }
    }
  }

  pub fn into_http_response(self) -> Response {
    let body = json!({
        "success": false,
        "message": self.message.clone()
    });
    (self.status, Json(body)).into_response()
  }
}
impl fmt::Display for HttpError {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    write!(
      f,
      "HttpError: message: {}, status: {}",
      self.message, self.status
    )
  }
}
impl std::error::Error for HttpError {}
impl IntoResponse for HttpError {
  fn into_response(self) -> Response {
    self.into_http_response()
  }
}
