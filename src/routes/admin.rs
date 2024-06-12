use rocket::{get, http::Status, serde::json::Json, State};

use crate::{db::conn::DB, models::admin::DBStats};

use super::{Response, ResponseData};

#[get("/stats")]
pub fn stats(db: &State<DB>) -> Response<Json<DBStats>> {
    let stats = db.stats().map_err(|e| {
        log::error!("failed to load stats: {e}");
        ResponseData::Status(Status::InternalServerError)
    })?;

    Ok(ResponseData::Body(Json(stats)))
}
