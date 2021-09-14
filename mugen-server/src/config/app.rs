use std::path::{Path, PathBuf};

use crate::{
    config::db::{run_migrations, DbPool},
    controller::auth_controller,
};
use rocket::{
    build,
    fairing::AdHoc,
    fs::{FileServer, NamedFile},
    get,
    response::Redirect,
    routes, Rocket,
};

#[get("/", rank = 1)]
fn index() -> Redirect {
    // NOTE: Redirect everything that does not match a route to the entrypoint of the application
    Redirect::to("/app/main")
}

#[get("/<_..>", rank = 2)]
async fn app_page() -> Option<NamedFile> {

    // NOTE: We only need to serve the index.html as Yew has its own routing logic
    let mut path: PathBuf = Path::new("static").into(); //TODO: get relativ or absolute path from Rocket config
    if !path.is_dir() {
        return None;
    }
    path.push("index.html");

    NamedFile::open(path).await.ok()
}

pub fn configure() -> Rocket<rocket::Build> {
    build()
        .attach(DbPool::fairing())
        .attach(AdHoc::on_ignite(
            "Running Diesel Migrations",
            run_migrations,
        ))
        .mount("/api", routes![index]) //TODO: implement guard for authorization
        .mount("/auth", routes![
            auth_controller::login, //TODO: implement authentication
            auth_controller::signup,
            index])
        .mount("/", FileServer::from("static"))
        .mount("/", routes![index])
        .mount("/app", routes![index, app_page])
}
