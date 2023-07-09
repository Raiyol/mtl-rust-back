use diesel::prelude::*;
use crate::models::chapter::{Chapter};

use crate::schema::schema::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_chapter_by_id(conn: &mut MysqlConnection, novel_url: String, number: u32) -> Result<Option<Chapter>, DbError> {
    let res = chapter::table
        .inner_join(novel::table)
        .filter(novel::url.eq(novel_url))
        .filter(chapter::number.eq(number))
        .select(Chapter::as_select())
        .first::<Chapter>(conn)
        .optional()?;

    Ok(res)
}
