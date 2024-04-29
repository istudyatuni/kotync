use rocket::http::Status;

pub mod base;
pub mod resource;

pub type Response<T, E = String> = Result<T, (Status, E)>;
