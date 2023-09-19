use crate::models::chapter::ChapterInfo;
use crate::models::novel::Novel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NovelWithShortChapters {
    #[serde(flatten)]
    pub novel: Novel,
    pub genres: Vec<String>,
    pub chapters: Vec<ChapterInfo>,
}
