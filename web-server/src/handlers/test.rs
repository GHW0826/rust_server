use actix_web::{web, Error, HttpResponse};
use crate::common::tests::{Tests};
use crate::logger::{ Header, log };

pub async fn get_test(_query: web::Query<Option<Tests>>) -> Result<HttpResponse, Error> {
    println!("get_test");
    Ok(HttpResponse::Ok().body("test"))
}

pub async fn add_test(_new_part: web::Json<Tests>) -> Result<HttpResponse, Error> {
    println!("add_test");
    log(Header::INFO, format!("echo: {}", "test").as_str());
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_test(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("delete_test");
    Ok(HttpResponse::Ok().finish())
}

pub async fn put_test(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("put_test");
    Ok(HttpResponse::Ok().finish())
}