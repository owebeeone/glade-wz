# GLP-0006 — grazel + gryth: the glade supplier program

Status: proposed (plan written 2026-07-11, post E2E-stage-1 audit: MET)
Owner: maintainer (coordinating); agent-parallel by design
Prereqs: glade-wz @ root `f9671a1` pins or later; E2E stage-1 audited
(`dev-docs/GladeE2EStage1Audit.md`)

## Goal

Build the application layer above glade: **grazel** (the gryth node — app
authority, storage owner, UI server) and the **glade suppliers** — generic,
app-agnostic endpoint provider modules grazel composes. The glade demo grows
one tab per supplier and stays the driving force; gryth-ui
(`gryth-wz/gryth-ui`) is fleshed out against the same suppliers. grip-lab
(grip-pyrolyze-dev) is the prototype reference — lessons, not code.

North-star tie-in: this is the GripLab golden path landing on the audited
substrate — declarative discipline on the server side, where each endpoint is
a declared surface with a supplier standing behind it, never bespoke server
code per app.

## Vocabulary (ruled 2026-07-11 — GDL-040)

- **Supplier** — the authority-side module that stands behind one or more
  declared surfaces and answers for them; the counterpart of a tap. Drip
  irrigation is the metaphor: suppliers feed the mainline (glade), taps draw
  at the ends. Registered as **glade suppliers**.
- **Repo naming**: repos say *what*, terminology says *role* —
  `glade-terminal`, `glade-files`, `glade-chat`, … (beside `glade-decl`);
  never `supplier-*`. Each README's first line: "a glade supplier: …".
- Distinct from `ServiceDefinition` (dynamic, instantiated computations):
  suppliers are static authorities you compose.

## Architecture posture (P00 confirms)

**Attachment model — RULED 2026-07-12 (P00-a): wire-attached (session model).**
A supplier is an ordinary authority session that subscribes its declared
surfaces (exactly R4's provider mechanism, live-proven in the audit).
Suppliers depend on the wire + a client lib only — language-neutral, zero
node internals, GDL-038-aligned. In-process embedding is a grazel-internal
optimization (loopback attach), never the contract.

**Grazel's data seam.** Grazel's database (files for now) is app-owned
storage glade never sees. Private data = never declared. Shared data = a
declared surface a supplier serves from that storage. The file↔surface
mapping is grazel's alone. This keeps "files for now" unable to leak into
glade's model.

**Modes.** `grazel --mode local|peer|both` composes existing node profiles:
local = serves UI + client sessions; peer = mesh participant holding claims;
both = the dev-box default (one process, embedded node doing both).

**UI serving = the session-placement layer** (GDL-032): grazel serves the
gryth-ui SPA + a json bootstrap (session/grant handoff). HTTP lives in
grazel; glade stays transport.

## Decision gates (Gianni)

| Gate | Decision | Blocks |
| --- | --- | --- |
| P00-a | ~~Confirm wire-attachment~~ **RULED 2026-07-12: wire-attached sessions** | — |
| P00-b | ~~Repo homes~~ **RULED 2026-07-12: glade-wz members** (grazel → repo `grazel-node`, path `grazel`) | — |
| P00-c | ~~F2~~ **RULED 2026-07-12: BUILD, fused into P0.S2** (creation mints the records) | — |
| P2-gate | WD-1 root custody (the big one) + AZ-1/2/3 v1 scoping | all of P2 |
| P3-gate | Blob strategy for large binaries (iroh-blobs vs content-addressed store; NOT ops-in-chains) | P3.S2 |
| P4-gate | `swmr`: first-class Shape vs policy on `value`; text-crdt contract scope | P4.S1 |

Prereq mounts: `gryth-wz` (gryth-ui) and `grip-pyrolyze-dev` (grip-lab
reference) are not mounted in this environment — mount at P1.S4 / as needed.

## Phases

Aspirational <500 hand-written LOC per step. Foundational-first;
steps within a phase are agent-parallel unless marked sequential.
Every supplier step is: **atlas trace → build → demo tab → live verify**.
Per-supplier needs (identity, shapes, storage, grants, dependencies) are
normative in `SupplierRequirements.md` — phase ordering honors its
dependency spine.

### P0 — Foundations (milestone: a supplier can exist)

- **S1 — vocabulary + supplier model doc.** GDL-040 recorded; supplier row in
  `GladeDeclSurface.md`; new `dev-docs/glade/GladeSupplierModel.md` (one page:
  attachment contract, registration = ordinary records, lifecycle, the
  supplier/service distinction, grazel data seam). Doc-only.
- **S2 — F1: live WorkspaceEntry/ServeClaim minting.** The audit's one
  substantive gap: nothing production mints workspace entries/claims. The
  app-loading node (or config/CLI) mints + lease-renews its WorkspaceEntry +
  ServeClaim so live cross-node routing works outside tests. Include the F2
  ruling outcome (P00-c). glade repo, small.
- **S3 — rust wire client.** `glade/client-rs` (or crate in glade): connect,
  subscribe, ops, and the provider loop (answer ExchangeReq). The TS client +
  audit harness are the reference choreography. Needed for rust suppliers
  under wire attachment.
- **S4 — supplier kit.** Thin `supplier` helper in both langs (TS: in glial
  or client-ts; rust: on S3): declare surfaces → attach → serve/answer →
  re-attach on drop. Wraps R4's protocol; no new wire.
- **S5 — typed manifest + demo tab chassis.** The compile-wall ask: glade ids
  referenced via typed identifiers (typed manifest object / grip-derived per
  GQ-6) — undefined surface = TS build error. Demo gains tab navigation
  (one tab per supplier), existing four surfaces become the first tab.
  glial + glade/demo.
- **S6 — grazel skeleton.** New repo (per P00-b): embedded/spawned glade
  node, `--mode local|peer|both`, static HTTP + json bootstrap stub, loads
  `grazel-app.glade`, composes zero suppliers yet. Boots, audits clean.
- **S7 — principals minimal (identity, NOT management).** Sessions bind to a
  principal (`Hello.principal` — the wire field exists), principal records
  land in `dir.principals` (named by GDL-038) as ordinary registry appends,
  suppliers get attribution. Stage-1 posture: identity as DATA, nothing
  enforced; replaces the demo's `user=` URL stub. User LIFECYCLE (enroll/
  attenuate/revoke) stays P2/glade-users — do not smear the layers. Added
  2026-07-12: the chat→users inversion fix (`SupplierRequirements.md`).

### P1 — First suppliers (milestone: two suppliers live in demo tabs; grazel serves gryth-ui)

- **S1 — glade-chat.** Group chat on the log shape; `ChatLine.user` becomes
  a principal ref (additive, corpus-gated); attribution via P0.S7. **Stage-1
  scope: groups PRE-DECLARED** (grazel-app.glade/config) — dynamic group
  creation is a create-a-share ceremony that rides F2 + P2, not P1. Trace
  (s-chat) → supplier → demo tab.
- **S2 — glade-gwz.** gwz commands over exchange (live-proven leg). Supplier
  wraps gwz invocation; results as exchange responses + log streams for long
  ops. Trace → supplier → demo tab.
- **S3 — grazel composes.** grazel runs chat+gwz suppliers against its node
  (loopback attach), declares them in `grazel-app.glade`, app-owned storage
  seam in place (chat history in grazel's files, served — private files
  simply not declared).
- **S4 — gryth-ui serving.** grazel serves the SPA + json bootstrap; gryth-ui
  gets its first two live panels (chat, gwz) via glial taps. Mount gryth-wz.

### P2 — Sharing + stage-2 (milestone: invite → grant → enforced access, live)

Gated by P2-gate (WD-1, AZ-1/2/3). Sequential-ish: S1 → S2 → S3.

- **S1 — glade-users.** Users + ACL management as ordinary bindings
  (GDL-038): reads = subscriptions to system shares, writes = record appends,
  admin verbs per the ratified ownership model. s-grant/s-admin traces are
  the spec. Demo tab: user list + grant editor.
- **S2 — glade-share.** Share points: invite mint → accept ceremony →
  membership grant (AZ-16 semantics: membership carries commons AND your
  private zone). Trace exists in stage-2 corpus; demo tab: invite/join.
- **S3 — stage-2 switch-on.** `check()` enforced on commons joins at the
  node; stage-2 traces + INV-4/INV-5 become live behavior; the demo's
  allow-all posture ends. The audit's stage-2 sweep re-run as the gate.

### P3 — Heavy shapes (milestone: files + live terminal)

- **S1 — window shape.** taut-shape contract + corpus (Lane C idiom) → node
  windowed delivery (viewport-first, bulk backfill — scheduler already
  built) → glial assembly. Closes the audit's s-window PARTIAL.
- **S2 — blob strategy** (P3-gate ruling) + implementation: large binaries
  content-addressed, never ops-in-chains; Chunk frame or iroh-blobs per
  ruling.
- **S3 — glade-files.** Read-first file supplier: windowed fast loads, blob
  handoff for large/binary. Trace → supplier → demo tab.
- **S4 — channels.** ChannelOpen/Data/Close become real (they echo today):
  ordered byte streams with control messages (WINCH rides here). Node +
  clients.
- **S5 — glade-terminal.** Scrollback = log (grip-lab's original path); live
  session = channel; resize = WINCH control. Trace → supplier → demo tab.

### P4 — Editing + the long tail (milestone: collaborative editing in gryth-ui)

- **S1 — crdt/swmr contracts** (P4-gate ruling first). Designed WITH the
  gryth-ui tap work, not ahead of it. taut-shape contracts + oracles.
- **S2 — glial delta path.** Consumer-chooses-delta at the grip surface
  (closes GAP-8's deferral); GC-2 conflation as needed by the editor.
- **S3 — glade-editing.** Supplier + demo tab + the gryth-ui editor tap —
  cursor-stable remote deltas (the GlialClientRuntime motivating case).
- **S4 — glade-razel.** razel command supplier over exchange — floats until
  razel is ready; slot reserved, no work here.

### Riders (attach to whichever step touches them)

GAP-11 offline outbox (may already be fixed in the side session — verify
before P1) · GAP-10 retention/eviction (needed before P3 files at latest) ·
F4 SubstrateV1 §11 stale-list sweep (doc-only, anytime) · taut
`--legacy-codec` migration (before taut v0.10) · S3 taut-shape value matrix
(when called).

## Discipline (unchanged from GLP-0005 + the lane builds)

Trace leads build; the demo is the live gate; suppliers never import node
internals; commit-per-step with all repo gates green (node cargo, wire-rs,
grip-share, client-ts, glial, ggg-viz, + each supplier's own); no attribution
trailers; pnpm never npm; tests never touch the real `~/.glade`; corpus gates
red = design event. Single-writer per repo per agent wave.

## Parallelism map

P0: S1‖S2‖S3‖S7 then S4(needs S3)‖S5‖S6. P1 requires P0.S7 (attribution)
+ P0.S2 (F1) landed: S1‖S2 then S3→S4.
P2: sequential by design (each step raises the security floor the next
stands on). P3: S1‖S2 then S3; S4 then S5. P4: S1→S2→S3.
Suppliers are repo-disjoint by construction, so cross-phase overlap is fine
once P0 lands (e.g. P3.S1 window contract can start during P2).
