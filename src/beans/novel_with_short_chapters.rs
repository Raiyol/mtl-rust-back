use serde::{Deserialize, Serialize};
use crate::models::chapter::ChapterInfo;
use crate::models::genre::Genre;
use crate::models::novel::Novel;

#[derive(Serialize, Deserialize)]
pub struct NovelWithShortChapters {
    #[serde(flatten)]
    pub novel: Novel,
    pub genres: Vec<Genre>,
    pub chapters: Vec<ChapterInfo>,
}
