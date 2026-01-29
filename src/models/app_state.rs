#[derive(Debug, Clone)]
pub struct AppState {
    pub env: config::Config,
    pub cache: utils::cache::Cache,
}
