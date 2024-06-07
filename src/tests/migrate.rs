use anyhow::Result;

use rocket::{http::Status, uri};
use utils::*;

use crate::{
    models::request,
    routes,
    tests::e2e::utils::{get_db, prepare_client_with_db},
};

#[test]
fn test_migrate_md5() -> Result<()> {
    let email = "test@example.com";
    let password = "test";
    let (db_conf, db) = get_db()?;
    seed_md5(&db, email, password)?;

    let client = prepare_client_with_db(db_conf, db)?;

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth::new(email, password))
        .dispatch();
    assert_eq!(resp.status(), Status::Ok);

    Ok(())
}

#[test]
fn test_migrate_md5_invalid_password() -> Result<()> {
    let email = "test@example.com";
    let password = "test";
    let (db_conf, db) = get_db()?;
    seed_md5(&db, email, password)?;

    let client = prepare_client_with_db(db_conf, db)?;

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth::new(email, "asdf"))
        .dispatch();
    assert_eq!(resp.status(), Status::BadRequest);

    Ok(())
}

mod utils {
    use anyhow::Result;

    use crate::{db::conn::DB, models::request::to_md5};

    pub fn seed_md5(db: &DB, user: &str, password: &str) -> Result<()> {
        db.create_user(user, &to_md5(password))?;
        Ok(())
    }
}
