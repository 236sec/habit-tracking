# Issue tracker: GitHub

Issues and specs for this repo live as GitHub issues. Use the `gh` CLI for all operations.

## Conventions

- Create: `gh issue create --title "..." --body "..."`.
- Read: `gh issue view <number> --json number,title,body,labels,comments`.
- List: `gh issue list --state open --json number,title,body,labels,comments`, with filters as needed.
- Comment: `gh issue comment <number> --body "..."`.
- Label: `gh issue edit <number> --add-label "..."` or `--remove-label "..."`.
- Close: `gh issue close <number> --comment "..."`.
- Link a child issue with `gh issue create --parent <parent>` or `gh issue edit <parent> --add-sub-issue <child>`. If sub-issues are unavailable, put `Part of #<parent>` at the top of the child body.

Infer the repository from the GitHub remote.

## Pull requests as a triage surface

**PRs as a request surface: no.**

## Skill operations

- "Publish to the issue tracker": create a GitHub issue.
- "Fetch the relevant ticket": read the GitHub issue.

## Wayfinding operations

- Map: one issue labelled `wayfinder:map`, containing Notes, Decisions-so-far, and Fog.
- Child ticket: a sub-issue labelled `wayfinder:<type>` (`research`, `prototype`, `grilling`, or `task`). Fall back to `Part of #<map>` in its body if sub-issues are unavailable.
- Blocking: use GitHub's native issue dependencies. Fall back to a `Blocked by: #<n>, #<n>` line if unavailable. A ticket is unblocked when every blocker is closed.
- Frontier: inspect the map's open children in map order; the first unassigned child without an open blocker wins.
- Claim: `gh issue edit <n> --add-assignee @me` before work.
- Resolve: comment with the answer, close the child, then add a context pointer to the map's Decisions-so-far.
