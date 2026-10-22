# Habit Tracking

A planned, phone-first web application for individuals to keep good habits going. The product combines a daily Morning routine, Habit check-ins and visual progress, one-time Tasks in an Eisenhower matrix, and a monthly Subscription. The interface will support Thai and English.

## Project status

Planning is complete and the application scaffolds are in place. Product features, including Supabase sign-in, have not been implemented yet.

## Local development

Requirements: Rust 1.97+, Node.js 20.9+, npm, Docker, and Make.

```bash
cd frontend && npm ci && npm run dev
```

In another terminal, start the database and backend:

```bash
cd backend
docker compose up -d database
cp .env.example .env
cargo run
```

The frontend runs at `http://localhost:3000` and the backend health endpoint is `http://localhost:8000/api/health`. Run `make check` at the repository root for Rust formatting/Clippy and frontend Prettier/ESLint/TypeScript checks. Run `make format` to apply formatting and `make test` for Rust tests. The tracked `.githooks/pre-commit` runs `make check`; activate it in a clone with `git config core.hooksPath .githooks` (already configured in this checkout). No GitHub Actions workflow is used.

The backend is derived from [236sec/rust-axum-boilerplate](https://github.com/236sec/rust-axum-boilerplate); see [backend/README.md](backend/README.md) for the source commit and changes.

## Source of truth

- [Product spec](https://github.com/236sec/habit-tracking/issues/1) — agreed behavior, scope, and browser-level testing decisions.
- [Implementation tickets](https://github.com/236sec/habit-tracking/issues/1) — 17 sub-issues with native blocking links and the `ready-for-agent` label. [Ticket #2](https://github.com/236sec/habit-tracking/issues/2) is the first unblocked slice.
- [Glossary](GLOSSARY.md) — canonical terms for Habit, Planned day, Morning routine, Task, Free plan, and Subscription.
- [ADR 0001](docs/adr/0001-preserve-historical-planned-days.md) — schedule changes preserve historical Planned days.
- [ADR 0002](docs/adr/0002-use-supabase-social-auth.md) — sign-in uses Supabase Auth with Google and GitHub.
- [Agent instructions](AGENTS.md) — issue tracker, triage label, and domain-document conventions.

## Continuing the work

Start with the first unblocked ticket and follow the blockers recorded on each GitHub issue. Each ticket is a user-visible vertical slice, including its browser-level acceptance behavior. Keep the glossary and ADRs aligned with any new domain decisions.

The exact monthly THB price and payment provider remain open launch decisions. The Subscription ticket calls for a configurable price and a verified sandbox checkout; public checkout needs a chosen amount before launch.
