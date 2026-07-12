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
