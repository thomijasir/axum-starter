use crate::constants::error::*;
use crate::models::{AppEnv, Environment};
use std::env::var;
use tracing_appender::non_blocking::WorkerGuard;

pub fn load_environment() -> Environment {
  let mode = var("APP_ENV")
    .unwrap_or_else(|_| "local".to_string())
    .parse::<AppEnv>()
    .unwrap();

  let secret = var("SECRET").expect(ERR047);

  let port = var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect(ERR048);

  // Default 300 seconds (5 minutes) — was incorrectly 3000
  let timeout = var("TIMEOUT")
    .unwrap_or_else(|_| "300".to_string())
    .parse::<u64>()
    .expect(ERR049);

  let database_url = var("DATABASE_URL").expect(ERR050);

  let cors_origins = var("CORS_ORIGINS")
    .unwrap_or_else(|_| "http://localhost:5000,http://localhost:8080".to_string())
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect::<Vec<String>>();

  let log_dir = var("LOG_DIR").unwrap_or_else(|_| "data/logs".to_string());

  Environment {
    mode,
    secret,
    port,
    database_url,
    timeout,
    cors_origins,
    log_dir,
  }
}

/// Ensures required runtime directories exist, creating them if necessary.
pub fn ensure_directories(env: &Environment) {
  let dirs = [env.log_dir.as_str(), "data", "public", "public/uploads"];

  for dir in dirs {
    if !std::path::Path::new(dir).exists() {
      std::fs::create_dir_all(dir)
        .unwrap_or_else(|e| panic!("{} '{dir}': {e}", ERR051));
      tracing::info!(dir, "DIRECTORY_CREATED");
    }
  }
}

/// Initialise tracing/logging for the given environment.
///
/// Returns an `Option<WorkerGuard>` that **must be kept alive** for the
/// entire process lifetime (assign it to a variable in `main`). Dropping it
/// early flushes and stops the non-blocking file writer, causing log loss.
pub fn init_logging(env: &Environment) -> Option<WorkerGuard> {
  use tracing_subscriber::prelude::*;

  match env.mode {
    AppEnv::Production => {
      let file_appender = tracing_appender::rolling::daily(&env.log_dir, "app.log");
      let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

      let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

      let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_writer(non_blocking_file);

      let stdout_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_writer(std::io::stdout);

      tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(stdout_layer)
        .init();

      Some(guard)
    }
    _ => {
      tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
          tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

      None
    }
  }
}
