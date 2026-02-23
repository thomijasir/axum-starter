use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ResponsesMessage {
    OK,
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct HttpResponseFormat<T = serde_json::Value>
where
    T: serde::Serialize,
{
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct HttpResponse<T = serde_json::Value>
where
    T: Serialize,
{
    pub message: String,
    pub status: StatusCode,
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

pub fn pagination_response_formatted<T: Serialize>(
    data: Vec<T>,
    page: i64,
    limit: i64,
    total: i64,
) -> serde_json::Value {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    json!({
        "items": data,
        "pagination": {
            "total": total,
            "page": page,
            "limit": limit,
            "total_pages": total_pages
        }
    })
}
