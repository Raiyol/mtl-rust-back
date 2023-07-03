use diesel::prelude::*;

use crate::beans::pageable::Pageable;
use crate::models::novel::Novel;
use crate::schema::schema::novel::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_novel(conn: &mut MysqlConnection, pageable: Pageable) -> Result<Vec<Novel>, diesel::result::Error> {
    novel
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .load::<Novel>(conn)
}

pub fn find_novel_by_url(conn: &mut MysqlConnection, novel_url: String) -> Result<Option<Novel>, DbError> {
    let res = novel
        .filter(url.eq(novel_url))
        .first::<Novel>(conn)
        .optional()?;

    Ok(res)
}
