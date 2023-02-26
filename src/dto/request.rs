use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct PaginationDto {
    #[validate(range(min = 1, max = 100))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
}

impl PaginationDto {
    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(10)
    }

    pub fn offset(&self) -> i64 {
        (self.page.unwrap_or(1) - 1) * self.limit()
    }

    pub fn order_by(&self) -> String {
        self.sort_by
            .clone()
            .unwrap_or_else(|| "created_at".to_string())
    }
}
