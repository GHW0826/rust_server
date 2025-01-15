use actix_web::{web, Error, HttpResponse};

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