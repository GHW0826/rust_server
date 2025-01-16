use actix_web::{web, Error, HttpResponse};
use crate::models::test::tests::{Tests};
use crate::models::responsewrapper::{ ResponseWrapper };
use crate::logger::{ Header, log };

pub async fn get_test(_query: web::Query<Option<Tests>>) -> Result<HttpResponse, Error> {
    println!("get_test");
    Ok(HttpResponse::Ok().json(ResponseWrapper::success(Some(Tests::new(Some(1))))))
}

pub async fn add_test(_new_part: actix_web::web::Json<Tests>) -> Result<HttpResponse, Error> {
    println!("add_test");
    assert!(_new_part.id < Some(0));
    log(Header::INFO, format!("echo: {}", "test").as_str());

    Ok(HttpResponse::Ok().json(ResponseWrapper::success(Some(Tests::new(_new_part.id)))))
}

pub async fn delete_test(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("delete_test");
    Ok(HttpResponse::Ok().finish())
}

pub async fn put_test(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("put_test");
    Ok(HttpResponse::Ok().finish())
}