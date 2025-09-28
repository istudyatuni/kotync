-- manga
-- SAFE, SUGGESTIVE, ADULT
alter table manga add column content_rating text;
update manga set content_rating = 'SAFE' where is_nsfw = 0;
update manga set content_rating = 'ADULT' where is_nsfw = 1;

alter table manga drop column is_nsfw;

-- users
alter table users rename column password to password_hash;

-- favourites
alter table favourites add column pinned tinyint(1) not null default 0;
