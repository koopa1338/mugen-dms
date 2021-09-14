#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod controller;
mod error;
mod models;
mod services;

use chrono::Local;
use config::app;
use dotenv::dotenv;
use fern::{log_file, Dispatch};
use log::LevelFilter;
use rocket::{Build, Error as RocketError, Rocket};

fn setup_logging() {
    //TODO: Errorhandling to remove unwraps
    let base_logging = Dispatch::new();
    let rocket_logging = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}]\t{}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                message
            ))
        })
        .level_for("rocket", LevelFilter::Info)
        .chain(log_file("mugen.log").unwrap());

    let stdout_logging = Dispatch::new()
        .format(|out, message, record| {
            if record.target().ends_with("_") {
                out.finish(format_args!("\t{}", message));
            } else {
                out.finish(format_args!("{}", message));
            }
        })
        .level_for("rocket", LevelFilter::Info)
        .chain(std::io::stdout());

    base_logging
        .chain(rocket_logging)
        .chain(stdout_logging)
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), RocketError> {
    dotenv().ok();

    // Configure logger at runtime
    setup_logging();

    let app: Rocket<Build> = app::configure();

    app.launch().await
}
