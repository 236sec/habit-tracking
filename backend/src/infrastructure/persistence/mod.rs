//! Persistence adapters — the concrete implementations of repository ports.
//!
//! This is the *only* layer that knows about SQLx. It maps SQLx errors into
//! the shared `AppError` so the rest of the application stays database-agnostic.

use crate::usecases::common::error::AppError;

pub mod accounts;

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}
