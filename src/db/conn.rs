use anyhow::{Context, Result, anyhow};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

#[cfg(feature = "sqlite")]
use diesel::{prelude::SqliteConnection as DbConnection, sqlite::Sqlite as Backend};

#[cfg(feature = "mysql")]
use diesel::{mysql::Mysql as Backend, prelude::MysqlConnection as DbConnection};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::config::ConfDB;
use crate::models::admin::DBStats;
use crate::models::common::HistoryPackage;
use crate::models::db::{History, MangaTags};
use crate::models::{
    common::{FavouritesPackage, Time, UserID},
    db::{Category, Favourite, Manga, Tag, User, UserInsert},
};

#[cfg(feature = "sqlite")]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite");

#[cfg(any(
    all(feature = "mysql", not(feature = "original")),
    all(feature = "original", test),
))]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/mysql");

#[cfg(all(feature = "original", not(test)))]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/mysql-original");

type ConnManager = ConnectionManager<DbConnection>;
type Conn = PooledConnection<ConnManager>;

pub struct DB {
    conn: Pool<ConnManager>,
}

impl DB {
    pub fn new(db_conf: ConfDB) -> Result<Self> {
        let pool = Pool::builder()
            .max_size(16)
            .build(ConnectionManager::<DbConnection>::new(db_conf.url()))?;

        let conn = &mut pool.get()?;
        migrate(conn)?;

        #[cfg(all(test, feature = "mysql"))]
        conn.begin_test_transaction()?;

        Ok(Self { conn: pool })
    }
    pub fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        use super::schema::users::dsl::{email as user_email, users};

        Ok(users
            .filter(user_email.eq(email))
            .select(User::as_select())
            .first(&mut self.pool()?)
            .optional()?)
    }
    pub fn get_user(&self, user_id: UserID) -> Result<Option<User>> {
        use super::schema::users::dsl::users;

        Ok(users
            .find(user_id)
            .select(User::as_select())
            .first(&mut self.pool()?)
            .optional()?)
    }
    pub fn create_user(&self, email: &str, password_hash: &str) -> Result<User> {
        use super::schema::users::dsl::users;

        // assuming that this user doesn't exists
        diesel::insert_into(users)
            .values(UserInsert {
                email: email.to_string(),
                password_hash: password_hash.to_string(),
            })
            .execute(&mut self.pool()?)?;

        self.get_user_by_email(email).map(|u| u.unwrap())
    }
    #[cfg(feature = "migrate-md5")]
    pub fn update_user_password(&self, user_id: UserID, password_hash: &str) -> Result<()> {
        use super::schema::users::dsl::{id, password_hash as password_col, users};

        diesel::update(users)
            .filter(id.eq(user_id))
            .set(password_col.eq(password_hash))
            .execute(&mut self.pool()?)?;
        Ok(())
    }
    pub fn set_favourites_synchronized(&self, user_id: UserID, time: Time) -> Result<()> {
        use super::schema::users::dsl::{favourites_sync_timestamp, id, users};

        diesel::update(users)
            .filter(id.eq(user_id))
            .set(favourites_sync_timestamp.eq(time))
            .execute(&mut self.pool()?)?;
        Ok(())
    }
    pub fn set_history_synchronized(&self, user_id: UserID, time: Time) -> Result<()> {
        use super::schema::users::dsl::{history_sync_timestamp, id, users};

        diesel::update(users)
            .filter(id.eq(user_id))
            .set(history_sync_timestamp.eq(time))
            .execute(&mut self.pool()?)?;
        Ok(())
    }
    pub fn add_favourites_package(&self, pkg: &FavouritesPackage, user_id: UserID) -> Result<()> {
        log::info!("adding favourites_package for user {user_id}");

        let conn = &mut self.pool()?;

        conn.transaction::<_, anyhow::Error, _>(|conn| {
            for c in &pkg.categories {
                log::debug!("adding category {}", c.id);
                Self::add_category(conn, c.to_db(user_id))?;
            }
            for f in &pkg.favourites {
                log::debug!("adding manga {}", f.manga.id);
                Self::add_manga(conn, f.manga.to_db())?;
                log::debug!("adding tags for manga {}", f.manga.id);
                Self::add_tags(
                    conn,
                    f.manga.tags.iter().map(|t| t.to_db()).collect(),
                    f.manga_id,
                )?;
                log::debug!("adding favourite for manga {}", f.manga.id);
                Self::add_favourite(conn, f.to_db(user_id))?;
            }
            Ok(())
        })?;
        Ok(())
    }
    pub fn add_history_package(&self, pkg: &HistoryPackage, user_id: UserID) -> Result<()> {
        let conn = &mut self.pool()?;
        conn.transaction::<_, anyhow::Error, _>(|conn| {
            for h in &pkg.history {
                log::debug!("adding manga");
                Self::add_manga(conn, h.manga.to_db())?;
                log::debug!("adding tags");
                Self::add_tags(
                    conn,
                    h.manga.tags.iter().map(|t| t.to_db()).collect(),
                    h.manga_id,
                )?;
                log::debug!("adding history entry");
                Self::add_history(conn, h.to_db(user_id))?;
            }
            Ok(())
        })?;
        Ok(())
    }
    pub fn load_favourites_package(&self, user_id: UserID) -> Result<FavouritesPackage> {
        let categories = self.list_categories(user_id)?;
        let favourites = self.list_favourites(user_id)?;

        Ok(FavouritesPackage {
            categories: categories.iter().map(|c| c.to_api()).collect(),
            favourites: favourites
                .iter()
                .map(|(fav, manga)| -> Result<_> {
                    // todo: maybe try to load manga with tags in one query
                    // (it was hard to understand how to do it)
                    // https://diesel.rs/guides/relations.html
                    let tags: Vec<Tag> = self.list_tags(manga.id)?;
                    Ok(fav.to_api(manga.to_api(tags.iter().map(|t| t.to_api()).collect())))
                })
                .collect::<Result<Vec<_>>>()?,
            timestamp: self
                .get_user(user_id)?
                .and_then(|u| u.favourites_sync_timestamp),
        })
    }
    pub fn load_history_package(&self, user_id: UserID) -> Result<HistoryPackage> {
        let history = self.list_history(user_id)?;

        Ok(HistoryPackage {
            history: history
                .iter()
                .map(|(hist, manga)| -> Result<_> {
                    // todo: same as in load_favourites_package
                    let tags: Vec<Tag> = self.list_tags(manga.id)?;
                    Ok(hist.to_api(manga.to_api(tags.iter().map(|t| t.to_api()).collect())))
                })
                .collect::<Result<Vec<_>>>()?,
            timestamp: self
                .get_user(user_id)?
                .and_then(|u| u.history_sync_timestamp),
        })
    }
    fn list_favourites(&self, user_id: UserID) -> Result<Vec<(Favourite, Manga)>> {
        use super::schema::favourites::dsl::user_id as user_id_col;
        use super::schema::{favourites, manga};

        Ok(favourites::table
            .inner_join(manga::table)
            .filter(user_id_col.eq(user_id))
            .select((Favourite::as_select(), Manga::as_select()))
            .load(&mut self.pool()?)?)
    }
    fn list_history(&self, user_id: UserID) -> Result<Vec<(History, Manga)>> {
        use super::schema::history::dsl::user_id as user_id_col;
        use super::schema::{history, manga};

        Ok(history::table
            .inner_join(manga::table)
            .filter(user_id_col.eq(user_id))
            .select((History::as_select(), Manga::as_select()))
            .load(&mut self.pool()?)?)
    }
    fn add_category(conn: &mut Conn, category: Category) -> Result<()> {
        #[allow(unused)]
        use super::schema::categories::dsl::{categories, id, user_id};

        let q = diesel::insert_into(categories).values(&category);

        #[cfg(feature = "mysql")]
        let q = q.on_conflict(diesel::dsl::DuplicatedKeys);

        #[cfg(feature = "sqlite")]
        let q = q.on_conflict((id, user_id));

        q.do_update().set(&category).execute(conn)?;

        Ok(())
    }
    fn list_categories(&self, user_id: UserID) -> Result<Vec<Category>> {
        use super::schema::categories::dsl::{categories, user_id as user_id_col};

        Ok(categories
            .filter(user_id_col.eq(user_id))
            .select(Category::as_select())
            .get_results(&mut self.pool()?)?)
    }
    fn add_manga(conn: &mut Conn, manga: Manga) -> Result<()> {
        #[allow(unused)]
        use super::schema::manga::dsl::{id, manga as manga_table};

        let q = diesel::insert_into(manga_table).values(&manga);

        #[cfg(feature = "mysql")]
        let q = q.on_conflict(diesel::dsl::DuplicatedKeys);

        #[cfg(feature = "sqlite")]
        let q = q.on_conflict((id,));

        q.do_update().set(&manga).execute(conn)?;
        Ok(())
    }
    fn add_history(conn: &mut Conn, history: History) -> Result<()> {
        #[allow(unused)]
        use super::schema::history::dsl::{history as history_table, manga_id, user_id};

        let q = diesel::insert_into(history_table).values(&history);

        #[cfg(feature = "mysql")]
        let q = q.on_conflict(diesel::dsl::DuplicatedKeys);

        #[cfg(feature = "sqlite")]
        let q = q.on_conflict((manga_id, user_id));

        q.do_update().set(&history).execute(conn)?;
        Ok(())
    }
    pub fn get_manga(&self, manga_id: i64) -> Result<Option<(Manga, Vec<Tag>)>> {
        use super::schema::manga;

        let manga: Option<Manga> = manga::table
            .find(manga_id)
            .first(&mut self.pool()?)
            .optional()?;
        let Some(manga) = manga else {
            return Ok(None);
        };

        let tags = self.list_tags(manga.id)?;
        Ok(Some((manga, tags)))
    }
    pub fn list_manga(&self, offset: usize, limit: usize) -> Result<Vec<(Manga, Vec<Tag>)>> {
        use super::schema::{manga, tags};

        let conn = &mut self.pool()?;
        let all_manga = manga::table
            .select(Manga::as_select())
            .offset(offset as i64)
            .limit(limit as i64)
            .load(conn)?;
        let manga_list = manga::table.select(Manga::as_select()).load(conn)?;

        let all_manga_tags = MangaTags::belonging_to(&manga_list)
            .inner_join(tags::table)
            .select((MangaTags::as_select(), Tag::as_select()))
            .load(conn)?;

        Ok(all_manga_tags
            .grouped_by(&manga_list)
            .into_iter()
            .zip(all_manga)
            .map(|(tag, m)| (m, tag.into_iter().map(|(_, t)| t).collect()))
            .collect())
    }
    fn add_favourite(conn: &mut Conn, favourite: Favourite) -> Result<()> {
        use super::schema::favourites::dsl::favourites;

        diesel::replace_into(favourites)
            .values(vec![favourite])
            .execute(conn)?;
        Ok(())
    }
    fn add_tags(conn: &mut Conn, tags: Vec<Tag>, manga_id: i64) -> Result<()> {
        use super::schema::manga_tags::dsl::manga_tags;
        #[allow(unused)]
        use super::schema::tags::dsl::{id as id_col, tags as tags_table};

        // assuming that add_tags called inside transaction
        for t in tags {
            let q = diesel::insert_into(tags_table).values(&t);

            #[cfg(feature = "mysql")]
            let q = q.on_conflict(diesel::dsl::DuplicatedKeys);

            #[cfg(feature = "sqlite")]
            let q = q.on_conflict((id_col,));

            q.do_update().set(&t).execute(conn)?;

            diesel::replace_into(manga_tags)
                .values(t.to_join(manga_id))
                .execute(conn)?;
        }

        Ok(())
    }
    fn list_tags(&self, manga_id: i64) -> Result<Vec<Tag>> {
        use super::schema::{
            manga_tags::dsl::{manga_id as manga_id_col, manga_tags, tag_id as manga_tag_id_col},
            tags::dsl::{id as tags_id_col, tags},
        };

        let conn = &mut self.pool()?;
        let tag_ids: Vec<i64> = manga_tags
            .filter(manga_id_col.eq(manga_id))
            .select(manga_tag_id_col)
            .load(conn)?;
        Ok(tags
            .filter(tags_id_col.eq_any(tag_ids))
            .select(Tag::as_select())
            .load(conn)?)
    }
    fn pool(&self) -> Result<Conn> {
        self.conn.get().context("cannot get db.pool")
    }
}

// admin
impl DB {
    pub fn stats(&self) -> Result<DBStats> {
        use super::schema::manga::table as manga;
        use super::schema::users::table as users;

        let conn = &mut self.pool()?;
        let users_count: i64 = users.count().get_result(conn)?;
        let manga_count: i64 = manga.count().get_result(conn)?;
        Ok(DBStats {
            users_count: users_count as u32,
            manga_count: manga_count as u64,
        })
    }
}

fn migrate(conn: &mut impl MigrationHarness<Backend>) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("failed to run migrations: {e}"))?;
    Ok(())
}
