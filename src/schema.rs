// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id, user_id) {
        id -> BigInt,
        created_at -> BigInt,
        sort_key -> Integer,
        title -> Text,
        order -> Text,
        user_id -> Integer,
        track -> Bool,
        show_in_lib -> Bool,
        deleted_at -> BigInt,
    }
}

diesel::table! {
    favourites (manga_id, category_id, user_id) {
        manga_id -> BigInt,
        category_id -> BigInt,
        sort_key -> Integer,
        created_at -> BigInt,
        deleted_at -> BigInt,
        user_id -> Integer,
    }
}

diesel::table! {
    history (manga_id, user_id) {
        manga_id -> BigInt,
        created_at -> BigInt,
        updated_at -> BigInt,
        chapter_id -> BigInt,
        page -> SmallInt,
        scroll -> Double,
        percent -> Double,
        chapters -> Integer,
        deleted_at -> BigInt,
        user_id -> Integer,
    }
}

diesel::table! {
    manga (id) {
        id -> BigInt,
        title -> Text,
        alt_title -> Nullable<Text>,
        url -> Text,
        public_url -> Text,
        rating -> Float,
        is_nsfw -> Bool,
        cover_url -> Text,
        large_cover_url -> Nullable<Text>,
        state -> Nullable<Text>,
        author -> Nullable<Text>,
        source -> Text,
    }
}

diesel::table! {
    manga_tags (manga_id, tag_id) {
        manga_id -> BigInt,
        tag_id -> BigInt,
    }
}

diesel::table! {
    tags (id) {
        id -> BigInt,
        title -> Text,
        key -> Text,
        source -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        password -> Text,
        nickname -> Nullable<Text>,
        favourites_sync_timestamp -> Nullable<BigInt>,
        history_sync_timestamp -> Nullable<BigInt>,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(favourites -> manga (manga_id));
diesel::joinable!(favourites -> users (user_id));
diesel::joinable!(history -> manga (manga_id));
diesel::joinable!(history -> users (user_id));
diesel::joinable!(manga_tags -> manga (manga_id));
diesel::joinable!(manga_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    favourites,
    history,
    manga,
    manga_tags,
    tags,
    users,
);
