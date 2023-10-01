use crate::models::{chapter::Chapter, novel::Novel};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterLightBean {
    pub id: u32,
    pub name: String,
    pub number: u32,
    pub date: Option<NaiveDateTime>,
    pub title_en: Option<String>,
    pub title_cn: Option<String>,
    pub content: Option<Value>,
    pub dict: Option<Value>,
    pub first_chapter: u32,
    pub last_chapter: u32,
}

impl ChapterLightBean {
    pub fn map(
        chapter: Chapter,
        novel: Novel,
        first_chapter: u32,
        last_chapter: u32,
    ) -> ChapterLightBean {
        ChapterLightBean {
            id: chapter.id,
            name: novel.name,
            number: chapter.number,
            date: chapter.date,
            title_en: chapter.title_en,
            title_cn: chapter.title_cn,
            content: chapter
                .content
                .map(|content| serde_json::from_str(&content).expect("wrong string format")),
            dict: chapter
                .dict
                .map(|dict| serde_json::from_str(&dict).expect("wrong string format")),
            first_chapter,
            last_chapter,
        }
    }
}
