use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pageable {
    pub page_number: i64,
    pub page_size: i64,
    pub sort: Option<String>,
    pub sort_order: Option<String>,
}
