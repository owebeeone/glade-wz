# Taut Shape Catalogue — Glade/GWZ adoption record

Status: **adopted 2026-08-28** (`GDL-041`)

This workspace adopts the canonical catalogue from
`taut-dev/dev-docs/TautShapeCatalogDecision.md` at commit
`10dc0eba5f81d2ee92c0b09754b68a9fcc8148b7` (SHA-256
`ab29cd16aaa6e1089229f8dc9e4691004d62a3b1e0d43c9052eb3f0b72110573`).
The source decision remains the authority for semantic detail. This record fixes
its consequences for Glade, Glial, and GLP-0006.

The coordinated `0.9.*` release is recorded in
`taut-shape/dev-docs/TautShapeReleaseCompatibility.md`: contract `v0.9.0`, Rust
`taut-shape 0.9.0`, and TypeScript/Python `taut-shape 0.9.1`.

## Adopted catalogue

| Public name | Class | GWZ interpretation |
| --- | --- | --- |
| `unary` | interaction kind | One request and one whole response; it owns no delivery engine. |
| `value`, `atom`, `log`, `stream`, `swmr`, `crdt` | engine shapes | Each name resolves to its exact versioned engine and corpus. |
| `snapshot_delta` | profile over `swmr` | Expired reconstruction uses out-of-band refresh; it MUST NOT be a silent alias for ordinary SWMR reset/repair. |
| `text_crdt` | profile over `crdt` | Text identity, merge, resume, and convergence semantics; public spelling uses an underscore. |
| `exchange` | Glade service interaction | Dedicated directed/correlated request-response path; it MUST NOT enter a delivery fold. |
| `message` | reserved/unsupported | It MUST be rejected until a separately ratified interaction contract exists. |
| `window` | application view/projection | It MUST declare an explicit base delivery shape, such as `swmr` or `log`; it is not a Taut shape. |

Retention, expiry, range selection, and application view identity are distinct
concerns. A retention token MUST NOT create or imply a delivery shape.

## Current capability boundary

Catalogue recognition does not grant runtime capability.

- The Glade node and its Rust/TypeScript clients currently accept `value` and
  `log` for durable binding/fold paths. They reject every other delivery name
  before registration, store, or chain mutation.
- Glade serves `exchange` through its existing provider/service path, never
  through fold dispatch.
- Glial has tested standalone adapters for `atom`, `stream`, `crdt`, and
  `text_crdt`, as well as `value` and `log`. Its durable binder/mount path remains
  exactly `value`/`log` until a versioned Glade adapter is added.
- `swmr` and `snapshot_delta` have released portable engines and corpora but no
  Glade/Glial binding adapter yet.
- Terminal live I/O remains Glade's ordered channel mechanism. It MUST NOT be
  relabelled as the Taut `stream` engine without a versioned adapter contract.

## Normative GWZ requirements

| ID | Requirement | Present evidence |
| --- | --- | --- |
| `GSC-01` | A fold or durable mount MUST dispatch only to an exact implemented adapter; current Glade/Glial folds are `value` and `log`. | `glade/client-ts/test/session.test.ts`; `glade/client-rs/src/session.rs`; `glial/test/shapes.test.ts` |
| `GSC-02` | `exchange` MUST remain a separate service path and MUST NOT be folded. | `glade/client-rs/tests/integration.rs`; `glial/test/shapes.test.ts` |
| `GSC-03` | `message` and `window` MUST be rejected as delivery shapes. | `glade/node/src/appdecl.rs`; `glade/client-ts/test/session.test.ts`; `glial/test/shapes.test.ts` |
| `GSC-04` | Unknown, unimplemented, or unsupported capabilities MUST fail before registration or mutation. | `glade/dev-docs/GladeShapeDispatch.md`; the three dispatch suites cited above |
| `GSC-05` | A profile MUST reuse its canonical engine core and MUST retain its own conformance rows. | `taut-shape/release/compatibility.v1.json`; `taut-shape/corpus/` |
| `GSC-06` | Every application view MUST name an explicit base delivery shape and recovery policy. | GLP-0006 P3 and `dev-docs/glade/suppliers/glade-files.md` |
| `GSC-07` | A new Glade adapter MUST pin a contract/corpus version and MUST add node, client, and Glial success, failure, and edge-case gates before declarations are accepted. | `glade/dev-docs/GladeShapeDispatch.md`; GLP-0006 checkpoints |
| `GSC-08` | Retention MUST remain separate from shape and view identity. Ambiguous policies MUST be resolved explicitly, not inferred. | `glade/dev-docs/GladeShapeDispatch.md` (`term.log` follow-up) |

## Roadmap consequences

1. Taut-shape contract creation and the three-language value matrix are complete;
   roadmap work moves to Glade/Glial adapters and live integration.
2. GLP-0006 P3.S1 is an `swmr` adapter plus a file-window projection. The D8
   guarantees remain: viewport-first delivery, background backfill, and one
   coherent `{workspace_id, path, revision}` generation.
3. `snapshot_delta` is used only when expiry deliberately requires an
   out-of-band full refresh. The default mutable-file plan uses SWMR's typed
   in-band reset/repair.
4. GLP-0006 P4.S1 is `crdt`/`text_crdt` Glade transport and binder integration,
   not contract invention. H-P4's simultaneous-editing and cursor guarantees
   remain unchanged.
5. P3/P4 capability work MAY proceed independently of the P2 governance rulings,
   but no effect supplier may bypass the B1–B5 security substrate.

## Acceptance gates for a newly exposed shape/profile

Before Glade accepts a new delivery name, all of the following MUST be true:

1. the contract version, corpus version, operations, and recovery behavior are
   pinned as an exact adapter capability;
2. the Glade node rejects malformed and wrong-shape declarations before state
   mutation and passes a positive end-to-end route;
3. Rust and TypeScript clients pass matching positive and fail-closed cases;
4. Glial passes adapter, durable-store boundary, reconnect, and consumer-event
   tests; and
5. the owning GLP checkpoint proves the application-level invariant (for files,
   generation coherence; for editing, convergence and cursor stability).
