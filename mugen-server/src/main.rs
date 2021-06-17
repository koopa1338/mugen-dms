#[macro_use]
extern crate log;

mod config;
mod controller;
mod services;
mod error;

use actix_files as fs;
use actix_web::{dev::Service, http, web, App, HttpResponse, HttpServer, Result as WebResult};
use clap::Arg;
use config::db;
use std::path::PathBuf;
use std::time::Duration;


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

fn get_config() -> AppConfig {
    let app = clap::App::new("mugen-server")
        .about("Mugen Server")
        .version("0.1.0")
        .arg(
            Arg::with_name("ui")
                .long("ui")
                .value_name("DIR")
                .required(true)
                .help("Path to the directory containing UI js & wasm script files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("static")
                .long("static")
                .value_name("DIR")
                .required(true)
                .help("Path to the directory containing HTML & static files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .default_value("8080")
                .help("Port for the HTTP server")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("slow")
                .long("slow")
                .help("Slow down each request. Used for development purpose"),
        );

    let matches = app.get_matches();
    return AppConfig {
        ui: PathBuf::from(matches.value_of("ui").expect("Missing UI parameter")),
        pages: PathBuf::from(matches.value_of("static").expect("Missing pages parameter")),
        port: matches.value_of("port").unwrap_or("8080").to_string(),
        slow: matches.is_present("slow"),
    };
}

async fn index() -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, "/app/main")
        .finish()
}

async fn app_page(state: web::Data<AppState>) -> WebResult<fs::NamedFile> {
    Ok(fs::NamedFile::open(state.config.pages.join("index.html"))?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = get_config();

    let _pool = db::get_db_pool("localhost");

    let addr = format!("127.0.0.1:{}", config.port);
    info!("Listening on {}", addr);
    HttpServer::new(move || {
        let app_state = AppState {
            config: config.clone(),
        };

        let slow = config.slow;
        App::new()
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
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
