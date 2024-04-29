#![allow(unused)]

use anyhow::Result;
use rocket::{post, State};

use crate::{db::DB, request::ApiToken};

#[post("/favourites")]
pub fn save_favourites(token: ApiToken, db: &State<DB>) -> Result<(), ()> {
    unimplemented!()
}
