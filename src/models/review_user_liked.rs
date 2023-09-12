use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = review_user_liked)]
pub struct ReviewUserLiked {
    pub id_review: i32,
    pub id_user: i32
}
