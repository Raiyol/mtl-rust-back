use diesel::prelude::*;
use diesel::sql_types::Datetime;

use super::super::schema::schema::user;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub confirmed: bool,
    pub role: String,
    pub pfp: String,
    pub date: Datetime
}
