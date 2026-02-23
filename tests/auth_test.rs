mod common;

use common::TestApp;

const EMAIL: &str = "test@example.com";
const USERNAME: &str = "testuser";
const PASSWORD: &str = "password123";

#[tokio::test]
async fn register_new_user_returns_201() {
  let app = TestApp::spawn().await;
  let resp = app.register(EMAIL, USERNAME, PASSWORD).await;

  assert_eq!(resp.status(), 201);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert_eq!(body["success"], true);
  assert!(body["data"]["access_token"].is_string());
  assert!(body["data"]["refresh_token"].is_string());
}

#[tokio::test]
async fn register_duplicate_email_returns_409() {
  let app = TestApp::spawn().await;

  let first = app.register(EMAIL, USERNAME, PASSWORD).await;
  assert_eq!(first.status(), 201);

  // Second registration with the same email
  let second = app.register(EMAIL, "otheruser", PASSWORD).await;
  assert_eq!(second.status(), 409);
}

#[tokio::test]
async fn login_with_correct_credentials_returns_200() {
  let app = TestApp::spawn().await;

  app.register(EMAIL, USERNAME, PASSWORD).await;

  let resp = app.login(EMAIL, PASSWORD).await;
  assert_eq!(resp.status(), 200);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert!(body["data"]["access_token"].is_string());
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
  let app = TestApp::spawn().await;

  app.register(EMAIL, USERNAME, PASSWORD).await;

  let resp = app.login(EMAIL, "wrongpassword").await;
  assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn login_unknown_email_returns_401() {
  let app = TestApp::spawn().await;

  let resp = app.login("nobody@example.com", PASSWORD).await;
  assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn refresh_with_valid_token_returns_200() {
  let app = TestApp::spawn().await;

  let reg_resp = app.register(EMAIL, USERNAME, PASSWORD).await;
  let reg_body: serde_json::Value = reg_resp.json().await.unwrap();
  let refresh_token = reg_body["data"]["refresh_token"].as_str().unwrap();

  let resp = app
    .client
    .post(format!("{}/auth/refresh", app.address))
    .json(&serde_json::json!({ "refresh_token": refresh_token }))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 200);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert!(body["data"]["access_token"].is_string());
  // New refresh token should differ from the original
  let new_rt = body["data"]["refresh_token"].as_str().unwrap();
  assert_ne!(new_rt, refresh_token);
}

#[tokio::test]
async fn refresh_with_invalid_token_returns_401() {
  let app = TestApp::spawn().await;

  let resp = app
    .client
    .post(format!("{}/auth/refresh", app.address))
    .json(&serde_json::json!({ "refresh_token": "completely-fake-token" }))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn register_short_password_returns_400() {
  let app = TestApp::spawn().await;

  let resp = app.register(EMAIL, USERNAME, "short").await;
  assert_eq!(resp.status(), 400);
}
