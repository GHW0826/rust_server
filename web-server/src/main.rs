use actix_web::{middleware, App, HttpServer};
// use web_server::app_config::config_app;
use web_server::config::config_test_svc::config_test_svc;
use web_server::config::config_default_svc::config_default_svc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    // HttpServer::new(|| {
    //     App::new()
    //         .configure(config_app)
    //         .wrap(middleware::Logger::default())
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    HttpServer::new(|| {
        App::new()
            .configure(config_test_svc)
            .configure(config_default_svc)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}