# Keep Google and GitHub Auth in separate Supabase projects

Status: Provisional. This is the implementation choice for configured projects, pending the live provider experiment below.

Supabase Auth automatically links identities with the same verified email within one project. The product requires those sign-ins to remain separate until the user explicitly links them. Use one Google-only Supabase Auth project and one GitHub-only project. Each project has its own publishable key, redirect configuration, and signing keys.

The application account is a UUID mapped from the validated `(JWT issuer, JWT subject)` pair. Email is display information and is never an account lookup or merge key. A future explicit linking flow can attach a second verified pair to an existing account after the user proves control of both sessions. This cross-project linking is application-owned; Supabase `linkIdentity()` cannot link users across projects.

This refines [ADR 0002](0002-use-supabase-social-auth.md). Separate projects increase configuration and operational work.

Before release, configure disposable Google-only and GitHub-only Supabase projects with real OAuth credentials and authorized redirects. Sign in to both with the same verified email and confirm distinct issuer/subject pairs and separate app Account UUIDs, then sign out and return to each account on phone and desktop. Verify that a future explicit link can be implemented with two authenticated sessions and a transactional app mapping. Record the project IDs, JWT claims, and result without recording secrets or tokens. If the provider behavior or claims do not support this mapping, revisit this ADR and issue #16 before release. This experiment has not run because live credentials are unavailable.
