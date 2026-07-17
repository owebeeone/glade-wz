# glade-editing — collaborative editing over glade (supplier spec)

Status: full spec v1 (2026-07-12, rev CRDT) — expands the `SupplierOutlines.md`
entry. The editing-shape gate is **RULED: text CRDT for v1** (`RulingWorksheet.md`
§V H-P4; single-writer/multi-reader is **NOT** an allowed fallback), so this doc
LANDS that ruling, it no longer weighs it. Common contract:
`GladeSupplierModel.md`. Context: s-zones / `GladeZones.md` (private-zone
selections, IMPLEMENTED 2026-06-14) · AZ-16 (membership = commons + your private
zone) · B4 (symbolic `self:` derived from the authenticated principal) ·
`GlialClientRuntime.md` rule 3 + GC-1/GC-2 + glial-DecisionLog GAP-8 (the
ChangeEvent delta path this supplier motivates) + A1 (the identity-set logDelta
fix) — the identity-based, out-of-order-tolerant delta is a **SHARED glial
primitive**, not editing-local (§3).

## 1. The editing shape — text CRDT (H-P4)

The P4 gate asked ONE thing: does "edit live, neither losing our cursor" need
simultaneous keystrokes? **RULED (H-P4): yes — v1 is a text CRDT.** swmr
(single-writer-multi-reader over `log` + an `EditClaim` write-lease) is **not an
allowed fallback**. It was overruled because **its write-fence was never cheap**:
a real fence needs an `EditClaim` schema, an epoch on EVERY body op, stale-writer
reject, and accepted-head handoff (the SR56-2-35 finding). Once you pay for the
fence the single-writer story stops saving substrate — and it still can't do the
concurrent case the done-criterion wants. So CRDT is v1, outright.

### 1.1 The Shape
`doc.body` is a first-class **`text-crdt`** Shape, added to the glade Shape enum
alongside `value·log·message·stream·exchange·window…` (today `text-crdt` is named
only in glial's fold-engine list, NOT yet declared — this ruling promotes it to a
declared Shape). The P4-gate taut-shape `text-crdt` contract is now **REQUIRED for
v1**, not deferred to a v2 (§7, §10).

### 1.2 First-class identities + operations (not an opaque patch blob)
The protocol names identities and operations; it never ships a diff blob:
- **element IDs** are `{actor_id, counter}` (a Lamport-style pair) carrying
  **causal dependencies** — the ops each element saw when it was created;
- **insert** names an **anchor** (the element it goes after) plus the new
  element id;
- **delete** names element ids and creates **tombstones** — elements never
  leave, they are marked dead so a concurrent reference stays valid;
- **duplicate ops are idempotent** — replaying an op the fold already holds is a
  no-op (dedup by element id);
- **concurrent siblings** inserted at the same anchor take a **deterministic
  order** (tie-break on `{actor_id, counter}`), so every replica converges to the
  same text regardless of arrival order (RGA/Yjs family — resonant with glade's
  set-union / lamport folds).

### 1.3 Compaction
Tombstones and superseded history compact only behind a **causal checkpoint
acknowledged past the compacted frontier** (H-P4): you may drop only what every
actor has provably seen, so no still-in-flight (out-of-order) op can reference a
compacted element. Compaction is a fold-side GC, not a wire feature.

## 2. Cursors + selections — element-ID-anchored, private under B4

Per-editor cursors/selections are a **value in a private zone** — s-zones,
**already implemented** (`zones.ts`, GDL-039, 2026-06-14) — but they carry
**element-ID anchors**, not offsets.

- **Element-ID + affinity anchor (H-P4).** A caret is `{element_id, affinity}`
  (affinity = which side it sticks to when an insert lands exactly at the
  boundary), so a remote insert/delete NEVER moves your caret — the same element
  identity the merge uses is the identity the cursor rides. Offsets would jump on
  every remote edit; element ids don't. This is the SAME anchoring the CRDT merge
  already needs.
- **Private under B4 — by derivation, not spelling.** `doc.selection` is keyed
  `self:` — a **symbolic** self the node **derives from the authenticated
  principal (B4, over the B3 caller context)** and rejects if a caller spells a
  mismatched literal. A peer never receives your caret even inside the shared
  commons body: a foreign `self:` cannot be forged and never matches the routing
  table. The self-chain refinement keeps each self a contiguous chain, filterable
  from a peer's feed without breaking chain verification.
- glade-editing BINDS this value-in-a-private-zone; it invents nothing. **Cursors
  NEVER ride the document stream** (AZ-16: body is commons, selection is private —
  two keys, never smeared).

## 3. The delta path — the shared identity-based glial primitive

The done-criterion ("neither losing our cursor") lives or dies on the delta path.
Its core contract is **NOT editing-local** — it is a **shared glial primitive**:
an **identity-based delta that tolerates out-of-order delivery**.

- **The shared primitive.** A delta names records/elements by **identity** and is
  applied by **set-diff against what the consumer already holds**, so duplicate
  and reordered arrivals converge. This is exactly the contract behind **glial's
  logDelta fix (A1)** — replacing the positional `slice(emittedLen)` (which
  dupes/drops over a re-sorted list) with an identity-set diff + an out-of-order
  regression test — and the same contract the **D8 window reassembler** imports.
  glade-editing's CRDT deltas are one more CONSUMER of it, not a private variant.
- **Exists today:** the `ChangeEvent` envelope `{glade_id, shape,
  kind:refresh|delta, base_seq, origin_meta, payload}` (GC-1 RULED,
  `glial/events.ts`); the GAP-8 write seam (`GlialTapController` set/append +
  `PayloadCodec`); GAP-9 reload-resume (own-origin ops reach a rebuilt session;
  the semantic echo-guard folds catch-up with no remount).
- **This spec FORCES:** the **`text-crdt` delta payload** (the P4.S1 taut-shape
  contract, now required) carrying element-id ops; the **consumer-chooses-delta**
  tail (GAP-8's deferred half, P4.S2) — an active-cursor editor applies the delta
  incrementally and rebases its element-anchored caret, while an idle/unmounted
  viewer takes the whole refresh (`GlialClientRuntime` rule 3, glade-editing is its
  motivating consumer); and **GC-2 conflation/backpressure** (coalesce selection
  moves, batch body ops) — the editor is the first consumer that needs it.

## 4. Document identity + save — D12 compare-and-replace, D13 at-rest truth

- **Identity is a glade-files binding.** WHICH file you edit is a glade-files
  surface (specced in parallel — referenced generically); the editing SESSION is
  keyed by that file identity. **`open` records the saved base revision (H-P4)**
  the session builds on.
- **Save = D12 compare-and-replace.** `doc.save` MUST delegate to authenticated
  **`files.write`/`replace` with the expected base revision + the required
  workspace lock/lease (D12)** — editing does NOT write the tree itself and does
  NOT duplicate files' AZ-1 enforcement. **Conflict is explicit**: if the base
  changed under the session, save fails loudly; last-writer-wins MUST NOT silently
  overwrite a changed base. One explicit `doc.save` exchange, result as data.
- **At-rest truth = the glade-files snapshot (D13).** The workspace file is the
  authoritative at-rest document; editing LAYERS the live CRDT generation over it.
  Files, GWZ, and diff consumers read the **last successfully saved revision**
  unless an API explicitly requests the live editing generation; a `doc.editing`
  marker records that a live session exists. Autosave cadence and the `gwz
  pull`-under-open-session conflict resolve through the SAME compare-and-replace
  path (§10 Q3).

## 5. Who may edit — glade-share membership (no lease)

- **Who-may-edit = glade-share membership, full stop.** Editing the body = writing
  the doc's commons, which s-zones gates by a write grant. An editor is invited by
  the **share family — `share.create`/`share.invite`/`share.grant`/`share.revoke`
  (E-share-1)** over ordinary grant records; the grant carries commons + the
  recipient's own private zone (AZ-16); revoke cuts both in one act (and, per B4,
  cuts subsequent `self:` resolution).
- **No lease. Membership is orthogonal to CRDT concurrency.** The **EditClaim /
  swmr write-lease machinery is dropped entirely** — no single-writer fence, no
  pen, no handoff, no `doc.owner`. Membership says who MAY edit; the CRDT says how
  concurrent writes converge. **All members write concurrently — that is the whole
  point of a CRDT.** Grant gates entry; merge handles simultaneity.

## 6. Surfaces (declared per GladeSupplierModel)

| glade id | shape | zone | content |
| --- | --- | --- | --- |
| `doc.body` | `text-crdt` | commons | the CRDT op set (insert names an anchor; delete names element ids → tombstones); folds to the converged text — one world for all members |
| `doc.selection` | value | private (`self:`, B4) | per-editor caret/selection as `{element_id, affinity}` — private by DERIVATION from the authenticated principal, no grant, no `check()` (§2) |
| `doc.save` | exchange | — | compare-and-replace flush — delegates to `files.write/replace` w/ expected base revision + lock (D12); conflict explicit; result as data (§4) |
| `doc.editing` | value | commons | D13 marker: a live collaborative session exists; consumers still read last-saved unless they request the live generation |
| policy binding entries | log | the doc's policy binding | write grants (glade-share, E-share-1; AZ-16) — NOT this supplier's data, referenced |

The supplier is thin: serve the CRDT ops and orchestrate save. Records are
ordinary appends in existing shares (GDL-038 — no privileged plane). The one new
substrate is the `text-crdt` Shape + its P4.S1 taut-shape contract (now REQUIRED,
not deferred) + the element-id allocator + the merge fold + the compaction
checkpoint.

## 7. Stage split — security is stage-1; the identity-delta primitive is a prerequisite

The B1–B5 rulings re-scope the stage split: **"stage-1 = allow-all" is retired.**
Security substrate is a **stage-1 must**, and the shared delta primitive gates the
whole build.

- **Prerequisite (before any editing flow is honest):** the **identity-based,
  out-of-order-tolerant delta primitive** (§3 — glial's A1 logDelta fix + the
  shared reassembler contract). Without it the CRDT deltas dupe/drop on reorder
  and no convergence claim holds.
- **Stage-1 security (B1–B5), not allow-all:** the provider MUST authenticate and
  attach under a `provider.attach` grant advancing a monotonic epoch (**B1**);
  every call carries an authenticated `ProviderCallContext` (**B3**); `self:`
  selection keys are **derived from that principal**, never caller-spelled
  (**B4**); governance records are signed (**B5**). Attribution is real from day
  one.
- **Stage-2 (grants gate):** the write grant gates commons edit; revoke ends it
  (commons AND private, one act). The private selection needs no grant (by key).
- **New code for v1:** the `text-crdt` Shape + P4.S1 contract + element-id
  allocator + merge fold + compaction checkpoint + the consumer-chooses-delta tail
  (P4.S2) + GC-2 conflation. The private-selection zone and the ChangeEvent
  envelope already exist.

## 8. Traces to author before building (atlas leads)

- **s-edit-crdt** — two editors type into one `doc.body` SIMULTANEOUSLY; concurrent
  inserts at the same anchor and interleaved deletes converge to identical text on
  both replicas (deterministic sibling order; deletes leave tombstones; duplicate
  op replay is a no-op). Proves the `text-crdt` Shape + element-id/anchor/tombstone
  model (H-P4); forces the P4.S1 taut-shape contract.
- **s-edit-cursor** — an active-cursor editor consumes `doc.body` deltas
  incrementally; its `{element_id, affinity}` caret stays put across remote
  insert/delete, AND stays correct when deltas arrive **out of order or
  duplicated** (the shared identity-delta primitive); an idle viewer of the SAME
  surface takes a whole refresh; rapid ops coalesce. Proves H-P4 cursor anchoring +
  GAP-8's deferred tail (P4.S2) + GC-2 conflation + the A1 out-of-order contract.
- **s-edit-offline-merge** — an editor edits while partitioned; on reconnect its op
  set and the peers' merge/heal into one converged document with no lost or
  duplicated elements, regardless of delivery order. Proves causal-dependency
  replay + idempotent dedup end to end.
- **s-edit-save-conflict** — `doc.save` delegates to `files.write/replace` with the
  expected base revision; when the base changed under the session the save returns
  an **explicit conflict** (no silent last-writer overwrite); a clean base
  replaces. Proves the D12 compare-and-replace seam + D13 at-rest truth.
- **s-edit-compaction** — tombstones/history compact only behind a causal
  checkpoint acknowledged past the compacted frontier; a reopen after compaction
  reconstructs identical text, and a late out-of-order op referencing pre-frontier
  state is handled safely. Proves the H-P4 compaction rule.

## 9. Dependencies + user-testable-when

- Depends on: **glade-files** (document identity, at-rest snapshot, the
  `files.write/replace` save target — D12/D13), **glade-users** (per-editor
  principal + the B4 `self:` derivation + attribution), **glade-share** (the
  membership grants — E-share-1), **glial** (the shared identity-based delta
  primitive + rule 3's consumer-chooses-delta tail; the A1 logDelta fix),
  **glade-workspaces** (which workspace's file). Forces: the `text-crdt` Shape +
  its P4.S1 taut-shape contract (now REQUIRED), glial's consumer-chooses-delta tail
  (P4.S2, closes GAP-8's deferral), GC-2 conflation.
- **User-testable when** (normative, `SupplierOutlines.md`): the user I invited via
  glade-share edits the same workspace file with me from another machine, live —
  **we both type at once and the text converges**, each keeps their own private
  element-anchored cursor (neither jumps on the other's edits, even under
  out-of-order delivery), and on save the file lands in the gwz working tree via
  compare-and-replace with an explicit conflict if the base moved. Neither of us
  loses our cursor.

## 10. Open questions (Gianni)

- **H-P4 — RESOLVED: text CRDT for v1.** The P4 gate is CLOSED — `doc.body` is a
  first-class `text-crdt` Shape; swmr is not an allowed fallback (§1). The former
  Q1 (simultaneous vs turn-based), Q2 (policy-over-`log` vs Shape), and Q4
  (lease-handoff grant) are MOOT: there is no lease. The `text-crdt` taut-shape
  contract (P4.S1) is REQUIRED, not deferred.
- **Q3 (save, per §4).** D12/D13 fix the seam (live CRDT layered over the saved
  snapshot; explicit compare-and-replace flush). Still open: autosave cadence, and
  how a `gwz pull` that changes the file under an open session surfaces as a
  compare-and-replace conflict.
- **Q5.** Very-large documents: does `doc.body` need the `window` shape (viewport)
  composed with `text-crdt`, or is that deferred? (Ties to D8 — same reassembler;
  slot reserved.)
- **Q6.** GC-2 conflation for the editor (coalesce rapid selection moves; batch
  body deltas) — decided with the gryth-ui tap or here?
