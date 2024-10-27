use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use r2d2;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(DieselError),
    PoolError(r2d2::Error),
    BlockingError,
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> Self {
        ServiceError::DatabaseError(error)
    }
}

impl From<r2d2::Error> for ServiceError {
    fn from(error: r2d2::Error) -> Self {
        ServiceError::PoolError(error)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            ServiceError::PoolError(ref err) => write!(f, "Connection pool error: {}", err),
            ServiceError::BlockingError => write!(f, "Blocking operation failed"),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().body("Database query error")
            }
            ServiceError::PoolError(_) => {
                HttpResponse::InternalServerError().body("Database connection pool error")
            }
            ServiceError::BlockingError => {
                HttpResponse::InternalServerError().body("Blocking operation failed")
            }
        }
    }
}
