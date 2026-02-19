use axum_starter::{config, models::AppState, services::DBSqlite, utils::Cache};

#[tokio::main]
async fn main() {
    // Load Environment
    let env = config::load_environment();
    // Create in memory cache
    let cache = Cache::default();
    // Create DB
    let db = DBSqlite::new(&env.database_url).expect("Failed to create database pool");
    // Create App State
    let app_state = AppState { env, cache, db };
    println!("RUN");
}
