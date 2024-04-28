use anyhow::Result;
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::config::ConfJWT;

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
