use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T: ToSchema> {
  pub page: u32,
  pub per_page: u32,
  pub total_items: u32,
  pub total_pages: u32,
  pub items: Vec<T>,
}

impl<T: ToSchema> PaginatedResponse<T> {
  pub fn new(
    items: Vec<T>,
    page: u32,
    per_page: u32,
    total_items: u32,
  ) -> Self {
    let total_pages = if per_page == 0 {
      1
    } else {
      total_items.div_ceil(per_page)
    };
    Self {
      page,
      per_page,
      total_items,
      total_pages,
      items,
    }
  }
}

/// Query parameters for paginated list endpoints.
///
/// Usage: `Query<PaginationQuery>` in handler parameters.
/// Defaults to page 1, limit 20; limit is capped at 100.
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PaginationQuery {
  #[param(default = 1)]
  pub page: u32,
  #[param(default = 10, maximum = 100)]
  pub limit: u32,
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
