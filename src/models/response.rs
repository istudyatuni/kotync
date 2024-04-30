use serde::Serialize;

use super::common::UserID;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Me {
    pub id: UserID,
    pub email: String,
    pub nickname: Option<String>,
}
