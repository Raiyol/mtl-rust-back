use diesel::prelude::*;

use crate::beans::recent_chapter::RecentChapter;
use crate::models::chapter::{Chapter, ChapterInfo};
use crate::models::novel::Novel;
use crate::schema::schema::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_chapter_by_id(
    conn: &mut MysqlConnection,
    novel_url: String,
    number: u32,
) -> Result<Option<Chapter>, DbError> {
    let res = chapter::table
        .inner_join(novel::table)
        .filter(novel::url.eq(novel_url))
        .filter(chapter::number.eq(number))
        .select(Chapter::as_select())
        .first::<Chapter>(conn)
        .optional()?;

    Ok(res)
}

pub fn get_recent_chapters(
    conn: &mut MysqlConnection,
) -> Result<Vec<RecentChapter>, diesel::result::Error> {
    let chapters_with_novel: Result<Vec<(ChapterInfo, Novel)>, diesel::result::Error> =
        chapter::table
            .inner_join(novel::table)
            .select((ChapterInfo::as_select(), Novel::as_select()))
            .limit(20)
            .order(chapter::date.desc())
            .load::<(ChapterInfo, Novel)>(conn);

    chapters_with_novel.map(|op| {
        op.iter()
            .map(|(chapter_info, nov)| RecentChapter {
                id: chapter_info.id,
                url: nov.url.clone(),
                name: nov.name.clone(),
                chapter: chapter_info.number,
                date: chapter_info.date,
            })
            .collect()
    })
}
