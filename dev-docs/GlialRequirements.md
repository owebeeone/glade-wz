# Glial Requirements

Status: transitional draft

Purpose: preserve the earlier consolidated Glial requirements framing while the
scoped stack documents absorb its authoritative content.

Transitional note:
- The authoritative stack split now lives in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/StackMap.md`.
- Core share-kernel requirements now belong primarily in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeRequirements.md`.
- Rapid-development requirements now belong in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/RapidDevEnvironment.md`.
- This document remains useful source material until the scoped docs fully
  absorb its authoritative content.

Non-goals:
- This is not the final architecture.
- This does not define transport protocols.
- This does not define storage formats in detail.

## 1. Framing

Glial SHOULD be described as a system of shares, scopes, projections, and
execution ownership.

Glial SHOULD NOT be framed primarily as:
- a browser issuing imperative requests to a server
- a server replying with ad hoc application payloads
- individual taps being remotely mirrored one-to-one as transport objects

The share-oriented mental model is:

- participants attach to a `HybridAppInstance`
- each instance exposes one or more share scopes
- share scopes carry both interest declarations and materialized projections
- primary execution owners perform side-effectful or upstream work
- followers consume replicated projections

## 2. Requirement Set

## Share Model

- `GLR-001` Glial MUST model collaboration through explicit share scopes rather
  than implicit cross-session coupling.
- `GLR-002` A share scope MUST have an explicit ownership and visibility policy.
- `GLR-003` State MUST declare whether it is session-private,
  principal-private, instance-shared, or control-plane state.
- `GLR-004` A `GraphSpace` MUST be the primary concrete container for shared
  application state unless a later architecture explicitly introduces a more
  granular container.

## Session and Participant Model

- `GLR-010` A `HybridAppInstance` MUST support multiple attached sessions.
- `GLR-011` A session MAY be human-facing or headless.
- `GLR-012` A headless agent session MUST be attachable to the same
  `HybridAppInstance` as human sessions without being treated as a special
  transport case.
- `GLR-013` Sessions MUST be able to see different scopes of the same instance
  based on policy.

## Interest Model

- `GLR-020` Interest in a mutable external data source MUST be representable as
  shared state.
- `GLR-021` The shared declaration of desired upstream state MUST be distinct
  from the materialized values produced from it.
- `GLR-022` Glial MUST support more than one session contributing to one
  effective shared interest state.
- `GLR-023` A share MAY contain interest in a source even before materialized
  source data is available.
- `GLR-024` A share MUST be able to carry stable references such as anchors,
  ranges, highlights, or selection markers when those references are
  intentionally shared or delegated.

## Upstream Registration Ownership

- `GLR-030` For a mutable source that SHOULD NOT be subscribed to independently
  by every replica, Glial MUST designate one execution owner for upstream
  interest registration.
- `GLR-031` The execution owner for such a source MUST normally be a primary tap
  or equivalent primary execution role.
- `GLR-032` Non-primary followers MUST consume the resulting shared projection
  instead of duplicating upstream registration by default.
- `GLR-033` Glial MUST support at least the current execution modes:
  `replicated`, `origin-primary`, and `negotiated-primary`.
- `GLR-034` Primary ownership MUST be explicit control state, not a hidden
  client convention.

## Projection and Materialization

- `GLR-040` Shared projections MUST be addressable by explicit context path and
  grip key.
- `GLR-041` Shared projections MUST include enough tap metadata for followers to
  materialize passive local views.
- `GLR-042` Followers MUST be able to consume a shared projection without
  replaying the full internal state of the original tap implementation.
- `GLR-043` A share SHOULD support both value projection and source-binding
  projection.

## Progressive Delivery

- `GLR-050` Glial MUST support progressive delivery of source-driven shared
  values.
- `GLR-051` Progressive delivery MUST preserve a coherent order within one
  source-generation stream.
- `GLR-052` When the effective source binding changes, Glial MUST create a new
  generation boundary so old and new source results are not mixed silently.
- `GLR-053` Followers MUST be able to render partial results before final
  completion when the producing tap supports progressive output.
- `GLR-054` Error, stale, and loading state MUST be representable as shared
  projection state, not only as local UI state.

## Private State and Delegation

- `GLR-060` Private session state MUST remain private by default.
- `GLR-061` Glial MUST allow explicit export of selected private references into
  a shared or delegated scope without forcing all private UI state to be shared.
- `GLR-062` A delegated agent-visible scope MUST be narrower than full session
  visibility unless policy explicitly grants broader access.
- `GLR-063` Delegated references SHOULD use stable anchors or canonical ids
  rather than ephemeral widget-local handles where possible.

## Persistence and Resync

- `GLR-070` A share MUST support snapshot-based hydration plus incremental
  change replay.
- `GLR-071` Shared state and shared projection state MUST both be recoverable
  after disconnect or handoff.
- `GLR-072` A follower MUST be able to rejoin a share without requiring every
  upstream source to be re-registered locally.
- `GLR-073` Glial MUST distinguish local pending change state from confirmed
  shared state.

## Control and Placement

- `GLR-080` Primary selection, route visibility, and placement MUST be modeled
  as control-plane state rather than application content.
- `GLR-081` App content state and control-plane state MUST remain separate even
  when both are eventually replicated.
- `GLR-082` A share MUST be movable between hosts or primaries without changing
  its public identity.

## Boundary Requirements

- `GLR-090` `grip/grok` MUST remain responsible for local graph execution, not
  mesh routing.
- `GLR-091` `glymik` MUST remain responsible for replication state and
  convergence, not session policy or transport identity.
- `GLR-092` `glial-mesh` MUST remain responsible for peer connectivity, relay,
  and placement, not app-level share semantics.
- `GLR-093` `glial` MUST remain responsible for share policy, session policy,
  source-binding semantics, and mapping app concepts onto graph spaces.

## 3. Current Design Implications

The current code already points toward these requirements:

- local changes are addressed by session id, context path, and grip key
- shared projections already carry contexts, drips, tap metadata, and leases
- tap execution modes already distinguish replicated vs primary-owned execution

The missing architectural step is not basic synchronization. The missing step
is a first-class, share-oriented model for:
- interest declaration
- upstream registration ownership
- progressive source delivery
- delegation of selected private references

## 4. Immediate Design Questions

The next architecture pass MUST resolve the open decisions tracked in
`/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md`.
