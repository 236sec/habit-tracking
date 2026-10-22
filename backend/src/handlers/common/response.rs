//! Shared response envelope for the HTTP layer.
//!
//! Every endpoint returns the same shape so clients get a consistent contract:
//! `{ "success": bool, "data"?: T, "error"?: { "code", "message" } }`.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorBody>,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: &'static str, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorBody { code, message }),
        }
    }
}
