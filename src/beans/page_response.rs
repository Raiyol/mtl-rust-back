use serde::{Deserialize, Serialize};

use super::pageable::Pageable;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub results: Vec<T>,
    pub total: i64,
    pub page_total: i64,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T> PageResponse<T> {
    pub fn create(content: Vec<T>, total: i64, pageable: Pageable) -> PageResponse<T> {
        PageResponse {
            results: content,
            total,
            page_total: if total > 0 {
                pageable.page_size / total
            } else {
                total
            },
            has_next: pageable.page_number * pageable.page_size <= total,
            has_previous: pageable.page_number > 0,
        }
    }
}
