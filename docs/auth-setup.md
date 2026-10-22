# Social sign-in setup

Create two Supabase projects: one with only Google OAuth enabled and one with only GitHub OAuth enabled. Do not enable a second social provider, password, or email-link sign-in in either project. Copy `frontend/.env.example` to `frontend/.env.local` and `backend/.env.example` to `backend/.env`. Set the frontend publishable keys and URLs for each project, set `APP_ORIGIN` to the public browser origin, and point `BACKEND_URL` to Axum. The backend URLs must match the project URLs exactly. Keep OAuth client secrets in the Supabase dashboard, outside this repository.

Configure each Supabase project's allowed redirect URL to include `http://localhost:3000/auth/callback` for local development and the same path on each deployed origin. Configure Google and GitHub OAuth callback URLs to the respective Supabase project's `/auth/v1/callback` URL. The Next.js callback performs the PKCE code exchange and stores provider-specific session cookies. The proxy refreshes sessions. Axum accepts only a signed JWT from either configured issuer, with the `authenticated` audience and role and a valid expiry. Its `/api/me` endpoint creates or returns the stable app account UUID for that issuer and subject.

Run PostgreSQL, then start Axum and Next.js. Migrations run automatically at Axum startup. A live verification still needs test credentials: sign in with each provider using the same verified email, confirm different `/api/me` account IDs, sign out and return with each provider on phone and desktop widths, and confirm the account ID remains stable. Test that a token from an unknown issuer, expired token, and unsigned token receive 401. Verify authorized callback URLs in both provider dashboards. This provider-backed check remains unverified until credentials are supplied.

## Local acceptance

`cd backend && cargo test` runs the HTTP authentication checks without Docker. The persistence test is explicitly ignored by default; run it against an existing test PostgreSQL database with `cd backend && TEST_DATABASE_URL=postgres://... cargo test --test account_access -- --ignored`. No frontend test runner or local provider service is required. The owner will test the configured Google and GitHub browser journeys manually. Run `make check` for format, lint, and type checks.

## Manual provider verification for #2

Record the result in [issue #2](https://github.com/236sec/habit-tracking/issues/2) before closing it. Record project references and Account UUIDs if useful; keep full email addresses, tokens, and secrets out of the issue.

- [ ] Google-only and GitHub-only projects have the expected OAuth providers and allowed callback URLs.
- [ ] The same verified email signs in through each provider and reaches Today at phone and desktop widths.
- [ ] The two providers map to different Account UUIDs. After each sign-in, inspect the app database with `SELECT issuer, account_id FROM account_identities ORDER BY issuer;` and compare the two rows. Do not post subjects or tokens.
- [ ] After sign-out and return through each provider, its Account UUID is unchanged.
- [ ] Returning through either provider never changes that provider's Account UUID. Recheck data isolation when #4 adds user-owned Habits.

Record the test date, project references, pass/fail for each item, and any error message in the issue. Keep #2 open and PR #21 in draft until this check passes.
