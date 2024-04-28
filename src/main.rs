use std::path::PathBuf;

use anyhow::{Context, Result};
use config::Conf;
use confique::Config;
use db::DB;
use log::LevelFilter;
use rocket::routes;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

mod config;
mod db;
mod jwt;
mod models;
mod request;
mod response;
mod routes;
mod schema;

#[rocket::main]
async fn main() -> Result<()> {
    init_logger()?;
    dotenv()?;

    let config = Conf::builder().env().file("config.toml").load()?;

    log::info!("loaded config: {config:#?}");

    let db = DB::new(&config.db.url)?;

    let _rocket = rocket::build()
        .manage(config)
        .manage(db)
        .mount("/", routes![routes::root, routes::auth])
        .launch()
        .await?;

    Ok(())
}

fn dotenv() -> Result<()> {
    let path = PathBuf::from(".env");
    if !path.exists() {
        log::warn!(".env file not found, using environment variables");
        return Ok(());
    }

    dotenvy::from_path(path).context("failed to load .env")?;

    Ok(())
}

fn init_logger() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().set_time_format_rfc3339().build(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    Ok(())
}
