-- manga content_rating
alter table manga add column content_rating char(20);
update manga set content_rating = 'SAFE' where is_nsfw = 0;
update manga set content_rating = 'ADULT' where is_nsfw = 1;

alter table manga drop column is_nsfw;

-- manga
-- order of fields must be the same as in the "manga" table for "insert into manga_new select * from manga" to work
create table manga_new (
    id              bigint       not null,
    title           varchar(255) not null, -- updated
    alt_title       varchar(255),          -- updated
    url             varchar(255) not null,
    public_url      varchar(255) not null,
    rating          float        not null,
    -- SAFE, SUGGESTIVE, ADULT
    cover_url       varchar(255) not null,
    large_cover_url varchar(255),
    state           char(24),
    author          varchar(64),           -- updated
    source          varchar(32)  not null,
    -- ONGOING, FINISHED, ABANDONED, PAUSED, UPCOMING, RESTRICTED
    content_rating  char(20),              -- new
    primary key (id)
);

insert into manga_new select * from manga;

-- PRAGMA is needed because other tables references this table
PRAGMA foreign_keys = OFF;
drop table manga;
alter table manga_new rename to manga;
PRAGMA foreign_keys = ON;

-- users
alter table users rename column password to password_hash;

create table users_new
(
    id            integer primary key autoincrement not null,
    email         varchar(320) not null unique, -- updated
    password_hash varchar(128) not null,        -- updated
    nickname      varchar(100),                 -- updated
    favourites_sync_timestamp bigint,
    history_sync_timestamp    bigint
);
insert into users_new select * from users;

-- PRAGMA is needed because other tables references this table
PRAGMA foreign_keys = OFF;
drop table users;
alter table users_new rename to users;
PRAGMA foreign_keys = ON;

-- favourites
alter table favourites add column pinned tinyint(1) not null default 0;
