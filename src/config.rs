use crate::models::{AppEnv, Environment};

pub fn load_environment() -> Environment {
    let mode = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".to_string())
        .parse::<AppEnv>()
        .expect("Invalid APP_ENV value");

    let secret = std::env::var("SECRET").expect("SECRET must be set");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Environment {
        mode,
        secret,
        port,
        database_url,
    }
}
