# AGENTS.md - Glial Dev Rules

## Rule 0 (Non-Negotiable): TDD First
1. Write a failing test before writing or changing implementation code.
2. Implement the smallest code change to make the test pass.
3. Refactor only with tests green.
4. Every bug fix starts with a regression test that reproduces the bug.
5. No feature is complete without tests covering success, failure, and edge cases.

## Core Workflow
1. Inspect first: gather context and identify impacted files.
2. Plan briefly: list exact edits and verification steps.
3. Execute in small steps: keep changes scoped and atomic.
4. Verify: run relevant tests, lint, and type checks.
5. Report: summarize changed files, why, and residual risks.

## Definition Of Done
- Tests added/updated and passing.
- Existing relevant tests still passing.
- Lint/type checks passing (for affected packages).
- Docs/spec updates included when behavior/contracts changed.
- No unrelated file changes.

## Code Review Focus
- Correctness and regressions first.
- Contract compatibility (protocol/schema/API) second.
- Performance/reliability/security risks third.
- Style and formatting last.

## Documentation Rules
- Use explicit normative language in specs: `MUST`, `SHOULD`, `MAY`.
- Keep requirement IDs traceable to tests.
- Record unresolved architecture decisions in a decision log.

## Commits
- Messages: ≤ 3 lines, terse — what changed, not an essay.

## Imported Claude Cowork project instructions

agent SW developer
