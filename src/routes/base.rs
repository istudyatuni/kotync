use anyhow::Result;
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::{
    config::Conf,
    db::DB,
    jwt,
    models::request,
    models::response,
    request::{ApiToken, AuthError},
};

use super::Response;

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
    let req = req.parse().map_err(|e| (Status::BadRequest, e))?;

    log::debug!("getting user");
    let user = db.get_user(&req.email).map_err(|e| {
        log::error!("failed to get user: {e}");
        (Status::InternalServerError, "")
    })?;
    let user = match user {
        Some(u) => u,
        None => {
            if !config.allow_new_register {
                return Err((Status::Forbidden, "registration of new users is disabled"));
            }
            log::debug!("creating user");
            db.create_user(&req.email, &req.password).map_err(|e| {
                log::error!("failed to save user: {e}");
                (Status::InternalServerError, "")
            })?
        }
    };

    let token = jwt::generate(user.id, &config.jwt).map_err(|e| {
        log::error!("failed to generate jwt: {e}");
        (Status::InternalServerError, "failed to generate token")
    })?;
    Ok(Json(response::Auth { token }))
}

#[get("/me")]
pub fn me(token: Result<ApiToken, AuthError>, db: &State<DB>) -> Response<Json<response::Me>> {
    let token = token.map_err(|e| (Status::Unauthorized, e.to_string()))?;
    let user = db
        .get_user_by_id(token.user_id)
        .map_err(|e| {
            log::error!("failed to select user: {e}");
            (Status::InternalServerError, "".to_string())
        })?
        .ok_or((Status::InternalServerError, "user not found".into()))?;

    Ok(Json(response::Me {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    }))
}
