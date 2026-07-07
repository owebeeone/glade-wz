# AGENTS.md - Plan Docs Workflow

## Purpose

`plan-docs/` coordinates non-trivial work across the root repo and submodules.
Use it for plannable tasks, not quick bug fixes.

A task is plannable when it is likely to exceed roughly `100 LOC`, crosses a
module boundary, changes contracts, or needs multiple agents/checkouts.

## Core Rule

The plan instance is the unit of coordination.

Do not scatter one plan across global lifecycle folders. Create one plan folder
with a stable ID and keep its plan, state, reviews, support, and handoff notes
together.

## Required Shape

```text
plan-docs/
  Registry.md
  ActiveWork.md
  plans/
    GLP-0001-short-name/
      Plan.md
      State.md
      Workstreams.md
      Checkpoints.md
      Decisions.md
      Risks.md
      Reviews/
      Support/
      Handoff.md
  archive/
    finished/
    abandoned/
    superseded/
```

## Required Files

| File | Purpose |
| --- | --- |
| `Registry.md` | Global index of plan id, title, status, owner, branch, and affected modules. |
| `ActiveWork.md` | Current who-is-doing-what board. |
| `Plan.md` | Scope, phases, acceptance gates, and execution order. |
| `State.md` | Current status, current phase, blockers, and next checkpoint. |
| `Workstreams.md` | Agent ownership, write boundaries, inputs, outputs, and merge risks. |
| `Checkpoints.md` | Phase checkpoints, roll-build tags, verification, and rollback notes. |
| `Decisions.md` | Plan-local decisions and revision history. |
| `Risks.md` | Plan-local risk register and fallback paths. |
| `Handoff.md` | Final or partial handoff summary. |

## Status Values

Use one of:

```text
draft
proposed
accepted
active
blocked
paused
split
superseded
finished
partial-finished
abandoned
merged
```

Status lives in `State.md` and `Registry.md`. Folder movement is archival only.

## Workflow

1. Identify a plannable task.
2. Create a `GLP-####-short-name/` folder under `plans/`.
3. Draft `Plan.md`, `State.md`, `Workstreams.md`, `Checkpoints.md`, and
   `Risks.md`.
4. Review the plan and revise until it is executable.
5. Move status to `accepted`, then `active`.
6. Execute phase by phase.
7. Record every checkpoint in `Checkpoints.md`.
8. Record plan changes in `Decisions.md`.
9. If the plan gets too large, set status `split` and create child plans.
10. Finish as `finished`, `partial-finished`, `abandoned`, or `superseded`.
11. Merge to main when the plan output is integrated.
12. Archive the plan only after merge or explicit stop.

## Agent Ownership

Every active plan MUST state:

- coordinating agent or owner
- branch or checkout identifier
- owned files/folders
- files/folders the agent MUST NOT touch
- dependencies on other plans or agents
- expected outputs
- merge risks

Shared root navigation files SHOULD be edited only by the coordinating agent.

## Checkpoints

Use checkpoint tags only for meaningful integration gates.

For normal commits, prefer commit trailers:

```text
Plan: GLP-0001
Phase: P02
Checkpoint: terminal-contract
```

## Roll-Build Method

When the user asks for a phased rollout using the roll-build method, the agent
MUST read this section before implementation and execute the plan through its
declared phases and checkpoints.

- Start from a clean git tree and tag that point before implementation begins.
- Use the requested start tag name when one is given. If none is given, ask or
  use a clearly scoped phase-start tag name.
- An unqualified `roll-build` means: run all phases for that plan in sequence,
  committing and tagging each completed phase, and continue into the next phase
  without stopping unless the guardrails below require a pause.
- Run the roll-build in the current owning checkout and current branch. Do not
  create git worktrees, sibling checkouts, or parallel rollout branches unless
  the user explicitly asks for them in that request.
- Do not split phases or adjacent roll-build requests into parallel branches.
  If one roll-build has already produced commits, the next roll-build starts on
  top of those commits after they are integrated into the current branch.
- If the current branch is not the intended integration branch, stop and ask
  before creating or switching branches. Do not invent a branch/worktree
  strategy from the tag prefix.
- Implement one phase at a time.
- After a phase is complete, only commit and tag it if:
  - the phase goal is actually met
  - focused verification passes
  - the remaining ambiguities are minor and non-blocking
- Record the completed phase, tag, verification, and rollback notes in the
  plan's `Checkpoints.md`.
- If there are no more phases, or if confidence drops because of material
  ambiguity or instability, stop and wait instead of forcing the next phase.
- If work starts cycling on the same persistent bug or bug family, stop, report
  the cycle clearly, and ask for direction.

## When To Push Back On Roll-Build

- Push back when the next phase has too many unresolved ambiguities to produce a
  trustworthy checkpoint.
- Push back when the requested phase is too large or too coupled to complete
  safely as one checkpoint.
- Push back when implementation reveals facts that materially break the current
  design or plan assumptions.
- Push back when the resulting checkpoint would be misleadingly partial,
  unstable, or hard to recover from.

## Plan Changes

If a plan changes:

- small correction: update `Plan.md` and record the reason in `Decisions.md`
- major scope change: revise status to `paused` or `split`
- wrong direction: mark `abandoned` or `superseded`
- useful partial result: mark `partial-finished` and write `Handoff.md`

## Authority Boundary

`plan-docs/` is not stable architecture.

Stable internal design SHOULD move into the owning module's `dev-docs/`.
Public support promises SHOULD move into the owning module's `docs/`.
Scratch analysis MUST stay in `scratch/`.
