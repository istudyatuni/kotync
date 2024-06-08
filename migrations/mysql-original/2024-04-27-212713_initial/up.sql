-- for migrating old db
alter table users modify column password char(128) not null;
