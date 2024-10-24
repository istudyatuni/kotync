use anyhow::Result;
use rocket::{get, http::Status, post, response::status::Custom, serde::json::Json, State};

use crate::{
    config::Conf,
    db::conn::DB,
    jwt,
    models::{common, request, response},
    request::{ApiToken, AuthError},
};

#[cfg(feature = "migrate-md5")]
use crate::models::request::MD5_LEN;

use super::{user_by_token, Response, ResponseData};

#[get("/")]
pub fn root() -> &'static str {
    "Alive"
}

#[post("/auth", data = "<req>")]
pub fn auth(
    req: Json<request::Auth>,
    config: &State<Conf>,
    db: &State<DB>,
) -> Response<Json<response::Auth>, &'static str> {
    let req = req
        .parse()
        .map_err(|e| ResponseData::StatusMessage(Custom(Status::BadRequest, e)))?;

    log::debug!("getting user");
    let user = db.get_user_by_email(&req.email).map_err(|e| {
        log::error!("failed to get user: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;
    let user = match user {
        Some(u) if req.check_password(&u).is_err() => {
            // todo: wait for 2s (configurable)
            return Err((Status::BadRequest, "Wrong password").into())
        }
        #[cfg(feature = "migrate-md5")]
        Some(u) if u.password.len() == MD5_LEN => {
            match db.update_user_password(u.id, &req.password) {
                Ok(()) => (),
                Err(e) => log::error!("failed to update user password: {e}"),
            }
            u
        }
        Some(u) => u,
        None => {
            if !config.server.allow_new_register {
                return Err((Status::Forbidden, "registration of new users is disabled").into());
            }
            log::debug!("creating user");
            db.create_user(&req.email, &req.password).map_err(|e| {
                log::error!("failed to save user: {e}");
                ResponseData::Status(Status::InternalServerError)
            })?
        }
    };

    let token = jwt::generate(user.id, &config.jwt).map_err(|e| {
        log::error!("failed to generate jwt: {e}");
        ResponseData::StatusMessage(Custom(
            Status::InternalServerError,
            "failed to generate token",
        ))
    })?;
    Ok(Json(response::Auth { token }).into())
}

#[get("/me")]
pub fn me(token: Result<ApiToken, AuthError>, db: &State<DB>) -> Response<Json<response::Me>> {
    let user = user_by_token(token, db)?;
    Ok(Json(response::Me {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    })
    .into())
}

#[get("/manga/<id>")]
pub fn get_manga(id: i64, db: &State<DB>) -> Response<Option<Json<common::Manga>>> {
    let manga = db.get_manga(id).map_err(|e| {
        log::error!("failed to get manga {id}: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;
    Ok(manga
        .map(|(manga, tags)| Json(manga.to_api(tags.iter().map(|t| t.to_api()).collect())))
        .into())
}

#[get("/manga?<offset>&<limit>")]
pub fn list_manga(
    offset: Option<usize>,
    limit: Option<usize>,
    db: &State<DB>,
) -> Response<Json<Vec<common::Manga>>, &'static str> {
    let Some(offset) = offset else {
        return Err((Status::BadRequest, "offset is required").into());
    };
    let Some(limit) = limit else {
        return Err((Status::BadRequest, "limit is required").into());
    };
    if limit > 1000 {
        return Err((Status::BadRequest, "max limit is 1000").into());
    }

    let list: Vec<_> = db
        .list_manga(offset, limit)
        .map_err(|e| {
            log::error!("failed to list manga: {e}");
            ResponseData::Status(Status::InternalServerError)
        })?
        .into_iter()
        .map(|(manga, tags)| manga.to_api(tags.into_iter().map(|t| t.to_api()).collect()))
        .collect();
    Ok(Json(list).into())
}
