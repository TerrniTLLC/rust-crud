use derive_more::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

#[derive(Debug, Display)]
pub enum NoodleError {
    NoNoodlesFound,
    NoodleCreationError,
    NoSuchNoodleErorr,
}

impl ResponseError for NoodleError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NoodleError::NoNoodlesFound => StatusCode::NOT_FOUND, // 404
            NoodleError::NoodleCreationError => StatusCode::INTERNAL_SERVER_ERROR, // 500
            NoodleError::NoSuchNoodleErorr => StatusCode::NOT_FOUND, // 404
        }
    }
}
