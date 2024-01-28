use actix_web:: 
        {http::{header::ContentType, StatusCode},
        HttpResponse, ResponseError};

use derive_more::Display;
#[derive(Debug,Display)]
pub enum PeopleError{
    NoPeoplesFound,
    AddingPeopleFailed,
    NoSuchPeopleFound,
    ValidateError(String),
}

impl ResponseError for PeopleError{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self{
            PeopleError::NoPeoplesFound => StatusCode::NOT_FOUND,
            PeopleError::AddingPeopleFailed => StatusCode::INTERNAL_SERVER_ERROR,
            PeopleError::NoSuchPeopleFound => StatusCode::NOT_FOUND,
            PeopleError::ValidateError(_) => StatusCode::NOT_ACCEPTABLE,
        }


    }
}
