use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: String,
    pub auth_issuers: [String; 2],
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());
        let google_url = env::var("GOOGLE_SUPABASE_URL").expect("GOOGLE_SUPABASE_URL must be set");
        let github_url = env::var("GITHUB_SUPABASE_URL").expect("GITHUB_SUPABASE_URL must be set");
        let auth_issuers =
            [google_url, github_url].map(|url| format!("{}/auth/v1", url.trim_end_matches('/')));
        assert_ne!(
            auth_issuers[0], auth_issuers[1],
            "Google and GitHub require separate Supabase projects"
        );

        Self {
            database_url,
            bind_addr,
            auth_issuers,
        }
    }
}
