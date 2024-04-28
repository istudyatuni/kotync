use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::categories,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Category {
    pub id: i64,
    pub created_at: i64,
    pub sort_key: i32,
    pub title: String,
    pub order: String,
    pub user_id: i32,
    pub track: bool,
    pub show_in_lib: bool,
    pub deleted_at: i64,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::favourites,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Favourite {
    pub manga_id: i64,
    pub category_id: i64,
    pub sort_key: i32,
    pub created_at: i64,
    pub deleted_at: i64,
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::history,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct HistoryEntry {
    pub manga_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub chapter_id: i64,
    pub page: i16,
    pub scroll: f64,
    pub percent: f64,
    pub chapters: i32,
    pub deleted_at: i64,
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::manga,
	check_for_backend(diesel::sqlite::Sqlite)
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

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::tags,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Tag {
    pub id: i64,
    pub title: String,
    pub key: String,
    pub source: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug, Default)]
#[diesel(
	table_name = crate::schema::users,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub password: String,
    pub nickname: Option<String>,
    pub favourites_sync_timestamp: Option<i64>,
    pub history_sync_timestamp: Option<i64>,
}
