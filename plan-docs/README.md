# G* Plan Documents

Status: active coordination area

Purpose: hold plan instances, execution state, and supporting design notes that
are not yet owned by a specific module's `dev-docs/` or public `docs/`.

`plan-docs/` is root-owned. It is the coordination layer for multi-agent work
across the full `glial-dev` checkout and its submodules.

Workflow rules are defined in
`/Users/owebeeone/limbo/glade-wz/plan-docs/AGENTS.md`.
That file also defines the plan-docs roll-build method that agents MUST follow
when a user requests a phased rollout or `roll-build`.

## Directory Contract

| Path | Meaning |
| --- | --- |
| `Registry.md` | Global index of plan id, status, owner, branch, and affected modules. |
| `ActiveWork.md` | Current who-is-doing-what board. |
| `plans/` | Active and inactive plan instances that have not been archived. |
| `archive/finished/` | Completed plans retained for traceability. |
| `archive/abandoned/` | Stopped plans retained for lessons and context. |
| `archive/superseded/` | Plans replaced by newer plan instances. |
| `templates/` | Minimal starter files for new plans. |
| `support/` | Cross-plan support material only. Prefer plan-local `Support/`. |

## Rules

1. A plan instance MUST state its status in `State.md` and `Registry.md`.
2. A plan document MUST name the intended owning module or say `root` if it is
   cross-cutting.
3. A plan document MUST define agent ownership boundaries when multiple agents
   can work in parallel.
4. Supporting material MUST NOT become authoritative architecture by accident.
5. Once a plan produces stable module design, that material SHOULD move into the
   relevant module's `dev-docs/`.
6. Once a plan produces public support guarantees, that material SHOULD move
   into the relevant module's `docs/`.
7. Scratch files MUST stay in `scratch/`, not `plan-docs/`.

## Plan Lifecycle

Use `plans/GLP-####-short-name/` while a plan is being shaped or executed.
Move it to `archive/` only after it is finished, abandoned, or superseded.

## Multi-Checkout Agent Model

The expected execution model is multiple full checkouts of `glial-dev` plus
submodules, with one or more agents assigned to separate phases or modules.

To keep merge phases tractable:

- root-level plans SHOULD allocate write ownership by module, folder, or named
  file set
- agents SHOULD avoid editing shared navigation files unless assigned the
  integration role
- generated fixtures SHOULD be treated as shared contracts and changed through
  explicit plan updates
- module-owned docs SHOULD be edited inside the module that owns the behavior
- root `plan-docs/` SHOULD coordinate dependencies between agents

## Relationship To Other Top-Level Directories

| Directory | Audience | Authority |
| --- | --- | --- |
| `docs/` | end users, adopters, integrators | public promise and supported contract |
| `dev-docs/` | root engineering and integration | internal root-module design |
| `plan-docs/` | agents and maintainers executing work | planning, progress, and handoff |
| `scratch/` | temporary local analysis | ignored, non-authoritative |

Each submodule SHOULD eventually follow the same pattern:

```text
<module>/
  docs/
  dev-docs/
  scratch/
```

The root repository SHOULD NOT permanently own detailed subsystem docs that
belong inside a module.
