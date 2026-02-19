use crate::{services::DBSqlite, utils};

#[derive(Clone, Debug)]
pub enum AppEnv {
    Local,
    Development,
    Production,
}

impl std::fmt::Display for AppEnv {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AppEnv::Local => write!(f, "local"),
            AppEnv::Development => write!(f, "development"),
            AppEnv::Production => write!(f, "production"),
        }
    }
}

impl std::str::FromStr for AppEnv {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(AppEnv::Local),
            "development" | "dev" => Ok(AppEnv::Development),
            "production" | "prod" => Ok(AppEnv::Production),
            _ => Err(format!("Unknown environment: {}", s)),
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
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Environment,
    pub cache: utils::Cache,
    pub db: DBSqlite,
}
