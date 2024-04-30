use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::{jwt::validate, models::common::UserID};

/// When added to request handler arguments, performs validation of Bearer token
///
/// ```
/// #[get("/me")]
/// pub fn me(token: ApiToken) {}
/// ```
///
/// Error could be retrived:
///
/// ```
/// #[get("/me")]
/// pub fn me(token: Result<ApiToken, AuthError>) {}
/// ```
#[derive(Debug)]
pub struct ApiToken {
    pub user_id: UserID,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("authorization missing")]
    MissingAuthorization,
    #[error("invalid token")]
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
