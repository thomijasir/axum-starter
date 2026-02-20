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

    let timeout = var("TIMEOUT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u64>()
        .expect("TIMEOUT must be a valid number");

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    Environment {
        mode,
        secret,
        port,
        database_url,
        timeout,
    }
}
