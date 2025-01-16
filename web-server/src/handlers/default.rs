use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::err::{Error};

pub async fn not_found2() -> Result<HttpResponse, Error> {
//    Err(Error::NotFound.not_found())
    Ok(HttpResponse::Ok().body("test"))

}

pub async fn not_found(_req: HttpRequest) -> HttpResponse {
    // This API will send  bad request error as response inXMl format
    Error::bad_request("Value is required".to_string()).error_response_json()
}

pub async fn get_default() -> Result<HttpResponse, Error> {
    println!("get_default");
    Ok(HttpResponse::Ok().body("test"))
}

pub async fn post_default(_new_part: web::Json<String>) -> Result<HttpResponse, Error> {
    println!("post_default");
    Ok(HttpResponse::Ok().body("test"))
}

pub async fn delete_default(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("delete_default");
    Ok(HttpResponse::Ok().finish())
}

pub async fn put_default(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("put_test");
    Ok(HttpResponse::Ok().finish())
}