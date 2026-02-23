use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
  pub page: u32,
  pub per_page: u32,
  pub total_items: u32,
  pub total_pages: u32,
  pub items: Vec<T>,
}

impl<T> PaginatedResponse<T> {
  pub fn new(
    items: Vec<T>,
    page: u32,
    per_page: u32,
    total_items: u32,
  ) -> Self {
    let total_pages = if per_page == 0 {
      1
    } else {
      (total_items + per_page - 1) / per_page
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
