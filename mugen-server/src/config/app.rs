use crate::controller::auth_controller;
use crate::db::Pool;
use rocket::{ignite, routes, Rocket};
use rocket_contrib::serve::StaticFiles;

pub fn configure(pool: Pool) -> Rocket {
    ignite()
        .manage(pool)
        .mount("/api", routes![]) //TODO: implement guard for authorization
        .mount("/auth", routes![auth_controller::login]) //TODO: implement authentication
        .mount("/", StaticFiles::from("static")) //TODO: get path from rocket configuration toml
}
