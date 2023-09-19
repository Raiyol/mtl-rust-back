use crate::beans::comment_lightbean::CommentLightBean;
use crate::beans::page_response::PageResponse;
use crate::beans::pageable::Pageable;
use crate::models::comment::Comment;
use crate::models::user::User;
use diesel::prelude::*;

use crate::schema::schema::*;

pub fn get_comments_by_chapter_id(
    conn: &mut MysqlConnection,
    chapter_id: u32,
    pageable: Pageable,
) -> Result<PageResponse<CommentLightBean>, diesel::result::Error> {
    let com: Result<Vec<(Comment, User)>, diesel::result::Error> = comment::table
        .inner_join(user::table)
        .filter(comment::id_chapter.eq(chapter_id))
        .select((Comment::as_select(), User::as_select()))
        .limit(pageable.page_size)
        .offset(pageable.page_number * pageable.page_size)
        .load(conn);

    let total = comment::table
        .filter(comment::id_chapter.eq(chapter_id))
        .count()
        .get_result(conn)?;

    com.map(|op| {
        PageResponse::create(
            op.iter()
                .map(|(comment, user)| CommentLightBean::map(comment, user))
                .collect(),
            total,
            pageable,
        )
    })
}
