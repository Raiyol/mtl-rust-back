use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SearchQuery {
    pub search: Option<String>,
}
