//! Maps the single application error into an HTTP response.
//!
//! This is the single place an `AppError` becomes a status code and a JSON
//! body. The status, machine code, and message are owned by the error variant
//! itself (`AppError::status` / `AppError::code` / its `Display` message);
//! here we only turn them into the shared response envelope.

use axum::{
    Json,
    response::{IntoResponse, Response},
};

use crate::{handlers::common::response::ApiResponse, usecases::common::error::AppError};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self, "request failed");

        let status = self.status();
        let code = self.code();
        let body = ApiResponse::error(code, self.to_string());

        (status, Json(body)).into_response()
    }
}
