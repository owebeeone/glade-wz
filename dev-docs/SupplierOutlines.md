# Supplier Outlines — the generic glade suppliers, enumerated

Status: outline spec (2026-07-12, **ratified-rulings pass**) — one brief entry
per supplier + its dependencies. Each entry expands into a full spec
(`dev-docs/glade/suppliers/<name>.md`); this file is the enumeration + the
dependency truth, kept in sync with those specs and the ratified
`plan-docs/plans/GLP-0006-grazel-gryth-suppliers/RulingWorksheet.md`. Common
contract: `dev-docs/glade/GladeSupplierModel.md` (wire-attached authority
sessions; registration = ordinary records; failure as data).

**Genericity rule:** a supplier depends only on base glade and on other
*generic suppliers' declared surfaces* — never on grazel, never on another
supplier's internals. grazel COMPOSES suppliers and owns app storage; gryth-ui
consumes surfaces through glial taps. Any other app could compose the same set.

**User-testable when** is normative per entry — a supplier is done when a
person can exercise its flow end to end, not when its gates are green.

## Security substrate is STAGE-1 (B1–B5, the reframe)

Security is NOT uniformly deferred to stage-2. Attribution is a stage-1 must,
and it needs machinery every supplier consumes from day one (RATIFIED B1–B5;
`GladeSupplierModel.md` §8, `GladeAuthzModel.md` §3b/§11):

- **B1** authenticated, non-replaceable provider attach (the composition wall is
  real, not just "declare-or-not"); **B2** fail-closed decode (no panic-DoS);
  **B3** `ProviderCallContext` — a node-authenticated requester delivered BESIDE
  the request, never a caller-payload principal; **B4** identity-bound `self:`
  keys (privacy is an identity-bound key, derived from the B3 principal); **B5**
  device-possession proof + signed governance ops.

Every effect supplier below consumes these; "stage-1 allow-all" now means the
*grant check* is stubbed, NOT that identity/attach/decode are unsafe.

---

## Foundation suppliers (the user-flow spine)

### glade-users — identity, onboarding, access lifecycle
- Account identity = the **root-key fingerprint** (E-users-1); browser/device
  keys are root-certified; account merge / root transition lands before
  governance depends on it. Session proves device possession; governance ops
  (grant/revoke/name-claim) are **signed by a certified device** (B5). Invite =
  a durable `users.invite.records` log + a `users.invites` exchange (E-users-2)
  via the session-placement bootstrap. Names registry DEFERRED — v1 uses
  fingerprint + local display names + fp-suffix (E-users-3).
- Depends on: base glade + the B5 signed-op substrate. **Everything multi-user
  depends on this.**
- User-testable when: I mint an invite, a second real user opens it elsewhere,
  onboards with their own (SSH-rooted or device) key, and both of us appear in
  a visible user list — same list on both sides.

### glade-workspaces — directory, selection, creation
- A **stable workspace/share ID is the sole routing + authorization identity**
  (E-ws-1): the node resolves ID→root; display names NEVER route (duplicates
  tolerated, H-C3); an asserted-root mismatch fails closed. Creation is a
  **durable state machine** intent→materialized→registered→claimed→complete
  (C-gwz-8), whose disk leg is a gwz create/init/clone member (C-gwz-7).
  `eligible_hosts` is an **OR-set** (E-ws-2, LWW lost concurrent clones); clone
  = eligible+warm, not seize (H-C5). Role state is distributed (H-R2).
- Depends on: base glade only.
- User-testable when: I see my real workspaces listed, select one (by stable
  ID), create a new one and clone an existing one, and the selection visibly
  drives the tools (gwz/files/terminal).

### glade-share — membership + links + the grant-request lifecycle
- Owns the **direct membership ceremony** — first-class `share.create`,
  `share.invite`, `share.grant`, `share.revoke`, `share.status` exchanges over
  ordinary grant records (E-share-1). This is the normative "share this
  workspace/group with that principal" flow; **links layer on top**, they are
  not the only path. AZ-16: a membership grant carries commons + the recipient's
  own private zone; revoke cuts both in one act. A late-comer **knock** is a
  DIRECTED, authenticated request to the authority + durable offline queue (D11
  — read does NOT imply append); v1 link capture carries portable commons +
  inline IDs only (E-share-2, private/account refs rejected).
- Depends on: glade-users (principals/grants), glade-workspaces (things to
  share).
- User-testable when: I share a workspace directly with the user I onboarded,
  they see it appear; a late-comer clicks a shared link, knocks, I approve; I
  revoke and access ends (fully honest at stage-2).

## Tool suppliers (operate ON a selected workspace)

### glade-gwz-* — the gwz supplier FAMILY (25 interfaces, derived)
- **RATIFIED (C-gwz-1):** the family is a **mechanical function of the pinned IR**
  — one surface per `(method, capability-class ∈ {read, local-mutate,
  egress-read, egress-write})`; op-enums split by class; indivisible multi-effect
  ops take their highest class; twin-merge only on identical UI + class. Over
  gwz-core **v0.9.2** (24 `role="in"` methods) that yields **25 members** (tag→4,
  stash→2, branch→2, −4 twin-merges); the generator recomputes the count and
  FAILS ON DRIFT. Never a hand-count.
- The wire payload is a **glade-owned path-free DTO** projected server-side into
  the canonical request (C-gwz-3) — NOT the canonical Request on the wire; host
  paths come from the ID→root map or a scoped `RepoImportHandle`. Per-member
  gating = ordinary surface grants (no allow-list); the final result is a
  **closing replicated log record** (C-gwz-4). create/init/clone members are the
  workspaces materializer leg (C-gwz-7); `ws.ops` retired; `forall` stays
  glade-terminal (absent from the protocol).
- Depends on: glade-workspaces (selection + ID→root), glade-users (attribution),
  gwz-core v0.9.2 (pinned).
- User-testable when: each member's panel exercises ITS flow on a real selected
  workspace; a composition without the mutating/egress members visibly lacks
  their panels.

### glade-files — file tree, windowed reads, blobs
- Full **mutable typed window** v1 (D8): identity `{workspace_id, path,
  revision}`, range = interest; base glade routes, **glial reassembles**,
  glade-files owns the snapshot, no mixed generations. `ws.tree` per-directory
  keyed (D7, so a `/src` grant hides `/secret`). Blobs via **one
  `ws.blob.fetch` exchange** with delivery-time authz (D6 — hash = integrity,
  not authority; no bare-hash bearer token). `files.write` = compare-and-replace
  with expected base revision (D12); at-rest truth + `doc.editing` marker (D13);
  one `RootRelativePath` + safe-open (D14). Retention per F-GAP10.
- Depends on: glade-workspaces (which tree), glade-users (attribution).
- User-testable when: I browse the real tree and open a big file with a fast
  first paint; a big binary fetches by hash without stalling.

### glade-diff — cross-surface diff (the first demand-instantiated supplier)
- Composed artifact = a structural, versioned **`DemandServiceDefinition`**
  (D1); the compute key = program digest + sandbox-policy version + ordered
  source revisions (viewer excluded); delivery identity is per-viewer (D1's
  two-level identity). Instances derive structurally, per-node by default,
  global behind a leased monotonic-epoch claim (D2). Authorization is
  per-principal `can_read(left) && can_read(right)` re-checked per hop (D3,
  INV-7). Generation state `pending|ready|stale|absent|denied|error` (D4);
  sandbox = separate define-vs-execute capabilities, no ambient authority (D5).
  Working-tree diff stays a glade-gwz member; this is the cross-surface case.
- Forces base glade's service-instantiation machinery — glade-diff is its build
  driver.
- Depends on: glade-workspaces + glade-files (sources), glade-users.
- User-testable when: I pick two workspaces (possibly two peers), watch a live
  diff update on source change, and closing tears the instance down.

### glade-terminal — terminal sessions
- Scrollback = a log; live session over channels with `TermOut{generation,
  offset, bytes}` out and `driver_epoch` in (D9 — atomic epoch handoff, lossless
  replay/cutover, WINCH as a control message). Commons keyed by an **unguessable
  `session_id`** (D10); local sessions never forward/advertise; watchers need
  `term.read`, the driver `term.write`+`shell.exec`. The `gwz forall` handoff is
  owner-self-exec on the owner's own pty. Sharpest security surface.
- Depends on: glade-workspaces (cwd/ID), glade-users (owner identity, B3).
- User-testable when: I open a terminal, run vim, resize, and a second session
  of MINE re-attaches with no byte doubled or lost.

### glade-editing — collaborative editing
- **RULED text CRDT v1** (H-P4; swmr is NOT a fallback): element IDs
  `{actor_id, counter}`, anchored inserts, tombstone deletes, deterministic
  sibling order, **identity-based deltas tolerant of out-of-order** (the shared
  glial primitive, also the A1 logDelta fix + the D8 reassembler). Cursors use
  element IDs + affinity, **private under B4**. `open` records a base revision;
  `save` = D12 compare-and-replace; compaction via causal checkpoint.
- Depends on: glade-files (document + save target), glade-users, glade-share
  (who may edit — grant, not lease).
- User-testable when: the user I invited edits the same file with me, live,
  neither of us losing our cursor.

### glade-chat — group chats
- Each group owns **its own share**, minted by the glade-share `share.create`
  ceremony (E-chat-1; keyed-commons retired to a stage-1 migration source).
  Dynamic declaration is a typed HOME `dir.bindings` record (E-chat-2 — the
  built `chat.decl` JSON is non-authoritative). taut `ChatLine` is the sole wire
  codec (E-chat-3). Any authenticated principal may create a group under the
  immutable **`ChatQuotaSettingsV1`** (max 50/principal, E-chat-4). Edit/delete
  = signed tombstone/supersede (E-chat-5). Attribution from B3.
- Depends on: glade-users (participants), glade-share (group = share, membership).
- User-testable when: I invite the onboarded user to a group and we chat from
  two machines — not two URL-stub tabs on one machine.

### glade-razel — razel commands (RESERVED, deferred)
- Premise corrected: razel-wire-api **0.1.0 EXISTS** (10 methods, ratified IR).
  Deferral is a deliberate SURFACE decision, not "wait for a protocol"
  (G-razel). **Defer ALL razel-facing interfaces until the gwz family WORKS**
  (generated inventory + composition + per-surface authz + result-closure + a
  real local AND forwarded path). Candidate `razel.*` names are reservations
  only; `ws.relations` stays reserved until razel's `query` lands.
- Depends on: the gwz-family gate; glade-workspaces, glade-users; razel-wire-api
  0.1.0 (external).
- User-testable when: nothing in v1 — surfaces do not exist until the gate fires.

---

## Fit: gryth + grazel

- **grazel** (the gryth node) composes users + workspaces + share as the spine,
  then the tools; owns app storage (the ID→real-path map, chat/history, invite
  secrets); serves gryth-ui + the bootstrap (GDL-032 — where invite links land).
- **gryth-ui**: one plugin per supplier consuming surfaces via glial taps.
- **glade demo**: one tab per supplier remains the driving/verification surface
  — against the user-testable-when line.

## Dependency spine (build order follows it)

```
B1–B5 security substrate ──▶ (stage-1 prerequisite for every effect supplier)
        │
glade-users ──────────────┬─▶ glade-share ─▶ glade-chat (group = share)
                          │        │
glade-workspaces ─────────┼────────┴─▶ glade-editing (with glade-files, CRDT)
      │                   │
      ├─▶ glade-gwz-* (25) ├─▶ glade-terminal
      ├─▶ glade-files ─────┘
      │        └─▶ glade-diff (demand-instantiated; forces service spawn)
      └─▶ glade-razel (deferred until the gwz gate)
```

Sequencing: the **security substrate (B1–B5) lands first**; then users +
workspaces complete their user-testable flows before any tool supplier is
called done. Chat/gwz as built are bring-up artifacts that reshape into their
ratified forms.
