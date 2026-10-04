# Storage-attempt contract — remediation 2 evidence

Date: 2026-10-04. Status: **DRAFT merged correction; six finding IDs OPEN.
Kernel remains refusing. This evidence does not close findings or authorize implementation.**

Inputs: [RemPlan-2](GladeIndependentCrdtStorageAttemptContract-RemPlan-2.md), full
[Code-2](GladeIndependentCrdtStorageAttemptContract-ReviewCode-2.md) and
[State-2](GladeIndependentCrdtStorageAttemptContract-ReviewState-2.md).
Draft input root `c712b4abd9078b8d0ea2b78d692ce87fdd4c4fe1`, Glade
`346d963f09089a0636a01fac8a257f067908147d`; unchanged Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. These are input pins, not eventual
review pins. The owner MUST settle this one merged patch before re-review.

## 1. Scoped corrections

| Findings | Existing-method correction | Executable regression |
| --- | --- | --- |
| Code-2 P2-1 / State-2 P2-1 | Original multiline historical Inspect40 setup now calls register_lookup before counter advancement. Both indexes and next41 are asserted. | New fixture source check identifies this exact residual setup; original historical ExactRetry receipt remains. Added kernel pair requires unchanged state/unknown-plan reservation + CallbackMismatch for unissued reply, then ExactRetry for the identical fully issued reply. Both kernel cases remain RED against refusing step. |
| Code-2 P2-2 / State-2 P2-3 | After exact retained-plan deduplication/unresolved checks, a new Prepare binding with incompatible actual revision refuses before queued/Reserved exposure. No terminal/revision is fabricated. | Public journey loops all four kinds, ahead/stale expectations, Begin/Fence follow-ups and held/unheld preparation:32 combinations. Custody, revision, attempts, queue and attempt floor stay unchanged; recover_plan remains Pending, authentic restore succeeds, original terminal dedup/repeat and valid revision2 publication remain. New rejected work cannot replace Started work. |
| State-2 P2-2 | Restore validates unique complete AttemptIds, one unresolved instance owner across attempts/queue, unique plans, matching unresolved current revision and combined instance capacity. | Eight negative image mutations cover duplicate active ID, distinct active overlap, queued+active, multiple queued, duplicate historical ID, mismatched Reserved revision, combined instance overflow and mismatched queued revision. Positive recovery retains two historical terminals plus current X and permits independent Y publication and authentic restore. |
| Code-2 P2-3 | Open validates a cloned complete retained image under requested limits and existing reserved bytes before owner/floor mutation. Incompatible reductions refuse; reservations cannot silently shrink. | Nine reductions cover batch/receipt/window/name/per-attempt critical/aggregate critical/attempt/history/instance bounds. Refusal preserves the old image; compatible generation2 retry proves the refused open did not consume the generation. Identities/outcomes/revisions/issuance floors/count survive and recovery restores. |

Changed Glade paths are exactly:

- `contracts/crdt-storage-attempt-api/tests/public_contract.rs`
- `contracts/crdt-storage-attempt-api/tests/support/mod.rs`
- `contracts/crdt-admission-core/tests/fixture_composition.rs`
- `contracts/crdt-admission-core/tests/records_host_contract.rs`

Root changes are the controlling Contract and this new evidence only. Shared API
is byte-identical to the input pin, SHA256
`86cf4baf80aa532848a9a016e3c18a34a8896fd39a266919b84cf24ca06ac03d`.
No new field, method, assembly seam, package, dependency, role, mutation boundary,
compatibility or platform assumption is introduced. All corrections remain within
the existing development provider and consumer fixture grammar.

## 2. Chronological test-first evidence

Commands below were executed from `glade/contracts` with `--locked --offline`.
Each root regression was added before implementation edits. Logs are diagnostic
`/tmp` files; checked-in tests and commands reproduce the checkpoint.

1. Added three public regressions and the historical fixture source regression.
   Executed each individually against unchanged implementation:
   - `cargo test -p glade-crdt-storage-attempt-api --test public_contract fresh_revision_mismatch`
     compiled and failed: stale Candidate expected0/current1 returned Prepared,
     expected Refused(BindingMismatch). `/tmp/sta-rem2-revision-red.log`.
   - Same public target, filter `restoration_rejects_identity`, compiled and failed:
     duplicate complete AttemptId image accepted. `/tmp/sta-rem2-overlap-red.log`.
   - Same public target, filter `reopen_validates`, compiled and failed:
     nine-byte retained binding reopened under one-byte bound.
     `/tmp/sta-rem2-reopen-red.log`.
   - Core fixture target, filter `historical_prior_cut`, compiled and failed:
     original secondary-only multiline Inspect40 insertion detected.
     `/tmp/sta-rem2-historical-red.log`.
2. Added the actual kernel issued/unissued prior-cut pair before fixture correction.
   First run had a test-authoring type error (Bytes policy assigned its digest),
   corrected only that test expression, then reran. It compiled and failed the
   required unissued CallbackMismatch report against refusing step.
   `/tmp/sta-rem2-pair-red.log` records the compiling RED; the compiler error is
   not presented as behavioral RED. No kernel implementation followed.
3. Added minimum provider checks and corrected the historical fixture registration.
   All23 public journeys passed. Six existing fixture journeys passed, while the
   new source guard still failed because its initial broad `.lookups` match also
   detected the newly added legitimate index assertion. Refined the guard to the
   exact original multiline insertion (which still fails on the input source),
   retaining helper/authoritative-index checks. All7 fixture checks then passed.
   `/tmp/sta-rem2-api-first-green.log`, `/tmp/sta-rem2-fixture-green.log`.
4. Before additional validator changes, extended the recovery regression with
   historical duplicate ID, incompatible Reserved/queued revision and combined
   current/queued instance bound edges. It compiled and failed mutation5: Reserved
   expected1/current0 accepted. `/tmp/sta-rem2-restored-revision-red.log`.
   Added matching unresolved revision validation. Rerun compiled and failed
   mutation6: current X plus queued Y accepted under one-instance capacity.
   `/tmp/sta-rem2-instance-union-red.log`. Added combined finite instance validation;
   all23 public journeys passed, including all eight negative images and the legal
   historical/current/Y control. `/tmp/sta-rem2-api-green.log`.
5. Ran final affected consumer, source, architecture, lint and measured compilation
   checks below. No successful core behavior was implemented; the new pair and all
   existing42 kernel consumers remain compiling behavioral RED.

The loop rows are finite, deterministic and use explicit caller-owned parameters.
No sleeps, network, live storage, admission receipts synthesized by a helper or
successful step fallback was used. Historical initial TDD deviations remain in
prior evidence and are not rewritten by this restarted correction.

## 3. Final focused results and timings

Monotonic wall seconds include Cargo startup/build/test. All Cargo verification
uses `--locked --offline`; selected fmt checks only the two affected packages.

| Command/target | Exact result | Wall seconds |
| --- | --- | ---: |
| `cargo test -p glade-crdt-storage-attempt-api --all-targets` | GREEN:23 public +1 probe rejecting six mutants +5 source =29 | 1.881 |
| `cargo test -p glade-crdt-admission-core --test fixture_composition --test representation --test source_boundaries` | GREEN:7 fixture +2 representation +3 source =12 | 2.038 |
| core `--test records_host_contract` | Intentional RED:22 compiling assertion failures, including new historical pair | 1.756 |
| core `--test recovery_contract` | Intentional RED:all6 original consumers | 1.540 |
| core `--test storage_attempt_contract` | Intentional RED:all15 retained consumers | 1.623 |
| selected API/core `cargo test --all-targets --no-run` | GREEN:all selected targets compile | 0.521 |
| selected API/core `cargo fmt -- --check` | GREEN | 0.312 |
| selected API/core `cargo clippy --all-features --all-targets -- -D warnings` | GREEN | 0.460 |
| `cargo run --quiet --manifest-path ../../glade-discover/tools/architecture-check/Cargo.toml -- .` | Architecture boundaries PASS | 0.642 |
| root `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | Intentional RED:exact ten rows; unchanged released-corpus three-order positive control PASS | 1.951 |
| Glade `python3 scripts/checks/check_process_globals.py` | GREEN:82 files,3 permanent entries,0 debt,nothing new | 0.559 |
| root `node glial/test/independent_admission_source.mjs` | GREEN:unchanged compound-body guard | 0.225 |

All43 domain failures compile and fail assertions against unchanged refusing step;
there are no seed-constructor or compiler failures in the final run. All42 existing
kernel test names remain, and the original historical success assertion remains.
Records assertions grow52 to61 (three issuance assertions plus six new pair
assertions); recovery55 and STA46 remain unchanged. Original loops and ten text
rows/canonical corpus bytes remain unchanged. The lookup audit includes multiline
insertions in all core tests/examples: only register_lookup owns ordinary secondary
insertion; fixture_composition intentionally inserts one secondary-only corrupt
image and requires refusal. Recovery's explicit invalidation moves issued lookup
history before clearing its index. The residual historical insertion is corrected.

Warm direct executable times, separate from Cargo: API public0.0041s,
mutant probe0.0034s, API source0.0296s, core fixture0.0046s,
representation0.0034s, core source0.0457s; intentional Records RED0.0047s,
recovery RED0.0040s and STA RED0.0045s. Fresh `CARGO_TARGET_DIR` selected
all-target compilation/no-run was5.285s (GREEN), with dependencies already cached;
this is not a clean-download measurement. Final incremental compilation was0.521s.
Logs/timing JSON use `/tmp/sta-rem2-*` and are reproducible diagnostic aids.

Selectors, finite budgets, source allowlists, manifests/dependencies/classifications,
architecture tooling, all12 ARCH002 refusal and synthetic13th regression are
unchanged. The positive architecture/source gates ran; unchanged selection/tooling
negative fixtures were not rerun. External Gyld is untouched; no allocation,
frozen source/ledger, engine/evaluator or budget change requires its runner/Mypy
rerun. Historical results and baseline limitations remain preserved.

## 4. Stop boundary

Kernel `src/lib.rs` is byte-identical to the input pin, SHA256
`5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.
No physical host, crypto/disk/restart/antirollback/import, async cancellation,
live duplex or activation qualification is claimed. No Git mutations, root
plan/report/ledger writes, external Gyld writes, push or live operations occurred.

All six current IDs remain OPEN. Current full Code/State reviewers MUST independently
re-verdict their counterexamples and the corrected range at the owner's settled
tuple. They classified these defects nonarchitectural; the architectural count
remains one. This is remediation round2, not a new implementation tranche.
