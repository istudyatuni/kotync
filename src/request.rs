use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::jwt::validate;

/// When added to request handler arguments, performs validation of Bearer token
#[derive(Debug)]
pub struct ApiToken {
    pub user_id: i32,
}

#[derive(Debug)]
pub enum AuthError {
    MissingAuthorization,
    InvalidToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            Some(key) => match validate(key.trim_start_matches("Bearer ")) {
                Ok(user_id) => Outcome::Success(ApiToken { user_id }),
                Err(_) => Outcome::Error((Status::Unauthorized, Self::Error::InvalidToken)),
            },
            None => Outcome::Error((Status::Unauthorized, Self::Error::MissingAuthorization)),
        }
    }
}
