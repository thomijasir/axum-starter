use crate::models::{AppEnv, Environment};
use std::env::var;

pub fn load_environment() -> Environment {
  let mode = var("APP_ENV")
    .unwrap_or_else(|_| "local".to_string())
    .parse::<AppEnv>()
    .expect("Invalid APP_ENV value");

  let secret = var("SECRET").expect("SECRET must be set");

  let port = var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid number");

  // Default 300 seconds (5 minutes) — was incorrectly 3000
  let timeout = var("TIMEOUT")
    .unwrap_or_else(|_| "300".to_string())
    .parse::<u64>()
    .expect("TIMEOUT must be a valid number");

  let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

  let cors_origins = var("CORS_ORIGINS")
    .unwrap_or_else(|_| "http://localhost:5000,http://localhost:8080".to_string())
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect::<Vec<String>>();

  Environment {
    mode,
    secret,
    port,
    database_url,
    timeout,
    cors_origins,
  }
}
