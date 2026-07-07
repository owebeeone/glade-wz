# G* Documentation Structure Plan

Status: proposed

Purpose: define a document layout that can support multiple agents working in
parallel without turning `dev-docs/` into an unowned pile of overlapping specs.

This document proposes the target structure only. It does not move the current
documents yet.

Revision note:
The root repository should not permanently own all subsystem design documents.
As Glade, Glial, Grip Share, and related libraries become module boundaries,
each module SHOULD own its own `docs/`, `dev-docs/`, and `scratch/` areas.
The root repository SHOULD own integration maps, plans, cross-module risks, and
handoff coordination.

## Core Problem

`dev-docs/` is already carrying several different kinds of documents:

- stable architecture contracts
- roadmap and risk planning
- design studies
- declaration examples
- generated-shape sketches
- spike specifications
- historical context

If these stay in one mixed namespace, agents will collide on the same files and
the authoritative boundary of each document will become unclear.

There is a second problem: many documents currently in root `dev-docs/` really
belong to module-owned documentation. For example, most Glade substrate design
SHOULD eventually live under a Glade module's own `dev-docs/`, not permanently
inside the root integration repository.

## Top-Level Directory Model

Use three visible documentation levels plus ignored scratch:

| Directory | Audience | Authority | Ownership |
| --- | --- | --- | --- |
| `docs/` | end users, adopters, integrators | public promise and supported contract | owning module |
| `dev-docs/` | engineers changing the module | internal design and implementation contract | owning module |
| `plan-docs/` | agents and maintainers executing work | plans, progress, handoff, and temporary support | root integration repo |
| `scratch/` | local analysis | ignored, non-authoritative | local checkout |

The root `dev-docs/` SHOULD shrink over time. Its final role SHOULD be root
integration docs only:

- stack map
- cross-module decision log
- module ownership map
- integration contracts that cannot belong to one module
- migration notes until they are assigned

Detailed subsystem docs SHOULD move into the relevant module:

```text
glade/
  docs/
  dev-docs/
  scratch/

glial-core/
  docs/
  dev-docs/
  scratch/

grip-core/
  docs/
  dev-docs/
  scratch/
```

The Glade module does not currently exist. Until it exists, Glade docs MAY stay
in root `dev-docs/` as staged material, but they SHOULD be treated as pending
module-owned docs.

## Structure Rules

1. Every document MUST have one primary ownership area.
2. Every stable subsystem document SHOULD live in the module that owns the
   behavior.
3. Cross-cutting documents MUST live in root integration docs or root
   `plan-docs/`, not inside a subsystem folder.
4. Spike documents MUST be isolated from stable architecture contracts.
5. Generated fixtures MUST be separate from human-authored architecture docs.
6. Historical documents MUST remain readable but MUST NOT be treated as
   authoritative.
7. Scratch notes MUST remain under `scratch/` and MUST NOT be required to build
   the authoritative design.
8. Each folder SHOULD have an index document before it grows beyond a handful
   of files.
9. Multi-agent work SHOULD assign write ownership by folder or by named file set.
10. Plan documents SHOULD define what will move into module `dev-docs/` when
    the plan stabilizes.

## Proposed Layout

This is the proposed root coordination layout, not the final layout of every
module:

```text
docs/

dev-docs/
  README.md

  00-map/
    StackMap.md
    DecisionLog.md
    ModuleOwnership.md
    IntegrationContracts.md
    PlanIndex.md

  10-glade/
    kernel/
    declarations/
    records/
    exchange/
    lifecycle/
    transport/
    control-plane/
    provisioning/
    scale/

  20-glial/
    environment/
    application-management/
    orchestration/
    trust-capability/
    application-definition/

  30-grip-share/
    adapter/
    tap-mapping/
    terminal/
    files/
    agent-context/

  40-requirements/
    golden-path/
    silver-path/
    rapid-dev/
    harsh-reality/
    product-spaces/

  50-spikes/
    phase1-libp2p/
      README.md
      FrameSchema.md
      DiagnosticsSchema.md
      Observations.md
      Decision.md

  60-fixtures/
    griplab/
      declarations/
      generated-typescript/
      generated-python/
      canonical-records/
      tap-shapes/

  70-examples/
    griplab/
    frankenapp/
    bug-query/

  80-history/

  90-inbox/

plan-docs/
  README.md
  AGENTS.md
  Registry.md
  ActiveWork.md
  plans/
    GLP-####-short-name/
  archive/
    finished/
    abandoned/
    superseded/
  templates/
  support/

scratch/
```

For module repositories and submodules, use the smaller module-owned pattern:

```text
<module>/
  docs/
  dev-docs/
  scratch/
```

Root `plan-docs/` coordinates the work. Module `dev-docs/` owns the resulting
stable internal design.

## Directory Meanings

### `00-map`

Owns navigation, decisions, module ownership, integration-contract pointers,
and links into `plan-docs/`.

In the revised model, most planning documents move out of root `dev-docs/` and
into root `plan-docs/`. `00-map` SHOULD stay small and SHOULD mostly point to
module docs and active plans.

Agent ownership:
- one integration agent SHOULD own `StackMap.md`, `DecisionLog.md`,
  `ModuleOwnership.md`, `IntegrationContracts.md`, and `PlanIndex.md`
- all agents MAY propose decision-log entries, but one integration agent SHOULD
  apply final decision-log edits

### `10-glade`

Temporary staging area for the stable Glade substrate until a Glade module
exists:

- declaration language and package model
- canonical records
- exchange semantics
- live channels and append logs
- transport requirements
- lifecycle and retention
- provider claims, leases, routes, and projections
- control-plane mechanics
- scale modes

It MUST NOT own Grip UI state, product workflow, or app-specific feature design.

Target:
Most of this area SHOULD migrate into the future Glade module's `dev-docs/`.
The root repository SHOULD retain only cross-module references and migration
notes.

### `20-glial`

Owns environment composition above Glade:

- application definition collections
- session/workspace attachment
- application managers
- capability issuance policy
- service mounting
- orchestration rules

It MUST NOT own low-level record replication or transport framing.

### `30-grip-share`

Owns the Grip-to-Glade adapter boundary:

- tap mapping
- mock-to-real transitions
- terminal tap shape
- file/window tap shape
- agent-visible UI references

It MUST NOT own Glade kernel semantics or Grip/Grok local runtime internals.

### `40-requirements`

Owns product-facing requirements and pressure tests:

- GripLab golden path
- frankenapp silver path
- rapid development requirements
- harsh reality triage
- later product-space studies

These documents SHOULD explain why the architecture matters, not define low
level contracts.

### `50-spikes`

Owns disposable investigations.

In the revised model, spike plans and progress SHOULD live under
`plan-docs/plans/GLP-####-.../`. Spike outputs that are stable enough to
preserve MAY live in root `dev-docs/50-spikes/` temporarily, but SHOULD
ultimately move into the owning module's `dev-docs/` or into
`plan-docs/archive/finished/`.

Spike folders MUST distinguish:

- planned test surface
- frame/schema used by the spike
- raw observations
- final decision
- what the spike did not prove

Spike output MAY influence architecture docs, but spike docs MUST NOT become
canonical architecture by accident.

### `60-fixtures`

Owns concrete generated and canonical examples:

- `.glade` source fixtures
- generated TypeScript helper shapes
- generated Python provider shapes
- canonical record JSON fixtures
- hash/signature fixtures
- mock and Glade-backed tap shapes

This area is the primary decoupling point for parallel agents. Once a fixture
is agreed, JS, Python, Rust, Grip adapter, and console work can proceed with
less coordination.

### `70-examples`

Owns scenario examples that are useful but not necessarily canonical fixtures.

Examples SHOULD be promoted into `60-fixtures/` only when they become testable
contract inputs.

### `80-history`

Owns superseded documents.

Files in this area MAY contain useful design context, but MUST NOT be cited as
authoritative by new implementation work.

### `90-inbox`

Owns temporary documents that have not yet been classified.

The inbox SHOULD be drained regularly. Any file left here too long is a signal
that ownership is unclear.

## Root `plan-docs/`

`plan-docs/` is a new root-only coordination area.

It owns:

- plan instances
- active-work tracking
- plan registry state
- archived finished, abandoned, and superseded plans
- support material for plans
- multi-agent ownership splits
- progress and handoff notes

It does not own:

- stable subsystem architecture
- public documentation promises
- scratch-only analysis
- generated implementation artifacts unless they are explicitly plan support

Suggested structure:

```text
plan-docs/
  README.md
  AGENTS.md
  Registry.md
  ActiveWork.md
  plans/
    GLP-####-short-name/
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
  templates/
  support/
```

### `plan-docs/plans`

Contains plan instances that are being shaped, reviewed, executed, paused, or
waiting for merge.

Each plan folder SHOULD use a stable id:

```text
GLP-####-short-name/
```

Each plan SHOULD define:

- target module or `root`
- problem statement
- proposed work order
- agent boundaries
- acceptance gates
- open risks

### `plan-docs/archive`

Contains plans that are finished, abandoned, or superseded.

Archived plans SHOULD record:

- final outcome
- why the plan stopped if abandoned or superseded
- commits or branch references when available
- verification performed
- unresolved follow-up

### `plan-docs/templates`

Contains starter files for plan instances.

### `plan-docs/support`

Contains cross-plan supporting information:

- reviewer responses
- risk notes
- research summaries
- temporary design fragments
- migration inventories

Support docs are not authoritative by default. Stable content SHOULD be
promoted into module `dev-docs/` or public `docs/`.

## Multi-Checkout Agent Model

Expected execution model:

```text
checkout A: integration / roadmap / merge steward
checkout B: transport spike
checkout C: terminal contract and fixtures
checkout D: declaration compiler surface
checkout E: Grip adapter seam
```

This is why the structure must separate coordination from module docs.

Rules:

1. Each agent SHOULD have an assigned module, folder, or named file set.
2. Shared root files SHOULD be edited only by the integration agent unless the
   plan explicitly says otherwise.
3. Stable fixtures SHOULD be treated as cross-agent contracts.
4. Agents SHOULD write implementation-adjacent docs in the owning module, not
   in root, once that module exists.
5. Root `plan-docs/` SHOULD record dependencies, blocked states, and handoff
   requirements between agents.
6. Merge phases SHOULD prefer small contract docs and fixtures over giant
   speculative design edits.

## Proposed Agent Workstreams

| Workstream | Primary Write Area | Initial Output |
| --- | --- | --- |
| Roadmap and risk | `plan-docs/plans/GLP-####-.../` | roadmap, risk register, integration gates |
| Phase 1 transport | `plan-docs/plans/GLP-####-.../`, future Glade `dev-docs/transport/` | frame schema, diagnostics schema, observations, decision |
| Terminal golden thread | `plan-docs/plans/GLP-####-.../`, `30-grip-share/terminal/`, `60-fixtures/griplab/` | terminal contract, tap shape, generated helper fixture |
| Declaration compiler surface | future Glade `dev-docs/declarations/`, `60-fixtures/griplab/` | package model, generated TS/Python shapes |
| Canonical records | future Glade `dev-docs/records/`, `60-fixtures/griplab/canonical-records/` | canonical JSON/hash fixture |
| Control plane | future Glade `dev-docs/control-plane/`, Glial module `dev-docs/application-management/` | provider claim, lease, route, console projection contract |
| Capability and trust | Glial module `dev-docs/trust-capability/`, future Glade `dev-docs/records/` | capability chain and signature coverage |
| Requirements pressure | root or module `docs/` plus `plan-docs/support/` while unstable | golden/silver path acceptance and punt list |

## Migration Plan

### Step 1: Establish Root Planning Area

Create root `plan-docs/` with:

- `README.md`
- `AGENTS.md`
- `Registry.md`
- `ActiveWork.md`
- `plans/`
- `archive/`
- `templates/`
- `support/`

Acceptance:
- plan lifecycle is explicit
- active work has a place that is not subsystem `dev-docs/`

### Step 2: Add Root And Module Indexes

Create:

- `dev-docs/README.md`
- `dev-docs/00-map/README.md`
- `dev-docs/10-glade/README.md`
- `dev-docs/20-glial/README.md`
- `dev-docs/30-grip-share/README.md`
- `dev-docs/40-requirements/README.md`
- `dev-docs/50-spikes/README.md`
- `dev-docs/60-fixtures/README.md`

Acceptance:
- each index lists authoritative docs and owner boundaries
- no files have moved yet

### Step 3: Move Planning Documents

Move roadmap, spike, and risk-oriented documents into `plan-docs/` first
because they drive parallel execution and should not be confused with stable
subsystem design.

Candidate moves:

- `GLDevPlan.md` -> `plan-docs/plans/GLP-0001-moonshot-roadmap/Plan.md`
- `Phase1Libp2pTest.md` -> `plan-docs/plans/GLP-0002-phase1-libp2p/Plan.md`
- selected review actions -> `plan-docs/support/ReviewActions.md`

Acceptance:
- links are updated
- no architecture text is changed during the move

### Step 4: Move Root Integration Documents

Move only root-owned integration documents inside root `dev-docs/`:

- `StackMap.md` -> `dev-docs/00-map/StackMap.md`
- `DecisionLog.md` -> `dev-docs/00-map/DecisionLog.md`
- `DocStructurePlan.md` -> `dev-docs/00-map/DocStructurePlan.md`

Acceptance:
- root `dev-docs/` is clearly integration-owned
- plans are not mixed with stable docs

### Step 5: Move Stable Architecture Documents

Move existing Glade, Glial, Grip Share, requirements, and examples into their
target module folders when those modules exist.

Before the Glade module exists, root `dev-docs/glade/` remains a staging area.

Acceptance:
- every moved file has exactly one new home
- `StackMap.md` remains the navigation source of truth

### Step 6: Create Fixture Area

Create the first agreed fixtures:

- terminal `.glade` subset
- generated TypeScript helper sketch
- generated Python provider sketch
- canonical record JSON fixture
- terminal tap shape

Acceptance:
- each fixture can be handed to a different implementation agent
- fixture docs reference requirement IDs or decision IDs where relevant

### Step 7: Drain Legacy And History

Classify transitional documents:

- keep as authoritative by moving into a stable folder
- split into smaller documents
- move to `80-history/`

Acceptance:
- new implementation work does not need to read unclassified legacy docs

## Open Questions

| ID | Question | Proposed Default |
| --- | --- | --- |
| `DSP-001` | Should old `docs/` be merged into `dev-docs/80-history/`? | No, keep it separate until the new G* roadmap stabilizes. |
| `DSP-002` | Should `scratch/ReviewActions.md` be promoted? | Promote selected items into `00-map/RiskRegister.md`; do not commit scratch wholesale. |
| `DSP-003` | Should examples and fixtures stay separate? | Yes. Examples explain; fixtures constrain implementations. |
| `DSP-004` | Should every folder have an `AGENTS.md`? | Later. Start with write-ownership tables, add folder AGENTS files only when collisions occur. |
| `DSP-005` | Where do detailed Glade docs live before a Glade submodule exists? | Root `dev-docs/glade/` as staging, then migrate to Glade module `dev-docs/`. |
| `DSP-006` | Should plan docs live in `dev-docs/`? | No. Plans live in root `plan-docs/`; stable design lives in module `dev-docs/`. |
| `DSP-007` | Should plan lifecycle be represented by folders? | No. Status lives in `State.md` and `Registry.md`; folders archive only. |

## Immediate Recommendation

Do not move everything in one commit.

First create root `plan-docs/` and indexes, then move only the roadmap and
spike plan documents. After that, create module-owned `dev-docs/` locations as
modules become real boundaries.

That gives multiple agents stable work areas without making a large
rename-only diff impossible to review.
