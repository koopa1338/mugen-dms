use crate::controller::auth_controller;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/login")
                            .route(web::post().to(auth_controller::login))
                    )
            )
    );
}
