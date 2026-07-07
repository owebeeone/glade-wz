# Handoff — GLP-0005 (M-LIMP reached)

Status: M-LIMP reached 2026-06-14. The glade substrate exists and is proven
end to end. Branch `gladev2` across root + `glade`, `taut`, `trial`, `grip-core`
submodules/repos; tags `gladev2/p0-start` → `gladev2/p4-mlimp`.

## What was built

| Layer | Where | Proof |
| --- | --- | --- |
| Wire + oracles | `taut/ir/glade.taut.py`, `taut/corpus/glade.*` | corpus + fold + op-hash byte-parity Rust/TS/Python |
| Rust node | `glade/node` | store, resume, routing, GQ-9 chain verify, echo; WS e2e |
| TS client | `glade/client-ts` | session, lww+log folds, WS, exchange, browser sha256 |
| Grip-share binder | `glade/grip-share` | binds shared taps; converges via real node |
| grip-core feature | `glial-dev/grip-core` | base-tap `share` decl + `listSharedTaps` (GQ-5) |
| Demo | `glade/demo` | gryth workspace panel — converges live in-browser |

Acceptance: `glade/grip-share/test/mlimp.test.ts` runs the whole §11 scenario
(converge lww+log → node restart resume → offline-write/reconnect reconcile →
echo EXCHANGE) as one scripted test. Live proof: `glade/demo` (run
`python3 glade/demo/run_demo.py`), two tabs converge.

## Bonus fixes on `gladev2` grip-core (not GLP-0005 contract)

- FunctionTap double-compute (bisected to 3b02f45) — `97821a6`.
- removeParent symmetric unlinkParent — `03a56ec` (was pre-existing WIP).
- grip-core suite 220/0.

## Carrying forward

- Contract retro + known gaps: `glade/dev-docs/GladeSubstrateV1.md` §12.
- Open: GQ-1 (MV conflict surfacing) — the one API-shaping question, sidestepped
  (M-LIMP ships conflict-free folds only).
- Post-LIMP order in §12 / Plan non-goals: keyed bindings → reassembler → iroh →
  grazel authority → security (per `GladeGrythSecurityModelAnalysisPrompt.md`).
- Dev note: grip-core `dist` is gitignored; rebuild (`npm run build`) so the
  share feature reaches the demo via the grip-react→grip-core symlink.

## Stash to dispose (grip-core)

`stash@{0}` holds throwaway async-tap deadline/abort debug logging — drop when
convenient.
