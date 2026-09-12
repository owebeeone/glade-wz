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

## Library Boundaries and Fast Feedback

Current delivery context: start Glade architecture/build work with
[`dev-docs/GladeBuildEntry.md`](dev-docs/GladeBuildEntry.md) and the linked
source-qualified problem capture. The owner has parked Gyld's next phase to
prioritize Glade. Read the selected slice's canonical sources; the capture is
not a replacement specification or authority to weaken existing contracts.

Before introducing a library, changing a public interface, adding a dependency,
or changing test selection, read and follow
[`dev-docs/LibraryBoundaryAndTestingPolicy.md`](dev-docs/LibraryBoundaryAndTestingPolicy.md).
It requires explicit library roles, meaningful trait/interface contracts for
replaceable implementations, minimal dependencies, and fast isolated tests.
Pure state machines and protocol/data libraries require justified classifications,
not artificial marker traits. Agents MUST NOT relax classifications or dependency
allowlists merely to make a check pass; record and obtain review of the change.

For Glade-specific package boundaries and staging, also read
[`dev-docs/GladePackageArchitecture.md`](dev-docs/GladePackageArchitecture.md).
Its constraints apply; package names and extractions remain proposals.
Run the adopting repository's architecture gate alongside relevant package tests.
In `glade-discover`, use `./scripts/check-architecture.sh`. Do not substitute a
whole-workspace test run for the normal minor-edit loop, or omit affected-consumer
checks when changing a contract. Other repositories need explicit gate adoption;
do not claim they are already covered.

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

Read and follow `AGENTS_GWZ.md` before doing any work in this workspace.
