use crate::services::DBSqlite;

/// Deployment environment the application is running in.
#[derive(Clone, Debug)]
pub enum AppEnv {
  /// Local development machine.
  Local,
  /// Staging / pre-production environment.
  Staging,
  /// Live production environment.
  Production,
}

impl std::fmt::Display for AppEnv {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      AppEnv::Local => write!(f, "local"),
      AppEnv::Staging => write!(f, "staging"),
      AppEnv::Production => write!(f, "production"),
    }
  }
}

impl std::str::FromStr for AppEnv {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "local" => Ok(AppEnv::Local),
      "staging" | "stag" => Ok(AppEnv::Staging),
      "production" | "prod" => Ok(AppEnv::Production),
      _ => Err(format!("INVALID_ENVIRONMENT {}", s)),
    }
  }
}

/// Runtime configuration loaded from environment variables at startup.
#[derive(Debug, Clone)]
pub struct Environment {
  /// Active deployment environment (local / staging / production).
  pub mode: AppEnv,
  /// JWT signing secret.
  pub secret: String,
  /// TCP port the HTTP server listens on.
  pub port: u16,
  /// Database connection URL.
  pub database_url: String,
  /// Request timeout in seconds.
  pub timeout: u64,
  /// Allowed CORS origins.
  pub cors_origins: Vec<String>,
  /// Directory where log files are written.
  pub log_dir: String,
}

/// Shared application state injected into every handler via Axum's `State` extractor.
#[derive(Debug, Clone)]
pub struct AppState {
  /// Resolved runtime configuration.
  pub env: Environment,
  /// SQLite database connection pool.
  pub db: DBSqlite,
}
