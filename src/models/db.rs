use std::str::FromStr;

use diesel::prelude::*;

#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite as Backend;

#[cfg(feature = "mysql")]
use diesel::mysql::Mysql as Backend;

use super::common::{
    Category as ApiCategory, Favourite as ApiFavourite, History as ApiHistory, Manga as ApiManga,
    MangaState, MangaTag as ApiMangaTag, Time, UserID,
};

#[derive(Queryable, Selectable, Insertable, Identifiable, AsChangeset, Debug)]
#[diesel(
    table_name = crate::db::schema::categories,
    check_for_backend(Backend),
    primary_key(id, user_id)
)]
pub struct Category {
    pub id: i64,
    pub created_at: Time,
    pub sort_key: i32,
    pub title: String,
    pub order: String,
    pub user_id: UserID,
    pub track: bool,
    pub show_in_lib: bool,
    pub deleted_at: Time,
}

impl Category {
    pub fn to_api(&self) -> ApiCategory {
        ApiCategory {
            id: self.id,
            created_at: self.created_at,
            sort_key: self.sort_key,
            track: self.track as i32,
            title: self.title.clone(),
            order: self.order.clone(),
            deleted_at: self.deleted_at,
            show_in_lib: self.show_in_lib as i32,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(
    table_name = crate::db::schema::favourites,
    check_for_backend(Backend)
)]
pub struct Favourite {
    pub manga_id: i64,
    pub category_id: i64,
    pub sort_key: i32,
    pub created_at: Time,
    pub deleted_at: Time,
    pub user_id: UserID,
}

impl Favourite {
    pub fn to_api(&self, manga: ApiManga) -> ApiFavourite {
        ApiFavourite {
            manga_id: manga.id,
            manga,
            category_id: self.category_id,
            sort_key: self.sort_key,
            created_at: self.created_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(
    table_name = crate::db::schema::history,
    check_for_backend(Backend)
)]
pub struct History {
    pub manga_id: i64,
    pub created_at: Time,
    pub updated_at: Time,
    pub chapter_id: i64,
    pub page: i16,
    pub scroll: f64,
    pub percent: f64,
    pub chapters: i32,
    pub deleted_at: Time,
    pub user_id: UserID,
}

impl History {
    pub fn to_api(&self, manga: ApiManga) -> ApiHistory {
        ApiHistory {
            manga_id: manga.id,
            manga,
            created_at: self.created_at,
            updated_at: self.updated_at,
            chapter_id: self.chapter_id,
            page: self.page as i32,
            scroll: self.scroll,
            percent: self.percent,
            chapters: self.chapters,
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Identifiable, AsChangeset, Debug)]
#[diesel(
    table_name = crate::db::schema::manga,
    check_for_backend(Backend)
)]
pub struct Manga {
    pub id: i64,
    pub title: String,
    pub alt_title: Option<String>,
    pub url: String,
    pub public_url: String,
    pub rating: f32,
    pub is_nsfw: bool,
    pub cover_url: String,
    pub large_cover_url: Option<String>,
    pub state: Option<String>,
    pub author: Option<String>,
    pub source: String,
}

impl Manga {
    pub fn to_api(&self, tags: Vec<ApiMangaTag>) -> ApiManga {
        ApiManga {
            id: self.id,
            title: self.title.clone(),
            alt_title: self.alt_title.clone(),
            url: self.url.clone(),
            public_url: self.public_url.clone(),
            rating: self.rating,
            is_nsfw: self.is_nsfw as i32,
            cover_url: self.cover_url.clone(),
            large_cover_url: self.large_cover_url.clone(),
            tags,
            state: self
                .state
                .clone()
                // todo: probably need better error handling
                .map(|s| MangaState::from_str(&s).unwrap_or_default()),
            author: self.author.clone(),
            source: self.source.clone(),
        }
    }
}

#[derive(Identifiable, Selectable, Insertable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Manga))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = crate::db::schema::manga_tags)]
#[diesel(primary_key(manga_id, tag_id))]
pub struct MangaTags {
    pub manga_id: i64,
    pub tag_id: i64,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, AsChangeset, Debug)]
#[diesel(
    table_name = crate::db::schema::tags,
    check_for_backend(Backend)
)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub key: String,
    pub source: String,
}

impl Tag {
    pub fn to_join(&self, manga_id: i64) -> MangaTags {
        MangaTags {
            manga_id,
            tag_id: self.id,
        }
    }
    pub fn to_api(&self) -> ApiMangaTag {
        ApiMangaTag {
            id: self.id,
            title: self.title.clone(),
            key: self.key.clone(),
            source: self.source.clone(),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Default)]
#[diesel(
    table_name = crate::db::schema::users,
    check_for_backend(Backend)
)]
pub struct User {
    pub id: UserID,
    pub email: String,
    pub password: String,
    pub nickname: Option<String>,
    pub favourites_sync_timestamp: Option<Time>,
    pub history_sync_timestamp: Option<Time>,
}

#[derive(Insertable, Debug)]
#[diesel(
    table_name = crate::db::schema::users,
    check_for_backend(Backend)
)]
pub struct UserInsert {
    pub email: String,
    pub password: String,
}
