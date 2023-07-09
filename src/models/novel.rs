use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::chapter::{ChapterInfo};

use crate::schema::schema::novel;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq)]
#[diesel(table_name = novel)]
pub struct Novel {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub cn_name: String,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub img: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub completed: i8,
}


#[derive(Serialize, Deserialize)]
pub struct NovelWithShortChapters {
    #[serde(flatten)]
    pub novel: Novel,
    pub chapters: Vec<ChapterInfo>,
}