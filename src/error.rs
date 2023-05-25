use actix_web::{body::BoxBody, HttpResponse, HttpResponseBuilder as Response, ResponseError};
use hextacy::drivers::DriverError;
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;
use validify::ValidationErrors;

use crate::{cache::CacheAdapterError, db::adapters::AdapterError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Adapter: {0}")]
    Adapter(#[from] AdapterError),
    #[error("Cache: {0}")]
    Cache(#[from] CacheAdapterError),
    #[error("Driver: {0}")]
    Driver(#[from] DriverError),
    #[error("Validation: {0}")]
    Validation(#[from] ValidationErrors),
}

impl Error {
    pub fn new<E: Into<Self>>(e: E) -> Self {
        e.into()
    }

    /// Returns error message and description
    pub fn message_and_description(&self) -> (&'static str, String) {
        match self {
            Self::Validation(_) => ("VALIDATION", "Invalid request parameters".to_string()),
            _ => ("INTERNAL_SERVER_ERROR", "Internal server error".to_string()),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => todo!(),
        }
    }

    /// Transform the error to an `ErrorResponse` struct that implements actix's `ErrorResponse` trait.
    /// Flattens all validation errors to a vec, if any
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();
        let (message, description) = self.message_and_description();
        let error_response = match self {
            Error::Validation(errs) => {
                ErrorResponse::new(status.as_u16(), message, &description, Some(errs))
            }
            _ => ErrorResponse::new(status.as_u16(), message, &description, None),
        };

        Response::new(status).json(error_response)
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse<'a, T> {
    code: u16,
    message: &'a str,
    description: &'a str,
    details: Option<T>,
}

impl<'a, T> ErrorResponse<'a, T>
where
    T: Serialize,
{
    pub fn new(code: u16, message: &'a str, description: &'a str, details: Option<T>) -> Self {
        Self {
            code,
            message,
            description,
            details,
        }
    }
}

impl<T> std::fmt::Display for ErrorResponse<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Message: {}, Description: {}",
            self.message, self.description
        )
    }
}
