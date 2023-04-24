use diesel::prelude::*;
use diesel::sql_types::Datetime;

use super::super::schema::schema::comment;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = comment)]
#[diesel(belongs_to(Chapter))]
#[diesel(belongs_to(User))]
pub struct Comment {
    pub id: i32,
    pub id_chapter: i32,
    pub id_user: i32,
    pub text: String,
    pub date: Datetime
}
