use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::super::schema::schema::genre;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = genre)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}
