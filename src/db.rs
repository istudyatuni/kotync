use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::models::db::User;

pub struct DB {
    conn: Pool<ConnectionManager<SqliteConnection>>,
}

impl DB {
    pub fn new(db_url: &str) -> Result<Self> {
        let pool = Pool::builder()
            .max_size(16)
            .build(ConnectionManager::<SqliteConnection>::new(db_url))?;
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
    fn pool(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.conn.get().context("cannot get db.pool")
    }
}
