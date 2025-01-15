use actix_web::web;
use crate::handlers::{ test };

pub fn config_test_svc(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.service(
        web::scope("/test")
            .service(
                web::resource("")
                    .route(web::get().to(test::get_test))
                    .route(web::post().to(test::add_test))
                    .route(web::delete().to(test::delete_test))
                    .route(web::put().to(test::put_test)),
            )
        ,
    );
}