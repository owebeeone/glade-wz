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

## P1.S2 glade-gwz — build notes (2026-07-12)

Built the `glade-gwz` member (new repo `glade-gwz`, member path `glade-gwz`):
rust lib + bin on `glade-client` (path dep `../glade/client-rs`) + `glade-wire`.
Exchange command surface + log output-stream surface; `cargo test` 14 green
(9 lib + 5 integration), zero warnings, clippy clean. Nothing outside glade-gwz
touched. CLI: `glade-gwz --node ws://HOST:PORT --root DIR [--share ws-razel]
[--glade-id gwz.ops] [--output-id gwz.output] [--principal P] [--gwz-bin gwz]
[--timeout-secs 30]`. Ambiguities resolved (smallest faithful call):

1. **Stage-1 verb allow-list = `{status, ls, diff}`** — the pure read verbs
   (verified live: exit 0, no member/lock mutation, no arbitrary exec). Excluded
   with rationale in `exec.rs`: `forall` (runs an ARBITRARY command — the
   sharpest surface), `capture`/`snapshot` (write the workspace lock — NOT
   read-only despite `capture`'s "no mutation" of *members*), and
   `add`/`commit`/`pull`/`push`/`clone`/`init`/`materialize`/`branch`/`tag`/
   `repo`/`stash` (mutate). Stage-2 replaces the list with per-verb GRANTS (the
   s-verbs taxonomy; `gwz.*` seeds already ride `grazel-app.glade`).
2. **`cwd` accepted but IGNORED for execution in stage-1.** The envelope carries
   `cwd?` (parsed, echoable, forward-compat) but the supplier ALWAYS runs
   `gwz --root <config root>` with no cwd override. Honors the emphatic rule —
   "the root is the app's, never derived from the request" (SupplierRequirements
   §glade-gwz; SupplierModel §5). Request-supplied paths are an AZ-1 / stage-2
   path-scoping concern. Also enforced positively: `--root` is prepended FIRST,
   and requests carrying scope/force global levers (`--root`/`--force`/
   `--target`/`--member(-path)`/`--all`) are refused as data before exec.
3. **The WIRE `ExchangeRes.ok` is ALWAYS true; the PAYLOAD `ok` carries
   command success.** Every outcome (ran-clean, non-zero exit, disallowed verb,
   bad envelope, timeout, spawn error) is a well-formed `GwzResponse` — the
   handler never returns the kit's `Err`. This keeps failure uniformly as DATA
   (SupplierModel §6): the exchange always produced a structured answer, and
   `payload.ok = (exit == 0)` (or "stream accepted"). A disallowed verb / bad
   envelope never invokes gwz (`exit` absent).
4. **Streaming is OPT-IN via `stream:true`, not a wall-clock threshold.** A
   threshold would have to buffer the whole run to measure it — defeating
   streaming. `stream:true` → answer `{run_id, done:false}` immediately, then
   append each stdout/stderr line as a LOG op keyed by `run_id` on the output
   surface (default `gwz.output`), closed by a `{stream:"end", done:true, exit}`
   marker. from-cursor backfill makes a late subscriber converge the full run
   (proven in the test). Stage-1 read verbs are fast, so no allow-listed verb
   NEEDS streaming yet — the path is exercised by `status --stream` + a
   subscriber, and is the seam a genuine long op (a stage-2 `pull`/`materialize`)
   will ride.
5. **Blocking exec in the sync exchange handler.** The kit's
   `Supplier::serve_exchange` takes a synchronous `Fn`; the non-streaming path
   runs gwz via `std::process` with a hard timeout + thread-drained pipes
   (deadlock-safe). This blocks one tokio worker for the (sub-second) read verb;
   per-surface answers already serialize at the authority (the `workspace.lock`
   in discovery.ts phase D), so it is honest for stage-1. A `spawn_blocking`
   refactor is a stage-2 nicety, not a contract change. The streaming path is
   fully async (tokio `Command`, spawned off a captured runtime `Handle`).
6. **Test app fixture** `tests/fixtures/gwz-test-app.glade` declares exactly the
   two surfaces glade-gwz stands behind (`service gwz gwz.ops` +
   `binding gwz.output log`) plus `workspace ws-razel razel` (so the booted node
   mints the ServeClaim and `(ws-razel, gwz.ops)` routes Local — audit F1). Node
   booted exactly as the client-rs harness does (temp GLADE_HOME/HOME/store,
   never the real `~/.glade`).
7. **Deps: `serde`/`serde_json` (JSON envelopes — the responsible choice over
   hand-rolled escaping) + `tokio` (process/signal/io). No clap** — the CLI is a
   ~40-line hand-rolled flag parser (dep-light, sibling convention). `Cargo.lock`
   untracked (matches node/wire-rs/client-rs/grazel).
8. **Behavioral spec = the existing atlas** (per brief): discovery.ts phase D
   (`gwz.ops` exchange, corr 1:1) + s-fanout-exchange X1 (exchanges never fan
   out — only the authority answers). NO new trace was added this step; the
   supplier is the executable landing of those two traces.

Note for P1.S3 (grazel composes): `gwz.output` is NOT in `grazel-app.glade` yet
(only `gwz.ops` service + `ws.tree/ws.files/ws.diff/term.log` bindings). When
grazel composes glade-gwz, add `binding gwz.output log share commons from-cursor`
(or reuse `term.log`) so the streaming surface is declared in the real app file,
as the test fixture already does.

## P1.S1 glade-chat — build notes (2026-07-12)

Built the trace (ggg-viz `s-chat`), the `ChatLine.principal` field (glade), the
`@owebeeone/glade-chat` supplier (new repo), and the demo Chat tab (glade).
Gates: ggg-viz 241 (was 228: +7 chat.test, +6 parametrized suite); glade-chat 7
vitest + tsc; glade demo build + grip-share 18/18 + client-ts 9 + node cargo 56.
Live-verified two browser tabs (alice/bob) converge in #general both ways with
per-line attribution, #dev isolated by keying, and a group-switch catch-up.
Commits: ggg-viz `4afb9a1`; glade `7a8f8e6` (ChatLine field) + `666ff7c` (demo
tab); glade-chat `9238d21`. Ambiguities resolved (smallest faithful call):

1. **`ChatLine.principal` = tag 4, OPTIONAL** (the recorded rule; resolves the
   P1.S1 flag above). Regen path = the demo IR's OWN:
   `taut.ir.export.export_to(SCHEMA, workspace.ir.json)` (no committed regen
   script; round-trips byte-identical). NOT `--legacy-codec` — that flag is only
   for the node's Rust-GENERATED `sysdata.rs`; the demo IR is schema JSON loaded
   at runtime by client-ts's codec, codec-independent. taut compat:
   `0 breaking, 1 compatible`. Verified round-trip through the real demo codec:
   present ⇒ on wire; absent (legacy line) ⇒ decodes to `null`.
2. **Group shape = ONE keyed commons surface per group** (SupplierRequirements):
   one glade id `chat.msgs`, group id as the KEY (UTF-8 bytes) — a group is a
   key, not a new glade id. `chatManifest(groups)` yields one `Surface` handle
   per group (shared glade id, distinct key) + a `chat.groups` VALUE surface.
3. **Catalog `Z2` minted** ("keyed-commons isolation", ROUTE internal) rather
   than overload `Z1` (private/self-specific). Group isolation is the SAME
   keyed-routing mechanism as private zones but a shared commons per key, each
   stage-2-gated by a membership grant (AZ-16) — a distinct semantic, so a new
   Z-area state. New `chat-sup` pool actor (role `provider`): the supplier is a
   wire-attached authority session (P00-a) drawn distinct from the node so its
   declare + serve-metadata role reads as OUT of the message hot path.
4. **glade-chat tests = fake-session only** (the glial kit's ws-free fallback).
   The brief's "spawned-node test if cheap" is conditional; a real glade-node
   needs a rust build (not cheap for a TS pkg) and the in-process real-`Session`
   test couples to `glade/client-ts` internals (cross-repo reach). Kept the repo
   dependency-clean (glial + glade-decl); the LIVE demo verify against the real
   node is the end-to-end. `chat.groups` metadata codec = JSON (glial default,
   language-neutral); `ChatLine` stays taut.
5. **Supplier is NOT in the message hot path** (brief + §2). `startChatSupplier`
   only (a) appends BindingDecl-shaped declaration records per group via
   `session.append` (runtime append path §3) and (b) `serveShare`s the
   `chat.groups` value. It never appends `chat.msgs` — posting is a CLIENT append
   (`postChat`). Asserted in the unit test (no `chat.msgs` op from the supplier).
6. **Demo chat wiring bypasses `manifestScope`.** The demo's WORKSPACE_MANIFEST
   scope resolves only doc/account domains; chat uses `share:"chat"`. Rather than
   extend the demo's grant/scope/domain policy, chat mounts build the wire route
   directly from the typed surface (share/gladeId/shape/key all on the handle).
   Stage-1 is allow-all, so no grant entry is needed; `stubGrant` untouched. One
   glial log mount per group registered up front; the selected-group grip atom
   picks the view (grip-style, no React state). `client.hello?.(user)` added to
   `startGladeSync` (the P0.S7 stub replacement); `postChat` stamps
   `principal = user`.

## P1.S3 grazel composes the two suppliers — build notes (2026-07-12)

Owned repos only: **grazel** + **glade** (one commit each). grazel now spawns
glade-gwz as a child supplier; the demo gained a Gwz tab. Gates: grazel 17 lib +
2 integration (the new `grazel_both_mode_composes_gwz_supplier` boots both +
supplier and round-trips `gwz.status` over the wire) + clippy clean; glade demo
build + grip-share 18/18 + client-ts 9 + node cargo 56. Live-verified in-browser
(vite → grazel node :9099): the Gwz tab ran `status` (exit 0, real gwz output,
attributed to the demo user), a disallowed verb (`commit`) came back as data
(`ok:false`, allow-list error), and a `stream:true` run's `gwz.output` converged
live keyed by run id (10 lines + `done, exit 0`). Clean shutdown tore down node +
supplier. Ambiguities resolved (smallest faithful call):

1. **App-file byte-identity RECONCILED (they had DIVERGED).** The two
   `grazel-app.glade` copies were NOT byte-identical at the start of this step:
   `glade/apps/` carried the `workspace ws-razel razel` block (added at P0.S2/F1)
   that `grazel/apps/` lacked. Resolved by making both byte-identical
   (`shasum 34b547b1…`), which required grazel's copy to GAIN the workspace line —
   independently REQUIRED for function: without it the loading node mints no
   ServeClaim for `ws-razel`, so `(ws-razel, gwz.ops)` never routes to the
   composed supplier. Both copies now hold: 7 bindings (the 4 workspace surfaces +
   `gwz.output` + `chat.msgs` + `chat.groups`), 1 service, 2 seeds, 1 workspace.
   A DUAL-MAINTENANCE note is in the file header — future dedup item.
2. **The glade copy is a NODE TEST FIXTURE — growing it forced node-test edits.**
   `glade/node/{appdecl,exchange}.rs` load `../apps/grazel-app.glade` and assert
   its exact record count. The +3 supplier bindings broke 3 tests
   (`grazel_file_matches_the_trace_shape`, `registering_twice_appends_nothing`,
   `grazel_attach_end_to_end`) + the E2E binding-list assertion. Updated the
   counts (4→7 bindings; 8→11 records) + the asserted id set + comments. In scope
   (I own glade/node); a direct consequence of the required app-file change.
3. **ggg-viz `s-app-register` trace is now STALE (flagged, not owned).** Its
   "4 bindings, 1 service, 2 ACL seeds" shape no longer matches grazel's declared
   surfaces (7 bindings). ggg-viz is read-only for this step; the trace should be
   reconciled to the composed shape in a follow-up (a background task was
   spawned). Discipline note: normally trace-leads-build, but the app-file growth
   is the P1.S3 deliverable and the ggg-viz update is a separate repo/step.
4. **Supplier root = `<data>/files`** (literal to the brief). The gwz supplier
   serves the app-owned `files` slot of the `<data>/{sys,files,config}` layout —
   the data-seam rule (glade only ever sees the DECLARED surface). Live verify
   `gwz init`'d that path in a temp dir (never the real `~/.glade`; HOME pinned).
5. **Grazel passes the node's ACTUAL ws port** to the supplier (`--node
   ws://127.0.0.1:<parsed-port>`), not `--node-port` — handles OS-assigned ports
   (`--node-port 0`). Composition is non-fatal: `--no-suppliers` disables; an
   absent `--gwz-supplier-bin` (default `../glade-gwz/target/debug/glade-gwz`) is
   a loud SKIP; a spawn failure or supplier exit only LOGS (grazel lives on — a
   supplier is optional). A SIGINT/SIGTERM to grazel, OR a node exit, tears the
   supplier child down (a second PID tracked beside the node's).
6. **Chat surfaces are DECLARED, not served by grazel.** grazel does NOT run the
   chat supplier in P1 (it is TS/in-process in the UI host); per the brief the
   chat groups ride grazel-app.glade as pre-declared bindings (`chat.msgs` log +
   `chat.groups` value) so the surfaces exist node-side regardless of a running
   TS host. Composition note recorded in the app-file comment + README.
7. **Deny demo reconciles "picker limited to allow-list" with "run a disallowed
   verb".** The verb picker is limited to `{status, ls, diff}` (the happy path,
   per the brief). To satisfy the live-verify "run a disallowed VERB → failure as
   data", the panel adds an explicit **deny-demo** button that sends `commit` —
   the picker stays limited while the honest `ok:false` failure is demonstrable.
8. **Streaming mount = a glial log tap keyed by a run-id PARAM.** The `gwz.output`
   mount's fill key is `{ param: GWZ_RUN_ID }`; `streamGwz` exchanges `stream:true`,
   `client.subscribe`s `(ws-razel, gwz.output, run_id)`, then sets GWZ_RUN_ID so
   the tap remounts to the run key and its fold converges live (grip-style, no
   React state hook). Codec = JSON (`GwzOutputRecord`, the glial default). The
   typed `gwz.output` surface handle is defined in the demo (no glade-gwz TS pkg
   exists — it is a rust supplier).
9. **grazel integration probe uses `glade-client` as a dev-dep** (path
   `../glade/client-rs`, per the brief) + `tokio`; it string-matches the response
   payload (`"exit":0` / `"ok":false` + `allow-list`) rather than adding a
   `serde_json`/`glade-gwz` dep. Both the node AND glade-gwz binaries are built
   (skip-if-absent, twice-retried); `gwz` absent ⇒ loud SKIP (needs `gwz init`).
10. **Fixed a pre-existing clippy warning** in grazel `main.rs::serve_http`
    (`io_other_error`, rust 1.96) since it sat in a file this step already
    rewrites — a 1-line, behavior-identical change to keep the crate clippy-clean.

## 2026-07-12 — P1.S4 decisions (gryth-ui wiring; recorded for the agent)

gryth-ui branch `glp-0006-p1s4-gryth-panels` @ bffbdd0 (Gianni integrates;
gryth-wz pushes are his). 74 tests + build green, live-verified end-to-end
(grazel --ui serving dist; chat attributed both ways; gwz run/stream/deny).

1. **chat.msgs codec = JSON in gryth-ui** (demo uses taut ChatLine) — the two
   UIs are NOT wire-interoperable on chat payloads; fine while each is its own
   grazel deployment; unify (taut codec in @grythjs/glade) before mixing
   clients on one node. INTEROP CAVEAT.
2. MemoryStoreEngine (not IndexedDb) — convergence-first; persistence upgrade
   later (avoids top-level await in plugin init).
3. New @grythjs/glade package = the bootstrap seam (plugin-api idiom).
4. gryth-wz grip-core/grip-react promoted to pnpm workspace members
   (workspace:* — fixes nested file: resolution, preserves grip singleton).
5. tsconfig.app relaxed (erasableSyntaxOnly/noUnused*) because tsc deep-checks
   foreign glade-wz TS source — TECH DEBT, restore when glade pkgs ship .d.ts.
6. glade.ir.json VENDORED into @grythjs/glade — refresh on wire change.
7. Cross-workspace file: links are INTERIM (publish-or-member later).

## 2026-07-12 — glade-share ruled (Gianni) + spine planning wave landed

glade-share rulings: (1) chat sharing grants to the MEMBERSHIP SNAPSHOT at
share time (per-principal grants make not-future-members the default, no
expiry machinery); (2) sharer-isn't-owner → mint what I can, AUTO-REQUEST
the rest; (3) link values are ID-type k:v pairs, developer's discipline —
fat inline state is a documented smell, no enforcement; (4) access requests
ARE pending grants — CapabilityGrant-shaped with a disposition lifecycle,
stored in the target share's policy binding (same shape, same place);
approve↔revoke flips are INSTANCE-MINTS (revocation-wins preserved; the
chain is the history). Spec: dev-docs/glade/suppliers/glade-share.md;
traces queued: s-link-share, s-link-knock.

Wave landed: 6 spine traces (ggg-viz 3a4d59f, 242→311, INV-6 as a real suite
invariant) + PlanGladeUsers.md + PlanGladeWorkspaces.md. Consolidated design
feedback AWAITING Gianni: R1 materialization seam (lean A: node-issued
ws.materialize, disk-gates-records) · R2 glade-workspaces = distributed role
not single session · R3 accept/mint record authorship · C1-C6 confirmations
(dir.principals replication pin, selection grip-only, ws name dups
tolerated, clone manifest-with-records, clone=eligible-not-seize, per-verb
gating) · minor pins (Hello post-migration, flat sponsorship log, petname
surface home, fp6 fallback, invites' declaring share, single-root→map is
substrate).

## 2026-07-12 — diff ruled IN + gwz-core canonicality (Gianni)

- **Cross-surface diff is NOT deferred.** The E2E audit's KNOWN-DEFERRED was
  a statement of substrate build-status that I wrongly carried into the
  roadmap/enumeration. grip-lab already has the feature (parity), the atlas
  covers it end-to-end (s-diff-service auto-instantiation; s-svc-shared
  discovery-before-spawn), and WD-8 (2026-07-05) already rules the dedup
  policy. **glade-diff** added to SupplierOutlines + SupplierRequirements as
  the first DEMAND-INSTANTIATED supplier — the build driver for base glade's
  service-instantiation machinery. **local/global ≠ two suppliers/specs**:
  the axis IS WD-8's dedup policy (`per-node` recompute default; `global`
  opt-in) on one definition. Working-tree diff within a workspace stays the
  glade-gwz `diff` verb. GladeSupplierModel §1 amended (a supplier may be
  demand-instantiated; the composed artifact is its ServiceDefinition).
- **gwz-core canonical home = the github RELEASE.** glade-gwz (and any glade
  consumer) depends on the released version, pinned. Bugfix path: gwz-core
  MAY be added as a glade-wz member and modified, but changes push/pull back
  to gwz-dev (kept working there) and likely cut a new release — never a
  fork-in-place.
- glade-gwz shape: the one-supplier / ActionKind-gating answer recorded here
  was OVERRULED the same day — see the next section (per-request-type
  supplier family). `forall` terminal-routed (absent from the protocol)
  stands.

## 2026-07-12 — glade-gwz split RULED: one supplier per gwz request type (Gianni)

"The various gwz-core requests are different enough that they need a
different UI and interface — best served as separate glade suppliers."
Overrules the one-supplier shape argued twice by the assistant; conceded for
cause — the split is the glade-native answer:

1. **Gating**: per-operation authorization becomes ORDINARY surface grants
   (existing AZ machinery) instead of a new sub-surface "granted ActionKinds"
   vocabulary inside one exchange — a parallel authz plane, GDL-038-shaped
   smell.
2. **Legibility**: the .glade file enumerates every operation the app
   exposes (the de-noising-lens value); one `gwz.ops` id hid ~12 operations
   behind a payload discriminator.
3. **Composition seams**: a read-only composition simply never attaches the
   mutating suppliers — the read/mutate/egress wall is structural
   (seams-at-inception), not a grant check. The create/materialize suppliers
   ARE the R1 materializer seam.
4. The one-supplier case rested on implementation economy ("N copies of one
   mold") — a design-not-impl-cost violation. The uniform mold is instead
   the shared kit, and makes the family generatable from the canonical
   protocol (gwz-core github release, pinned).

Open dials (Gianni): **grain** — strictly 1:1 per request type vs
split-by-interface (the ruling's own criterion), which merges literal UI
twins (PullHead/PullSnapshot = one pull interface; Capture/Snapshot/Tag a
candidate versioning panel); **packaging** — one `glade-gwz` family repo
hosting N independently-attachable suppliers (supplier = unit of
declaration/grant/composition, not necessarily repo/process) vs N repos.
P1's single-exchange supplier = bring-up artifact; the allow-list dies in
the reshape. Updated: SupplierOutlines, SupplierRequirements,
glade-workspaces.md §2.3/§4.

## 2026-07-12 — supplier spec wave landed (5 agents + 1 stub, all verified)

`dev-docs/glade/suppliers/` now covers the full enumeration (glade-gwz.md
landed last — 247L, verified: 12 role="in" request types counted in the
protocol, repository_path host-path field confirmed at gwz.taut.py:623;
proposes an 8-member split-by-interface partition, both dials carried as
PROPOSED; F5 review addendum F5-10/11 flags its create-family selection
assumptions + events-visibility): glade-files
(216L), glade-terminal (228L), glade-editing (229L), glade-chat (217L),
glade-diff (231L) — each agent-authored, then verified against its cited
sources (trace steps, node/glial/glade-chat source, .glade declarations)
before acceptance — plus glade-razel (deliberate stub; expands when
razel-dev publishes a protocol). ~22 new trace arms named across specs for
the next atlas wave (incl. glade-share's queued s-link-share/s-link-knock).

Substrate gaps the wave surfaced (each spec names its forcing role):
- **Channel→authority route MISSING** (node server.rs routes
  ChannelOpen/Data/Close to the echo provider only) — glade-terminal forces
  P3.S4: a THIRD attach path in GladeSupplierModel §2 beside ServeClaim and
  the exchange provider map.
- **GAP-8's deferred tail** (consumer-chooses delta-vs-refresh against live
  UI + GC-2 conflation) — glade-editing's P4.S2; swmr v1 reaches
  user-testable on that tail alone.
- **Service instantiation machinery** (definition fold, spawn, refcount,
  lease, teardown) — glade-diff is its build driver (P3 tail).
- **Fixture drift**: grazel-app.glade:24 declares `ws.diff` as
  `log … from-cursor`; trace A5 + outline say `value` — fix queued for
  Gianni (dual-maintained node-test fixture; changes with the reshape).

CONSOLIDATED RULING QUEUE (per-spec §10s hold the detail):
- **P4 gate (editing Q1/Q2)**: does the done-criterion demand simultaneous
  keystrokes (crdt v1) or is live cursor-stable handoff enough (swmr v1,
  the lean — swmr as write-ownership policy over `log`, EditClaim =
  ServeClaim re-keyed)?
- **diff CONTENTIOUS pair**: derived-binding home = structural derivation
  (`svc` reserved namespace, never a stored share) + borrowed capability =
  sponsor-delegated attenuated agent principal with the leak guard
  (diff-read must subsume both source-reads).
- **chat group-as-share vs group-as-key**: AZ-16's grant unit is a share;
  lean = a real group graduates to its own share (create-a-group ===
  create-a-share); decides whether keyed-commons survives stage-2.
- **files P3 gates**: window Shape-vs-projection (lean: one contract,
  shape-dependent fill) + blob carrier (iroh-blobs native / Chunk-frame
  browser?) + lazy-vs-eager bytes + `ws.blob` declared-vs-bare + AZ-1 in v1.
- **terminal**: TermIn union vs in-band escape; disconnect =
  survive-detach (lean) vs grace-kill; local-only by zone-convention vs
  claim marker; forall fresh-vs-reused session.
- **GAP-10 retention now unavoidable**: named independently by files
  (window caches + blobs), chat (first heavy unbounded append), terminal
  (scrollback) — needs one ruling, not three.
- Minor: chat codec forward-cut as a hard P2 gate; who may create groups /
  define services (lean: any onboarded principal, ordinary append);
  message edit/delete tombstone-or-never; editing save/autosave + gwz-pull
  conflict; diff policy defaults.

## 2026-07-12 — second review (SR56) verified; gwz protocol staleness CONFIRMED

SupplierSpecReview-56.md landed (29 findings, independent). F5 verified its
heaviest claims against disk — all confirmed:

- **SR56-01, the big one: glade-gwz.md was derived from a STALE protocol
  copy — the F5 session's own error.** Canonical gwz-core v0.9.1 (gwz-dev;
  github owebeeone/gwz-core) = 1476 lines, **24** `role="in"` methods; the
  glial-dev copy the spec agent was pointed at = 727 lines, **12** methods,
  different hash. The agent brief asserted the copies were "the same
  protocol" WITHOUT diffing them. The split RULING and the kit mold
  survive; the 8-member partition and inventory do NOT — regenerate from
  v0.9.1 (missing: clone_workspace, stage, commit, branch, stash, ls,
  list_snapshots, repo_sync, member ops, working-tree diff, …).
- SR56-02 confirmed: canonical requests CARRY host paths
  (`CreateWorkspaceRequest.workspace_root`:957, `StageRequest.cwd`:1134) —
  "payload IS the canonical Request" + "path-free" cannot both hold; needs
  a glade-owned path-free DTO projected server-side (endorsed).
- SR56-03 confirmed: `TagRequest`:1102 carries `TagOp` + `remote`
  (fetch/push) + `all` (push --all) — tag is NOT pure local-mutate; the
  capture/snapshot/tag merge crosses the egress wall. Regenerated partition
  must be capability-audited per METHOD, not per label.
- SR56-04 confirmed: `glade-gwz/src/supplier.rs:146-147` trusts a
  caller-supplied `req.principal` — attribution is forgeable; effect
  suppliers never receive node-authenticated requester context. THE
  cross-cutting stage-2 seam (lean endorsed: trusted provider-call context,
  never caller payload).
- SR56-10 confirmed: the node registry watches `dir.bindings` only
  (registry.rs:48); chat.decl JSON appends are never consumed — chat's
  runtime-declaration claim is decorative. F5 concession: its landing check
  verified the supplier WRITES chat.decl, not that anything reads it.
- F5 also concedes SR56-24 (swmr body-append fencing under-specified — the
  s-takeover reuse direction stands; the fence must ride the body ops).
- One SR56 framing sharpened, not disputed: SR56-16 — the share in the
  binding route IS the natural selection conveyance (each workspace is a
  share); their own lean says so. An underdetermination in
  glade-workspaces §2.1, not an architecture hole.

RULED (Gianni, 2026-07-12): the pin is **gwz-core v0.9.2 on taut v0.8.1**
(v0.8.1 = latest; corrected from an initial v0.8.0 misstatement — SR56-2-01
caught the drift, Gianni ruled "use the latest taut, v0.8.1"). Respec
commissioned same day against the gwz-dev checkout (verified at the v0.9.2
release commit, 24 role="in" methods). NOTE: the deeper half of SR56-2-01
(regen.py floats to latest PyPI with no in-artifact version assertion / CI
gate) is SEPARATE and still open — "use latest" settles the version, not the
reproducibility gate; a stamped pin + corpus regen remains a pre-plan item.

Respec LANDED + VERIFIED same day (glade-gwz.md v2, 413L): the 24-method
inventory is LINE-EXACT against v0.9.2 (all cites checked), the 3-method
role="out" trio (events.subscribe/operation.result/diff.output) and the
forall "gwz-core MUST NOT handle" comment verified verbatim. Partition
PROPOSED: **21 members** (4 read · 10 local-mutate · 4 egress · 3 create)
= 24 − 4 identical-blast-radius twin-merges + the **tag split at the
egress wall** (tag-local {create,list,delete} / tag-remote {fetch,push});
pull_snapshot regrouped with materialize per the protocol's own comment
("Materialize members to a named snapshot" — soft edge: a fetch-on-missing-
objects nuance rides the grain ruling). All 9 review findings dispositioned
in §13 (path-free DTO projection, canonical trio + result surface, creation
arm + idempotent create recovery, share-address-as-selection, requester
context consumed-not-built, events visibility split read/local-commons vs
egress-grant-keyed). Opens §14: grain (21 vs 24; read-band merge), tag
split vs per-op gate, DTO + repository_path constraint, result surface
shape, events grain/visibility, packaging + dispatch, requester-context
seam confirmation.

## 2026-07-12 — SR56 SECOND review (SR56-2): went into built code; 6 spot-checks all held

SupplierSpecReview-56-2.md (37 findings) is DEEPER than round 1 — it read the
node/glial/wire source, not just the specs, and its center of gravity is
substrate correctness+security, not spec seams. F5 verified the six
load-bearing claims on disk; ALL confirmed:

- **SR56-2-21 — a LIVE correctness bug in shipped glial** (not a spec issue).
  `instance.ts:133-140 assembleLog()` re-sorts ALL ops by (lamport,origin,seq)
  every fold; `foldAndBroadcast()`:156-163 computes the delta as
  `whole.slice(emittedLen)` — an INDEX over a re-sorted list. A late op with a
  lower lamport shifts the list: `[B]` emitted, then A@lower arrives →
  `[A,B]`, `slice(1)`=`[B]` re-emits B and never emits A. Out-of-order mesh
  delivery triggers it. Directly falsifies the editing spec's "logDelta is the
  cursor-stable basis." **RESOLVED 2026-07-12** — `instance.ts` now diffs the
  delta by op IDENTITY (an `emitted: Set` of `(origin,seq)`), not a positional
  slice: each op is emitted exactly once regardless of the whole's re-sorting;
  `hydrate` seeds the set so a reload does not re-emit. The whole stays in
  `(lamport,origin,seq)` order (still matches `foldLog`/the fold oracle). The
  "immutable append order" alternative was ruled out — it would diverge from the
  cross-language fold. Regression tests in `glial/test/session.test.ts`
  (interleaved out-of-order + reload arms) fail on the pre-fix code, green after;
  suite 62/62, typecheck clean.
- **SR56-2-03 — provider hijack, CONFIRMED.** `exchange.rs attach_provider`
  does an unconditional LWW `providers.insert((share,glade_id), sid)` — no
  existing-provider check, no attach auth. Any session subscribing a DECLARED
  exchange becomes THE provider. Defeats glade-gwz's "unattached member is
  structurally absent" wall: grazel-app.glade DECLARES the whole family, so an
  untrusted session attaches to `gwz.push` or replaces the status provider.
- **SR56-2-02 — panic-decode DoS, CONFIRMED in shape.** `declared_exchange`
  (exchange.rs:60-78) calls `from_cbor(&cbor::decode(op.payload))` with NO
  fallible path over every dir.services/dir.bindings op; one malformed
  persisted record = restart-stable denial of every exchange classification.
- **SR56-2-24 — the diff leak-guard formula F5 accepted is BACKWARDS.**
  `read(ws.diff) ⊇ read(left) ∪ read(right)` under the reader-set reading says
  "every source reader reads the diff" — the opposite of the intent. Safe form:
  `Readers(diff) ⊆ Readers(left) ∩ Readers(right)`, per-principal
  `can_read(left) && can_read(right)`. F5 concedes — accepted it uncritically.
- **SR56-2-27 — razel protocol EXISTS.** razel-wire-api 0.1.0, 10 methods,
  ratified IR + fail-closed wire. glade-razel's "no protocol exists" premise is
  STALE (the deferral may still hold on the wait-for-pinned-RELEASE rule —
  Gianni's earlier razel-naming ruling — but the stated trigger has fired).
- **SR56-2-14 — glade-users §2 overclaims.** `Op` has no signature
  (generated.rs), store ACCEPTS absent `prev` (store.rs:121-147, verified), so
  §2's "glade already has every primitive — signed chains, equivocation proofs"
  contradicts §5's own "signatures structural where the crypto seam is stubbed."
  The fork proof convicts a STRING origin, not a keyholder. §2 must walk back.

Not fully verifiable by F5: **SR56-2-01 taut pin** — regen.py pins to a PyPI
taut-proto release (real, external) but the reviewer's `.regen-venv` path
showed **0.8.1** while Gianni's pin statement said **0.8.0**. RESOLVED
(Gianni): use the latest, **taut v0.8.1** — the disk was right, the earlier
v0.8.0 statement was the error. The reproducibility concern stands as a
SEPARATE open item (no in-artifact version assertion / CI gate; regen floats
to latest PyPI) — settling the number does not make the pin executable.

CATEGORIZATION (the reframe this review forces):
- **(A) Live bug, fix now, spec-independent:** SR56-2-21 logDelta — DONE 2026-07-12 (identity-diff; see the entry above).
- **(B) Substrate seams the specs defer to "stage-2" but are STAGE-1
  prerequisites** (the review's deepest structural point, SR56-2-09/13):
  provider-attach auth (03), fail-closed decode (02), requester context (04 /
  2-09), identity-bound `self:` keys (05/20), per-op signatures (14). Model §4
  makes attribution a STAGE-1 must, but attribution needs requester context
  that the specs schedule for stage-2 — so "stage-1 allow-all" is not actually
  exercisable/safe as written. This moves items OUT of the stage-2 batch.
- **(C) Spec design holes (round-1 class):** leak-guard formula (24), blob
  shape+authz (17), ws.tree subtree redaction (18), tag fetch≠push least-priv
  (10/11), diff private-vs-dedup (23), diff teardown stale replica (25),
  service-def exec safety (26), SWMR fence (35), doc.save delegation (22).
- **(D) Normative-doc drift:** SupplierOutlines/SupplierRequirements still
  describe v1 gwz (2-04) and still assign glade-share a direct membership
  ceremony the spec dropped (2-30 / round-1 SR56-08) — the normative files lag.
- **(E) Stale premises:** razel (27), taut-pin reproducibility (01).

Cross-validation: SR56-2 concordance AGREES with F5-1/2/4/6/9/12 (incl. the v2
addendum) and correctly DISAGREES with two F5 landing claims already conceded
(chat.decl "verified" — never consumed; EditClaim "sound" — SR56-2-35 no fence).

## 2026-07-12 — RULING WORKSHEET RATIFIED (Gianni); propagation wave launched

`RulingWorksheet.md` is the ratified decision set for spec→plan conversion —
Gianni ruled all ~40 rows (§I–§VIII). Structural landmarks:

- **Security substrate is STAGE-1, not stage-2** (B1–B5, all RULING A): provider-
  attach auth + reject-replacement + monotonic epoch (B1); fail-closed decode +
  quarantine (B2); `ProviderCallContext` — node-authenticated requester beside
  (never inside) the DTO (B3); identity-bound `self:` zone keys derived from the
  B3 principal (B4); device-possession proof + signed security ops (B5). These
  gate everything and re-scope every supplier's stage split.
- **gwz family = 25 interfaces** (C-gwz-1), mechanically DERIVED (one surface per
  `(method, capability-class)`; op-enums split by class; the generator
  recalculates from the IR and fails on drift). 24 methods − 4 twin-merges + tag
  4-way (`tag-list/mutate/fetch/push`) + stash 2 (`list/mutate`) + branch 2
  (`list/mutate`). Corrected up from the inconsistent 23 — F5 caught that the
  "don't merge read+mutation authority" rule, applied to tag, forces stash/branch
  too. Plus: path-free DTO + `RepoImportHandle` for out-of-tree adds (C-gwz-3);
  closing replicated `OperationResult` log record (C-gwz-4); commons events +
  egress-grant reads (C-gwz-5); one repo / one process-N-sessions / in-process
  gwz-core (C-gwz-6); workspaces owns `workspace.create`, gwz owns the internal
  materializer, `ws.ops` retired (C-gwz-7); durable
  intent→materialized→registered→claimed→complete create machine (C-gwz-8).
- **Editing = text CRDT v1** (H-P4) — swmr disallowed as a fallback (my swmr-cheap
  lean was wrong: the write-fence was never cheap). Element IDs `{actor_id,
  counter}`, tombstone deletes, identity-based deltas tolerant of out-of-order,
  cursors private under B4.
- **diff** (D1–D5): structural versioned `DemandServiceDefinition`; compute key =
  digest + sandbox-version + ordered sources (NOT viewer); per-principal
  `can_read(left) && can_read(right)` re-evaluated per hop (D3 fixed the backwards
  leak-guard, candidate INV-7); generation envelope
  `pending|ready|stale|absent|denied|error`; sandbox = separate define-vs-execute
  capabilities, no ambient authority.
- **files** (D6/7/8/12/13/14): blob fetch exchange w/ delivery-time authz; tree
  per-directory-keyed + revision; FULL mutable window v1 `{workspace,path,revision}`
  (base-glade routes, **glial reassembles**, files owns snapshot); `doc.save` =
  compare-and-replace; at-rest truth + `doc.editing` marker; one `RootRelativePath`
  + safe-open. **F-GAP10 retention** ruled once with a defaults table.
- **New mechanisms needing spec homes + trace arms:** `ProviderCallContext`,
  `RepoImportHandle`, `DemandServiceDefinition`+sandbox record, `ChatQuotaSettingsV1`
  (E-chat-4, 50 groups/principal), `TermOut{generation,offset,bytes}` (D9), the
  `share.create/invite/grant/revoke/status` family (E-share-1).
- **AZ-16 amendment (B4):** "privacy is a key" → "privacy is an IDENTITY-BOUND key
  (derived from the authenticated principal, never caller-asserted)."
- **razel** (§VII): defer ALL razel-facing interfaces until the gwz family is
  working (concrete gate); candidate names are reservations only.
- **A1 promoted:** the glial logDelta identity-delta fix (running as a task) is now
  a SHARED contract H-P4 (CRDT deltas) + D8 (window reassembly) both import —
  design it as a general identity-based-delta primitive, not a narrow patch.

Propagation wave (10 opus agents, one writer per file, worksheet = source of
truth, cite-by-ruling-ID) launched to write each disposition into the specs +
model docs. Atlas trace-arm authoring (INV-7 + the CRDT/quota/knock-denial/
provider-hijack-negative/decode-fuzz/self-key-negative/create-recovery/tag-egress
arms) is the NEXT wave (trace-leads-build), after specs land.

**WAVE COMPLETE + VERIFIED (2026-07-12).** All 10 agents landed; 12 files
propagated (9 supplier specs + glade-razel stub + GladeSupplierModel §8
(B1–B3) + GladeAuthzModel §3b/§11 + AZ-16 §9 B4 amendment). Consolidated
consistency sweep PASSED: no stale pre-ruling language in any spec (only the
review docs quote old state); shared mechanism names consistent across
definers + all consumers (`ProviderCallContext` ×7, `RootRelativePath`,
`RepoImportHandle`, `INV-7` diff↔authz, `DemandServiceDefinition`,
`ChatQuotaSettingsV1`, `TermOut`, the `share.create/invite/grant/revoke/status`
family); the D8 reassembler is one shared glial primitive across
files/editing/logDelta(A1); gwz = exactly 25 members, names match
workspaces' refs; the three honesty retractions (leak-guard flip, users §2.4
overclaim, swmr→CRDT) all on disk. Specs are now internally plan-ready.

REMAINING before plan conversion: (1) the NORMATIVE plan-docs still lag —
`SupplierOutlines.md` + `SupplierRequirements.md` describe pre-ruling gwz
(v1 Request-on-wire) and the dropped direct-share (SR56-2-04) — they need a
catch-up pass; (2) the atlas trace-arm wave (INV-7 + the named negative/
security arms); (3) A2 taut-pin reproducibility gate; (4) the grazel-app.glade
fixture at 25×3 surfaces now REQUIRES codegen (F5-14, no longer optional).

**(1) DONE 2026-07-12:** SupplierOutlines.md + SupplierRequirements.md rewritten
to the ratified specs (security-substrate-is-stage-1 section; 25-member gwz;
direct share.create ceremony; CRDT; razel gwz-gate deferral) — verified no
stale terms, all anchors present in both.

**(2) DONE 2026-07-12 — atlas trace-arm wave COMPLETE (workflow, 23 agents).**
44 scenarios authored across 10 clusters into ggg-viz (gwz/diff/files/terminal/
editing/chat/users/share/workspaces + B1/B4/B5 security), INV-7 landed in
invariants.ts + test vectors (four-combo derived-surface authz), all registered
in index.ts/catalog.ts. Suite GREEN (579/583 scenario tests, tsc clean) with
NO invariant weakened. The workflow's adversarial-faithfulness stage caught 2
real defects (F5-verified, both fixed + re-run green): (a) s-edit-cursor
rendered "Xhell" — a DROPPED insert contradicting its no-drop claim; corrected
to "XhellY" (Y survives after the tombstoned "o"). [Addendum: the faithfulness
pass ALSO flagged a secondary anchor bug that F5 wrongly dismissed as a reviewer
misread — in s-edit-cursor's own numbering e0="h", so inserting "X" @e0 folds to
"hXello" not "Xhello"; fixed (by a second, user-spawned agent) with an explicit
⊥ doc-head sentinel so @⊥ prepends. Both fixes composed cleanly; suite
re-verified green. Lesson: element ids are document-scoped — don't import one
trace's e0 convention into another.] (b) s-chat-edit's deny arm
faked a "sig bad" rejection of a VALIDLY-signed cross-author supersede — the
3-check verify-as-ingest wouldn't catch it; rewrote as a distinct EDIT-AUTHORITY
check (signer must be author or hold a moderation grant). That surfaced a real
SPEC GAP — glade-chat §3.5 never ruled WHOSE authority may edit — now flagged as
an open in glade-chat.md §3.5 (v1 lean: author-only, moderation deferred).
Honest atlas-vocabulary follow-ups the authoring agents flagged (not defects,
out-of-scope types.ts changes): `text-crdt` is absent from the TS `Shape` enum
(editing doc.body rides `shape:'log'` — honest, flip when it lands) and there is
no `CHANNEL` FrameKind (terminal's live pty channel / the P3.S4 third attach
path is modeled via the C-family keyed-subscribe). A types.ts pass (add
`text-crdt` Shape + a `CHANNEL` frame) is queued.

REMAINING before plan conversion: A2 taut-pin reproducibility; the
grazel-app.glade codegen (F5-14); the small types.ts atlas-vocabulary pass
above. Then the plan wave.

## 2026-07-12 — INDEPENDENT atlas review (56s) — deep, 13 findings, F5 verified 4 sharpest on disk (all CONFIRMED)

AtlasGladeReview-56s.md (independent, per AtlasGladeReviewPrompt.md) is far
deeper than the workflow's internal faithfulness pass. F5 verified the four
load-bearing findings against disk — ALL hold:

- **THE ROOT CAUSE (F56S-01/02/03/04/05/06):** the atlas fold engine
  (`fold.ts::actorStateAt`) applies `sets:` patches LITERALLY — it has no
  DERIVATION for computed results (CRDT text, window generation, diff
  freshness, link-closure expansion). So a trace claiming a derived outcome
  can only HAND-ASSERT it via a patch; the suite (structure + opt-in
  invariants) cannot recompute it. **F56S-03 verified: s-edit-crdt's delete
  refolds only `local2` (→"H"); `local1`'s fold value stays "HW" (line 130
  never updated) and both readers' views stay "HW" — divergence, while the
  trace claims convergence.** This is a genuine contradiction, not just
  unproven narration.
- **F56S-07 verified:** s-attach-authn (the B1 composition-wall trace) is
  keyed **19× to the RETIRED `gwz.ops`** multiplexer (only 8 `gwz.status`,
  tunneled as a verb) — it blesses the surface whose retirement IS the
  capability-grain ruling. Retarget to a real final member (`gwz.status`).
- **F56S-08 verified:** INV-7 (`invariants.ts:84-104`) is FAIL-OPEN — it only
  fires when the sender fold carries a `derived <share>/<gladeId>` key; omit
  that key (or `payload.gladeId`) and it returns clean. A leak guard should be
  fail-closed. (Same opt-in-by-shape design as INV-4/5/6, but for a SAFETY
  invariant that's too weak.) Also: `account <src>` exempts a source grant
  even when the source is a workspace, not an account domain.

**HONEST RESCOPE (F5 over-claimed):** "the atlas closed the last design gate /
these are the conformance targets" is TRUE for the ~15 STRUCTURAL/AUTHZ/ROUTING
traces (the review's "What held" — diff-authz, tree-subtree, gwz-compose/
tag/requester, term-takeover/remote-denied, share-revoke, knock, ws-*,
device-cert, chat-edit) — those properties ARE foldable and genuinely proven.
It is FALSE for the DERIVED-COMPUTATION traces (editing ×5, s-file-window,
s-diff-generation, s-blob-fetch, s-link-share) — those NARRATE. F5's earlier
s-edit-cursor/s-chat-edit fixes corrected the asserted VALUES but added no
derivation (the atlas can't), so they remain narration (F56S-02).

**types.ts pass is now a PREREQUISITE, not optional (F56S-09/10):** `doc.body`
riding `shape:'log'` (no `text-crdt` Shape) and terminal's live channel riding
`SUBSCRIBE/OPS` (no `CHANNEL` frame) are exactly what LET the faithfulness
breaks pass. Adding the typed shapes + structured element/delta payloads +
a render-convergence / tombstone-non-resurrection invariant (and making the
window/diff-gen derivations real) is required before those traces are trusted.

DISPOSITION (Gianni to direct): the derived-computation traces need EITHER (a)
real derivation machinery + typed vocabulary in the atlas (bigger — a CRDT
text-fold, a window reassembler, a diff-gen computer, + convergence
invariants), OR (b) restructure them to only claim what fold-patches prove.
F56S-07 (retarget B1) + F56S-08 (fail-close INV-7) + F56S-11/12/13 (missing
negative arms) are smaller, independent fixes. The STRUCTURAL traces stand.

## 2026-07-12 — discovery reified as `glade-discover` (Gianni direction)

Design captured: `dev-docs/glade/GladeDiscoveryModel.md`. Discovery — the
routing spine every supplier is found through — becomes a named, independently
testable base-glade kernel (`glade-discover`) behind a swappable
**`CommsSubstrate`** seam (real = iroh carrier; sim = a deterministic in-memory
network with injectable latency/loss/reorder/dup/partition + a controllable
clock). This is the mock→real discipline AND the direct answer to the
AtlasGladeReview-56s finding: the trace atlas can't DERIVE discovery
computation (convergence, failure handling), but an executable kernel run
against a seeded sim can — so the spine's failure cases (spoofing, equivocation,
lease lapse, epoch takeover, partition/heal, precedence conflict, diff
lifecycle, ACL-deny, offline, reorder/dup/drop) get PROVEN, not narrated.

Three model pieces, all OPEN (Gianni's rulings, doc §9):
- **(a) serve-grant chain (§4):** a ServeClaim routes only with an owner-rooted
  signed serve-grant, verified at fold time — a signature ≠ authorization (the
  s-chat-edit lesson). serve ≠ author bounds blast radius; B1/B5 imply it.
- **(b) locality rendezvous (§5):** DEFERRED — no chubby/Paxos (WD-8 already
  replicated-fold + rendezvous); topology is a latency choice behind a monotone
  fold, so multi-level rendezvous (folds into WD-6) drops in later with NO
  data-model change; v1 flat + iroh.
- **(c) most-specific-match (§6):** advertisements are `(share, glade-id?,
  key-pattern?)` match records; routing = authorized-most-specific-match,
  precedence-by-specificity SCOPED per-binding (the diff instance-claim wins its
  exact binding, not the whole workspace). Makes demand-instantiated suppliers
  fall out as data, not code.

RULED addendum (Gianni, same day) — **virtual time** (doc §2.1): the seam's
clock is a scheduling contract, `now()` + async `notify_at(t_abs)` (ABSOLUTE
deadlines — leases are absolute); the sim clock IS an ordered event queue
(deliveries + wakeups unified — send = event at now+latency); congestion drift
fires events at `t+d, d ≥ 0` — **late, never early** — making
correctness-under-late-wakeups a testable kernel obligation (late renewal
re-claims at epoch+1, never resurrects); `now()` is **monotone non-decreasing**
with deterministic tie-break (time, insertion seq) for seeded replay; the
clock never enters the fold (no-time-in-fold stands — `now` only in
projections + action scheduling). §7 matrix gained late-renewal-race,
congestion-storm, and retry-under-drift cases.

GladeDiscoveryDesign v3 (2026-07-17) — the SEMANTIC FREEZE, closing Review56-2
(GD56R2-01..12) with Gianni's amendments. Meta-ruling (Gianni): the sim proves an
impl only against the semantics its oracle encodes, so v3 DECIDES + freezes the
public types / state transitions / invariants / failure outcomes; the s-disc-*
scenarios are then written as RED tests BEFORE code; the next review evaluates
code+scenarios, not prose. Amendments landed: R2-01 discovery consumer-authz-BLIND
+ internal-only API (node authorizes before exposing NodeId/def/instantiation);
R2-02 derived-service authority plane (exec-grant scoped to (def_ref, compute_key)
+ derived principal bound to node; a general exec grant ≠ arbitrary instance
claims); R2-03 Append→OpAccepted→Gossip (pure kernel never signs; gossip only
after persist; restart-idempotent); R2-04 PERSISTED effective-wall watermark +
restart→clock-uncertain fail-closed (in-memory-only wouldn't survive restart);
R2-06 stable (epoch, claim_id) tie-break — NOT record-hash (renewal changes the
hash → still alternates; conceded); renewal≠takeover, loser tears down;
R2-09 (NEW fork Gianni caught) definition-matching moved ENTIRELY to the service
manager — discovery returns only Matched/NoClaim; R2-10 IngressId + Reply effect,
pending DELETED; R2-07 GrantId=record identity + owner proof + authorized-revoker;
R2-05 full sync-round SM (SyncId/SyncStart/SyncOps/SyncEnd + timeout/retry/dup);
R2-08 three verdicts (structural / publish-authority / live-capability) + durable
pending-proof index; R2-11 compaction DEFERRED (retain chain + bounds). Six
frozen invariants INV-D0..D6 are the oracle the scenarios assert. Meets 3 of
Gianni's 4 acceptance conditions (R2-09 assigned; tie-break stable; restart-safe
clock); the 4th — RED scenario data — is the NEXT artifact (needs the repo +
harness). "All closed" is NOT claimed until the red scenarios exist.

---
Superseded: GladeDiscoveryDesign v2 (2026-07-12) — closed Review56 (GD56-01..15). All 4
blockers were verified on disk (wall-clock leases `claims.rs:163`; stream-close
completion `peer.rs:201`; flat `CapabilityRevocation`; my own no-time-in-`Inbound`)
— the review was sound, and its sharpest findings were MY OWN rulings
under-applied (B3 requester on the wire; D3/INV-7 reduced to target-read;
no-time-in-fold violated; B5 grant-chain claimed as reuse-as-is). Gianni ruled
"go with the leans" on the 4 forks; v2 lands them + fixes: `step(state, ctx,
event)` with time+B3-principal on every event; two clock domains (WallMs record /
MonoInstant schedule + fail-closed skew); 3-layer ingest→retained-fold→project
(no-time/authz-in-fold restored); resolution-ONLY kernel (one terminal RouteAns;
forwarding+unreachable = node layer); instantiation OUT (`Absent{needs-
instantiation}`, D5-gated, no unauthorized compute); total order `(epoch, lamport,
origin)` for equal-epoch partitions; equivocation vs contention split; grant v2
(grant_id/issuer/revokes) with serve-grants in the HOME serve-plane
(self-verifying) vs content ACLs riding the workspace; slot+gen / return-path+corr
state keys + wake tokens; explicit `SyncComplete`; frozen determinism + DoS
bounds + typed ingest verdicts. Every fix is a numbered DR-nn with an `s-disc-*`
scenario (traceability table §11). The doc is now buildable; a phased plan is the
next artifact (kernel crate + glade-discover.taut.py + scenario harness).

RULINGS 2026-07-12 (Gianni, GladeDiscoveryModel.md): §4 serve-grant chain +
§6 most-specific-match — **lean adopted (both RULED)**. §5 locality-rendezvous
**DEFERRED** (folds into WD-6); the kernel is topology-BLIND by construction
(only folds records + emits addressed messages), so the overlay drops in later
as new record types + a smarter environment, zero kernel change. **glade-discover
= a new gwz MEMBER REPO in glade-wz** (its own repo; repo creation is a gwz op,
not yet done). **CommsSubstrate DISSOLVED** (Gianni's simplification): discovery
speaks purely taut messages, so there is no transport substrate to emulate — the
kernel is a **pure state machine** `(inbound taut msg | wakeup) → (addressed taut
msgs, wakeups)`, whose only dependency is the §2.1 clock; transport (resolve/dial)
is the node layer's, not discovery's. Tests are **data-driven scenarios** (the
ggg-viz discipline for computation): a scenario = topology + time-windowed faults
+ scripted taut-message inputs + expected converged-state/routing/declared-
failures; the harness runs N kernels over the virtual-time event queue and
asserts. "Other clients" = scripted taut messages, never bespoke client code.

STRATEGIC REFRAME (Gianni's bet): **`glade-discover` + sim is the foundational
build target** — get the routing kernel stable + failure-complete and "the rest
is already well-defined" (every supplier = advertise; every consumer = route).
This is the program's real-risk (integration surface) collapsed to one testable
kernel. Sequencing implication: the two OPEN rulings (a, c) + the CommsSubstrate
trait shape gate this; then the kernel + sim is buildable ahead of the supplier
plan wave. Worth folding into GladeProgramStatus.md as the new lead build.

MERGED pre-plan order (both reviews): (1) regenerate glade-gwz.md from
v0.9.2 + rule the path-free-DTO projection; (2) the security-seam batch —
requester context (SR56-04), identity-bound `self:` keys (SR56-05 — touches
AZ-16's "routing not policy" phrasing), blob-fetch authz (F5-4/SR56-22),
service-definition trust (SR56-25), knock append capability (SR56-09); (3)
restore glade-share's core membership ceremony (SR56-08) + the F5-1/F5-2
zone/coherence rulings; (4) the spec fix-wave; (5) the plan wave.
