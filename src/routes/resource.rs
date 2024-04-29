#![allow(unused)]

use anyhow::Result;
use rocket::{post, State};

use crate::{db::conn::DB, request::ApiToken};

#[post("/favourites")]
pub fn save_favourites(token: ApiToken, db: &State<DB>) -> Result<(), ()> {
    unimplemented!()
}
