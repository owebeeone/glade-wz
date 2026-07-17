# Supplier Requirements — GLP-0006

What each supplier NEEDS (identity, shapes, node capabilities, storage,
grants), what it PROVIDES, and its stage-1 scope vs stage-2 upgrade. The
dependency truth the phase ordering must respect. **Ratified-rulings pass
2026-07-12** — reconciled to `RulingWorksheet.md` (all §I–§VIII) and the
propagated specs; supersedes the pre-ruling gwz/share/editing framing
(SR56-2-04).

## Security substrate is a STAGE-1 prerequisite (B1–B5)

The reframe: attribution is a stage-1 must, and it needs machinery the specs
formerly deferred. These land FIRST and re-scope every supplier's stage split
(`GladeSupplierModel.md` §8, `GladeAuthzModel.md` §3b/§11):

- **B1** authenticated, non-replaceable provider attach + monotonic epoch — the
  composition wall is real (the LWW attach is overturned).
- **B2** every decode entry point fallible; typed system records validated
  before persist/fold; no panic-DoS.
- **B3** `ProviderCallContext` — node-authenticated requester delivered BESIDE
  the request DTO; a DTO principal field never overrides it.
- **B4** `self:` is identity-bound — derived from the B3 principal, mismatched
  literals rejected (subscribe/append/replay/forward).
- **B5** device-possession proof at session; governance ops signed by a
  certified device + strict predecessor, verified before persist/fold.

"Stage-1 allow-all" now means the GRANT check is stubbed — identity, attach,
and decode are load-bearing from day one.

## The identity split (the correction that created this file)

"Users" is two layers: (1) **principals + attribution** — sessions bind a
principal (`Hello.principal`, `dir.principals`), suppliers ATTRIBUTE via the B3
context; (2) **user management + enforcement** — lifecycle, grant ceremonies,
`check()`. Layer 1 + the B5 signed-op substrate are stage-1; layer 2 is glade-users.

## glade-chat (P1 built → ratified reshape)

- **Needs — identity:** per-message attribution from the B3 context (built:
  `ChatLine.principal` tag 4); the principal is the B5 device-certified
  fingerprint.
- **Needs — shapes:** log. **Each group owns its own share** (E-chat-1) minted
  by glade-share `share.create`; keyed-commons is the stage-1 migration source.
- **Needs — declaration:** a typed HOME `dir.bindings` record per group
  (E-chat-2); the built `chat.decl` JSON is non-authoritative.
- **Needs — codec:** taut `ChatLine` is the sole payload (E-chat-3, hard cut).
- **Needs — policy:** `ChatQuotaSettingsV1` (immutable, 50 groups/principal,
  E-chat-4). Edit/delete = signed tombstone (E-chat-5).
- **Provides:** group chat; the first multi-principal attribution consumer.

## glade-gwz-* family (25 interfaces, derived from the pinned IR)

- **RULED (C-gwz-1):** grain = a mechanical IR function (one surface per
  `(method, capability-class)`; op-enums split; twin-merge on identical UI +
  class) → **25 members** over gwz-core **v0.9.2**; generator fails on drift.
- **Needs — wire:** a glade-owned **path-free DTO** projected server-side into
  the canonical request (C-gwz-3); host paths from ID→root or a scoped
  `RepoImportHandle`; DTO ops are closed disjoint enums (the tag split, C-gwz-2).
- **Needs — identity:** attribution from B3; gating = ordinary per-surface
  grants (no allow-list, no ActionKind sub-vocabulary).
- **Needs — result:** a closing REPLICATED `OperationResult` log record (C-gwz-4).
- **Needs — storage/host:** real gwz workspace dirs; roots from the ID→root map
  (the data seam); AZ-1 path scoping stage-2.
- **Provides:** per-operation gwz surfaces; the create/init/clone members are
  the workspaces materializer leg (C-gwz-7, `ws.ops` retired).

## glade-users (P2; identity substrate is stage-1)

- **Needs:** the B5 signed-op substrate (authenticated sessions + per-op
  signatures — NEW load-bearing work, retracting §2's "already have it"
  overclaim); the HOME share (H-C1); GDL-034 ownership. **Account id = root-key
  fingerprint** (E-users-1) resolves the old WD-1 custody block for identity;
  device certs + merge ceremony before governance.
- **Provides:** the management surface (GDL-038); lifecycle (enroll/attenuate/
  revoke). Invites = `users.invite.records` log + `users.invites` exchange
  (E-users-2). Names registry DEFERRED (E-users-3).
- **Note:** P0.S7 provides principal *records*; this supplier their *lifecycle*.

## glade-share (P2; direct ceremony is the core)

- **Needs:** glade-users (grants to mint + B3 identity), session placement
  (invite URL → bootstrap, GDL-032), stage-2 enforcement.
- **Provides:** the **direct membership ceremony** — `share.create`,
  `share.invite`, `share.grant`, `share.revoke`, `share.status` over grant
  records (E-share-1); AZ-16 membership (commons + your private zone, revoke
  cuts both). Links layer on top (v1 capture = portable commons + inline IDs,
  E-share-2). The knock = a directed authenticated request + offline queue (D11,
  read ≠ append).

## glade-files (P3)

- **Needs — shapes:** full mutable **window** `{workspace_id, path, revision}`
  (D8 — base glade routes, glial reassembles, files owns the snapshot, no mixed
  generations); `ws.tree` per-directory keyed (D7); **`ws.blob.fetch` exchange**
  with delivery-time authz (D6, never bare-hash).
- **Needs — identity/grants:** read attribution (B3); `files.write` =
  compare-and-replace + expected base revision (D12), AZ-1 path scoping stage-2;
  one `RootRelativePath` + safe-open (D14).
- **Needs — storage:** THE app-owned storage case; at-rest truth + `doc.editing`
  marker (D13). **Retention (F-GAP10) ruled** — a shared TTL/size/pressure
  policy (blob/window cache row).
- **Provides:** file surfaces; the window + blob + path-type consumer.

## glade-terminal (P3)

- **Needs — shapes:** log (scrollback) + channels with `TermOut{generation,
  offset, bytes}` / `driver_epoch` (D9 — atomic epoch handoff, lossless
  replay/cutover, WINCH as a control message).
- **Needs — identity:** owner principal from B3; commons keyed by an unguessable
  `session_id` (D10); attach/handoff s-takeover-adjacent.
- **Needs — host capability:** SPAWNS PROCESSES — sharpest surface. Stage-1:
  owner-only, no forward/advertise; watchers need `term.read`, driver
  `term.write`+`shell.exec`. `forall` = owner-self-exec here.
- **Provides:** terminal surfaces (scrollback + live + resize); retention (F-GAP10).

## glade-editing (P4)

- **Needs — shapes:** **RULED text CRDT** (H-P4 — swmr not a fallback): element
  IDs, tombstones, **identity-based deltas tolerant of out-of-order** (the shared
  glial primitive + A1 fix + D8 reassembler); the `text-crdt` taut contract is
  now REQUIRED.
- **Needs — identity:** per-editor cursors, element-ID-anchored, **private under
  B4**; who-may-edit = the glade-share grant (not a lease).
- **Needs — save:** D12 compare-and-replace into the glade-files snapshot (D13).
- **Provides:** collaborative editing; the CRDT + delta-primitive consumer.

## glade-diff (P3 tail — the first demand-instantiated supplier)

- **Needs — substrate:** base glade SERVICE INSTANTIATION (definition registry +
  spawn + refcount/teardown) — this supplier is its build driver. The
  derived-binding home (structural derivation, D2) and borrowed-capability
  (sandbox + attenuated sources, D5) are RULED, not open.
- **Needs — record:** a versioned `DemandServiceDefinition` (D1) — compute key =
  digest + sandbox-version + ordered source revisions (viewer excluded);
  per-viewer delivery identity (D1 two-level).
- **Needs — authz:** per-principal `can_read(left) && can_read(right)` per hop
  (D3, INV-7); generation envelope (D4); define ≠ authorize-execute (D5).
- **Provides:** cross-surface diff (grip-lab parity); the proving ground for
  demand-instantiated suppliers.

## glade-razel (P4, deferred until the gwz gate)

- **Needs:** the gwz family "working" (generated inventory + composition +
  per-surface authz + result-closure + a real local AND forwarded path) —
  G-razel. razel-wire-api 0.1.0 exists (premise corrected) but NO `razel.*`
  surface is normative/declared/granted before the gate. Reuse gwz's kit + B3
  context + result closure + authz tests after it; never copy the 10 methods
  mechanically.

## Dependency spine (what the phases must honor)

```
B1–B5 security substrate ───▶ EVERY effect supplier (attribution, attach, decode)
P0.S7 principals-minimal ───▶ chat, gwz, files, terminal, editing, diff
glade-users (B5 signed) ────▶ glade-share; every supplier's stage-2 upgrade
glade-share direct ceremony ▶ chat (group = share), editing (who may edit)
stable-ID authority ────────▶ gwz/files/terminal/diff targeting
window + blobs + reassembler▶ glade-files (glial owns reassembly)
TermOut + channels ─────────▶ glade-terminal (live half)
text-crdt + identity-delta ─▶ glade-editing
service instantiation ──────▶ glade-diff (its own build driver)
the gwz-family gate ────────▶ glade-razel (deferred)
```
