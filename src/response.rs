use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Auth {
    pub token: String,
}
