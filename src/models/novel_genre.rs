use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = novel_genre)]
#[diesel(belongs_to(Novel))]
#[diesel(belongs_to(Genre))]
pub struct NovelGenre {
    pub id_novel: i32,
    pub id_genre: i32
}
