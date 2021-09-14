use crate::controller::auth_controller;
use crate::db::Pool;
use rocket::{
    http::uri::Segments,
    get,
    ignite,
    response::Redirect,
    routes,
    Rocket
};

use rocket_contrib::serve::{ Options, StaticFiles };

pub fn configure(pool: Pool) -> Rocket {
    let files = StaticFiles::new("static", Options::DotFiles).rank(2);
    ignite()
        .manage(pool)
        .mount("/api", routes![]) //TODO: implement guard for authorization
        .mount("/auth", routes![auth_controller::login]) //TODO: implement authentication
        .mount("/", files) //TODO: add route for serving files the index.html needs
        .mount("/", routes![index]) //TODO: add route for serving files the index.html needs
        .mount("/app", routes![index, app_page])
        
}

#[get("/", rank = 3)]
fn index() -> Redirect {
    Redirect::to("/app/main")
}

#[get("/<_path..>", rank = 4)]
fn app_page(_path: Segments) {
    unimplemented!();
}
