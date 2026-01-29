use crate::utils::errors::HttpError;
use axum::{
    async_trait,
    body::Body, // Use axum::body::Body directly
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use validator::{Validate, ValidationErrors}; // Import the Validate trait and ValidationErrors

// Define the extractor struct
pub struct BodyJson<T>(pub T);

// Implement Deref for easy access to the inner value
impl<T> Deref for BodyJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Helper function to format validation errors
fn format_validation_errors(errors: &ValidationErrors) -> String {
    // if validation error are empty, return errors
    let map_error = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let formatted: Vec<String> = errors
                .iter()
                .map(|e| {
                    let mut parts: Vec<String> = Vec::new();
                    // Always include validation code
                    parts.push(e.code.to_string());

                    // Optionally include custom message
                    if let Some(msg) = &e.message {
                        parts.push(msg.to_string());
                    }

                    // Optionally include provided value param (commonly exists)
                    if let Some(v) = e.params.get("value") {
                        parts.push(format!("value={v}"));
                    }

                    // Include any other params except "value" to be thorough
                    for (k, v) in e.params.iter().filter(|(k, _)| **k != "value") {
                        parts.push(format!("{k}={v}"));
                    }

                    parts.join(" | ")
                })
                .collect();

            if formatted.is_empty() {
                format!("{field} | Invalid value")
            } else {
                format!("{field} | {}", formatted.join(", "))
            }
        })
        .collect::<Vec<String>>()
        .join(";"); // Join field errors with a semicolon and space

    if map_error.is_empty() {
        errors.to_string()
    } else {
        map_error
    }
}

#[async_trait]
impl<S, T> FromRequest<S, Body> for BodyJson<T>
// Specify Body explicitly
where
    S: Send + Sync, // State must be Send + Sync
    // The target type T must be Deserializable, Validatable, and Sendable
    T: DeserializeOwned + Validate + Send,
{
    // Our rejection type is HttpError
    type Rejection = HttpError;

    async fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 1. Attempt to deserialize the JSON body using Axum's Json extractor
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            // Map Axum's JsonRejection to our HttpError::bad_request
            .map_err(|e| HttpError::bad_request(format!("INVALID_BODY_REQUEST:{}", e)))?;

        // 2. Attempt to validate the deserialized value using the validator crate
        value
            .validate()
            // Format the validation errors using our helper function
            .map_err(|e| {
                HttpError::bad_request(format!(
                    "INVALID_VALIDATION | {}",
                    format_validation_errors(&e)
                ))
            })?;

        // 3. If both deserialization and validation succeed, return the wrapped value
        Ok(BodyJson(value))
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use serde::Deserialize;
    use tower::ServiceExt; // For `app.oneshot()`
    use validator::Validate;

    // Example payload struct with validation rules
    #[derive(Deserialize, Validate, Debug)]
    struct TestPayload {
        #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
        username: String,
        #[validate(range(min = 18, message = "Must be 18 or older"))]
        age: u32,
    }

    // Simple handler that uses our extractor
    async fn test_handler(BodyJson(payload): BodyJson<TestPayload>) -> StatusCode {
        println!("Received valid payload: {:?}", payload); // Optional: for debugging tests
        StatusCode::OK
    }

    // Helper to create the test router
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
        // Username is too short
        let request_body = r#"{"username": "a", "age": 30}"#;
        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(request_body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Expecting Bad Request due to validation error
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn invalid_json_format() {
        let app = test_app();
        // Malformed JSON (missing closing brace)
        let request_body = r#"{"username": "testuser", "age": 25"#;
        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(request_body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Expecting Bad Request due to deserialization error
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn invalid_json_missing_field() {
        let app = test_app();
        // Missing the 'age' field
        let request_body = r#"{"username": "testuser"}"#;
        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(request_body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Expecting Bad Request due to deserialization error (missing field)
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
