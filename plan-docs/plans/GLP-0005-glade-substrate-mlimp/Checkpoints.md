# Checkpoints — GLP-0005

Roll-build tag prefix: `gladev2/`. Primary checkout: root `glial-dev`,
branch `gladev2` (off `main`). Tag a phase only when its goal is met, focused
verification passes, and remaining ambiguities are minor (per plan-docs
roll-build method).

Commit trailers for normal commits:

```text
Plan: GLP-0005
Phase: P0
Checkpoint: contract-lock
```

| Tag | Phase | Goal | Verification | Status |
| --- | --- | --- | --- | --- |
| `gladev2/p0-start` | base | clean tagged start; plan execution-ready | tree committed; plan files present; status active | **done — 2a5a4ef** |
| `gladev2/p0-contract-lock` | P0 | wire IR + corpus + codecs + fold vectors frozen | corpus byte-exact in Python ref + Rust + TS; fold vectors pass | **done** |
| `gladev2/p1-node` | P1 | rust node limps localhost (store/resume/route/verify/echo) | P1.S1–S4 + S6 + WS e2e green (19 tests) | **done** |
| `gladev2/p2-client` | P2 | TS session converges through node; hydration | hash+fold oracle parity; cross-process e2e (7 tests) | **done** |
| `gladev2/p3-grip` | P3 | sharable base-tap + binder + working gryth demo | S1-S4 green; demo converges live in-browser | **done** |
| `gladev2/p4-mlimp` | P4 | full §11 scenario scripted + measured | mlimp.test.ts green (stable 3/3); control RTT p50 0.13ms | **done** |

## P1 log

- S1 store glade@b4c254e (4 tests) · S2 resume glade@b11b1cf (7) ·
  S4 chain taut@d98ff68 + glade@62aa5ce (op-hash oracle 4, node 13) ·
  S3 routing + S6 echo glade@01e4e62 (node 18). taut suite 118.
- Deviation: glade@2129bdb committed pre-verification (didn't compile);
  amended to 01e4e62. Lesson: confirm `cargo test` green before commit.
- WS carrier glade@e7d414c: binary `ws.rs` + `server.rs`; **end-to-end test**
  (converge + late-join resume + echo over a real socket), stable 5/5 runs.
- 2026-06-13 — **`gladev2/p1-node`** (root). glade submodule pinned @ e7d414c.
  taut @ d98ff68, trial @ 5a73890. Node runs the §11 localhost role with rust
  test clients; the TS client (P2) is the remaining §11 piece.

## P2 log

- S1-S3 glade@f825aab: TS session/store/folds; reproduces hash+fold oracles;
  converge + hydration (6 tests). S4 glade@5e5a24a: WS client + `glade-node`
  binary; cross-process TS↔node convergence over websocket, stable 4/4 (7 tests).
- Constructor parameter-properties don't work under node `--strip-types`; fields
  declared explicitly. IndexedDB destination deferred (memory + hydration cover
  the shape).
- 2026-06-13 — **`gladev2/p2-client`** (root). glade pinned @ 5e5a24a.

## P3 log

- S1 grip-core 9522add (share feature) + bonus FunctionTap fix 97821a6
  (bisected to 3b02f45) + unlinkParent 03a56ec. grip-core suite 220/0.
- S2 binder 34f9bb1, S3 log d7a1418, S4-core 27a49dd (converge via real node),
  S4 demo 5085781 + launcher 86cbf25. Browser-compat sha256.
- 2026-06-14 — **`gladev2/p3-grip`**: grip-core @ 97821a6, glade @ 86cbf25,
  taut @ d98ff68. Gryth demo converges live in-browser (rust+glade+react).

## P4 log

- S1 §11 acceptance harness glade@577d44c (mlimp.test.ts: converge + restart-
  resume + offline-reconnect + echo; GladeClient.exchange + binder.resync).
- S2 latency: control RTT localhost p50 0.13ms / p90 0.32ms / max 0.88ms (50).
  Priority OutQueue unit-tested but not wired into the WS server (FIFO) — gap.
- S3 retro: GladeSubstrateV1 §12 + Handoff.md; status finished.
- 2026-06-14 — **`gladev2/p4-mlimp`** = M-LIMP. glade @ <p4 head>, grip-core
  @ 97821a6, taut @ d98ff68.

## Rollback notes

Each phase tag is a recoverable point. Submodule (`glade`, `grip-core`) pins
are recorded in the root commit at each tag; rolling back a root tag restores
the matching submodule pins. No pushes during the spike unless requested.

## Log

- 2026-06-13 — `gladev2/p0-start` @ 2a5a4ef. Root on `gladev2` off `main`
  (f40a3da). glade submodule committed eb15991 (design docs). Excluded
  embedded vendor repos, `.aiedit/token`, DS_Store, `glade_sec_model-55/`.
  Deviation: first `git add -A` trapped embedded repos; corrected (see
  Decisions D7). Next: P0.S2 author `taut/ir/glade.taut.py`.
- 2026-06-13 — P0.S2 (taut `gladev2` @ 9983392): `ir/glade.taut.py` validates
  (16 msgs, 4 enums, 0 services); codegen smoke test rust/ts/python OK (R3).
  lamport+refs kept (revisit later).
- 2026-06-13 — P0.S3 (taut `gladev2` @ 64d6a08): `corpus/glade.golden.json`
  37 vectors; Python byte-exact + round-trip; taut suite 109 green; wired into
  `run_tests.py`. Next: P0.S4 Rust/TS byte-parity, then P0.S5 fold vectors.
- 2026-06-13 — P0.S4: Rust crate `glade/wire-rs` cargo test green (37 vectors
  byte-exact, generated rust compiles — R3 retired); TS parity via taut generic
  codec `node --test` green. Commits taut 0aa8a31, glade 47b8143, trial 5a73890.
- 2026-06-13 — P0.S5 (taut @ d834331): fold reference + 12 vectors →
  `corpus/glade_folds.json`; reference caught a tiebreak error (fixed by hand
  review); taut suite 114 green.
- 2026-06-13 — **`gladev2/p0-contract-lock`** (root). Component heads pinned:
  glade submodule @ 47b8143 (in-tree gitlink); non-submodule repos recorded —
  taut @ d834331, trial @ 5a73890 (R8: not pinnable by root; recorded here).
  IR + corpus + fold oracle frozen for M-LIMP.
