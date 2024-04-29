use anyhow::Result;
use rocket::{
    http::{Header, Status},
    local::blocking::Client,
    uri,
};

use crate::{
    config::{Conf, ConfDB, ConfJWT},
    models::{request, response},
    rocket, routes,
};

fn prepare_client(id: usize) -> Result<Client> {
    let db_url = format!("target/data{id}.db");
    let _ = std::fs::remove_file(&db_url);
    let config = Conf {
        db: ConfDB { url: db_url },
        jwt: ConfJWT {
            secret: "test".to_string(),
            issuer: "http://test.com".to_string(),
            audience: "http://test.com/resource".to_string(),
        },
        allow_new_register: true,
    };
    Ok(Client::tracked(rocket(config)?)?)
}

#[test]
fn test_root() -> Result<()> {
    let client = prepare_client(1)?;
    let resp = client.get(uri!(routes::root)).dispatch();

    assert_eq!(resp.status(), Status::Ok);
    assert_eq!(resp.into_string().unwrap(), "Alive");

    Ok(())
}

#[test]
fn test_auth() -> Result<()> {
    let client = prepare_client(2)?;
    let email = "test@example.com";

    let resp = client
        .post(uri!(routes::auth))
        .json(&request::Auth {
            email: email.to_string(),
            password: "test".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Auth = resp.into_json().unwrap();
    let token = format!("Bearer {}", resp.token);

    let resp = client
        .get(uri!(routes::me))
        .header(Header::new("Authorization", token))
        .dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Me = resp.into_json().unwrap();
    assert_eq!(resp.email, email);

    Ok(())
}
