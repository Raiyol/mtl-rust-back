use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::schema::novel;

#[derive(QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = novel)]
pub struct Novel {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub cn_name: String,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub img: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub completed: u8,
}
