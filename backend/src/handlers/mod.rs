//! HTTP feature routers are added here.

use axum::{Json, Router, routing::get};

use crate::{bootstrap::state::AppState, handlers::common::response::ApiResponse};

pub mod common;
mod me;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/me", get(me::me))
}

async fn health() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::ok("ok"))
}
