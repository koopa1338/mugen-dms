mod config;
mod handler;
mod error;
mod models;
mod services;

use config::app;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "mugen-server=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let frontend = app::static_routes();
    let backend = app::api_routes();

    tokio::join!(frontend, backend);
}
