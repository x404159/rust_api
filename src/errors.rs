//custom errors
use actix_web::{
    dev::HttpResponseBuilder, error::ResponseError, http::header, http::StatusCode, HttpResponse,
};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use jsonwebtoken::errors::Error as JWTError;
use std::convert::From;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    #[display(fmt = "NotFound")]
    NotFound,
    #[display(fmt = "jsonwebtoken error")]
    JsonWebTokenError,
}

//for actix_web error
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::InternalServerError | Self::JsonWebTokenError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

//for diesel error
impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_owned();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<JWTError> for ServiceError {
    fn from(_: JWTError) -> Self {
        Self::JsonWebTokenError
    }
}
