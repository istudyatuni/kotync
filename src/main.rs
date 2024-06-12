use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
    time::SystemTime,
};

use anyhow::{anyhow, Context, Result};
use confique::Config;
use log::LevelFilter;
use rocket::{routes, Build, Rocket};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use config::Conf;
use db::conn::DB;
use models::common::Time;

mod config;
mod db;
mod jwt;
mod models;
mod request;
mod routes;

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
    log::info!("loaded config\n{config}");

    let db = DB::new(config.db.clone())?;
    rocket(config, db)?.launch().await?;

    Ok(())
}

// passing these arguments here to be able to call this from tests
//
// some tests needs to reuse DB connecion
fn rocket(config: Conf, db: DB) -> Result<Rocket<Build>> {
    CONFIG.get_or_init(|| config.clone());

    let mut rocket = rocket::build()
        .configure(rocket::Config {
            port: 8080,
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .manage(config.clone())
        .manage(db)
        .mount(
            "/",
            routes![
                routes::base::root,
                routes::base::auth,
                routes::base::me,
                routes::base::get_manga,
                routes::base::list_manga,
            ],
        )
        .mount(
            "/resource",
            routes![
                routes::resource::save_favourites,
                routes::resource::get_favourites,
                routes::resource::save_history,
                routes::resource::get_history,
            ],
        );

    if let Some(admin) = &config.admin_api {
        if !admin.starts_with('/') {
            log::error!("ADMIN_API should start with /");
            return Err(anyhow!("invalid env, exiting"));
        }
        rocket = rocket.mount(admin, routes![routes::admin::stats, routes::admin::info]);
    }

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
    #[cfg(not(debug_assertions))]
    const DEFAULT: LevelFilter = LevelFilter::Error;
    #[cfg(debug_assertions)]
    const DEFAULT: LevelFilter = LevelFilter::Info;

    let level: LevelFilter = std::env::var("RUST_LOG")
        .map(|s| {
            LevelFilter::from_str(&s)
                .inspect_err(|_| eprintln!("invalid RUST_LOG"))
                .unwrap_or(DEFAULT)
        })
        .unwrap_or(DEFAULT);

    eprintln!("set log level {level}");

    TermLogger::init(
        level,
        ConfigBuilder::new().set_time_format_rfc3339().build(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    Ok(())
}

/// Current system time in milliseconds
pub fn current_timestamp() -> Option<Time> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_millis() as Time)
}
