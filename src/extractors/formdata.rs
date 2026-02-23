/// Re-export of `axum_typed_multipart::TypedMultipart` for file/form upload handlers.
///
/// Requires axum's `multipart` feature. Example (not compiled in tests):
///
/// ```rust,ignore
/// use axum::response::IntoResponse;
/// use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
///
/// #[derive(TryFromMultipart)]
/// struct UploadForm {
///     #[form_data(field_name = "file")]
///     file: axum_typed_multipart::FieldData<axum::body::Bytes>,
/// }
///
/// async fn upload(TypedMultipart(form): TypedMultipart<UploadForm>) -> impl IntoResponse {
///     // handle the upload
/// }
/// ```
pub use axum_typed_multipart::TypedMultipart;
