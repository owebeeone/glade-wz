# State

Plan: `GLP-0005`
Status: **finished** — M-LIMP reached (`gladev2/p4-mlimp`)
Current phase: P4 complete. The substrate exists end to end; see Handoff.md.
Branch/checkouts: root `glial-dev` on `gladev2` (off `main`); roll-build tag
prefix `gladev2/`. Submodules (`glade`, `grip-core`) branched `gladev2` when a
phase first writes them.

## Current Position

Plan created 2026-06-13 from the substrate design session that produced
`glade/dev-docs/GladeSubstrateV1.md` (resolved: GQ-2 iroh, GQ-4 rust core /
no wasm, GQ-8 browser folds, GQ-9 hybrid causal-ref encoding) and
`glade/dev-docs/GladeGrythSecurityModelAnalysisPrompt.md` (security punted
with retrofit seams; analyses to run in parallel).

No implementation has started. The first action is P0.S1 (module layout
decision), then authoring `glade.taut.py`.

Canonical grip-core confirmed 2026-06-13: `/Users/owebeeone/limbo/glial-dev/grip-core`
on `main` @ 7d52fb1 (delayedupdates-rekey merged in; clean of wip2 persistence
code). `grip-dev/grip-core` @ `codex/glial-stumbling-wip2` is reference-only.
See the Plan's "Canonical checkouts" table.

## Progress

- P0.S1 done: layout decided (Decisions D3/D6). taut is its own repo;
  IR lives at `taut/ir/glade.taut.py`.
- P0.S2 done (pending user review): `taut/ir/glade.taut.py` authored and
  validates (16 messages, 4 enums, 0 services — transport, not a service).
  Codegen smoke test passed (rust/ts/python emit) → R3 largely retired.
  Committed on taut `gladev2` @ 9983392.
- P0.S2 reviewed: lamport+refs kept as-is (revisit later, user call). IR
  committed taut `gladev2` @ 9983392.
- P0.S3 done: `taut/corpus/glade.golden.json` (37 vectors); Python byte-exact
  + round-trip; @ 64d6a08.
- P0.S4 done: **Rust** crate `glade/wire-rs` (`cargo test` green, 37 vectors
  byte-exact, generated rust compiles+links — R3 retired) and **TS** parity
  via taut's generic codec on `glade.ir.json` (`node --test` green). Commits:
  taut 0aa8a31, glade 47b8143, trial 5a73890.
- P0.S5 done: fold reference `taut/crdt/glade_fold.py` (value-lww + log +
  equivocation), 12 conformance vectors → cross-lang oracle
  `corpus/glade_folds.json`; reference caught a hand-authored tiebreak error
  (fixed); taut suite 114 green; wired into `run_tests.py`. @ d834331.

## P1 progress (glade submodule `gladev2`) — node logic complete

All carrier-free node logic done and green (**18 node tests**, taut **118**):

- S1 store @ b4c254e: per-(share,origin) append log, restart-safe, scan/heads. (D8)
- S2 resume @ b11b1cf: frame codec + heads-exchange/gap-ship; bidirectional
  convergence after disconnect (the S2 exit).
- S4 chain @ taut d98ff68 / glade 62aa5ce: op-hash oracle (D10, Rust reproduces
  Python `8a87b62f…`); chain-link + equivocation rejection → `Error` frame.
- S3 routing + S6 echo @ glade 01e4e62: fan-out minus origin; interactive
  preempts 1 MB bulk backfill; exchange/channel echo with corr + close.

WS carrier done @ glade e7d414c: binary websocket (`ws.rs`), `server.rs` ties
store+router+echo over sockets. **End-to-end test green** (stable over 5 runs):
two clients converge over a real WS, a late joiner resumes the op via gap-ship,
echo exchange round-trips with its corr. **19 node tests.** P1.S5 (cached-fold
late-join) left as a documented stretch — late join works via full gap-ship.

## P2 done (glade submodule `gladev2`) — TS client, the browser half

- S1-S3 @ f825aab: `glade/client-ts` session (own origin log + prev-hash),
  in-memory store + hydration, `lww`+`log` folds. **TS reproduces the hash +
  fold oracles byte-for-byte**; two sessions converge; offline-write hydration.
  Vendors taut TS codec (cbor/schema/codec).
- S4 @ 5e5a24a: WS client (Node global `WebSocket`) + `glade-node` binary.
  **Cross-process e2e: two TS sessions converge through the real rust node over
  websocket** (browser folds). Stable 4/4. 7 TS tests.
- IndexedDB destination left as documented follow-up (memory destination +
  hydration round-trip cover the shape; IndexedDB is a swap of the same dump).

## P3 progress (grip-core `gladev2`)

- S1 done @ grip-core 9522add: base-tap glade `share` feature (GQ-5) —
  `ShareDecl` + capture/apply hooks on `Tap`/`AtomValueTap` +
  `grok.listSharedTaps()`. Protocol-free; 4 tests; suite otherwise unchanged
  (see D11 for the grip-core baseline: pre-existing unlinkParent WIP left in
  place, pre-existing function_tap failure).
- grip-core canonical: `glial-dev/grip-core` on `gladev2` (off main 7d52fb1).

- S2 done @ glade 34f9bb1: `glade/grip-share` binder — walks
  `grok.listSharedTaps()`, binds by glade id to a glade Session;
  grip-core-agnostic (structural hooks). Two binders converge over loopback via
  real folds, echo-guarded; `collectGladeIds` (GQ-6 input); late-join hydrate.
  4 tests.
- S3 done @ glade d7a1418: log-shaped binding — `appendLog` emits discrete
  entry ops; materializes ordered list; replay cold + from cursor; two writers
  converge. 6 grip-share tests total.
- Also fixed alongside (grip-core, D12): FunctionTap double-compute (97821a6);
  unlinkParent re-resolve (03a56ec). grip-core suite 220/0. Root pins grip-core
  @ 97821a6.

## P3.S4 redefined (user, 2026-06-14): NEW gryth demo, not retrofit

Instead of retrofitting `grip-react-demo`, build a fresh share-first demo that
exercises the gryth toolchain (rust + glade + react). Concept: **gryth
workspace panel** — shared selection (lww), shared notes (lww), activity log
(log). Location: **`glade/demo`** (glade submodule). Replaces P3.S4 demo-parity
and folds in P4 (becomes the M-LIMP app-level proof).

- **Toolchain core proven @ glade 27a49dd**: `GladeClient` made composable
  (shared session + `onOps`/`sendOps`); grip-share binder converges the
  workspace panel (lww + notes + log) **through the real rust node over WS**
  between two clients — headless, stable 4/4. So rust + glade + grip-share is
  done end-to-end; only the React UI layer remains.

- S4 done @ glade 5085781: **`glade/demo` gryth workspace panel** — vite +
  react + grip-react (`useGrip`) + grok share-taps + grip-share binder +
  GladeClient over WS to the rust `glade-node`. **Verified live in the browser
  preview**: a React tab (origin 0iblqg) and a headless participant (probe42)
  converge on selection + notes (lww) and the activity log (log) through the
  real node; the browser updates reactively. The full rust+glade+react
  toolchain works end to end. Browser-compat sync sha256 added (replaces
  node:crypto; oracle-verified). grip-core dist rebuilt (gitignored) so the
  share feature reaches the demo via the grip-react→grip-core symlink chain.

## Next Checkpoint

P3 is functionally complete (S1-S4). Tag `gladev2/p3-grip` (pin grip-core
97821a6 + glade 5085781). Then **P4** — the scripted M-LIMP proof + retro
fold-back; the gryth demo already serves as the app-level proof, so P4 is
mostly the harness + doc reconciliation. Note: original S4 "delete wip2
projector path" is moot — the new demo never used it (clean grip-core main).
