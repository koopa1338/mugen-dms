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

/*
#[derive(Debug, Clone)]
pub struct AppConfig {
    ui: PathBuf,
    pages: PathBuf,
    slow: bool,
    port: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    config: AppConfig,
}

async fn index() -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, "/app/main")
        .finish()
}


async fn app_page(state: web::Data<AppState>) -> WebResult<fs::NamedFile> {
    Ok(fs::NamedFile::open(state.config.pages.join("index.html"))?)
}
*/

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let app: Rocket<Build> = app::configure();
    app.launch().await

    // TODO: rocket ignite with handler
    /*
    HttpServer::new(move || {
        let app_state = AppState {
            config: config.clone(),
        };

        let slow = config.slow;
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .data(pool.clone())
            .app_data(web::PayloadConfig::default().limit(1024 * 1024 * 500))
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 500))
            .wrap_fn(move |req, srv| {
                if slow {
                    std::thread::sleep(Duration::from_millis(100));
                }
                srv.call(req)
            })
            .data(app_state)
            .service(fs::Files::new("/ui", &config.ui))
            .service(fs::Files::new("/static", &config.pages.join("static")))
            .route("/", web::get().to(index))
            .route("/app{_:/?}", web::get().to(index))
            .route("/app/{app:[a-zA-z0-9_\\-/]+}", web::get().to(app_page))
            .configure(config::app::config_services)
    })
    .bind(&addr)?
    .run()
    .await?;
    */
}
