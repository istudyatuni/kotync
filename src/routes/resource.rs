use anyhow::Result;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{
    current_timestamp,
    db::conn::DB,
    models::common,
    request::{ApiToken, AuthError},
};

use super::{user_by_token, Response, ResponseData};

#[post("/favourites", data = "<req>")]
pub fn save_favourites(
    req: Json<common::FavouritesPackage>,
    token: Result<ApiToken, AuthError>,
    db: &State<DB>,
) -> Response<Json<common::FavouritesPackage>> {
    let user = user_by_token(token, db)?;

    db.add_favourites_package(&req.0, user.id).map_err(|e| {
        log::error!("failed to add favourites package: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;

    db.set_favouries_synchronized(user.id, current_timestamp().unwrap_or_default())
        .map_err(|e| {
            log::error!(
                "failed to set favourites_sync_timestamp for user {}: {e}",
                user.id
            );
            ResponseData::Status(Status::InternalServerError)
        })?;

    let data = db.load_favourites_package(user.id).map_err(|e| {
        log::error!(
            "failed to load favourites_package for user {}: {e}",
            user.id
        );
        ResponseData::Status(Status::InternalServerError)
    })?;

    match req.0 == data {
        // is this real usecase?
        true => Ok(ResponseData::Status(Status::NoContent)),
        false => Ok(ResponseData::Body(Json(data))),
    }
}

#[get("/favourites")]
pub fn get_favourites(
    token: Result<ApiToken, AuthError>,
    db: &State<DB>,
) -> Response<Json<common::FavouritesPackage>> {
    let user = user_by_token(token, db)?;
    let data = db.load_favourites_package(user.id).map_err(|e| {
        log::error!(
            "failed to load favourites_package for user {}: {e}",
            user.id
        );
        ResponseData::Status(Status::InternalServerError)
    })?;
    Ok(ResponseData::Body(Json(data)))
}