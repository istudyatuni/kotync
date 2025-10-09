use rocket::{State, get, http::Status, serde::json::Json};

use crate::{db::conn::DB, models::admin};

use super::{Response, ResponseData};

const SERVER_VERSION: &str = env!("VERSION");

#[get("/stats")]
pub fn stats(db: &State<DB>) -> Response<Json<admin::DBStats>> {
    let stats = db.stats().map_err(|e| {
        log::error!("failed to load stats: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;

    Ok(Json(stats).into())
}

#[get("/info")]
pub fn info() -> Json<admin::ServerInfo> {
    Json(admin::ServerInfo {
        server_version: SERVER_VERSION.to_string(),
    })
}
