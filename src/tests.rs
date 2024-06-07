//! API tests

use anyhow::Result;
use rocket::{
    http::{hyper::header::AUTHORIZATION, uri::Origin, Header, Status},
    uri,
};

use crate::{
    current_timestamp,
    models::{common, request, response},
    routes,
};

use utils::*;

static RESOURCE: Origin<'static> = uri!("/resource");

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
            AUTHORIZATION.as_str(),
            format!("Bearer {}", resp.token),
        ))
        .dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let resp: response::Me = resp.into_json().unwrap();
    assert_eq!(resp.email, email);

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth {
            email: "test2@example.com".to_string(),
            password: "test".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

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

#[test]
fn test_auth_invalid_password() -> Result<()> {
    let client = prepare_client()?;
    let email = "test@example.com".to_string();

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth {
            email: email.clone(),
            password: "test".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let resp = client
        .post(uri!(routes::base::auth))
        .json(&request::Auth {
            email,
            password: "adsf".to_string(),
        })
        .dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
    assert_eq!(resp.into_string().unwrap(), "Wrong password".to_string());

    Ok(())
}

#[test]
fn test_sync_favourites() -> Result<()> {
    let client = prepare_client()?;
    let auth = make_user(&client);

    let data = data::favourites_package();

    let resp = client
        .post(uri!(RESOURCE.clone(), routes::resource::save_favourites))
        .json(&data)
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let resp = client
        .get(uri!(RESOURCE.clone(), routes::resource::get_favourites))
        .header(Header::new(AUTHORIZATION.as_str(), auth))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let mut data = data;
    let sent_timestamp = data.timestamp.take();

    let mut resp: common::FavouritesPackage = resp.into_json().unwrap();
    let timestamp = resp.timestamp.take();

    similar_asserts::assert_eq!(data, resp);
    assert!(timestamp < current_timestamp());
    if let Some(sent_timestamp) = sent_timestamp {
        assert!(timestamp.is_some_and(|t| t > sent_timestamp));
    } else {
        assert!(timestamp.is_some());
    }

    Ok(())
}

#[test]
fn test_sync_history() -> Result<()> {
    let client = prepare_client()?;
    let auth = make_user(&client);

    let data = data::history_package();

    let resp = client
        .post(uri!(RESOURCE.clone(), routes::resource::save_history))
        .json(&data)
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let resp = client
        .get(uri!(RESOURCE.clone(), routes::resource::get_history))
        .header(Header::new(AUTHORIZATION.as_str(), auth))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let mut data = data;
    let sent_timestamp = data.timestamp.take();

    let mut resp: common::HistoryPackage = resp.into_json().unwrap();
    let timestamp = resp.timestamp.take();

    assert_eq!(data, resp);
    assert!(timestamp < current_timestamp());
    if let Some(sent_timestamp) = sent_timestamp {
        assert!(timestamp.is_some_and(|t| t > sent_timestamp));
    } else {
        assert!(timestamp.is_some());
    }

    Ok(())
}

#[test]
fn test_list_manga_invalid_params() -> Result<()> {
    let client = prepare_client()?;
    let auth = make_user(&client);

    let req = client
        .get("/manga")
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()));

    let resp = req.clone().dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
    assert_eq!(resp.into_string().unwrap(), "offset is required");

    let mut req = req;
    req.set_uri(uri!("/manga?offset=0"));
    let resp = req.clone().dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
    assert_eq!(resp.into_string().unwrap(), "limit is required");

    req.set_uri(uri!("/manga?offset=0&limit=10000"));
    let resp = req.clone().dispatch();

    assert_eq!(resp.status(), Status::BadRequest);
    assert_eq!(resp.into_string().unwrap(), "max limit is 1000");

    Ok(())
}

#[test]
fn test_list_manga() -> Result<()> {
    let client = prepare_client()?;
    let auth = make_user(&client);

    // prepare

    let req = client
        .post(uri!(RESOURCE.clone(), routes::resource::save_favourites))
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()));

    let data = data::favourites_package();
    let resp = req.clone().json(&data).dispatch();
    assert_eq!(resp.status(), Status::Ok);

    let new_manga_id = 2;

    let mut data = data;
    data.favourites[0].manga_id = new_manga_id;
    data.favourites[0].manga.id = new_manga_id;
    let resp = req.clone().json(&data).dispatch();
    assert_eq!(resp.status(), Status::Ok);

    // check

    let req = client
        .get(uri!(routes::base::list_manga(Some(0), Some(1))))
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()));

    let resp = req.clone().dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let resp: Vec<common::Manga> = resp.into_json().unwrap();
    assert_eq!(resp.len(), 1);

    let mut manga = data::manga();
    similar_asserts::assert_eq!(resp[0], manga);

    let mut req = req;
    req.set_uri(uri!(routes::base::list_manga(Some(1), Some(2))));
    let resp = req.clone().dispatch();
    assert_eq!(resp.status(), Status::Ok);

    let resp: Vec<common::Manga> = resp.into_json().unwrap();
    assert_eq!(resp.len(), 1);

    manga.id = new_manga_id;
    similar_asserts::assert_eq!(resp[0], manga);

    Ok(())
}

#[test]
fn test_get_manga() -> Result<()> {
    let client = prepare_client()?;
    let auth = make_user(&client);

    let data = data::favourites_package();

    let resp = client
        .post(uri!(RESOURCE.clone(), routes::resource::save_favourites))
        .json(&data)
        .header(Header::new(AUTHORIZATION.as_str(), auth.clone()))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let manga = &data.favourites[0].manga;

    let resp = client
        .get(uri!(routes::base::get_manga(id = manga.id)))
        .header(Header::new(AUTHORIZATION.as_str(), auth))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let resp: common::Manga = resp.into_json().unwrap();
    similar_asserts::assert_eq!(resp, *manga);

    Ok(())
}

mod data {
    use crate::{
        current_timestamp,
        models::{common, common::MangaState},
    };

    pub fn favourites_package() -> common::FavouritesPackage {
        let now = current_timestamp().unwrap();
        common::FavouritesPackage {
            categories: vec![common::Category {
                id: 1,
                created_at: now,
                sort_key: 0,
                track: 0,
                title: "test".to_string(),
                order: "NEWEST".to_string(),
                deleted_at: 0,
                show_in_lib: 1,
            }],
            favourites: vec![common::Favourite {
                manga_id: 1,
                manga: manga(),
                category_id: 1,
                sort_key: 1,
                created_at: now,
                deleted_at: now,
            }],
            timestamp: Some(now),
        }
    }
    pub fn history_package() -> common::HistoryPackage {
        let now = current_timestamp().unwrap();
        common::HistoryPackage {
            history: vec![common::History {
                manga_id: 1,
                manga: manga(),
                created_at: now,
                updated_at: now,
                chapter_id: 1,
                page: 1,
                scroll: 2.3,
                percent: 1.2,
                chapters: 34,
                deleted_at: 0,
            }],
            timestamp: Some(now),
        }
    }
    pub fn manga() -> common::Manga {
        common::Manga {
            id: 1,
            title: "test".to_string(),
            alt_title: None,
            url: "kotatsu://test".to_string(),
            public_url: "http://example.com/test".to_string(),
            rating: 2.3,
            is_nsfw: 0,
            cover_url: "http://example.com/cover".to_string(),
            large_cover_url: None,
            tags: vec![
                common::MangaTag {
                    id: 1,
                    title: "Test".to_string(),
                    key: "test".to_string(),
                    source: "source".to_string(),
                },
                common::MangaTag {
                    id: 2,
                    title: "Test 2".to_string(),
                    key: "test2".to_string(),
                    source: "source".to_string(),
                },
            ],
            state: Some(MangaState::Finished),
            author: Some("Author".to_string()),
            source: "source".to_string(),
        }
    }
}

mod utils {
    use anyhow::Result;
    use rocket::{http::Status, local::blocking::Client, uri};

    use crate::{
        config::{Conf, ConfDB, ConfJWT},
        models::{request, response},
        rocket, routes,
    };

    pub fn prepare_client() -> Result<Client> {
        prepare_client_with_conf(true)
    }

    #[cfg(feature = "sqlite")]
    fn get_db_conf() -> Result<ConfDB> {
        use std::{
            ops::Range,
            sync::{Mutex, OnceLock},
        };

        const TEST_DB_COUNT: usize = 100;
        static COUNTER: OnceLock<Mutex<Range<usize>>> = OnceLock::new();

        let id = COUNTER
            .get_or_init(|| Mutex::new(IntoIterator::into_iter(0..TEST_DB_COUNT)))
            .lock()
            .unwrap()
            .next()
            .unwrap();
        let db_url = format!("target/data{id}.db");
        match std::fs::remove_file(&db_url) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.into()),
        }
        Ok(ConfDB { url: db_url })
    }

    #[cfg(feature = "mysql")]
    fn get_db_conf() -> Result<ConfDB> {
        Ok(ConfDB {
            name: "kotatsu_db_test".to_string(),
            host: "0.0.0.0".to_string(),
            port: 3307,
            user: "root".to_string(),
            password: "".to_string(),
        })
    }

    pub fn prepare_client_with_conf(allow_new_register: bool) -> Result<Client> {
        let config = Conf {
            db: get_db_conf()?,
            jwt: ConfJWT {
                secret: "test".to_string(),
                issuer: "http://example.com".to_string(),
                audience: "http://example.com/resource".to_string(),
            },
            allow_new_register,
        };
        Ok(Client::untracked(rocket(config)?)?)
    }

    /// Returns "Bearer {token}"
    pub fn make_user(client: &Client) -> String {
        let resp = client
            .post(uri!(routes::base::auth))
            .json(&request::Auth {
                email: "test@example.com".to_string(),
                password: "test".to_string(),
            })
            .dispatch();
        assert_eq!(resp.status(), Status::Ok, "failed to make_user");
        let resp: response::Auth = resp.into_json().unwrap();

        format!("Bearer {}", resp.token)
    }
}
