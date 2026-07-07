# Glial Topology

Status: transitional draft

Purpose: preserve the earlier consolidated Glial mechanical topology while the
scoped stack documents absorb its authoritative content.

Transitional note:
- The authoritative stack split now lives in
  `/Users/owebeeone/limbo/glade-wz/dev-docs/StackMap.md`.
- Most of the mechanics in this document are now treated as `Glade` mechanics
  and are being carried forward into
  `/Users/owebeeone/limbo/glade-wz/dev-docs/glade/GladeKernel.md`.
- This document remains useful source material until the scoped docs fully
  absorb its authoritative content.

Non-goals:
- This is not a full system architecture document.
- This does not define transport channels or protocol frames.
- This does not yet freeze the final production package layout.

## Scope

This document only covers the topological mechanics that Glial must reason
about:
- what contains what
- who relates to what
- which state is shared vs private
- who is interested in what
- where primary responsibility lives

## 1. Layer Topology

| Layer | Mechanical Responsibility |
| --- | --- |
| `grip/grok` | Local graph execution, context resolution, taps, drips, and value propagation |
| `glymik` | Replication state, replica lifecycle, convergence, outbound packet production, inbound packet application |
| `glial-mesh` | Peer identity, route distribution, packet relay, placement coordination, shared flush policy |
| `glial` | HybridApp composition, session lifecycle, share policy, source binding semantics, and mapping app concepts onto graph spaces |

## 2. Runtime Containment Topology

Current working containment model:

```text
HybridApp
  -> HybridAppInstance
    -> Session (human or headless)
    -> GraphSpace
      -> Replica
        -> Grok
          -> Context graph
            -> Taps / Drips / Grips
```

Notes:
- A `HybridAppInstance` may contain multiple `Session` values.
- A `HybridAppInstance` may contain multiple `GraphSpace` values.
- A `Session` attaches to one or more `GraphSpace` values.
- A `Replica` is the local materialization of one `GraphSpace`.
- One `Grok` runtime is bound to one local replica view.

## 3. Share Topology

Glial must reason about sharing separately from session attachment.

### 3.1 Primary share units

- The current primary Glial share unit is the `GraphSpace`.
- A `Share Scope` is the collaboration boundary that product and application
  design reason about.
- A share scope SHOULD map to one explicit `GraphSpace` unless there is a clear
  reason to use multiple graph spaces for one share.

### 3.2 Scope classes

Current working scope classes:

- `session-private`: visible only to one attached session unless explicitly
  exported
- `principal-private`: visible to one principal or a tightly delegated set of
  sessions
- `instance-shared`: visible to multiple sessions attached to the same
  `HybridAppInstance`
- `control`: route, lease, and placement state rather than app data

### 3.3 Sharing rule

State MUST not become shared only because two sessions happen to name the same
grip. Sharedness is determined by:
- explicit placement inside a shared `GraphSpace`, or
- explicit export from a private scope into a shared scope

## 4. Relation Topology

Glial needs a clear model of data-source and data-consumer relations.

### 4.1 Local consumer relation

At the local runtime level:
- UI components and local agents express interest to `Grok`
- `Grok` resolves that interest to taps
- taps publish values into drips

This is local execution topology, not yet shared-interest topology.

### 4.2 Shared relation

At the Glial level:
- a share scope carries `Interest Spec` state
- one or more sessions may contribute to the effective interest
- one primary tap or equivalent execution owner interprets that interest
- followers consume the resulting `Shared Projection`

### 4.3 External source relation

For mutable external sources:
- the shared thing is normally the `Interest Spec`, not the tap object itself
- the primary execution owner performs upstream registration
- followers consume shared projected values instead of duplicating upstream
  registration by default

## 5. Interest Topology

Interest exists in three distinct places and MUST not be conflated.

### 5.1 Local interest

Local interest is:
- one local consumer asking its local `Grok` for a grip

This determines what the local runtime wants to observe now.

### 5.2 Shared interest

Shared interest is:
- replicated Glial state declaring what a collaboration scope wants to observe

Examples:
- which document a report editor is currently bound to
- which range or query window is active
- which highlight or selection anchor is intentionally delegated

### 5.3 Upstream interest

Upstream interest is:
- the actual registration performed against an external mutable source

This registration is owned by the primary execution owner for the relevant
binding, not by every follower replica.

## 6. Projection Topology

Glial currently leans toward projection sharing rather than full tap-state
sharing.

Current shared projection shape:
- `contexts[path]`
- `drips[grip_id]`
- `taps[tap_id]`

This means a follower can reconstruct:
- the shared context tree
- the currently projected grip values
- enough tap metadata to materialize passive follower taps

The current implementation already serializes shared values by context `path`
and `grip_id`, not by grip id alone.

## 7. Authority Topology

Authority is layered.

### 7.1 App authority

`glial` decides:
- which `HybridAppInstance` exists
- which sessions may attach
- which graph spaces exist
- which share scopes are visible to which participants

### 7.2 Execution authority

`grip/grok` decides:
- which local tap resolves a grip request
- how local values propagate
- whether a tap can execute locally under its current execution role

### 7.3 Replication authority

`glymik` decides:
- how a replica applies inbound replicated state
- how local changes become outbound replication packets

### 7.4 Control authority

`glial-mesh` decides:
- where traffic and work should go
- which node currently hosts a graph space or headless session
- which node currently holds relevant route or placement claims

## 8. Control Topology

Glial mechanics depend on a separate control topology.

Control state includes:
- route claims
- placement leases
- capability claims
- hosting claims

Control state is not app content and not ordinary shared document state.

Current working rule:
- app state lives in `GraphSpace`
- control state lives in `ControlSpace`

## 9. Current Implementation Anchors

The current codebase already contains the following partial topology:

- local or shared sessions
- persisted change records addressed by `session_id + path + grip_id`
- shared session snapshots containing contexts, drips, and taps
- tap leases for choosing a primary execution owner
- passive follower taps that materialize shared projected values

These pieces are useful because they already separate:
- local graph execution
- shared projected state
- primary ownership

They do not yet define a full first-class shared-interest model for mutable
external data sources.

## 10. Open Boundary Decisions

Active unresolved topology decisions are tracked in
`/Users/owebeeone/limbo/glade-wz/dev-docs/DecisionLog.md`.
