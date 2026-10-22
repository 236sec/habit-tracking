# Habit Tracking frontend

Next.js App Router with TypeScript, Tailwind CSS, and shadcn/ui. Run `npm ci` and `npm run dev` from this directory, then open `http://localhost:3000`.

Local checks: `npm run lint`, `npm run typecheck`, and `npm run format:check`. Use `npm run format` to apply formatting. Add UI components with `npx shadcn@latest add <component>`.

Google and GitHub sign-in and the protected Today screen are implemented. See [social sign-in setup](../docs/auth-setup.md) for the two Supabase project configurations. The owner will verify the browser journeys manually with configured provider credentials.
