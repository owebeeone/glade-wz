# Glade Kernel

Status: working draft

Purpose: define the minimum stable collaboration substrate that Glade owns.

## Core Claim

Glade is the share kernel.

Glade exists to provide a runtime-neutral model for:
- share identity
- share scope
- interest declaration
- bounded-work exchange semantics
- primary ownership
- shared projection
- resync and migration
- delegated capability boundaries

If a concept only exists because Grip currently has a particular runtime shape,
that concept does not belong in the Glade kernel.

## Non-Goals

Glade does not own:
- UI runtime execution
- application composition
- participant-facing session policy
- transport protocol details
- source-specific domain logic

Those concerns belong in other layers:
- local graph execution belongs to `Grip/Grok`
- app/session orchestration belongs to `Glial`
- integration mapping belongs to `Grip Share`

## Kernel Vocabulary

| Term | Meaning |
| --- | --- |
| `Share` | The stable public identity of one collaborative state domain. |
| `GraphSpace` | The concrete replicated state container for one `Share` in the current working model. |
| `Replica` | One local materialization of a `Share` or its control state. |
| `Interest Spec` | Declarative shared state describing what external mutable data the share wants. |
| `Source Binding` | One concrete binding inside an `Interest Spec`. |
| `Projection Envelope` | The shared materialized view derived from share state or source-backed execution. |
| `Ownership Term` | The fenced epoch for primary execution ownership. |
| `Delegated Capability` | A time-bounded, revocable right exposing a narrow reference or action surface to another participant. |
| `Control Record` | Explicit control-plane state for ownership, route, placement, or hosting. |

## Current Narrowing Discipline

The first Glade architecture slice SHOULD narrow before it generalizes:

- one `Share` maps to one `GraphSpace`
- one effective source binding exists per share for the first source-backed use
  case
- one fenced primary execution owner exists at a time
- source-derived projections are treated as replaceable derived state

These constraints are design discipline, not necessarily the final long-term
shape.

## Kernel Invariants

The Glade kernel MUST preserve the following invariants.

### 1. Identity

- A `Share` MUST have a stable identity that survives host changes and primary
  changes.
- Every `Source Binding` MUST have a canonical identity within its share.
- Every `Ownership Term` MUST be monotonic for one share.

### 2. Interest

- Interest in mutable external data MUST be represented as shared state.
- Interest declaration MUST be distinct from the derived projection produced
  from it.
- Effective interest MUST be reducible deterministically from participant
  contributions.

### 3. Ownership

- Primary execution ownership MUST be explicit control state.
- Ownership MUST be fenced by an `Ownership Term`.
- A follower MUST be able to determine whether projected data was produced by
  the current ownership term.

### 4. Projection

- The public projection contract MUST remain runtime-neutral.
- A projection MUST be consumable without replaying the producer's internal
  execution state.
- Projection state MAY include optimization metadata, but optimization metadata
  MUST NOT become the kernel's semantic contract.

### 5. Recovery

- A share MUST support snapshot hydration and incremental recovery.
- A follower MUST be able to rejoin a share without independently re-registering
  every upstream source.
- Migration and takeover MUST preserve the share's public identity.

### 6. Delegation

- Delegation MUST be capability-based, not only visibility-based.
- A delegated capability MUST be explicitly grantable and revocable.
- Delegation MUST be narrower than whole-session visibility by default.

## State Classes

Glade MUST treat the following state classes as different classes with
different semantics:

| State Class | Required Semantics |
| --- | --- |
| durable share state | stable identity, replayable, collaborative |
| source-interest state | deterministic reduction, explicit teardown rules |
| source-derived projection state | replaceable, generation-bounded, progressive |
| control state | fenced, authoritative, takeover-safe |

Glade SHOULD avoid treating these state classes as one generic replicated blob.

## Relationship To Other Layers

Glade sits below `Glial` and below `Grip Share`.

- `Glial` uses Glade to host product-visible shares and apply participation
  policy.
- `Grip Share` maps Grip runtime structures onto Glade's kernel model.
- `Grip/Grok` may be one producer and consumer runtime for Glade shares, but it
  does not define the kernel's meaning.

## Exchange Pattern Note

Request/reply is a valid mental model within Glade, but only as an exchange
pattern built from Glade share primitives rather than as a hidden RPC
primitive.

The current exchange semantics draft lives in
`/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeExchangeSemantics.md`.

## Immediate Kernel Questions

Active unresolved Glade questions are tracked in
`/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md`.
