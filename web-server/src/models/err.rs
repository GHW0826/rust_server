use bcrypt::BcryptError;
use tokio::task::JoinError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::to_string;
use crate::models::responsewrapper::{ResponseWrapper};

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request {message}")]
pub struct BadRequest {
    message : String
}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFounds {

}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {

    #[error("Error parsing ObjectID {0}")]
    ParseObjectID(String),

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFounds),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),
}


impl Error {
    pub fn error_response_json(&self) -> HttpResponse {
        let (status_code, code, msg) = self.get_error_info();
        let error_response = ResponseWrapper::fail(Some(code), Some(msg));
        HttpResponse::build(status_code).json(error_response)
    }

    // ?
    pub fn error_response_xml(&self) -> HttpResponse {
        let (status_code, code, msg) = self.get_error_info();
        let error_response = ResponseWrapper::fail(Some(code), Some(msg));

        let error_resp_string = match to_string(&error_response) {
            Ok(xml_string) => xml_string,
            Err(_) => "<error>".to_string(), // Handle serialization error gracefully
        };
        HttpResponse::build(status_code).content_type("application/xml").body(error_resp_string)
    }
}

impl Error {
    pub fn bad_request(message : String) -> Self {
        Error::BadRequest(BadRequest {message})
    }

    pub fn not_found() -> Self {
        Error::NotFound(NotFounds {})
    }

    fn get_error_info(&self) -> (StatusCode, String, String) {
        (self.get_status_code(), self.get_error_code(), self.get_error_msg())
    }

    fn get_status_code(&self) -> StatusCode {
        match *self {
            // 4XX Errors
            Error::ParseObjectID(_) => StatusCode::BAD_REQUEST,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Authenticate(AuthenticateError::WrongCredentials) => StatusCode::UNAUTHORIZED,
            Error::Authenticate(AuthenticateError::InvalidToken) => StatusCode::UNAUTHORIZED,
            Error::Authenticate(AuthenticateError::Locked) => StatusCode::LOCKED,

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RunSyncTask(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::HashPassword(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn get_error_code(&self) -> String {
        match *self {
            // 4XX Errors
            Error::ParseObjectID(_) => String::from("40001"),
            Error::BadRequest(_) => String::from("40002"),
            Error::NotFound(_) => String::from("40003"),
            Error::Authenticate(AuthenticateError::WrongCredentials) => String::from("40004"),
            Error::Authenticate(AuthenticateError::InvalidToken) => String::from("40005"),
            Error::Authenticate(AuthenticateError::Locked) => String::from("40006"),

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => String::from("5001"),
            Error::RunSyncTask(_) => String::from("5005"),
            Error::HashPassword(_) => String::from("5006"),
        }
    }

    fn get_error_msg(&self) -> String {
        self.to_string()
        // match *self {
        //     // 4XX Errors
        //     Error::ParseObjectID(_) => String::from("40001"),
        //     Error::BadRequest(_) => String::from("BadRequest"),
        //     Error::NotFound(_) => String::from("NotFound"),
        //     Error::Authenticate(AuthenticateError::WrongCredentials) => String::from("40004"),
        //     Error::Authenticate(AuthenticateError::InvalidToken) => String::from("40005"),
        //     Error::Authenticate(AuthenticateError::Locked) => String::from("40006"),

        //     // 5XX Errors
        //     Error::Authenticate(AuthenticateError::TokenCreation) => String::from("5001"),
        //     Error::RunSyncTask(_) => String::from("5005"),
        //     Error::HashPassword(_) => String::from("5006"),
        // }
    }
}