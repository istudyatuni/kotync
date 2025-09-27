use serde::{Deserialize, Serialize};
use strum::EnumString;

use super::{
    db::{
        Category as DBCategory, Favourite as DBFavourite, History as DBHistory, Manga as DBManga,
        Tag as DBTag,
    },
    IntToBool, TruncatedString,
};

pub type Time = i64;
pub type UserID = i32;

#[derive(
    Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Default, EnumString, strum::Display,
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum MangaState {
    #[default]
    Ongoing,
    Finished,
    Abandoned,
    Paused,
    Upcoming,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FavouritesPackage {
    #[serde(rename = "favourite_categories")]
    pub categories: Vec<Category>,
    pub favourites: Vec<Favourite>,
    pub timestamp: Option<Time>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HistoryPackage {
    pub history: Vec<History>,
    pub timestamp: Option<Time>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Category {
    #[serde(rename = "category_id")]
    pub id: i64,
    pub created_at: Time,
    pub sort_key: i32,
    pub track: i32,
    pub title: String,
    pub order: String,
    pub deleted_at: Time,
    pub show_in_lib: i32,
}

impl Category {
    pub fn to_db(&self, user_id: i32) -> DBCategory {
        DBCategory {
            id: self.id,
            created_at: self.created_at,
            sort_key: self.sort_key,
            title: self.title.truncated(120),
            order: self.order.clone(),
            user_id,
            track: self.track.to_bool(),
            show_in_lib: self.show_in_lib.to_bool(),
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Favourite {
    pub manga_id: i64,
    pub manga: Manga,
    // original is i32
    pub category_id: i64,
    pub sort_key: i32,
    pub created_at: Time,
    pub deleted_at: Time,
}

impl Favourite {
    pub fn to_db(&self, user_id: UserID) -> DBFavourite {
        DBFavourite {
            manga_id: self.manga_id,
            category_id: self.category_id,
            sort_key: self.sort_key,
            created_at: self.created_at,
            deleted_at: self.deleted_at,
            user_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct History {
    pub manga_id: i64,
    pub manga: Manga,
    pub created_at: Time,
    pub updated_at: Time,
    pub chapter_id: i64,
    pub page: i32,
    pub scroll: f64,
    pub percent: f64,
    #[serde(default = "minus_1")]
    pub chapters: i32,
    pub deleted_at: Time,
}

const fn minus_1() -> i32 {
    -1
}

impl History {
    pub fn to_db(&self, user_id: UserID) -> DBHistory {
        DBHistory {
            manga_id: self.manga.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            chapter_id: self.chapter_id,
            page: self.page as i16,
            scroll: self.scroll,
            percent: self.percent,
            chapters: self.chapters,
            deleted_at: self.deleted_at,
            user_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Manga {
    #[serde(rename = "manga_id")]
    pub id: i64,
    pub title: String,
    pub alt_title: Option<String>,
    pub url: String,
    pub public_url: String,
    pub rating: f32,
    #[serde(rename = "nsfw")]
    pub is_nsfw: Option<i32>,
    pub cover_url: String,
    pub large_cover_url: Option<String>,
    pub tags: Vec<MangaTag>,
    pub state: Option<MangaState>,
    pub author: Option<String>,
    pub source: String,
}

impl Manga {
    pub fn to_db(&self) -> DBManga {
        DBManga {
            id: self.id,
            title: self.title.truncated(84),
            alt_title: self.alt_title.clone().map(|t| t.truncated(84)),
            url: self.url.truncated(255),
            public_url: self.public_url.truncated(255),
            rating: self.rating,
            is_nsfw: self.is_nsfw.to_bool(),
            cover_url: self.cover_url.truncated(255),
            large_cover_url: None, // ignored?
            state: self.state.map(|s| s.to_string()),
            author: self.author.clone().map(|p| p.truncated(32)),
            source: self.source.truncated(32),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MangaTag {
    #[serde(rename = "tag_id")]
    pub id: i64,
    pub title: String,
    pub key: String,
    pub source: String,
}

impl MangaTag {
    pub fn to_db(&self) -> DBTag {
        DBTag {
            id: self.id,
            title: self.title.clone(),
            key: self.key.clone(),
            source: self.source.clone(),
        }
    }
}
