use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Me {
    pub id: i32,
    pub email: String,
    pub nickname: Option<String>,
}
