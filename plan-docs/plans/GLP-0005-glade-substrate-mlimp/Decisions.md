# Decisions — GLP-0005

Plan-local decisions and revision history. Substrate-level resolved questions
(GQ-1..9) live in `glade/dev-docs/GladeSubstrateV1.md` §10; this file records
plan-execution decisions and any contract changes after P0 lock.

## 2026-06-13 — plan creation

- **D1. Roll-build home = root `glial-dev`.** `gladev2` branch + `gladev2/`
  tags in root. IR authored at `taut/ir/glade.taut.py` (matches
  `razel.taut.py` / `griplab.taut.py` precedent), corpus at
  `taut/corpus/glade.*`. `glade` and `grip-core` submodules branched `gladev2`
  when a phase first writes them; root pins their commits at each phase tag.
- **D2. Base commit = everything modified in root**, on `gladev2`. Excludes
  the untracked `glade/dev-docs/glade_sec_model-55/` (prior analysis run, not
  this plan's). glade submodule design docs committed on its own `gladev2`.
- **D3. P0.S1 layout (resolved).** IR + corpus in root `taut/`; rust node →
  `glade/node`, TS client → `glade/client-ts` (glade submodule); `grip-share`
  beside `glial-dev/grip-core` (exact spot finalized at P3). Generated-codec
  placement decided at P0.S4. `taut/ir` already exists, so no new dirs gate
  P0 start.
- **D4. Canonical grip-core** = `/Users/owebeeone/limbo/glial-dev/grip-core` @
  `main` 7d52fb1 (clean of wip2 persistence). `grip-dev/grip-core` @ wip2 is
  reference-only. wip2 demo is a behavioral checklist for P3.S4, not a base.
- **D5. taut is the fold authority.** Its `crdt/engine.py` (`local_apply /
  merge / materialize / snapshot`, `lww` + `counter` reference) is the fold
  home; glade reuses it rather than reimplementing folds in node + client.
  Confirmed present and green (106 tests). Folds back into Plan P0.S5/P2.S2 as
  "drive taut's engine," not "reimplement."

## 2026-06-13 — base established (gladev2/p0-start = 2a5a4ef)

- **D6. `taut` is its own git repo** (`main` @ 3ba595b), NOT a registered
  submodule of root. Refines D1/D3: `glade.taut.py` is authored and committed
  **in the taut repo** (`taut/ir/glade.taut.py`, corpus `taut/corpus/glade.*`);
  root does not currently track taut. taut gets its own `gladev2` branch when
  P0.S2 first writes the IR. If root should later pin taut, register it as a
  proper submodule (separate task) — out of scope for M-LIMP.
- **D7. Base commit hygiene.** `git add -A` trapped ~50 embedded vendor repos
  (`taut`, `trial`, `vm_manager`, `third-party/*`) as broken gitlinks; redone
  to stage only WIP + real submodule pointer updates (`glade` @ eb15991,
  `grip-core`, `grip-lab`). Excluded from the base: the embedded vendor repos
  (left untracked), `.aiedit/token` (secret), `*.DS_Store` (junk),
  `glade/dev-docs/glade_sec_model-55/` (prior analysis run). Suggested
  follow-up (not done): gitignore the vendored embedded dirs so future
  `git add -A` doesn't re-trap them.

## 2026-06-13 — P1.S1 (store)

- **D8. Authoritative log = per (share, origin)** — one monotonic `seq` + one
  prev-hash chain per origin within a share (GladeSubstrateV1 §6). An op's
  `(glade_id, key)` is routing/fold addressing, not a separate log axis;
  heads/resume operate at (share, origin) granularity. The frozen wire
  `StreamHeads` (per-stream) is reinterpreted for M-LIMP as share-scoped origin
  heads (glade_id/key carry capacity the node doesn't yet use, like the
  security seams). Stream-scoped resume is a post-LIMP refinement if backfill
  volume needs it. No wire change — reinterpretation only.

## 2026-06-13 — P1.S2 (resume) + sequencing

- **D9. Node logic built carrier-first; WS socket consolidated once.** The
  resume/convergence, routing, chain-verify, and echo logic are all
  carrier-independent (ops self-order). They are built and tested in-process
  (faster, more thorough); the actual websocket socket is added once as the
  single integration carrier before the TS client (P2) connects. trial's
  hand-rolled `ws.rs` is text-frame only and will be adapted for binary frames
  at that point. Reordering, not a skip — P1.S2's resume *exit* (bidirectional
  convergence after disconnect) is met in-process.
- **D10 (planned, P1.S4). Op-hash is cross-language contract.** The per-origin
  chain `prev` (GQ-9) requires the Rust node and TS client to compute identical
  hashes, so P1.S4 adds a Python op-hash reference + conformance vectors (sha256
  over canonical op CBOR excluding the hash field) to the oracle, like the fold
  vectors — before wiring rust verification. Deferred to its own focused pass to
  avoid a rushed canonicalization spec.

## 2026-06-13 — P3.S1 (grip-core share feature)

- **D11. grip-core baseline facts (recorded honestly).** The `glial-dev/grip-core`
  working tree carries **pre-existing uncommitted WIP** that is NOT GLP-0005: a
  coherent `unlinkParent` change across `src/core/context.ts`,
  `src/core/tap_resolver.ts`, and `tests/tap_resolver.spec.ts` (makes resolver
  Scenario 7 a reevaluate-only hook). Left untouched in the working tree; my
  P3.S1 commit (`9522add`) stages only my four files. Also pre-existing on pure
  main: `tests/function_tap.spec.ts > "README simple stateful tap..."` fails
  (1/5) — unrelated to share or the WIP; flagged as a separate task.
  P3.S1 result: suite 219 passed (215 + 4 new share tests), tap_resolver 6/6,
  only the pre-existing function_tap failure. Lesson: check a submodule's own
  `git status` before editing; never partially stash a coherent source+test
  change (it produced a long false-trail blaming `atom_tap`).
- **GQ-5 shape (built).** grip-core gains `ShareDecl` (plain data) + optional
  `share` / `getShareValue` / `applyShareValue` / `subscribeShare` on `Tap`,
  implemented on `AtomValueTap`; `grok.listSharedTaps()` advertises. No glade
  types in core; share-free apps advertise `[]`.

## 2026-06-13 — grip-core fixes alongside P3 (not GLP-0005 contract)

- **D12. FunctionTap double-compute fixed** (grip-core 97821a6). The
  pre-existing `function_tap.spec.ts` failure (D11) was a real bug: bisected to
  `3b02f45` ("clear delayedUpdates after param wiring"). `FunctionTap.onAttach`
  subscribed to home params **a second time** (on top of BaseTap's
  `produceOnParams`); before 3b02f45 BaseTap's path was gated, so only
  FunctionTap's fired. 3b02f45 un-gated BaseTap → every home-param change
  computed twice, double-applying self-written state. Fix: drop FunctionTap's
  redundant subscription, keep param resolution, rely on the single BaseTap
  path. Full grip-core suite now 220/0 (was 219/1), stable 3/3. The test was
  correct. (Also committed on gladev2: D-removeParent fix 03a56ec.)

## Contract changes after P0 lock

(none yet — record here with date + reason; the corpus is the arbiter)
