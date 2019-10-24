use actix_web::{self, http, HttpResponse, ResponseError};
use r2d2;
use reqwest;
use serde_json;

use crate::utils::prepare_response;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Endpoint skipped because of guard: {}", _0)]
    GuardError(&'static str),

    #[fail(display = "Api request error: {}", _0)]
    ApiError(reqwest::Error),

    #[fail(display = "Json error: {}", _0)]
    JsonError(serde_json::Error),

    #[fail(display = "{}", _0)]
    DatabaseError(DatabaseError),

    #[fail(display = "Slack Error: {}", _0)]
    SlackError(String),

    #[fail(display = "Github Error: {}", _0)]
    GithubError(String),

    #[fail(display = "Something went wrong")]
    ServerError(String),

    #[fail(display = "{}", _0)]
    UrlParseError(UrlParseError),

    #[fail(display = "Something went wrong")]
    TemplateError(String),

    #[fail(display = "Record not found")]
    NotFoundError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::GuardError(e) => prepare_response(&format!(r#"{{ "error": "{}" }}"#, e)),
            Error::SlackError(e) | Error::GithubError(e) => {
                prepare_response(&format!(r#"{{ "error": "{}" }}"#, e))
            }
            Error::NotFoundError => prepare_response(r#"{ "error": "Record not found" }"#),
            _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ApiError(err)
    }
}

impl From<DatabaseError> for Error {
    fn from(err: DatabaseError) -> Self {
        Error::DatabaseError(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::DatabaseError(DatabaseError::ConnectionPool(err))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Error::DatabaseError(DatabaseError::NotFound),
            error => Error::DatabaseError(DatabaseError::Error(error)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Error::ServerError(format!("{}", err))
    }
}

impl From<UrlParseError> for Error {
    fn from(err: UrlParseError) -> Self {
        Error::UrlParseError(err)
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Error::TemplateError(format!("{}", err))
    }
}

impl From<http::header::ToStrError> for Error {
    fn from(err: http::header::ToStrError) -> Self {
        Error::ServerError(format!("{}", err))
    }
}

impl From<hyperx::Error> for Error {
    fn from(err: hyperx::Error) -> Self {
        Error::ServerError(format!("{}", err))
    }
}

#[derive(Fail, Debug)]
pub enum DatabaseError {
    #[fail(display = "Connection pool error")]
    ConnectionPool(r2d2::Error),

    #[fail(display = "There was a problem")]
    Error(diesel::result::Error),

    #[fail(display = "Record not found")]
    NotFound,
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => DatabaseError::NotFound,
            error => DatabaseError::Error(error),
        }
    }
}

impl From<r2d2::Error> for DatabaseError {
    fn from(err: r2d2::Error) -> Self {
        DatabaseError::ConnectionPool(err)
    }
}

#[derive(Fail, Debug)]
pub enum UrlParseError {
    #[fail(display = "Mallformed url")]
    MissingSegment,

    #[fail(display = "Mallformed url")]
    Parse(url::ParseError),
}
