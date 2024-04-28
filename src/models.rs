use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::categories,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Category {
    id: i64,
    created_at: i64,
    sort_key: i32,
    title: String,
    order: String,
    user_id: i32,
    track: bool,
    show_in_lib: bool,
    deleted_at: i64,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::favourites,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Favourite {
    manga_id: i64,
    category_id: i64,
    sort_key: i32,
    created_at: i64,
    deleted_at: i64,
    user_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::history,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct HistoryEntry {
    manga_id: i64,
    created_at: i64,
    updated_at: i64,
    chapter_id: i64,
    page: i16,
    scroll: f64,
    percent: f64,
    chapters: i32,
    deleted_at: i64,
    user_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::manga,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Manga {
    id: i64,
    title: String,
    alt_title: Option<String>,
    url: String,
    public_url: String,
    rating: f32,
    is_nsfw: bool,
    cover_url: String,
    large_cover_url: Option<String>,
    state: Option<String>,
    author: Option<String>,
    source: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::tags,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Tag {
    id: i64,
    title: String,
    key: String,
    source: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(
	table_name = crate::schema::users,
	check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct User {
    id: Option<i32>,
    email: String,
    password: String,
    nickname: Option<String>,
    favourites_sync_timestamp: Option<i64>,
    history_sync_timestamp: Option<i64>,
}
