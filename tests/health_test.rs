mod common;

use common::TestApp;

#[tokio::test]
async fn liveness_returns_200() {
  let app = TestApp::spawn().await;
  let resp = app
    .client
    .get(format!("{}/health/live", app.address))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 200);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert_eq!(body["success"], true);
}

#[tokio::test]
async fn readiness_returns_200_with_sqlite() {
  let app = TestApp::spawn().await;
  let resp = app
    .client
    .get(format!("{}/health/ready", app.address))
    .send()
    .await
    .expect("request failed");

  assert_eq!(resp.status(), 200);
  let body: serde_json::Value = resp.json().await.unwrap();
  assert_eq!(body["success"], true);
}
