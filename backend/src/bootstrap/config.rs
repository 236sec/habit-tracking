use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

        Self {
            database_url,
            bind_addr,
        }
    }
}
