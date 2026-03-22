use crate::services::HttpResponseFormat;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// JSON error body returned in all error responses.
///
/// Used as the OpenAPI schema for error responses via [`utoipa::ToSchema`].
/// The `data` field is omitted — use [`HttpResponseFormat`] for success payloads.
///
/// # Example JSON
/// ```json
/// { "success": false, "message": "ERR013-INVALID_CREDENTIALS" }
/// ```
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HttpErrorFormat {
  pub success: bool,
  pub message: String,
}

/// Typed HTTP error enum covering all application error codes.
///
/// Each variant maps to a fixed error code string (via [`thiserror::Error`]) and a
/// specific [`StatusCode`] returned by [`HttpError::status`]. The error code string
/// is used directly as the response `message` field.
///
/// # Naming convention
/// Variants are named after their error code (`ERR010`, `ERR013`, …) so the call
/// site makes the mapping explicit without needing a separate lookup table.
/// Tuple variants carry dynamic context (sizes, allowed types, upstream error text).
///
/// # Fallback
/// Any untyped [`anyhow::Error`] or plain [`String`] falls into [`HttpError::ERR500`]
/// via the [`From`] impls, which logs the original error and responds with
/// `500 Internal Server Error`.
///
/// # Usage
/// ```rust,ignore
/// // typed unit variant
/// return Err(HttpError::ERR013);
///
/// // typed tuple variant with context
/// return Err(HttpError::ERR033(e.to_string()));
///
/// // anyhow conversion via ?
/// let user = repo::find(db, id).await.map_err(HttpError::from)?;
/// ```
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
  // ── Auth ─────────────────────────────────────────────────────────────────
  /// `409 Conflict` — a user with this e-mail already exists.
  #[error("ERR010|EMAIL_ALREADY_EXISTS")]
  ERR010,

  /// `500 Internal Server Error` — bcrypt hashing failed.
  #[error("ERR011|PASSWORD_HASH_FAILED")]
  ERR011,

  /// `401 Unauthorized` — credentials not found or password mismatch.
  #[error("ERR013|INVALID_CREDENTIALS")]
  ERR013,

  /// `401 Unauthorized` — the supplied refresh token does not exist.
  #[error("ERR014|INVALID_REFRESH_TOKEN")]
  ERR014,

  /// `401 Unauthorized` — the stored `expires_at` timestamp could not be parsed.
  #[error("ERR015|INVALID_TOKEN_EXPIRY_FORMAT")]
  ERR015,

  /// `401 Unauthorized` — the refresh token exists but has passed its expiry date.
  #[error("ERR016|REFRESH_TOKEN_EXPIRED")]
  ERR016,

  /// `500 Internal Server Error` — JWT signing / creation failed.
  #[error("ERR017|TOKEN_CREATE_FAILED")]
  ERR017,

  // ── Token / JWT ──────────────────────────────────────────────────────────
  /// `401 Unauthorized` — JWT token is malformed or invalid.
  #[error("ERR018|INVALID_TOKEN")]
  ERR018,

  /// `401 Unauthorized` — JWT signature has expired.
  #[error("ERR019|EXPIRED_SIGNATURE")]
  ERR019,

  /// `401 Unauthorized` — JWT signature does not match.
  #[error("ERR020|INVALID_SIGNATURE")]
  ERR020,

  /// `401 Unauthorized` — generic unauthorized access.
  #[error("ERR021|UNAUTHORIZED")]
  ERR021,

  /// `401 Unauthorized` — missing or malformed `Authorization: Bearer` header.
  #[error("ERR022|MISSING_OR_INVALID_AUTHORIZATION_HEADER")]
  ERR022,

  // ── Attachment / file ────────────────────────────────────────────────────
  /// `404 Not Found` — the requested attachment does not exist.
  #[error("ERR023|ATTACHMENT_NOT_FOUND")]
  ERR023,

  /// `400 Bad Request` — no file was provided in the multipart form.
  #[error("ERR024|NO_FILE_PROVIDED")]
  ERR024,

  /// `400 Bad Request` — the provided file is empty (zero bytes).
  #[error("ERR025|EMPTY_FILE")]
  ERR025,

  /// `400 Bad Request` — file MIME type is not in the allowed list.
  /// The inner string carries context such as `"allowed=image/jpeg, image/png"`.
  #[error("ERR026|INVALID_FILE_TYPE:{0}")]
  ERR026(String),

  /// `400 Bad Request` — filename is missing or cannot be sanitized.
  #[error("ERR027|INVALID_FILENAME")]
  ERR027,

  /// `409 Conflict` — a file with this name already exists on disk.
  #[error("ERR029|FILE_ALREADY_EXISTS")]
  ERR029,

  /// `500 Internal Server Error` — saving the file to disk failed.
  #[error("ERR030|FILE_UPLOAD_FAILED")]
  ERR030,

  // ── Extractors / validation ───────────────────────────────────────────────
  /// `400 Bad Request` — uploaded file exceeds the configured size limit.
  #[error("ERR031|FILE_TOO_LARGE:{0}")]
  ERR031(String),

  /// `400 Bad Request` — path parameter could not be deserialized.
  #[error("ERR032|INVALID_PATH_PARAM:{0}")]
  ERR032(String),

  /// `400 Bad Request` — JSON request body is malformed.
  #[error("ERR033|INVALID_BODY_REQUEST:{0}")]
  ERR033(String),

  /// `400 Bad Request` — request body failed validation rules.
  #[error("ERR034|INVALID_VALIDATION:{0}")]
  ERR034(String),

  /// `400 Bad Request` — multipart form data could not be parsed.
  #[error("ERR035|INVALID_MULTIPART_DATA:{0}")]
  ERR035(String),

  /// `400 Bad Request` — a multipart field was invalid.
  #[error("ERR036|INVALID_MULTIPART_FIELD:{0}")]
  ERR036(String),

  /// `400 Bad Request` — failed to read bytes from a file part.
  #[error("ERR037|FAILED_TO_READ_FILE:{0}")]
  ERR037(String),

  /// `400 Bad Request` — failed to read text from a form field.
  #[error("ERR038|FAILED_TO_READ_FIELD:{0}")]
  ERR038(String),

  /// `400 Bad Request` — more files were uploaded than the configured maximum.
  #[error("ERR039|TOO_MANY_FILES:{0}")]
  ERR039(String),

  /// `400 Bad Request` — text field could not be serialized to JSON.
  #[error("ERR040|INVALID_FIELD_SERIALIZATION:{0}")]
  ERR040(String),

  /// `400 Bad Request` — text field value has an unexpected format.
  #[error("ERR400|INVALID_FIELD_FORMAT:{0}")]
  ERR400(String),

  // ── Server ────────────────────────────────────────────────────────────────
  /// `408 Request Timeout` — the request exceeded the server timeout.
  #[error("ERR408|REQUEST_TIMED_OUT")]
  ERR408,

  /// `500 Internal Server Error` — an unexpected error occurred.
  #[error("ERR043|UNEXPECTED_ERROR_OCCURRED")]
  ERR043,

  /// `404 Not Found` — no route matched the request path.
  #[error("ERR404|RESOURCE_NOT_FOUND")]
  ERR404,

  // ── Health ────────────────────────────────────────────────────────────────
  /// `503 Service Unavailable` — DB unreachable (readiness probe only).
  #[error("SERVICE_UNAVAILABLE")]
  ERR503,

  // ── Fallback ──────────────────────────────────────────────────────────────
  /// `500 Internal Server Error` — catch-all for unhandled errors.
  ///
  /// Wraps the original [`anyhow::Error`] so the full error chain is preserved
  /// internally, while the HTTP response only exposes the top-level message.
  #[error("ERR500|SOMETHING_WENT_WRONG:{0}")]
  ERR500(anyhow::Error),
}

impl HttpError {
  /// Returns the [`StatusCode`] that corresponds to this error variant.
  pub fn status(&self) -> StatusCode {
    match self {
      Self::ERR018
      | Self::ERR019
      | Self::ERR020
      | Self::ERR021
      | Self::ERR022
      | Self::ERR013
      | Self::ERR014
      | Self::ERR015
      | Self::ERR016 => StatusCode::UNAUTHORIZED,
      Self::ERR023 | Self::ERR404 => StatusCode::NOT_FOUND,
      Self::ERR024
      | Self::ERR025
      | Self::ERR026(_)
      | Self::ERR027
      | Self::ERR031(_)
      | Self::ERR032(_)
      | Self::ERR033(_)
      | Self::ERR034(_)
      | Self::ERR035(_)
      | Self::ERR036(_)
      | Self::ERR037(_)
      | Self::ERR038(_)
      | Self::ERR039(_)
      | Self::ERR040(_)
      | Self::ERR400(_) => StatusCode::BAD_REQUEST,
      Self::ERR029 | Self::ERR010 => StatusCode::CONFLICT,
      Self::ERR408 => StatusCode::REQUEST_TIMEOUT,
      Self::ERR503 => StatusCode::SERVICE_UNAVAILABLE,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

/// Converts an [`anyhow::Error`] into [`HttpError::ERR500`].
///
/// Logs the original error at `ERROR` level before wrapping so the full
/// context is visible in server logs even though the HTTP response is generic.
impl From<anyhow::Error> for HttpError {
  fn from(e: anyhow::Error) -> Self {
    tracing::error!(error = %e, "unhandled anyhow error");
    Self::ERR500(e)
  }
}

/// Converts a plain [`String`] message into [`HttpError::ERR500`].
impl From<String> for HttpError {
  fn from(s: String) -> Self {
    Self::ERR500(anyhow::anyhow!(s))
  }
}

/// Serialises this error into an Axum [`Response`].
///
/// The response body is a [`HttpResponseFormat`] JSON object with `success: false`
/// and `message` set to the variant's `#[error("…")]` string.
impl IntoResponse for HttpError {
  fn into_response(self) -> Response {
    let body = HttpResponseFormat {
      success: false,
      message: self.to_string(),
      data: None::<serde_json::Value>,
    };
    (self.status(), Json(body)).into_response()
  }
}

pub type Result<T> = std::result::Result<T, HttpError>;
