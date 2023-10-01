use diesel::prelude::*;

use crate::beans::novel_with_short_chapters::NovelWithShortChapters;
use crate::beans::page_response::PageResponse;
use crate::beans::pageable::Pageable;
use crate::models::chapter::ChapterInfo;
use crate::models::genre::Genre;
use crate::models::novel::Novel;
use crate::models::novel_genre::NovelGenre;
use crate::models::review::Review;
use crate::schema::schema::genre::name;
use crate::schema::schema::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_novel(
    conn: &mut MysqlConnection,
    pageable: Pageable,
) -> Result<PageResponse<NovelWithShortChapters>, diesel::result::Error> {
    let total: i64 = novel::table.count().get_result(conn)?;

    let nov = novel::table
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .select(Novel::as_select())
        .load::<Novel>(conn)?;

    let genres = NovelGenre::belonging_to(&nov)
        .inner_join(genre::table)
        .select((NovelGenre::as_select(), Genre::as_select()))
        .load::<(NovelGenre, Genre)>(conn)?;

    let grouped = genres
        .grouped_by(&nov)
        .into_iter()
        .zip(nov)
        .map(|(g, nov)| NovelWithShortChapters {
            novel: nov,
            chapters: vec![],
            genres: g.into_iter().map(|(_, genre)| genre.name).collect(),
        })
        .collect::<Vec<NovelWithShortChapters>>();

    Ok(PageResponse::create(grouped, total, pageable))
}

pub fn get_random_novels(conn: &mut MysqlConnection) -> Result<Vec<Novel>, diesel::result::Error> {
    sql_function!(fn rand() -> Text);

    novel::table.limit(5).order(rand()).load::<Novel>(conn)
}

pub fn find_novel_by_url_with_chapters_info(
    conn: &mut MysqlConnection,
    novel_url: String,
) -> Result<NovelWithShortChapters, DbError> {
    let nov = novel::table
        .filter(novel::url.eq(novel_url))
        .select(Novel::as_select())
        .get_result(conn)?;

    let chapters = ChapterInfo::belonging_to(&nov)
        .select(ChapterInfo::as_select())
        .load(conn)?;

    let genres = NovelGenre::belonging_to(&nov)
        .inner_join(genre::table)
        .select(name)
        .load(conn)?;

    return Ok(NovelWithShortChapters {
        novel: nov,
        chapters,
        genres,
    });
}

pub fn search(
    conn: &mut MysqlConnection,
    search: Option<String>,
) -> Result<Vec<Novel>, diesel::result::Error> {
    let mut nov = novel::table.into_boxed();

    if search.is_some() {
        let search_text = search.expect("Search text is null");
        nov = nov
            .filter(novel::name.like(format!("%{}%", &search_text)))
            .or_filter(novel::cn_name.like(format!("%{}%", &search_text)));
    }

    nov.select(Novel::as_select()).load::<Novel>(conn)
}

pub fn find_novel_review(
    conn: &mut MysqlConnection,
    novel_id: u32,
    pageable: Pageable,
) -> Result<PageResponse<Review>, diesel::result::Error> {
    let total = review::table
        .filter(review::id_novel.eq(novel_id))
        .count()
        .get_result(conn)?;

    let reviews = review::table
        .filter(review::id_novel.eq(novel_id))
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .load::<Review>(conn)?;

    Ok(PageResponse::create(reviews, total, pageable))
}
