use actix_web::web;
use crate::handlers::{ default };

pub fn config_default_svc(cfg: &mut web::ServiceConfig) {
    cfg.default_service(
        web::route().to(default::not_found),
    );
}