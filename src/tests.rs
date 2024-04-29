//! API tests

use anyhow::Result;
use rocket::{
    http::{Header, Status},
    uri,
};

use crate::{
    models::{request, response},
    routes,
};

use utils::*;

#[test]
fn test_root() -> Result<()> {
    let client = prepare_client()?;
    let resp = client.get(uri!(routes::base::root)).dispatch();

    assert_eq!(resp.status(), Status::Ok);
    assert_eq!(resp.into_string().unwrap(), "Alive");

    Ok(())
}

#[test]
fn test_auth_create_user() -> Result<()> {
    let client = prepare_client()?;
    let email = "test@example.com";

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth {
            email: email.to_string(),
            password: "test".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Auth = resp.into_json().unwrap();

    let resp = client
        .get(uri!(routes::base::me))
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", resp.token),
        ))
        .dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Me = resp.into_json().unwrap();
    assert_eq!(resp.email, email);

    Ok(())
}

#[test]
fn test_auth_get_token() -> Result<()> {
    let client = prepare_client()?;
    let email = "test@example.com";

    let req = client.post(uri!(routes::base::auth)).json(&request::Auth {
        email: email.to_string(),
        password: "test".to_string(),
    });
    req.clone().dispatch();
    let resp = req.dispatch();

    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Auth = resp.into_json().unwrap();

    let resp = client
        .get(uri!(routes::base::me))
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", resp.token),
        ))
        .dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Me = resp.into_json().unwrap();
    assert_eq!(resp.email, email);

    Ok(())
}

#[test]
fn test_auth_disabled() -> Result<()> {
    let client = prepare_client_with_conf(false)?;

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth {
            email: "test@example.com".to_string(),
            password: "test".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::Forbidden);
    assert_eq!(
        resp.into_string().unwrap(),
        "registration of new users is disabled"
    );

    Ok(())
}

mod utils {
    use std::{
        ops::Range,
        sync::{Mutex, OnceLock},
    };

    use anyhow::Result;
    use rocket::local::blocking::Client;

    use crate::{
        config::{Conf, ConfDB, ConfJWT},
        rocket,
    };

    const TEST_DB_COUNT: usize = 100;

    static COUNTER: OnceLock<Mutex<Range<usize>>> = OnceLock::new();

    pub fn prepare_client() -> Result<Client> {
        prepare_client_with_conf(true)
    }

    pub fn prepare_client_with_conf(allow_new_register: bool) -> Result<Client> {
        let id = COUNTER
            .get_or_init(|| Mutex::new(IntoIterator::into_iter(0..TEST_DB_COUNT)))
            .lock()
            .unwrap()
            .next()
            .unwrap();
        let db_url = format!("target/data{id}.db");
        match std::fs::remove_file(&db_url) {
            Ok(()) => (),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => return Err(e.into()),
            },
        }

        let config = Conf {
            db: ConfDB { url: db_url },
            jwt: ConfJWT {
                secret: "test".to_string(),
                issuer: "http://example.com".to_string(),
                audience: "http://example.com/resource".to_string(),
            },
            allow_new_register,
        };
        Ok(Client::tracked(rocket(config)?)?)
    }
}
