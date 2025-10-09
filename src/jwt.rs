use anyhow::Result;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};

use crate::{config::ConfJWT, models::common::UserID};

/// 30 days
const LIFETIME_SEC: u64 = 30 * 24 * 60 * 60;
const JWT_AUD_PREFIX: &str = "/resource";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub user_id: UserID,
    pub exp: u64,
}

pub fn generate(user_id: UserID, config: &ConfJWT) -> Result<String> {
    Ok(encode(
        &Header::default(),
        &Claims {
            aud: config.issuer.clone() + JWT_AUD_PREFIX,
            iss: config.issuer.clone(),
            user_id,
            exp: get_current_timestamp() + LIFETIME_SEC,
        },
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )?)
}

pub fn validate(token: &str) -> Result<UserID> {
    let config = crate::get_config()?;
    let mut validation = Validation::default();
    validation.set_audience(&[config.jwt.issuer.clone() + JWT_AUD_PREFIX]);
    validation.set_issuer(&[&config.jwt.issuer]);

    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &validation,
    )?;

    Ok(token.claims.user_id)
}
