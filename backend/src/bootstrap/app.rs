use axum::{Router, http};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    bootstrap::{state::AppState, tracing::init_tracing},
    handlers,
};

pub fn create_app(state: AppState) -> Router {
    init_tracing();

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:3000"
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([http::Method::POST, http::Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .nest("/api", handlers::router())
        .with_state(state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        )
}
