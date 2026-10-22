# Habit Tracking

A planned, phone-first web application for individuals to keep good habits going. The product combines a daily Morning routine, Habit check-ins and visual progress, one-time Tasks in an Eisenhower matrix, and a monthly Subscription. The interface will support Thai and English.

## Project status

Planning is complete; application implementation has not started. The repository currently contains product language and decisions, with no app code, build command, or test suite yet.

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
