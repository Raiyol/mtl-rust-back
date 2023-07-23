use diesel::prelude::*;

use crate::beans::pageable::Pageable;
use crate::models::chapter::ChapterInfo;
use crate::models::genre::Genre;
use crate::models::novel::{Novel, NovelWithShortChapters};
use crate::models::novel_genre::NovelGenre;
use crate::schema::schema::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_novel(conn: &mut MysqlConnection, pageable: Pageable) -> Result<Vec<Novel>, diesel::result::Error> {
    novel::table
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .load::<Novel>(conn)
}


pub fn get_random_novels(conn: &mut MysqlConnection) -> Result<Vec<Novel>, diesel::result::Error> {
    sql_function!(fn rand() -> Text);

    novel::table
        .limit(5)
        .order(rand())
        .load::<Novel>(conn)
}

pub fn find_novel_by_url(conn: &mut MysqlConnection, novel_url: String) -> Result<Option<Novel>, DbError> {
    let res = novel::table
        .filter(novel::url.eq(novel_url))
        .first::<Novel>(conn)
        .optional()?;

    Ok(res)
}

pub fn find_novel_by_url_with_chapters_info(conn: &mut MysqlConnection, novel_url: String) -> Result<NovelWithShortChapters, DbError> {
    let nov = novel::table
        .filter(novel::url.eq(novel_url))
        .select(Novel::as_select())
        .get_result(conn)?;

    let chapters = ChapterInfo::belonging_to(&nov)
        .select(ChapterInfo::as_select())
        .load(conn)?;

    let genres = NovelGenre::belonging_to(&nov)
        .inner_join(genre::table)
        .select(Genre::as_select())
        .load(conn)?;

    return Ok(NovelWithShortChapters{novel: nov, chapters, genres});
}
