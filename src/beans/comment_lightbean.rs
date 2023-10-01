use crate::models::comment::Comment;
use crate::models::user::User;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommentLightBean {
    pub id: u32,
    pub comment: String,
    pub date: Option<NaiveDateTime>,
    pub name: String,
    pub pfp: String,
}

impl CommentLightBean {
    pub fn map(comment: &Comment, user: &User) -> CommentLightBean {
        CommentLightBean {
            id: comment.id,
            comment: comment.text.clone(),
            date: comment.date,
            name: user.name.clone(),
            pfp: user.pfp.clone(),
        }
    }
}
