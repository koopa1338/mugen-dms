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

use config::app;
use dotenv::dotenv;
use rocket::{Build, Rocket};

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let app: Rocket<Build> = app::configure();

    app.launch().await
}
