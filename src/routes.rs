use anyhow::Result;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{config::Conf, db::DB, jwt, models::request, models::response};

#[get("/")]
pub fn root() -> &'static str {
    "Alive"
}

#[post("/auth", data = "<req>")]
pub fn auth(
    req: Json<request::Auth>,
    config: &State<Conf>,
    db: &State<DB>,
) -> Result<Json<response::Auth>, (Status, Option<&'static str>)> {
    let req = req.parse().map_err(|e| (Status::BadRequest, Some(e)))?;

    log::debug!("getting user");
    let user = db.inner().get_user(&req.email).map_err(|e| {
        log::error!("failed to get user: {e}");
        (Status::InternalServerError, None)
    })?;
    let user = match user {
        Some(u) => u,
        None => {
            if !config.allow_new_register {
                return Err((
                    Status::Forbidden,
                    Some("registration of new users is disabled"),
                ));
            }
            log::debug!("creating user");
            db.create_user(&req.email, &req.password).map_err(|e| {
                log::error!("failed to save user: {e}");
                (Status::InternalServerError, None)
            })?
        }
    };

    let Some(user_id) = user.id else {
        log::error!("user.id is empty; user.email = {}", user.email);
        return Err((Status::InternalServerError, None));
    };

    let token = jwt::generate(user_id, &config.jwt).map_err(|e| {
        log::error!("failed to generate jwt: {e}");
        (
            Status::InternalServerError,
            Some("failed to generate token"),
        )
    })?;
    Ok(Json(response::Auth { token }))
}
