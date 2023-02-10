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

