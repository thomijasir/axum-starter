use axum_starter::{config, models::AppState, server::AppServer, services::DBSqlite};
use std::sync::Arc;

#[tokio::main]
async fn main() {
  let env = config::load_environment();
  config::init_logging(&env);
  config::ensure_directories(&env);
  // Create DB connection pool
  let db = DBSqlite::new(&env.database_url).expect("DATABASE_POOL_FAILURE");
  // Run pending migrations
  db.run_migrations().expect("DATABASE_MIGRATION_FAILURE");
  // Log Start
  tracing::info!(mode = %env.mode, port = env.port, "SERVER_STARTED");
  // Create App State
  let app_state = Arc::new(AppState { env, db });

  AppServer::serve(app_state)
    .await
    .expect("SERVER_FAIL_TO_START");
}
