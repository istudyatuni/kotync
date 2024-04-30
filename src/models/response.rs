use serde::Serialize;

#[cfg(test)]
use serde::Deserialize;

use super::common::UserID;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct Me {
    pub id: UserID,
    pub email: String,
    pub nickname: Option<String>,
}
