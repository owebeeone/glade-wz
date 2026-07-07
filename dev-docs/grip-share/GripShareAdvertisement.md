# Grip Share Advertisement

Status: working draft

Purpose: define how a Grip application advertises which local Grip surfaces may
participate in a Glade share.

## Core Claim

A Grip app MUST advertise shareable participation surfaces before collaborators,
other Grip apps, or agents can safely attach to them.

The advertisement is not a capability grant. It is a typed map from local Grip
runtime concepts into shareable operations, projections, and source bindings.
Glial still decides which principals receive capabilities. Glade still validates
share records. Grip Share translates between local Grip behavior and those
distributed contracts.

## Why This Exists

Current Grip apps expose useful local handles such as `Lab.SelectedSession.Tap`.
Those handles are in-process controller objects. They are useful for React, but
they are not stable remote contracts.

A collaborator or AI agent needs a different view:

- which values may be read
- which inputs may be changed
- which controls may be invoked
- which outputs are provider-owned and read-only
- which source handle binds related logs, live channels, and materialized views

Without this layer, a runtime either shares too little or accidentally exposes
raw tap authority.

## Requirement IDs

- `GSA-001` A Grip Share advertisement MUST identify every advertised surface by
  explicit context path and Grip key when the surface maps to a Grip value.
- `GSA-002` A Grip Share advertisement MUST classify every advertised surface as
  `input`, `output`, `control`, `diagnostic`, `handle`, or `private`.
- `GSA-003` A Grip Share advertisement MUST declare state shape, delivery shape,
  and allowed mutation or control operations for each advertised surface.
- `GSA-004` A Grip Share advertisement MUST NOT expose raw in-process tap handle
  values as remote shared objects.
- `GSA-005` A Grip Share advertisement MAY map a local tap handle Grip to one or
  more serializable operations.
- `GSA-006` Provider-owned outputs MUST be advertised as read-only to ordinary
  participants and agents unless a narrower provider capability is explicitly
  granted.
- `GSA-007` Participant or agent writes MUST target advertised input, control,
  or interest surfaces, not provider result projections.
- `GSA-008` A surface that belongs to a concrete source MUST declare the source
  handle or handle field that binds related views and controls.
- `GSA-009` An advertisement MUST distinguish session-private, shared, and
  delegated visibility for each surface.
- `GSA-010` An advertisement MUST name the capability required for each allowed
  operation, not only for the surface as a whole.

## Advertisement Record Shape

The eventual canonical record format is open. The minimum semantic fields are:

| Field | Meaning |
| --- | --- |
| `advertisement_id` | Stable id for this adapter declaration. |
| `application_id` | App or facet bundle this advertisement belongs to. |
| `workspace_id` | Workspace security boundary. |
| `share_scope` | Share scope where the surface may be promoted or projected. |
| `context_path` | Grip context path for Grip-backed surfaces. |
| `grip_key` | Grip key, when the surface maps to a Grip value. |
| `surface_role` | `input`, `output`, `control`, `diagnostic`, `handle`, or `private`. |
| `state_shape` | `atom`, `record_map`, `append_log`, `live_channel`, `materialized_view`, or compatible shape. |
| `delivery_shape` | Snapshot, key delta, cursor replay, live stream, or equivalent delivery mode. |
| `operations` | Allowed read, write, mutation, or control operations. |
| `capabilities` | Capability required per operation. |
| `authority` | Who may produce or mutate the surface. |
| `visibility` | `private`, `session`, `shared`, or `delegated`. |
| `source_handle` | Stable source handle binding, if the surface belongs to a concrete source. |
| `legacy_mapping` | Optional mapping to current Grip taps, service methods, or snapshot streams. |

## Surface Roles

### `input`

Participant-controlled state that can change what the app requests or displays.

Examples:
- selected terminal session
- query filters
- file window
- selected peer

Inputs MAY be writable by humans, collaborators, or agents if capability allows.
Inputs SHOULD be modeled as source interest or view selection, not as provider
output mutation.

### `output`

Provider-produced or derived state.

Examples:
- database query result rows
- terminal output projection
- file snapshot
- collaborator health records

Outputs are normally read-only for collaborators and agents. If a participant
needs to affect an output, it SHOULD change the relevant input or interest
surface.

### `control`

An operation directed at a concrete source.

Examples:
- terminal input bytes
- terminal resize
- terminal close
- retry or refresh an async request

Controls MUST be serializable operations. They MUST be capability checked.
Controls SHOULD bind to a stable source handle when the operation targets a
specific resource.

### `diagnostic`

Status, error, progress, lease, or health information.

Diagnostics SHOULD be readable separately from content when policy allows.
Diagnostics MUST NOT imply authority to mutate the underlying source.

### `handle`

Stable identity for a source or shareable object.

Handles MAY be shared. In-process tap handles MUST NOT be shared directly.

### `private`

Local-only session or widget state.

Private surfaces MAY be advertised only to document that they are intentionally
not shareable by default. Promotion into a shared or delegated scope MUST be
explicit.

## Grip Handle Mapping

Grip's local pattern:

```text
Lab.SelectedSession -> value Drip
Lab.SelectedSession.Tap -> AtomTapHandle<string | null>
```

Grip Share's remote-safe pattern:

```text
surface selected_session
  grip Lab.SelectedSession
  role input
  allow set requires capability read_terminal
```

The local `Lab.SelectedSession.Tap` handle MAY implement the operation inside
one process. It MUST NOT be the remote object exposed to another process or
agent.

Async tap controllers follow the same rule. A local controller closure such as
`retry()` or `refresh()` MAY be mapped to an advertised control operation, but
the remote contract is the operation record and capability, not the closure.

## Sharing Semantics

Advertising a surface does not automatically make it shared.

A surface may be:

- `private`: visible only inside one local session
- `session`: visible to one attached session and its explicitly delegated agent
- `shared`: replicated or projected to collaborators in the share scope
- `delegated`: exposed to a narrower principal, usually an AI agent task

This matters for GripLab selections. `Lab.SelectedSession` can mean "my current
terminal" or "the shared terminal focus". Those are different collaboration
semantics even if a local UI currently uses one Grip key.

## Adapter Behavior

The Grip Share adapter SHOULD use advertisements to:

- generate follower taps that materialize readable projections
- generate local operation wrappers for writable inputs
- map local atom handles to serializable set/update operations
- map async controller handles to serializable retry/refresh/cancel operations
- hide private local UI handles from collaborators by default
- export only selected stable anchors or source handles to agents
- decide which Grip values need snapshot hydration and incremental replay

The adapter MUST keep Glade semantics stable. A non-Grip runtime should be able
to consume the Glade-facing surface without pretending to be Grip.

## GripLab Terminal Implication

For the terminal slice:

- `Lab.Sessions` is a materialized view over terminal session records, not the
  canonical terminal source.
- `Lab.SelectedSession`, `Lab.SessionSearch`, and `Lab.SessionFilters` are input
  surfaces.
- `Lab.SessionOutput` is a read-only output projection over terminal bytes.
- terminal input, resize, and close are control operations bound to the terminal
  source handle.
- `Lab.*.Tap` values are local handles and should not be advertised as remote
  shared objects.

This allows a collaborator or agent to participate by changing declared inputs,
reading declared outputs, and invoking authorized controls without gaining
authority to forge provider-owned results.

## Open Questions

- `GSA-OQ-001` What is the canonical Glade record kind for Grip Share
  advertisements?
- `GSA-OQ-002` How should a local Grip context path be stabilized across reloads,
  app refactors, and generated bindings?
- `GSA-OQ-003` Should shared focus surfaces use the same Grip key as local
  session focus, or should Grip Share synthesize separate shared-focus Grips?
- `GSA-OQ-004` How much type/schema information is imported from TypeScript
  Grip definitions versus declared explicitly in `.glade` files?
