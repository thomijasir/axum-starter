use axum_starter::{
  models::{AppEnv, AppState, Environment},
  modules::AppRoutes,
  services::DBSqlite,
};
use diesel::RunQueryDsl;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::net::TcpListener;

/// A running test server bound to an ephemeral port.
pub struct TestApp {
  pub address: String,
  pub client: reqwest::Client,
  /// Keep the tempfile alive for the lifetime of TestApp (drops and deletes on test end)
  _db_file: NamedTempFile,
}

impl TestApp {
  /// Spin up a real server on a random port backed by a fresh SQLite temp file.
  pub async fn spawn() -> Self {
    // Create a temporary SQLite file that is deleted when the test ends
    let db_file = NamedTempFile::new().expect("failed to create temp DB file");
    let db_path = db_file.path().to_str().unwrap().to_string();

    // Run migrations on the temp DB
    let db = DBSqlite::new(&db_path).expect("failed to create test DB pool");
    Self::run_migrations(&db).await;

    let env = Environment {
      mode: AppEnv::Local,
      secret: "test-secret-key-for-integration-tests".to_string(),
      port: 0, // not used — we bind via TcpListener directly
      database_url: db_path,
      timeout: 300,
      cors_origins: vec!["http://localhost:3000".to_string()],
      log_dir: "/tmp".to_string(),
    };

    let app_state = Arc::new(AppState { env, db });

    let router = AppRoutes::build(app_state.clone());

    let listener = TcpListener::bind("127.0.0.1:0")
      .await
      .expect("failed to bind test listener");
    let addr = listener.local_addr().expect("failed to get local address");

    tokio::spawn(async move {
      axum::serve(listener, router)
        .await
        .expect("test server failed");
    });

    TestApp {
      address: format!("http://127.0.0.1:{}", addr.port()),
      client: reqwest::Client::new(),
      _db_file: db_file,
    }
  }

  /// Apply Diesel migrations to the test database.
  async fn run_migrations(db: &DBSqlite) {
    db.transaction(|conn| {
      // Create users table
      diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users (
          id TEXT PRIMARY KEY NOT NULL,
          email TEXT NOT NULL UNIQUE,
          username TEXT NOT NULL UNIQUE,
          password TEXT NOT NULL,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL
        )",
      )
      .execute(conn)?;

      // Create refresh_tokens table
      diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS refresh_tokens (
          id TEXT PRIMARY KEY NOT NULL,
          user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
          token TEXT NOT NULL UNIQUE,
          expires_at TEXT NOT NULL,
          created_at TEXT NOT NULL
        )",
      )
      .execute(conn)?;

      Ok(())
    })
    .await
    .expect("failed to run test migrations");
  }

  /// Helper: POST /auth/register and return the response body as JSON
  pub async fn register(
    &self,
    email: &str,
    username: &str,
    password: &str,
  ) -> reqwest::Response {
    self
      .client
      .post(format!("{}/auth/register", self.address))
      .json(&serde_json::json!({
        "email": email,
        "username": username,
        "password": password,
      }))
      .send()
      .await
      .expect("request failed")
  }

  /// Helper: POST /auth/login and return the response
  pub async fn login(
    &self,
    email: &str,
    password: &str,
  ) -> reqwest::Response {
    self
      .client
      .post(format!("{}/auth/login", self.address))
      .json(&serde_json::json!({
        "email": email,
        "password": password,
      }))
      .send()
      .await
      .expect("request failed")
  }
}
