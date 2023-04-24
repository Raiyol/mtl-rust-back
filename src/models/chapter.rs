use diesel::prelude::*;
use diesel::sql_types::Datetime;

use super::super::schema::schema::chapter;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = chapter)]
#[diesel(belongs_to(Novel))]
pub struct Chapter {
    pub id: i32,
    pub id_novel: i32,
    pub number: i32,
    pub date: Datetime,
    pub title_en: String,
    pub title_cn: String,
    pub content: String,
    pub dict: String
}
