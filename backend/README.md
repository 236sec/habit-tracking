# Habit Tracking backend

Axum starter derived from [236sec/rust-axum-boilerplate](https://github.com/236sec/rust-axum-boilerplate) at commit `785d660e5e56f28b618c3956162eaa18eff6499e`. Its layer layout, SQLx/Postgres setup, response envelope, tracing, and Docker setup are retained. The boilerplate's sample password user API was removed because this project's sign-in decision is Supabase Auth with Google and GitHub (ADR 0002).

## Run locally

1. Provide a PostgreSQL connection URL (local or hosted).
2. Copy `.env.example` to `.env` and set `DATABASE_URL` and both Supabase project URLs.
3. Run `cargo run`; migrations apply on startup.
4. Check `http://localhost:8000/api/health`.

Use `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` for local checks. The HTTP auth tests run without PostgreSQL. The account persistence test is opt-in and needs `TEST_DATABASE_URL`; see [social sign-in setup](../docs/auth-setup.md). The `accounts` and `account_identities` tables are created by the first migration.
