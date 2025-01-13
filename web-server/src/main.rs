use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

// This struct represents state
struct AppState {
    app_name: String,
}
#[get("/")]
async fn index2(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

use std::sync::Mutex;
struct AppStateWithCounter {
    // <- Mutex is necessary to mutate safely across threads
    counter: Mutex<i32>, 
}
async fn index3(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

use actix_web::guard;


// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| {
    //     App::new()
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
    
    // HttpServer::new(|| {
    //     App::new().service(
    //         // prefixes all resources and routes attached to it...
    //         web::scope("/app")
    //             // ...so this handles requests for `GET /app/index.html`
    //             .route("/index.html", web::get().to(index)),
    //     )
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    // HttpServer::new(|| {
    //     App::new()
    //         .app_data(web::Data::new(AppState {
    //             app_name: String::from("Actix Web"),
    //         }))
    //         .service(index2)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    //  // Note: web::Data created _outside_ HttpServer::new closure
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    // HttpServer::new(move || {
    //     // move counter into the closure
    //     App::new()
    //         .app_data(counter.clone()) // <- register the created data
    //         .route("/", web::get().to(index3))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    // HttpServer::new(|| {
    //     App::new()
    //         .service(
    //             web::scope("/")
    //                 .guard(guard::Host("www.rust-lang.org"))
    //                 .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
    //         )
    //         .service(
    //             web::scope("/")
    //                 .guard(guard::Host("users.rust-lang.org"))
    //                 .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
    //         )
    //         .route("/", web::to(HttpResponse::Ok))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}