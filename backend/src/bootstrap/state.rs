//! Composition root for backend dependencies.

use std::sync::Arc;

use crate::bootstrap::{config::AppConfig, db::init_db};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub pool: sqlx::PgPool,
}

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = Arc::new(AppConfig::from_env());
    let pool = init_db(&config.database_url).await?;
    Ok(AppState { config, pool })
}
