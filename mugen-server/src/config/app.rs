use crate::{
    config::db::{run_migrations, DbPool},
    controller::auth_controller,
};
use rocket::{build, fairing::AdHoc, fs::FileServer, get, response::Redirect, routes, Rocket};

pub fn configure() -> Rocket<rocket::Build> {
    build()
        .attach(DbPool::fairing())
        .attach(AdHoc::on_ignite(
            "Running Diesel Migrations",
            run_migrations,
        ))
        .mount("/api", routes![]) //TODO: implement guard for authorization
        .mount("/auth", routes![auth_controller::login]) //TODO: implement authentication
        .mount("/", FileServer::from("static")) //TODO: add route for serving files the index.html needs
        .mount("/", routes![index]) //TODO: add route for serving files the index.html needs
        .mount("/app", routes![index, app_page])
}

#[get("/", rank = 1)]
fn index() -> Redirect {
    Redirect::to("/app/main")
}

#[get("/<_..>", rank = 2)]
fn app_page() {
    unimplemented!();
}
