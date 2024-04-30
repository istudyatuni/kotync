use anyhow::{anyhow, Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

#[cfg(feature = "sqlite")]
use diesel::{prelude::SqliteConnection as DbConnection, sqlite::Sqlite as Backend};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::models::{
    common::{FavouritesPackage, Time, UserID},
    db::{Category, Favourite, Manga, Tag, User},
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type ConnManager = ConnectionManager<DbConnection>;
type Conn = PooledConnection<ConnManager>;

pub struct DB {
    conn: Pool<ConnManager>,
}

impl DB {
    pub fn new(db_url: &str) -> Result<Self> {
        migrate(&mut DbConnection::establish(db_url)?)?;

        let pool = Pool::builder()
            .max_size(16)
            .build(ConnectionManager::new(db_url))?;
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
    pub fn create_user(&self, email: &str, password: &str) -> Result<User> {
        use super::schema::users::dsl::users;

        // assuming that this user doesn't exists
        diesel::insert_into(users)
            .values(User {
                email: email.to_string(),
                password: password.to_string(),
                ..Default::default()
            })
            .execute(&mut self.pool()?)?;

        self.get_user_by_email(email).map(|u| u.unwrap())
    }
    pub fn set_favouries_synchronized(&self, user_id: UserID, time: Time) -> Result<()> {
        use super::schema::users::dsl::{favourites_sync_timestamp, id, users};

        diesel::update(users)
            .filter(id.eq(user_id))
            .set(favourites_sync_timestamp.eq(time))
            .execute(&mut self.pool()?)?;

        Ok(())
    }
    pub fn add_favourites_package(&self, pkg: &FavouritesPackage, user_id: UserID) -> Result<()> {
        let conn = &mut self.pool()?;
        conn.transaction::<_, anyhow::Error, _>(|conn| {
            for c in &pkg.categories {
                Self::add_category(conn, c.to_db(user_id))?;
            }
            for f in &pkg.favourites {
                Self::add_manga(conn, f.manga.to_db())?;
                Self::add_tags(
                    conn,
                    f.manga.tags.iter().map(|t| t.to_db()).collect(),
                    f.manga_id,
                )?;
                Self::add_favourite(conn, f.to_db(user_id))?;
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
    fn list_favourites(&self, user_id: UserID) -> Result<Vec<(Favourite, Manga)>> {
        use super::schema::favourites::dsl::user_id as user_id_col;
        use super::schema::{favourites, manga};

        Ok(favourites::table
            .inner_join(manga::table)
            .filter(user_id_col.eq(user_id))
            .select((Favourite::as_select(), Manga::as_select()))
            .load(&mut self.pool()?)?)
    }
    fn add_category(conn: &mut Conn, category: Category) -> Result<()> {
        use super::schema::categories::dsl::categories;

        diesel::replace_into(categories)
            .values(vec![category])
            .execute(conn)?;

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
        use super::schema::manga::dsl::manga as manga_table;

        diesel::replace_into(manga_table)
            .values(vec![manga])
            .execute(conn)?;

        Ok(())
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
        use super::schema::tags::dsl::tags as tags_table;

        diesel::replace_into(tags_table)
            .values(&tags)
            .execute(conn)?;
        let tags: Vec<_> = tags.iter().map(|t| t.to_join(manga_id)).collect();
        diesel::replace_into(manga_tags)
            .values(tags)
            .execute(conn)?;

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

fn migrate(conn: &mut impl MigrationHarness<Backend>) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("failed to run migrations: {e}"))?;
    Ok(())
}
