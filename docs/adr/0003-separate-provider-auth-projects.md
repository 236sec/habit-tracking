# Keep Google and GitHub Auth in separate Supabase projects

Supabase Auth automatically links identities with the same verified email within one project. The product requires those sign-ins to remain separate until the user explicitly links them. Use one Google-only Supabase Auth project and one GitHub-only project. Each project has its own publishable key, redirect configuration, and signing keys.

The application account is a UUID mapped from the validated `(JWT issuer, JWT subject)` pair. Email is display information and is never an account lookup or merge key. A future explicit linking flow can attach a second verified pair to an existing account after the user proves control of both sessions. This cross-project linking is application-owned; Supabase `linkIdentity()` cannot link users across projects.

This refines [ADR 0002](0002-use-supabase-social-auth.md). Separate projects increase configuration and operational work. Provider login and cross-project linking require live credentials to verify before release.
