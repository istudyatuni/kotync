use anyhow::Result;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::config::ConfJWT;

/// 30 days
const LIFETIME_SEC: u64 = 30 * 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub user_id: i32,
    pub exp: u64,
}

pub fn generate(user_id: i32, config: &ConfJWT) -> Result<String> {
    Ok(encode(
        &Header::default(),
        &Claims {
            aud: config.audience.clone(),
            iss: config.issuer.clone(),
            user_id,
            exp: get_current_timestamp() + LIFETIME_SEC,
        },
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )?)
}

pub fn validate(token: &str) -> Result<i32> {
    let config = crate::get_config()?;
    let mut validation = Validation::default();
    validation.set_audience(&[&config.jwt.audience]);
    validation.set_issuer(&[&config.jwt.issuer]);

    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &validation,
    )?;

    Ok(token.claims.user_id)
}
