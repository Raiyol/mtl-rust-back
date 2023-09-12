use diesel::prelude::*;
use diesel::sql_types::Datetime;

use super::super::schema::schema::review;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = review)]
pub struct Review {
    pub id: i32,
    pub id_novel: i32,
    pub id_user: i32,
    pub rate: i32,
    pub text: String,
    pub date: Datetime
}
