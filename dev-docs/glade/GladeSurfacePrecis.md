# Glade Surface — Précis

Status: working draft (distillation)

Purpose: state, tightly, *what Glade needs to do* at the declarative surface.
This is the distilled mental model; the full language lives in
[GladeDeclarationModel](GladeDeclarationModel.md), the exchange pattern in
[GladeExchangeSemantics](GladeExchangeSemantics.md). It does not pick a
substrate — substrate selection is deferred (see the Rust-P2P research,
`../GLRustiesP2PStory.md`); the whole point is that the substrate sits behind
this surface and is swappable.

## Core claim

Glade lets you **declare** a piece of shared data along **three orthogonal
axes**, and then provides the behaviour, visibility, and sync for it — without
the consumer ever knowing *how* the data got there. A declaration is the
product of:

1. **Scope** — who can see/do which bits, and when.
2. **Semantic flow type** — *not what the data is*, but how it is read /
   written / updated / deleted, and how it presents.
3. **Value type** — what the data actually is (struct, int, str, …).

These three are independent. You pick one of each. The runtime composes them and
hides the substrate behind a provider seam (mock→real, JS→Rust, OrbitDB→Yjs→…)
with no consumer rewrite.

```
declare( value-type  ×  flow-type  ×  scope )  →  a tap the consumer reads/writes
                                                   (substrate hidden & swappable)
```

## Axis 1 — Scope: sessions → scopes → apps (who sees what, when)

The visibility/authority graph. It answers: **what bits, who, and when.**

- **app** — a declarative `.glade` surface: a set of data declarations +
  their providers/consumers. The unit of "what is shared."
- **scope** — a visibility/sharing boundary: a named region of shared state with
  access rules. Governs read / write / observe authority over the bits inside
  it. (Maps to `DeclarationSpace` + `Capability`/`Policy` in the declaration
  model.)
- **session** — a participant in one or more scopes: a principal, live, with a
  lease. This is the "who" and the "when" (liveness/presence).

So: **sessions** (who, live) participate in **scopes** (what's shared, under what
authority) which compose **apps** (the declarative surfaces). Characterizing
"what bits who can see and when" = scope membership × capability/policy ×
lease/lifetime. **This axis needs the most sharpening** — the exact
session/scope/app nesting and the per-bit visibility/authority rules are the
open design work (grounded on Genesis/Capability/Policy from the declaration
model).

## Axis 2 — Semantic flow type (how it reads/writes/updates/deletes + presents)

The canonical set. This axis — not the value type — is what selects the
substrate behaviour per node. Four kinds:

| Flow type | read | write | update | delete | presents as | persistence | sync |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **atom** | get latest | set | replace/merge | remove | a value (one signal/tap) | cached, latest-wins | replicate current value (small); LWW |
| **log** | iterate / replay from cursor | append | — (entries immutable) | — | a replayable list with history | durable append-only | deltas after declared cursor (don't re-pull); ordering scope explicit |
| **editable blob** | current materialized state | apply edit | apply edit | remove | a converging live document | CRDT state + ops | CRDT deltas (don't re-pull); multi-writer convergent |
| **stream** | subscribe to live now | emit | — | — | a transient event stream | none (ephemeral) | best-effort live; no history/replay |

Maps to existing content-model kinds: atom ≈ a mutable register; **log** ≈
`model:AppendLog`; **editable blob** ≈ `model:CrdtText` / `model:MutableFile`;
**stream** ≈ an ephemeral live channel (e.g. `model:TerminalPty` keystrokes).

Key consequence — **one source often composes two flow types.** A terminal is a
*stream* (live output, ephemeral) **and** a *log* (durable replayable history)
over the same source: the live/replay split. Glade must let a source expose
more than one flow-typed view.

This means "source" is not the same thing as "flow." A source is the real
resource or state producer, such as one PTY, one file, one chat room, or one
command run. Flow-typed declarations are views over that source. The source
MUST expose a stable handle when a consumer needs to bind several views to the
same resource. For a terminal, the open operation returns a terminal handle,
and that handle keys:
- the interactive live channel
- the live output stream
- the durable output log
- any materialized terminal-screen view

A materialized view is a derived presentation over a source or flow. It MAY be
cached and consumed as an atom or snapshot stream, but it MUST NOT replace the
canonical flow when history, replay, or authorization depends on the canonical
flow. For example, a decoded terminal screen is a materialized view over raw
terminal-output bytes; it is not the durable output log.

The substrate question (OrbitDB / Yjs / Iroh / Rust core) is *only* "what
implements each flow type behind the seam" — atom/log by a cache+delta-log,
editable-blob by a CRDT (yrs/Yjs/Loro), stream by a live channel. Swappable.

## Axis 3 — Value type (what the data is)

The payload schema: struct, int, str, bytes, nested records, etc. (Maps to
`schema_ref`/inline schema on a `Definition`.) Independent of how it flows — an
`int` can be an atom or appended to a log; a `struct` can be an editable blob or
a stream event.

## What Glade must do (runtime responsibilities)

1. **Resolve declarations** — given a tap declared as (value-type × flow-type ×
   scope), bind it to a provider that implements that flow type in that scope.
2. **Enforce scope/visibility/authority** — gate read/write/observe per the
   session's capabilities within the scope; honour lifetime/lease/retention.
3. **Provide per-flow-type behaviour** — the read/write/update/delete +
   presentation contract for atom/log/editable-blob/stream, including the
   live/replay split where a source exposes both.
4. **Hide and swap the substrate** — the provider seam: consumers never see the
   sync/persistence engine; it can be mocked, or swapped (JS↔Rust, engine↔engine)
   with no consumer change.
5. **Deliver declaratively to consumers** — the tap just gets "this kind of data,
   shared this way," not a transport.

## Boundary

This is the surface distillation. It does **not** define the wire envelope
([GladeRecordEnvelope](GladeRecordEnvelope.md)), the full declaration language
([GladeDeclarationModel](GladeDeclarationModel.md)), the exchange/request-reply
pattern ([GladeExchangeSemantics](GladeExchangeSemantics.md)), the access-control
cryptography, or the substrate. The substrate is deliberately deferred and
swappable — that is why this surface can be designed now, independent of the
Rust-vs-JS-vs-Go question.
