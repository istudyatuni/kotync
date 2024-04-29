use std::{path::PathBuf, sync::OnceLock};

use anyhow::{anyhow, Context, Result};
use confique::Config;
use log::LevelFilter;
use rocket::{routes, Build, Rocket};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use config::Conf;
use db::DB;

mod config;
mod db;
mod jwt;
mod models;
mod request;
mod routes;

#[rustfmt::skip]
mod schema;

#[cfg(test)]
mod tests;

static CONFIG: OnceLock<Conf> = OnceLock::new();

fn get_config() -> Result<&'static Conf> {
    crate::CONFIG
        .get()
        .ok_or(anyhow!("failed to get config from static"))
}

#[rocket::main]
async fn main() -> Result<()> {
    init_logger()?;
    dotenv()?;

    let config = Conf::builder().env().file("config.toml").load()?;
    log::info!("loaded config: {config:#?}");
    rocket(config)?.launch().await?;

    Ok(())
}

fn rocket(config: Conf) -> Result<Rocket<Build>> {
    let db = DB::new(&config.db.url)?;

    // get_or_init because of tests
    CONFIG.get_or_init(|| config.clone());

    let rocket = rocket::build()
        .manage(config)
        .manage(db)
        .mount(
            "/",
            routes![routes::base::root, routes::base::auth, routes::base::me],
        )
        .mount("/resource", routes![routes::resource::save_favourites]);
    Ok(rocket)
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
