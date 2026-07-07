# Glade Requirements

Status: working draft

Purpose: define the minimum technical requirements for the Glade share kernel.

## Framing

Glade MUST be specified as a small, stable substrate for shares rather than as
a UI runtime extension or a transport-specific server model.

Glade SHOULD be small enough that another local runtime besides Grip could map
onto it without inheriting Grip internals.

## Requirement Set

## Identity and Container Model

- `GLA-001` A `Share` MUST have a stable public identity.
- `GLA-002` The public identity of a share MUST remain unchanged across host
  movement, primary transfer, or reconnect.
- `GLA-003` The current first architecture slice MUST assume one `Share`
  corresponds to one `GraphSpace`.
- `GLA-004` Any later relaxation of `GLA-003` MUST define cross-space identity,
  versioning, and recovery semantics explicitly.
- `GLA-005` Each share-backed source MUST declare whether the source of truth is
  Glade-managed state, an external authoritative system, or a hybrid model.

## Interest Model

- `GLA-010` Interest in mutable external data MUST be representable as shared
  state.
- `GLA-011` Shared interest declaration MUST be distinct from shared derived
  projection.
- `GLA-012` Contributions from multiple participants MUST reduce to one
  deterministic effective interest state.
- `GLA-013` The reduction rule for effective interest MUST be explicit for each
  source kind.
- `GLA-014` Withdrawal of participant interest MUST have explicit teardown and
  retention semantics.

## Ownership and Fencing

- `GLA-020` Upstream registration or other side-effectful share execution MUST
  have one explicit owner at a time.
- `GLA-021` Ownership MUST be represented as fenced control state.
- `GLA-022` Ownership state MUST carry a monotonic term or epoch.
- `GLA-023` Derived projection data MUST be attributable to an ownership term.
- `GLA-024` Takeover behavior MUST define stale-owner detection, duplicate-owner
  handling, and replay expectations.

## Projection Contract

- `GLA-030` Glade MUST expose a runtime-neutral projection envelope.
- `GLA-031` The projection envelope MUST support progressive delivery.
- `GLA-032` Progressive delivery MUST define generation boundaries so results
  from different source bindings are not silently mixed.
- `GLA-033` Projection state MUST represent loading, partial, stale, complete,
  and error states explicitly.
- `GLA-034` Followers MUST be able to consume the projection envelope without
  replaying producer-specific internal state.

## Exchange Semantics

- `GLA-035` Glade MUST support a bounded-work exchange pattern as shared state
  rather than as a hidden request transport primitive.
- `GLA-036` Exchange semantics MUST distinguish intent, ownership, execution,
  publication, observation, and diagnostics as separate planes.
- `GLA-037` Exchange publication MUST be distinguishable from requester
  observation.
- `GLA-038` Retry MUST create a new attempt identity rather than mutating the
  original logical request identity.

## Failure, Resync, and Migration

- `GLA-040` Glade MUST define behavior for disconnect, reconnect, takeover, and
  host migration.
- `GLA-041` A follower MUST be able to rejoin a share from snapshot plus
  incremental state.
- `GLA-042` Rejoin MUST NOT require every follower to recreate upstream source
  registrations independently.
- `GLA-043` Migration MUST preserve share identity and current effective
  interest state.
- `GLA-044` Failure semantics MUST be defined for mid-stream primary loss and
  generation rollover.

## Delegated Capability Model

- `GLA-050` Delegation MUST be capability-based rather than visibility-only.
- `GLA-051` A delegated capability MUST be narrow, attributable, and revocable.
- `GLA-052` Delegated references SHOULD use stable anchors or canonical ids
  where possible.
- `GLA-053` Capability revocation MUST define how stale references are handled.

## Source Taxonomy

- `GLA-060` Glade MUST distinguish at least snapshot, stream, windowed-query,
  and mutable-object source kinds.
- `GLA-061` Each source kind MUST define its own reduction, hydration, replay,
  and invalidation semantics.
- `GLA-062` Glade MUST NOT assume one generic source-binding behavior is correct
  for all source kinds.

## Observability and Cost Discipline

- `GLA-070` The system MUST expose enough state to inspect effective interest,
  current owner, current ownership term, current generation, and rejoin status.
- `GLA-071` The system SHOULD expose lag and pending-versus-confirmed state.
- `GLA-072` The architecture MUST define bounds or controls for projection
  size, replay cost, and upstream subscription cardinality.

## Boundary Requirements

- `GLA-080` Glade MUST remain independent of Grip tap, drip, and context
  internals at the semantic contract level.
- `GLA-081` Glade MUST remain independent of app/session orchestration policy.
- `GLA-082` Glade MUST remain independent of transport protocol selection.

## Immediate Direction

The next architecture pass SHOULD prefer fewer mechanisms with harder
invariants over more vocabulary with softer boundaries.

Open unresolved calls are tracked in
`/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md`.
