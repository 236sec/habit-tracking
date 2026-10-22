# Habit Tracking backend

Axum starter derived from [236sec/rust-axum-boilerplate](https://github.com/236sec/rust-axum-boilerplate) at commit `785d660e5e56f28b618c3956162eaa18eff6499e`. Its layer layout, SQLx/Postgres setup, response envelope, tracing, and Docker setup are retained. The boilerplate's sample password user API was removed because this project's sign-in decision is Supabase Auth with Google and GitHub (ADR 0002).

## Run locally

1. Start Postgres: `docker compose up -d database`
2. Copy `.env.example` to `.env`.
3. Run `cargo run`.
4. Check `http://localhost:8000/api/health`.

Use `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` for local checks. Add SQLx migrations as product features are implemented. The database has no application tables yet.
