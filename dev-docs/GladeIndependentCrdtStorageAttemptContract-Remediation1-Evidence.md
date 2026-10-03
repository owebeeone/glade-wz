# Storage-attempt contract — remediation 1 evidence

Date: 2026-10-04. Status: **DRAFT correction, findings OPEN pending originating
verification and fresh full Code/State review. Kernel remains refusing.**

This is one merged patch against [RemPlan-1](GladeIndependentCrdtStorageAttemptContract-RemPlan-1.md),
using the complete filed [Code](GladeIndependentCrdtStorageAttemptContract-ReviewCode.md)
and [State](GladeIndependentCrdtStorageAttemptContract-ReviewState.md) reports.
Input root is `10e1f15fdb1c466d46884fc3c0f8a46db41e9e05`, Glade
`3cf1fa79cd752012acd0d2ff66d595e293b3433c`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. The first reviewed object was root
`a696f0eef38614fe0cfe2a6b053c352470e800b3`; these input pins are not eventual
remediation-review pins. The owner MUST settle this correction before reviews.
The drafter does not self-close findings or alter the filed reports/plan/ledger.

## 1. Exact correction and executable closure paths

| Merged root | Proposed correction | Executed fixture evidence |
| --- | --- | --- |
| State P2-1 | Shared Recovery adds actual `invocation_count`; restore no longer converts high-water into cardinality. Count/floor/declared limits and retained binding/phase consistency are validated. | Sparse prepare128 consumes one entry, authentic recovery/restoration rejects stale128, permits inspect129/fence130 and agrees with the live branch through exact count128 exhaustion. Five corrupt count/floor/history mutations and eight restored binding/limit/phase mutations refuse. |
| Code P2-1 / State P2-2 | `FixtureReplica` initializes the producer from the exact explicit core fixture policy digest/interval. Independent provider changes are injected separately from Begin expectations. | Public-port publication returns the original staged receipt/start cut; independently changed provider returns terminal NonCommit for the unchanged stale Begin. No Begin claim is copied into authority. |
| Code P2-2 | Persistent session per logical text replica and original multi-call rival/fork test. Independent fixture branches use explicit complete owned seeds. | Consecutive port publications reach revisions1/2 and retain first terminal; immutable revision0 custody survives. Pending reservations/full request history and committed receipt/accounting are validated. Every text call site uses its retained session; exact ten rows remain. |
| Code P2-3 | Full issued-lookup helper checks identities, complete attempt/binding, history capacity and counter, then records authoritative request and secondary index together. | GREEN fixture checks retain consumed Prepare7, issued Begin8, lookup40 and next counter41. Foreign namespace/capacity refuse unchanged. Issued Pending lookup retirement and identical unissued reply consumers compile and remain kernel RED. Each existing malformed-terminal mutation now starts with a fully issued lookup. |

[API recovery/cardinality](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/src/lib.rs),
[development provider](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/support/mod.rs),
[public journeys](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/tests/public_contract.rs),
[fixture assembly](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/support/mod.rs)
and [fixture closures](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/fixture_composition.rs)
are the exact implementation/test paths. Domain consumers are the existing Records,
recovery and STA files; actual text consumer is
[text_admission_trace.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/examples/text_admission_trace.rs).
Root edits are only the controlling Contract and this new evidence.

Production Recovery gains **only the cardinality field**. An initially considered
`genesis_custody` API declaration was a compiling scaffold, then removed before
successful custody handling: it is development metadata, not a production import
contract. Development `OwnedFixtureSeed` contains Recovery plus immutable finite
per-instance bytes of the explicit InstanceState debug snapshot. It retains all
existing custody/original receipts, budgets/usage, policy and derived recovery state;
Recovery retains attempt/outcome/preparing/reservation/identity state. The assembly
retains and validates full issued/consumed kernel request history alongside it.

The provider stores these exact seed bytes and exposes them through its development
seed. Aggregate seed bytes plus retained attempt critical reservation must fit the
unchanged finite critical capacity. Genesis0 seeds do not create commits, advance
revision or manufacture receipts. Nonzero seeded revisions require matching
retained terminals, original custody and installed accounting. This constructor
asserts an already-owned **test execution continuation**; it is not a real
close/reopen, lock acquisition, decoder/encoder, import, migration, clone exclusion
or antirollback qualification. No exception to live outcome coupling is introduced.

## 2. Chronological test-first failures

All successful fixture behavior followed executed regression failures. The old
checkpoint's separately disclosed initial TDD deviation remains historical and
unchanged; this patch does not invent a different chronology.

1. Added sparse authentic restoration regression using the existing public port.
   An initial invocation from the wrong Cargo directory did not execute tests and
   is not counted as RED. Rerun from contracts compiled and failed: live Pending
   versus restored Refused(Capacity) on fresh129 (`sta-rem1-sparse-red.log`).
2. Added four assembly regressions before helper changes: core-cut provider,
   immutable seeded custody/two publications, authoritative issued lookup and
   persistent text lifetime. Initial consumers failed compilation on missing
   helper/type/seed shape (`sta-rem1-shape-red.log`). Minimal scaffolds mirrored
   the old empty-host/secondary-only behavior; all four then compiled and failed
   behaviorally (`sta-rem1-fixture-red.log`). No successful assembly behavior was
   introduced at this step.
3. Added corrupt cardinality/floor/history-limit regression before adding count.
   Missing count failed compilation (`sta-rem1-recovery-shape-red.log`); exporting
   the existing live count while retaining old restore behavior then compiled and
   failed to reject corrupt count (`sta-rem1-corrupt-red.log`). Restore cardinality
   preservation/validation was implemented only after these failures.
4. Refined seed ownership to the development provider and added full pending
   seed/secondary-index corruption checks. The development seed accessor first
   failed compilation (`sta-rem1-owned-seed-shape-red.log`); its empty-image
   scaffold produced five compiling fixture failures (`sta-rem1-owned-seed-red.log`).
   Implemented explicit retained finite owned seeds, independent observations and
   complete lookup registration. Four passed while the text migration guard still
   failed (`sta-rem1-fixture-partial.log`); migrated all text sites and it passed.
5. Extended lookup fixture regression to require its consumed full Prepare request
   and next counter9. It failed on missing prepare history
   (`sta-rem1-prepare-history-red.log`), then the helper retained Prepare7 and
   Begin8 with checked finite accounting. Existing injected lookups were migrated
   to the helper; invalidated restoration lookups move unchanged into consumed
   full request history.
6. Added two kernel issued/unissued callback closures after fixture preconditions
   were corrected. They compile and fail against the unchanged refusing `step`;
   no kernel implementation followed. This order is explicit: fixture GREEN is
   not claimed as kernel-consumption RED/GREEN.
7. Added terminal seed coupling regression before its validation fix. A terminal
   without original custody was incorrectly accepted, producing an executed
   failure (`sta-rem1-terminal-seed-red.log`). Minimum validation now requires
   retained original receipts and sufficient installed original charges; valid
   complete terminal seed/restored lookup repeats the original outcome.
8. Added eight restored bound/instance/phase mutation cases before validation
   changes. Reduced batch limit was incorrectly accepted
   (`sta-rem1-restored-bounds-red.log`). Restore now reuses the live binding limit
   checker and verifies instance references and Started/terminal bound fields.
9. Added regressed owned-seed issuance-counter case. It compiled and failed
   because the seed helper silently raised the floor
   (`sta-rem1-counter-seed-red.log`). It now rejects a counter at/below an issued
   request. Reserved consumers retain consumed Prepare and a next counter above
   their injected resolve request. The unrelated evidence-query counter remains
   unchanged. Fixture closure suite passed again.

Logs named above live under `/tmp/` and are diagnostic aids, not durable source
artifacts. Tests and the commands below reproduce the corrected checkpoint.
A failed local text-substitution assertion during adding the last case wrote no
source and was followed by the actual executed regression; it is not RED evidence.

## 3. Focused verification and measurements

All Cargo commands use the contracts workspace and `--locked --offline`. Only the
API and affected core are selected. Monotonic wall measurements include Cargo
startup/build/test; runner-only and cold compilation are separated below.

| Command / target | Exact result | Wall seconds |
| --- | --- | ---: |
| API `cargo test -p glade-crdt-storage-attempt-api --all-targets` | GREEN:20 public journeys, one probe rejecting six mutants, five source checks =26 | 0.152 |
| core `cargo test -p glade-crdt-admission-core --test fixture_composition --test representation --test source_boundaries` | GREEN:6 fixture closures +2 representation +3 source =11; final counter correction rerun | 1.239 |
| core `cargo test --test records_host_contract` | Intentional RED:0pass/21fail; final helper consumer recompile | 0.866 |
| core `cargo test --test recovery_contract` | Intentional RED:0pass/6fail; final helper consumer recompile | 0.715 |
| core `cargo test --test storage_attempt_contract` | Intentional RED:0pass/15fail, including original13 and two closures | 0.729 |
| selected API+core `cargo test --all-targets --no-run` | GREEN:all selected tests/library/example compile | 0.196 |
| selected API+core `cargo fmt -- --check` | GREEN, final source | 0.165 |
| selected API+core `cargo clippy --all-features --all-targets -- -D warnings` | GREEN, final source | 0.287 |
| contracts `sh check.sh crdt-storage-attempt` | Architecture PASS, six assembly closures pass, then intentional core RED; later lint stages executed separately | 1.082 |
| contracts `sh test-selection.sh` | GREEN:unchanged exact API+core selectors and all12 | 0.107 |
| contracts `python3 -m unittest tests.test_tooling` | GREEN:3, including unchanged synthetic13th positive/negative/source regression | 4.573 |
| root `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | Intentional RED:exact ten rows; unchanged three-order released-corpus positive control PASS | 0.969 monotonic run; final recompile tool wall 0.786 |
| root `python3 glade/scripts/checks/check_process_globals.py` | GREEN:82 files,3 permanent allowlisted items,0 debt,nothing new | 0.481 |
| root `node glial/test/independent_admission_source.mjs` | GREEN:unchanged compound-body guard | 0.230 |

All42 domain failures are compiling assertions against refusing `step`, not
compiler errors or failed seed construction. The6 fixture closures GREEN prove
only corrected development composition. The ten text rows still exercise the
actual kernel with retained actual-port sessions; no private successful admission
model or fallback supplies missing receipts/reads. The unchanged released consumer
still verifies canonical payload/ref/origin bytes and reports its three-order
positive control before all ten kernel failures.

A read-only assertion-macro comparison with Glade input pin retained every
original assertion: Records52, recovery55, STA40. The two extra STA consumers add
six assertion macros. Original loop/matrix cases and all ten text assertions are
preserved. Normal selectors, manifests/dependencies/classifications, budgets,
allowlists, canonical corpus and Glial JS files are unchanged.

Runner-only warm executable measurements: public journeys0.0041s,
probe0.0037s, API source0.0244s, fixture closures0.0043s, core representation0.0033s,
core source0.0415s, Records RED0.0051s, recovery RED0.0040s, STA RED0.0043s.
The last three binaries were subsequently recompiled where their shared helper
changed; no runner timing is relabelled as build time. Earlier warm selected
all-targets/no-run was0.277s and a fresh CARGO_TARGET_DIR all-targets/no-run was
4.689s before the last counter guard. Final incremental compilation was0.196s;
fresh-target compilation was3.580s after that correction.
Dependencies were cached locally; no clean-download performance claim is made.

External Gyld is untouched and remains at `95a426595bba8e248a5f484272e483a070c73918`.
The changed fields/fixtures refine existing STA ownership rather than changing its
source-qualified allocation. No Gyld tests or unaffected wider workspace/Mypy
checks were rerun; historical budget results/baseline Mypy limitations are not
rewritten. All12 ARCH002 refusal remains unchanged historical evidence; its
synthetic13th regression and current positive architecture gate were executed.

## 4. Preserved stop boundary

The refusing core lib.rs is byte-identical to the input Glade pin, SHA256
`5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.
No production admission behavior, physical host, crypto/disk/restart/import,
async cancellation/drain, live synchronization or activation was implemented or
qualified. No Git mutations, pushes, live operations, root plan/report/ledger
writes or external Gyld writes were performed by the drafter.

All five finding IDs remain **OPEN**. Originating closures and fresh full blind
axes are required at the owner's settled tuple because shared Recovery changes.
This evidence supports review; it is not acceptance or authority to implement
successful kernel behavior.
