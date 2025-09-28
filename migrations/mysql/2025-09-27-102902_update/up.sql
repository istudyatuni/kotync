-- manga
alter table manga modify column title varchar(255);
alter table manga modify column alt_title varchar(255);
alter table manga modify column author varchar(64);

-- manga content_rating
alter table manga add column content_rating char(20);
update manga set content_rating = 'SAFE' where is_nsfw = 0;
update manga set content_rating = 'ADULT' where is_nsfw = 1;

alter table manga drop column is_nsfw;

-- users
alter table users rename column password to password_hash;

alter table users modify column email varchar(320) not null unique;
alter table users modify column password_hash varchar(128) not null;
alter table users modify column nickname varchar(100);

-- favourites
alter table favourites add column pinned tinyint(1) not null default 0;
