use std::sync::Arc;

use axum_starter::{
  config,
  models::AppState,
  server::AppServer,
  services::DBSqlite,
};

#[tokio::main]
async fn main() {
  let env = config::load_environment();
  let _log_guard = config::init_logging(&env);

  tracing::info!(mode = %env.mode, port = env.port, "SERVER_START");

  // Create DB connection pool
  let db = DBSqlite::new(&env.database_url).expect("DATABASE_POOL_FAILURE");
  // Create App State
  let app_state = Arc::new(AppState { env, db });

  AppServer::serve(app_state)
    .await
    .expect("SERVER_FAIL_START");
}
