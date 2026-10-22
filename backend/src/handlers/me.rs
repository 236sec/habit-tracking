use axum::{
    Json,
    extract::State,
    http::{
        HeaderMap, StatusCode,
        header::{AUTHORIZATION, CACHE_CONTROL, HeaderName},
    },
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    bootstrap::state::AppState,
    handlers::common::response::{ApiErrorBody, ApiResponse},
    infrastructure::{auth::verify_bearer, persistence::accounts::get_or_create_account},
};

#[derive(Serialize)]
pub struct Account {
    id: Uuid,
}

type MeResponse = (
    StatusCode,
    [(HeaderName, &'static str); 1],
    Json<ApiResponse<Account>>,
);

fn private_headers() -> [(HeaderName, &'static str); 1] {
    [(CACHE_CONTROL, "private, no-store")]
}

fn error(status: StatusCode, code: &'static str, message: &str) -> MeResponse {
    (
        status,
        private_headers(),
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some(ApiErrorBody {
                code,
                message: message.to_owned(),
            }),
        }),
    )
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> MeResponse {
    let Some(token) = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "A bearer token is required",
        );
    };
    let Some(identity) = verify_bearer(token, &state.config.auth_issuers).await else {
        return error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Invalid or expired session",
        );
    };
    match get_or_create_account(&state.pool, &identity).await {
        Ok(id) => (
            StatusCode::OK,
            private_headers(),
            Json(ApiResponse::ok(Account { id })),
        ),
        Err(cause) => {
            tracing::error!(error = ?cause, "account lookup failed");
            error(
                StatusCode::SERVICE_UNAVAILABLE,
                "database_unavailable",
                "Account service unavailable",
            )
        }
    }
}
