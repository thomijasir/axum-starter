use std::fmt;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError{
            message: message.into(),
            status
        }
    }
    pub fn server_error (message: impl Into<String>) -> Self {
        HttpError{
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
    pub fn unique_constraint_violation (message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT
        }
    }
    pub fn unauthorized (message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED
        }
    }
    pub fn timeout (message: impl Into<String>) -> Self {
        HttpError{
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

    pub fn into_http_response(self) -> Response {
        let body = json!({
            "success": false,
            "message": self.message.clone()
        });
        (self.status, Json(body)).into_response()
    }
}
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
