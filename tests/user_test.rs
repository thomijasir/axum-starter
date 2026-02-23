mod common;

use common::TestApp;

const EMAIL: &str = "usertest@example.com";
const USERNAME: &str = "usertest";
const PASSWORD: &str = "password123";

#[tokio::test]
async fn get_me_without_token_returns_401() {
  let app = TestApp::spawn().await;

  let resp = app
    .client
    .get(format!("{}/users/me", app.address))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn get_me_with_invalid_token_returns_401() {
  let app = TestApp::spawn().await;

  let resp = app
    .client
    .get(format!("{}/users/me", app.address))
    .header("Authorization", "Bearer not.a.valid.jwt")
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn get_me_with_valid_token_returns_200() {
  let app = TestApp::spawn().await;

  // Register and extract the access token
  let reg_resp = app.register(EMAIL, USERNAME, PASSWORD).await;
  assert_eq!(reg_resp.status(), 201);
  let reg_body: serde_json::Value = reg_resp.json().await.unwrap();
  let access_token = reg_body["data"]["accessToken"].as_str().unwrap();

  // Call /users/me with the token
  let resp = app
    .client
    .get(format!("{}/users/me", app.address))
    .header("Authorization", format!("Bearer {}", access_token))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 200);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert_eq!(body["success"], true);
  assert_eq!(body["data"]["email"], EMAIL);
  assert_eq!(body["data"]["username"], USERNAME);
  // Password must not be in the response
  assert!(body["data"]["password"].is_null());
}
