# Decisions — GLP-0006

Plan-local decisions + revision history. Corpus-level rulings get GDL rows;
this file records how they landed in the plan.

## 2026-07-11 — plan creation

- **Supplier vocabulary RULED (Gianni)** → GDL-040: "supplier" = the
  authority-side module behind declared surfaces; drip-irrigation register
  (suppliers feed the mainline, taps draw); registered as "glade suppliers".
  Rejected: provider (collides with grip's declaration/provider/facet
  vocabulary + colorless), spring (Java collision), spout (Storm), well
  (passive), main (`main()` fatal).
- **Repo naming RULED (Gianni)**: repos say what, terminology says role —
  `glade-terminal`, `glade-chat`, …; never `supplier-*`.
- **Attachment model — working assumption, P00-a confirms**: wire-attached
  session (R4's provider mechanism). In-process embedding = grazel-internal
  optimization only.
- **Grazel data seam stated** (plan §Architecture): app-owned storage glade
  never sees; shared = declared surface; private = undeclared. One-page
  design doc lands at P0.S1.
- Endpoint list ↔ shape mapping (from the 2026-07-11 discussion): chat=log,
  gwz=exchange, files=window+blobs, terminal=log+channel(+WINCH),
  editing=crdt/swmr (P4-gate), users/ACLs+share=stage-2 management surface,
  razel=exchange (floats).
- Typed-manifest compile wall (glade ids as typed identifiers, GQ-6-aligned)
  accepted into P0.S5 — from the "so many moving parts" design review, point 1.
- grip-lab is reference-only (lessons harvested); iroh+react already the
  decided substrate/front-end pairing.

## 2026-07-12 — the chat→users inversion (Gianni)

P1 glade-chat implicitly required users. Resolution: "users" split into two
layers — **principals minimal** (identity + attribution, stage-1-safe, new
P0.S7; wire `Hello.principal` + GDL-038's `dir.principals` already provide
the seams) vs **user management/enforcement** (unchanged, P2/glade-users).
`SupplierRequirements.md` added as the normative per-supplier needs matrix
(identity, shapes, storage, grants, dependency spine); P1.S1 rescoped to
pre-declared groups (dynamic creation = create-a-share, rides F2 + P2).
Also surfaced there: terminal is stage-1 owner-only/local-only (spawns
processes — `shell.exec` is the canonical stage-2 deny); gwz + files are the
app-owned-storage consumers foreshadowing AZ-1 path scoping.

## 2026-07-12 — P00 gates ruled; P0 launched

P00-a: wire-attached supplier sessions CONFIRMED. P00-b: new glade-wz
members; repo `grazel-node` / member path `grazel` (bare `owebeeone/grazel`
squatted by the June razel spike — untouched; same convention as
glial-runtime). P00-c: F2 BUILD — fused into P0.S2 since target-routed
creation IS the natural minting path for WorkspaceEntry/ServeClaim (F1).
P0 execution: wave 1 = S1(doc)‖S2+S7(glade, fable)‖S4ts+S5a(glial)‖S6
(grazel); wave 2 = S3+rust kit (glade); wave 3 = S5b demo chassis (glade).
Single-writer-per-repo preserved across waves.

## P0.S6 grazel skeleton — build notes (2026-07-12)

Built the `grazel` member (repo `grazel-node`): rust binary + tiny_http server +
storage seam + tests. Composes zero suppliers yet; boots + audits clean. CLI:
`grazel --mode local|peer|both [--name N] [--data DIR] [--http PORT]
[--node-port PORT] [--ui DIR] [--app FILE.glade] [--node-bin PATH]`.
Ambiguities resolved (smallest faithful call):

1. **`both` → node `--profile local`.** Verified from `glade-node.rs`: the node
   binary makes NO serve-only / mesh-only distinction — every *booted* profile
   (the `booted` branch) seeds the registry, serves the WS carrier to clients,
   AND binds the iroh mesh endpoint + accepts peers, unconditionally. The
   `--profile` flag only selects the default instance NAME. `both` (serve+mesh)
   is thus exactly what one booted node already does; grazel maps it to `local`
   (the dev-box entry node). Mapping: local→local, peer→peer, **both→local**.
   The three grazel modes are a grazel-layer distinction (naming + forward
   intent) that gains teeth when/if the node grows real serve-only / mesh-only
   levers. (Today `local` and `both` invoke the node identically.)
2. **`GLADE_HOME = <data>/sys`** (literal to the brief: "GLADE_HOME under --data
   DIR/sys"). The node nests its own `sys/<name>/` under GLADE_HOME, so the
   booted instance lands at `<data>/sys/sys/<name>/` (observed live:
   `data/sys/sys/grazel-it/{node.key,records.json,instance.lock,cache/}`). The
   double-`sys` segment is glade-node's own convention (it always appends
   `sys/<name>`), kept wholly inside the `sys` slot of `data/{sys,files,config}`
   so `files`/`config` stay clean siblings. NEVER the real `~/.glade`
   (GLADE_HOME is always set; the test also pins HOME to a temp dir and asserts
   the instance materialized under the temp data dir). Alternative not taken:
   `GLADE_HOME=<data>` flattens to `<data>/sys/<name>/` — a one-line change if
   the flatter tree is preferred.
3. **No positional store dir passed to the node.** grazel passes only the node
   port positional; the node defaults its app-data carrier store under its own
   instance cache (inside `<data>/sys`). Keeps glade's store layout glade's
   business — consistent with the data-seam rule.
4. **`--node-port 0` → actual port from node stdout.** grazel parses the node's
   `listening <port>` line and puts the ACTUAL bound port into
   `/bootstrap.json`'s `node_ws` (handles OS-assigned ports + confirms boot).
   Default `--node-port 9099` (matches run_demo.py).
5. **name default = `grazel`, forwarded to the node via `--name`.**
   bootstrap.json `name` == node instance name == grazel `--name` everywhere
   (rather than leaking the node's profile default names into the UI bootstrap).
6. **HTTP dep = tiny_http** (over axum): one small SYNCHRONOUS crate (transitive
   ascii, chunked_transfer, httpdate, +log) — no async runtime. grazel spawns
   the node as a subprocess (the ruled wire-attachment posture), so it needs no
   tokio; axum would drag in hyper+tower+tokio for a static server + one JSON
   route. Second dep: **libc** (already in the node's tree) for a SIGINT/SIGTERM
   handler that tears the node child down on clean shutdown (std has no portable
   signal handling).
7. **Integration-test node build**: attempted once, retried once, then SKIP with
   a loud banner (test passes-as-skipped) — per brief, since a parallel agent
   edits `../glade/node`. Both paths proven live: one run built the node clean
   and ran the full end-to-end assertions green (bootstrap + static + node WS
   TCP + clean shutdown); a later run hit the parallel agent's in-flight
   breakage (`E0583: file not found for module claims`) and correctly SKIPped,
   suite still green. Cargo.lock left untracked (sibling convention:
   node/wire-rs/grip-share do not commit it).

## 2026-07-12 — P0 complete (waves 1-3); notes for P1

- P0 checkpoint MET (see Checkpoints.md). Corrections mid-wave: doc placement
  (grazel repo = code+README only; notes → this plan dir, Gianni), the
  provider-vs-claim two-mechanism split (GladeSupplierModel §2 → kit GAP-12),
  Plan.md P00 table reconciled to RULED.
- **P1.S1 flag (from the S1 doc review): `ChatLine.user` str→principal-ref is
  NOT obviously additive** under the decl versioning rule — likely a NEW
  optional field (e.g. `principal`) beside `user`, not a reinterpretation.
  Corpus-gate the call when glade-chat lands.
- client-rs catch: offline `set()` retry inflated the local chain seq (node
  rejects as gap) — fixed fail-fast; buffered outbox remains GAP-11.
- grazel `both`≡`local` on today's node (booted profiles all serve+mesh);
  three-mode distinction is grazel-layer intent until the node grows levers.
