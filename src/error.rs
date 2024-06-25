use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use pg::error::Error as PGError;
use serde_json::Error as SerdeError;
use std::io::Error as IOError;
//use actix_web::Error as ActixError;
//use tokio_postgres::error::Error as PGError;
//use askama::Error as AskamaError;

#[derive(Display, From, Debug)]
pub enum Error {
    NotFound,
    PGError(PGError),
    PoolError(PoolError),
    IOError(IOError),
    SerdeError(SerdeError),
    // ActixError(ActixError),
    // AskamaError(AskamaError),
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound => HttpResponse::NotFound().finish(),
            Error::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::SerdeError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::IOError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            //Error::AskamaError(ref err) => {
            //HttpResponse::InternalServerError().body(err.to_string())
            //}
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
