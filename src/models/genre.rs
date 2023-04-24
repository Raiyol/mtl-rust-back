use diesel::prelude::*;

use super::super::schema::schema::genre;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = genre)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}
