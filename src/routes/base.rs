use anyhow::Result;
use rocket::{get, http::Status, post, response::status::Custom, serde::json::Json, State};

use crate::{
    config::Conf,
    db::conn::DB,
    jwt,
    models::response,
    models::{common, request},
    request::{ApiToken, AuthError},
};

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
        Some(u) => u,
        None => {
            if !config.allow_new_register {
                return Err(ResponseData::StatusMessage(Custom(
                    Status::Forbidden,
                    "registration of new users is disabled",
                )));
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
    Ok(ResponseData::Body(Json(response::Auth { token })))
}

#[get("/me")]
pub fn me(token: Result<ApiToken, AuthError>, db: &State<DB>) -> Response<Json<response::Me>> {
    let user = user_by_token(token, db)?;
    Ok(ResponseData::Body(Json(response::Me {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    })))
}

#[get("/manga/<id>")]
pub fn get_manga(id: i64, db: &State<DB>) -> Response<Option<Json<common::Manga>>> {
    let manga = db.get_manga(id).map_err(|e| {
        log::error!("failed to get manga {id}: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;
    Ok(ResponseData::Body(manga.map(|(manga, tags)| {
        Json(manga.to_api(tags.iter().map(|t| t.to_api()).collect()))
    })))
}

#[get("/manga?<offset>&<limit>")]
pub fn list_manga(
    offset: Option<usize>,
    limit: Option<usize>,
    db: &State<DB>,
) -> Response<Json<Vec<common::Manga>>> {
    let Some(offset) = offset else {
        return Err(ResponseData::StatusMessage(Custom(
            Status::BadRequest,
            "offset is required".to_string(),
        )));
    };
    let Some(limit) = limit else {
        return Err(ResponseData::StatusMessage(Custom(
            Status::BadRequest,
            "limit is required".to_string(),
        )));
    };
    if limit > 1000 {
        return Err(ResponseData::StatusMessage(Custom(
            Status::BadRequest,
            "max limit is 1000".to_string(),
        )));
    }

    let list = db
        .list_manga(offset, limit)
        .map_err(|e| {
            log::error!("failed to list manga: {e}");
            ResponseData::Status(Status::InternalServerError)
        })?
        .into_iter()
        .map(|(manga, tags)| manga.to_api(tags.into_iter().map(|t| t.to_api()).collect()))
        .collect();
    Ok(ResponseData::Body(Json(list)))
}
