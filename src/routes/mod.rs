use rocket::{http::Status, response::status::Custom, Responder, State};

use crate::{
    db::conn::DB,
    models::db::User,
    request::{ApiToken, AuthError},
};

pub mod base;
pub mod resource;

pub type Response<T, E = String> = Result<ResponseData<T>, ResponseData<E>>;
pub type ResponseErr<T, E = String> = Result<T, ResponseData<E>>;

#[derive(Debug, Responder)]
pub enum ResponseData<R> {
    Body(R),
    Status(Status),
    StatusMessage(Custom<R>),
}

fn user_by_token(token: Result<ApiToken, AuthError>, db: &State<DB>) -> ResponseErr<User> {
    let token = token
        .map_err(|e| ResponseData::StatusMessage(Custom(Status::Unauthorized, e.to_string())))?;
    db.get_user(token.user_id)
        .map_err(|e| {
            log::error!("failed to select user: {e}");
            ResponseData::Status(Status::InternalServerError)
        })?
        .ok_or(ResponseData::StatusMessage(Custom(
            Status::InternalServerError,
            "user not found".into(),
        )))
}
