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
    pub fn to_query(self) -> (i64, i64, String) {
        let limit = self.per_page.unwrap_or(10);
        let offset = (self.page.unwrap_or(1) - 1) * limit;
        let order_by = self.sort_by.unwrap_or_else(|| "id".to_string());
        (limit, offset, order_by)
    }
}
