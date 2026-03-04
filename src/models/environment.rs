use crate::{constants::error::*, services::DBSqlite};

#[derive(Clone, Debug)]
pub enum AppEnv {
  Local,
  Staging,
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
      _ => Err(format!("{} {}", ERR045, s)),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Environment {
  pub mode: AppEnv,
  pub secret: String,
  pub port: u16,
  pub database_url: String,
  pub timeout: u64,
  pub cors_origins: Vec<String>,
  pub log_dir: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
  pub env: Environment,
  pub db: DBSqlite,
}
