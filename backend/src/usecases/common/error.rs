//! Shared application error type for future feature use cases.

use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    pub fn code(&self) -> &'static str {
        "INTERNAL_ERROR"
    }
}
