use serde::Deserialize;

/// Query parameters for paginated list endpoints.
///
/// Usage: `Query<PaginationQuery>` in handler parameters.
/// Defaults to page 1, limit 20; limit is capped at 100.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
  #[serde(default = "default_page")]
  pub page: u32,
  #[serde(default = "default_limit")]
  pub limit: u32,
}

fn default_page() -> u32 {
  1
}

fn default_limit() -> u32 {
  20
}

impl PaginationQuery {
  /// Returns the offset for SQL queries: `(page - 1) * limit`.
  pub fn offset(&self) -> i64 {
    let page = self.page.max(1);
    let limit = self.effective_limit();
    ((page - 1) * limit) as i64
  }

  /// Returns the effective limit, capped at 100.
  pub fn effective_limit(&self) -> u32 {
    self.limit.clamp(1, 100)
  }
}
