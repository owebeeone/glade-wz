# BRST001 - Communication Shape Semantics

Status: brainstorm

Date: 2026-06-04

Purpose: capture the emerging idea that Glade declarations must describe the
semantic shape of data, not just the transport method used to deliver it.

This is deliberately exploratory. Nothing here is final syntax.

## Core Problem

GripLab exposed that words like `stream`, `log`, and `snapshot` are being used
too broadly. A backend may deliver a value over a WebSocket stream, but that
does not mean the semantic state shape is a stream.

Example:

```text
sessions.subscribe
```

Today this looks like a stream because it arrives through a subscription. But
semantically, terminal or command sessions are a keyed collection:

```text
TerminalSessions
  key: terminal_id or session_id
  value: one terminal/session record
  mutation: upsert, patch, delete, expire
```

The collection is the point. Delivery is secondary.

## Key Insight

Glade needs to distinguish:

- **state shape**: what kind of data exists
- **mutation shape**: how the data changes
- **delivery shape**: how a reader receives changes
- **cache shape**: how a runtime stores local state
- **ordering/cursor shape**: how reconnect, replay, and causality work
- **materialization shape**: what UI or consumer views are derived from it

If these are not declared, every adapter and tap will invent them locally. That
causes bugs in caching, invalidation, replay, and provider routing.

## Draft State Shapes

### `atom`

One logical value.

Examples:
- selected peer
- current view
- service connection state
- one settings object

Typical mutation:
- set whole value
- patch, if the value has structured fields

Typical cache:
- latest value

### `record_map`

A keyed collection of independently addressable records.

Examples:
- collaborators keyed by `peer_id`
- peer presence keyed by `peer_id`
- command sessions keyed by `session_id`
- terminal sessions keyed by `terminal_id`
- workspace repos keyed by `repo_path`

Typical mutation:
- upsert key
- patch key
- delete key
- expire key

Typical cache:
- latest value per key
- optional tombstones
- optional change history

This is distinct from an array snapshot. A list or table is usually a
materialized view over a `record_map`.

### `append_log`

An ordered sequence of immutable entries.

Examples:
- terminal output chunks
- command output chunks
- chat messages, if message history is append-only
- audit records

Typical mutation:
- append entry

Typical cache:
- append history
- cursor index
- segment or retention policy

### `mutable_document`

Content that can grow, shrink, and mutate by edits.

Examples:
- editable source file
- collaborative text buffer
- structured document

Typical mutation:
- edit operation
- patch
- CRDT operation, if multi-writer

Typical cache:
- document state
- operation history or CRDT state

### `window`

A bounded view over a larger state source.

Examples:
- file line window
- diff hunk window
- paged search results

Typical mutation:
- source-specific

Typical cache:
- window snapshot
- deltas inside the window
- generation/version metadata

### `derived_view`

A computed or materialized view over one or more canonical shapes.

Examples:
- terminal screen derived from terminal output bytes
- sorted session list derived from `record_map TerminalSessions`
- diff view derived from two file windows
- dependency graph derived from workspace repos

Typical mutation:
- none directly, except invalidation/recompute

Typical cache:
- latest materialized view
- derivation inputs and generation

### `live_channel`

An ephemeral interactive path.

Examples:
- terminal input
- terminal resize
- terminal close
- low-latency terminal output hot path

Typical mutation:
- control event
- input bytes

Typical cache:
- usually none
- optionally diagnostics/latest status

## Draft Delivery Shapes

Delivery is independent from state shape.

Examples:

- `get`: one read, no subscription
- `snapshot`: full value returned
- `snapshot_subscribe`: full current value sent whenever it changes
- `snapshot_plus_delta`: initial snapshot followed by edits/deltas
- `key_delta`: upsert/delete/patch events for a keyed collection
- `cursor_replay`: resume from last observed cursor
- `event_stream`: live events from now onward, no replay guarantee
- `poll`: repeated reads at runtime-controlled cadence
- `invalidation`: notification that a value changed and should be refetched

A `record_map` may be delivered as full snapshots in a legacy protocol, but the
adapter should still cache and expose it as a keyed map if the declaration says
that is the semantic shape.

## Draft Mutation Shapes

- `set`: replace whole value
- `patch`: structured partial update
- `upsert`: add or replace one keyed record
- `delete`: remove one keyed record
- `expire`: remove one keyed record because a lease/TTL ended
- `append`: add immutable log entry
- `edit_op`: mutate document content relative to version/cursor
- `control`: send command-like input to a live resource
- `claim`: claim responsibility or ownership
- `release`: release responsibility or ownership

## Draft Ordering And Cursor Shapes

- `none`: latest value only
- `per_key_version`: each keyed record has its own version
- `collection_generation_seq`: collection has a generation and sequence
- `provider_seq`: one provider sequences writes
- `owner_term_seq`: owner term plus sequence, useful for takeover safety
- `byte_offset`: cursor is offset in byte stream/log
- `content_id`: cursor is a content hash/CID/head
- `causal`: explicit causal references for multi-writer state

The cursor model must be declared. It should not be inferred from transport
delivery order.

## Why This Matters To The Adapter

The Glial/Glade adapter must interpret these semantics and choose correct local
behavior.

For `record_map`, the adapter should:
- cache by key
- apply upsert/patch/delete/expire per key
- derive lists, tables, trees, or filters as materialized views
- keep tombstones if the retention policy requires it
- replay missed key deltas if cursor delivery is available
- avoid replacing the whole collection unless the delivery event is declared as
  a full snapshot

For `append_log`, the adapter should:
- append entries immutably
- track cursor/offset/head
- coalesce entries if declared
- replay missed entries from cursor
- derive latest output, screen state, or summaries separately

For `mutable_document`, the adapter should:
- maintain document state
- apply edit operations against the declared version/cursor model
- reject or reconcile edits according to the declared authority and conflict
  policy

For `derived_view`, the adapter should:
- know that the view is not canonical
- recompute or invalidate from source changes
- avoid treating decoded UI state as durable source data

For `live_channel`, the adapter should:
- route by handle
- avoid promising replay unless paired with a log
- expose diagnostics separately from durable state

## GripLab Examples

### Collaborators

```glade
record_map Collaborators {
  key peer_id
  value schema "schemas/Collaborator.schema.json"

  mutate upsert, patch, delete
  deliver snapshot_plus_key_delta
  cache latest_per_key
  order per_key_version
  retain tombstones 7d
}
```

### Terminal Sessions

```glade
record_map TerminalSessions {
  key terminal_id
  value schema "schemas/TerminalSessionRecord.schema.json"

  mutate upsert, patch, delete, expire
  deliver snapshot_plus_key_delta
  cache latest_per_key
  order owner_term_seq
  retain tombstones 1d
}
```

### Terminal Output

```glade
append_log TerminalOutput {
  partition_by terminal_id
  entry schema "schemas/TerminalOutputChunk.schema.json"

  mutate append
  deliver cursor_replay
  cache append_history
  order owner_term_seq
  coalesce max_bytes 65536 max_latency 20ms
  retain 7d
}
```

### Terminal Session List

```glade
derived_view TerminalSessionList {
  source record_map TerminalSessions
  order by started_at desc
  filter hidden == false
  materialize list
}
```

## Open Questions

- Is `record_map` the right term, or should the DSL use `map`, `collection`,
  `entity_set`, or `table`?
- Is `append_log` distinct enough from `log`, or is `log` sufficient once state
  shape is explicit?
- Should delivery, mutation, cache, and ordering be separate clauses on every
  state shape, or should common bundles exist as presets?
- How much of this belongs in core Glade versus the Grip Share adapter layer?
- Should `derived_view` be a state shape, a declaration kind, or an annotation
  on a materialized projection?
- How should legacy full-snapshot subscriptions declare that they populate a
  semantic `record_map`?

## Next Work

1. Build a small taxonomy table from GripLab producers.
2. Rewrite the terminal `.glade` example using `record_map TerminalSessions`
   and `append_log TerminalOutput`.
3. Sketch adapter pseudocode that turns a declared `record_map` into a keyed
   cache plus generated Grip taps.
4. Add one compatibility example for a legacy `sessions.subscribe` full snapshot
   feeding a semantic `record_map`.

