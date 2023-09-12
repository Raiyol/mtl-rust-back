use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Pageable {
    #[validate(range(min = 0, message = "Page number must be positive"))]
    pub page_number: i64,
    #[validate(range(min = 1, max = 100, message = "Page size must be between 1 and 100"))]
    pub page_size: i64,
    pub sort: Option<String>,
    pub sort_order: Option<String>,
}
