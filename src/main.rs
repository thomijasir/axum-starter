use std::sync::Arc;

use axum_starter::{config, models::AppState, server::AppServer, services::DBSqlite, utils::Cache};

#[tokio::main]
async fn main() {
    // Load Environment
    let env = config::load_environment();
    // Create in memory cache
    let cache = Cache::default();
    // Create DB
    let db = DBSqlite::new(&env.database_url).expect("Failed to create database pool");
    // Create App State
    println!("RUN {:?}", env);
    let app_state = Arc::new(AppState { env, cache, db });

    AppServer::serve(app_state)
        .await
        .expect("SERVER_FAIL_START");
}
