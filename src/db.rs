use anyhow::{anyhow, Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

#[cfg(feature = "sqlite")]
use diesel::{prelude::SqliteConnection as DbConnection, sqlite::Sqlite as Backend};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::models::db::User;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DB {
    conn: Pool<ConnectionManager<DbConnection>>,
}

impl DB {
    pub fn new(db_url: &str) -> Result<Self> {
        let mut conn = DbConnection::establish(db_url)?;
        migrate(&mut conn)?;

        let pool = Pool::builder()
            .max_size(16)
            .build(ConnectionManager::<DbConnection>::new(db_url))?;
        Ok(Self { conn: pool })
    }
    pub fn get_user(&self, email: &str) -> Result<Option<User>> {
        use crate::schema::users::dsl::{email as user_email, users};

        Ok(users
            .filter(user_email.eq(email))
            .select(User::as_select())
            .first(&mut self.pool()?)
            .optional()?)
    }
    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        use crate::schema::users::dsl::users;

        Ok(users
            .find(user_id)
            .select(User::as_select())
            .first(&mut self.pool()?)
            .optional()?)
    }
    pub fn create_user(&self, email: &str, password: &str) -> Result<User> {
        use crate::schema::users::dsl::users;

        diesel::insert_into(users)
            .values(User {
                email: email.to_string(),
                password: password.to_string(),
                ..Default::default()
            })
            .execute(&mut self.pool()?)?;

        self.get_user(email).map(|u| u.unwrap())
    }
    fn pool(&self) -> Result<PooledConnection<ConnectionManager<DbConnection>>> {
        self.conn.get().context("cannot get db.pool")
    }
}

fn migrate(conn: &mut impl MigrationHarness<Backend>) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("failed to run migrations: {e}"))?;
    Ok(())
}
