use std::sync::Arc;

use axum_starter::{
  config,
  models::{AppEnv, AppState},
  server::AppServer,
  services::DBSqlite,
  utils::Cache,
};

// TODO
// [Partial Done] camelcase response
// [Partial Done] pagination
// [Not started] implement custom extractor handle for multipart
// [] Documentation still not perfect
// [] Docker not tested yet
#[tokio::main]
async fn main() {
  // Load Environment (also loads .env file)
  let env = config::load_environment();

  // Initialize structured logging based on environment
  match env.mode {
    AppEnv::Production => {
      tracing_subscriber::fmt()
        .json()
        .with_env_filter(
          tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    }
    _ => {
      tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
          tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();
    }
  }
  // Log Server information
  tracing::info!(mode = %env.mode, port = env.port, "SERVER_START");

  // Create in-memory cache
  let cache = Cache::default();
  // Create DB connection pool
  let db = DBSqlite::new(&env.database_url).expect("DATABASE_POOL_FAILURE");
  // Create App State
  let app_state = Arc::new(AppState { env, cache, db });

  AppServer::serve(app_state)
    .await
    .expect("SERVER_FAIL_START");
}
