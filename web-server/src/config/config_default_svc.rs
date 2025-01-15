use actix_web::web;
use crate::handlers::{ default };

pub fn config_default_svc(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::resource("")
                    .route(web::get().to(default::get_default))
                    .route(web::post().to(default::post_default))
                    .route(web::delete().to(default::delete_default))
                    .route(web::put().to(default::put_default)),
            )
        ,
    );
}