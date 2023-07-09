use diesel::prelude::*;

use crate::beans::pageable::Pageable;
use crate::models::chapter::ChapterInfo;
use crate::models::novel::{Novel, NovelWithShortChapters};
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

pub fn find_novel_by_url_with_chapters_info(conn: &mut MysqlConnection, novel_url: String) -> Result<NovelWithShortChapters, DbError> {
    let nov = novel
        .filter(url.eq(novel_url))
        .select(Novel::as_select())
        .get_result(conn)?;

    let chaps = ChapterInfo::belonging_to(&nov)
        .select(ChapterInfo::as_select())
        .load(conn)?;

    return Ok(NovelWithShortChapters{novel: nov, chapters: chaps});
}
