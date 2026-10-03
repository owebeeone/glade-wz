# IC-1 typed admission tranche — evidence

Date: 2026-10-04. Status: **refusing scaffold compiles; domain consumers intentionally
RED; scoped scaffold/tooling checks GREEN; Gyld full fast gate GREEN after scoped reuse; baseline type limitations below**.
Controlling semantics and boundaries are in
[GladeIndependentCrdtAdmissionContract.md](GladeIndependentCrdtAdmissionContract.md).
No successful kernel behavior, live activation, remote schema or production adapter
is implemented. This evidence is for contract review, not feature completion.

## Environment and measurement

Observed Darwin arm64, Rust/Cargo 1.96.0 (`ac68faa20`/`30a34c682`), Node 22.19.0,
Python 3.11.16, released installed `@owebeeone/taut-shape`0.9.1 and existing
TypeScript5.8.3. Gyld tooling is pinned Ruff 0.13.0/Mypy 1.18.1; mypy runs through
uvx Python3.13 with `--python-version 3.11`. These are this machine's measurements,
not promised budgets or all-platform qualification. Durations are subprocess
wall-clock seconds measured with Python `time.perf_counter`; runner execution is
reported separately where available. Commands were serialized within each repo;
there was no parallel Cargo timing contention in these final measurements.

Cold means a **fresh temporary Cargo target directory with cached dependencies**:
not a network cold install, OS cache purge or every workspace member build. Warm
uses the existing package target. No sockets/sleeps/ambient clocks run in the pure
component; harness entry points/tools consume explicit paths/inputs.

## Test-first evidence and scope

Before selector/ARCH-002 changes, `python3 -B -m unittest discover -s
glade/contracts/tests -p test_tooling.py` produced two failures in 0.262s: unknown
`crdt-admission` selector and member-versus-contract-only policy coverage. The
fixtures were then satisfied while retaining exact ALL-member framework refusal
and its positive control. A synthetic twelfth Pure member is included in the
regression. The actual workspace has eleven classified members.

The extended Rust source guard first failed its disabled-field regression:
`disabled_associated_foreign_and_field_declarations_are_checked`, actual0 versus
expected1 on a cfg-disabled struct field (exit101, runner0.00s). Only after this
regression did its generic attribute visitor replace the Item-only checker. The
three final syntax tests pass and include associated, foreign, variant, field and
out-of-line module negatives. Broader existing-code migration/macro expansion
coverage is not claimed.

Gyld overlay regression fixtures preceded the new host; initial3 errors were its
absent host in 0.001s. This is **tool-authoring TDD**, not the meaningful domain RED
proof. Final overlay3 tests pass. The Rust consumer/package and real JS consumer
compile/run before failing behavior assertions against the refusing scaffold.

## Final Glade commands/results

All commands below run from `/Volumes/projects/limbo/glade-wz`.

| Command | Exit/result | Wall seconds |
| --- | --- | --- |
| `CARGO_TARGET_DIR=FRESH_TEMP_TARGET cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --no-run` | GREEN; exit0 | 4.951 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --no-run` | GREEN; exit0 | 0.048 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test records_host_contract` | RED (intentional); exit101 | 0.436 |
| `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | RED (intentional); exit1 | 0.489 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test representation --test source_boundaries` | GREEN; exit0 | 0.840 |
| `cargo run --quiet --locked --offline --manifest-path glade-discover/tools/architecture-check/Cargo.toml -- glade/contracts` | GREEN; exit0 | 0.538 |
| `cargo fmt --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core -- --check` | GREEN; exit0 | 0.079 |
| `cargo clippy --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --all-targets -- -D warnings` | GREEN; exit0 | 0.430 |
| `python3 -B glade/scripts/checks/check_process_globals.py` | GREEN; exit0 | 0.523 |
| `node glial/test/independent_admission_source.mjs` | GREEN; exit0 | 0.232 |
| `python3 -B -m unittest discover -s glade/contracts/tests -p test_tooling.py` | GREEN; exit0 | 3.863 |
| `sh glade/contracts/arch002-fixture.sh` | GREEN; exit0 | 3.809 |
| `sh glade/contracts/test-selection.sh` | GREEN; exit0 | 0.098 |

The final cold/warm no-run commands compile the library, external Records-model
integration tests, representation checks and syntax guards. The text command also
compiles/runs the Rust example. Cold completion reports 4.90s Cargo build within
4.951s measured wall; warm no-run wall0.048s. These are distinct from test execution.

`records_host_contract` result is **0 passed,21 failed,0 ignored**; runner0.00s.
Failures name meaningful behavior, for example:

```text
ICD-002 valid isolated edit requires local receipt
ICD-005 evidence query required
ICD-006 entire conservative interval must fit permit
ICD-006 matched old-cut validation must require fresh policy before sealing
ICD-006 delayed committed prior-cut record is historical evidence, not retroactively cancelled
ICD-007 fresh canonical recovery origin must admit
ICD-008 request exact gap
ICD-010 unknown retains exact reserved plan
ICD-010 lookup cannot bind another operation
ICD-013 full edits refuse
ICD-013 exhausted recovery pauses rejoin explicitly
ICD-011 expected explicit local read cut
```

The RED suite includes invalid/ref/zero-base/epoch inputs, opposite-order AD and
qualified fork/dependent assertions, exact old retry, commit/lookup byte/instance/
revision callbacks, known/unknown outcomes, stale validation and delayed prior-cut
commit/recovery, reserved recovery/exhaustion and independent instances. Some cases
fail at their first assertion/read on the scaffold: later assertions are compiling
future acceptance obligations, not already executed success paths. Expected failure
is not filtered, inverted or converted to a passing test.

Released text output first confirms its actual positive reference:

```text
ICD released corpus reference: PASS (three orders; merge only)
```

Then all **eight exact mandatory rows** fail actual kernel behavior: AB/AD/A in both
orders and isolation report `ICD-011 kernel must return a local cut`; ABC reports
`ICD-002 both isolated edits need receipts` with actual `[false,false]` versus
expected `[true,true]`; final failures8 versus expected0 (exit1). Missing/duplicate
rows cannot pass. ABC operations/refs/payloads are checked against the frozen
canonical source, digest pinned in
`glial/test/fixtures/independent_concurrent_siblings.json`. No expected successful
operations are substituted for the unavailable kernel trace.

The representation/source run reports2 +3 passed, runner0.00s/0.03s. Its GREEN
claim is immutable derived operation bytes and state-preserving **refusal**, plus
syntax guard correctness, not admission success. Clippy (`-D warnings`) and format
pass; narrow `allow(dead_code)` on shared **test/example** support avoids reporting
helpers used only by the other test binary, with no production lint suppression.

Architecture output is `Architecture boundaries: PASS`. ARCH-002 prints exactly
`ARCH-002 <crate>: undeclared dependency normal:shaku` for all eleven actual
members, after untouched-copy PASS; no cargo-metadata failure counts as proof.
Selector prints `Contract test selection: PASS`. Process-global checker reports
**81 files,3 unchanged permanent allowlisted items,0 debt; nothing new**, including
new src via the existing contracts wildcard. JS AST guard reports
`ICD tranche JavaScript compound bodies: PASS`.

`sh glade/contracts/check.sh crdt-admission` is the selected normal command and
intentionally remains RED at the domain suite at IC-1; `all` now includes this
suite, so it is not claimed wholly GREEN. Its fmt/clippy must be run separately
while RED. No existing contract method/trait implementation was changed.

Final callback-custody refinement also pins the seal's original validation mode/
policy-time cut and retains the original verified query through sealing. Its new
mismatched-mode assertion compiles; no successful step was added. Post-refinement
no-run1.26s Cargo build, clippy0.30s and2+3 scaffold/source tests remain GREEN.
The actual selected command `sh glade/contracts/check.sh crdt-admission` then
reports architecture PASS and21/21 behavioral failures (exit101, wall 1.400s),
and the released-text command again reports its corpus positive control followed
by all8 expected failures (exit1, wall 0.470s). These are the settled code checkpoint.

## Final Gyld commands/results

Commands below run from `/Volumes/projects/limbo/gyld-wz/gyld`. Supplemental source
ledger/declarations are app-owned frozen inputs. No core/evaluator/weights changes. Timing entries record the stated measured
checkpoint; later source-status metadata does not imply an unmeasured new timing.

| Command | Exit/result | Wall seconds |
| --- | --- | --- |
| `PYTHONPATH=src:. python3 -B scripts/capture_glade_independent_crdt.py --output FRESH_OUTPUT_DIRECTORY` | GREEN; exit0 | 0.365 |
| `PYTHONPATH=src:. python3 -B -m unittest tests.test_architecture.IndependentCrdtCaptureTests` | GREEN final four tests; execution0.356s | not separately captured |
| `python3 -B scripts/check_architecture.py` | GREEN; exit0 | 0.149 |
| `python3 -B scripts/test_fast.py architecture --affected` | GREEN final171; execution 1.923s (first failed measurement retained below) | not separately captured |
| `uvx --quiet ruff@0.13.0 check scripts tests src` | GREEN; exit0 | 0.056 |
| `uvx --quiet ruff@0.13.0 format --check scripts tests src` | GREEN; exit0 | 0.045 |
| `MYPYPATH=src:. uvx --quiet --python 3.13 mypy@1.18.1 --python-version 3.11 --check-untyped-defs --explicit-package-bases scripts/capture_glade_independent_crdt.py` | GREEN; exit0 | 0.552 |
| `MYPYPATH=src:. uvx --quiet --python 3.13 mypy@1.18.1 --python-version 3.11 --check-untyped-defs --explicit-package-bases tests/test_architecture.py` | limitation below; exit1 | 0.525 |

Settled capture (after explicitly marking the ledger contract pending review and
recording its exact evidence tier) reports lineage `glade-independent-crdt-contract`, revision `v1`, digest
`38ef78e75c0dda1d5ecd419c3c8fff52593c2b7c324752154e945215f04f3211`, **31 allocations =24 inherited +7 new**, **124 obligations =107 inherited +17 ICD**, all
ICD-001..017 and four new journeys. Snapshot roundtrip succeeds; snapshot/inputs/
annotations are emitted to the selected new directory. Generated IDs differ from
the base lineage: original source-qualified relationships and owner labels, not
raw generated IDs, are preserved. The host validates inherited source equality,
unchanged frozen baseline capture and supplemental pins before exact graph checks.
The initial three regressions report0.522s execution; final four and cache checks are below. Removed ICD link and mutated source pin
refuse, and a temporary examples-only copy captures without any workzone reads.

The first full affected architecture selector ran **170 tests, all assertions OK**,
but execution **2.290s** exceeded its unchanged 2.0s budget by 0.290s and returned1.
A justified final quiet measurement (output streamed to a temporary file, same
selector/budget) still exceeded it:170 tests in 2.156s (wall2.283s, exit1). The pinned
baseline test module in a temporary equivalent app tree ran167 tests in 1.665s,
reported execution 1.666s (wall1.823s, exit0). Both failures are preserved; they were
not ignored, reclassified as GREEN or hidden by the focused command.

Only duplicated **new capture/test work** was then reduced: the existing immutable
validated architecture snapshot is reused through the new host's optional baseline,
matching the existing architecture host's optional problem-baseline pattern. Reuse
validates exact source-derived catalogue version, source-ledger context pin and
architecture lineage/revision, then still checks24 inherited owners/107 obligations
and exact combined links after fresh overlay capture. No engine/evaluator, weights,
base test, budget or selector is changed. New fixtures were written first: changed
source and changed ledger must refuse reuse; the initial run failed because the
optional baseline argument was absent. The final four overlay tests pass in0.356s.

The full required command **then passes:171 tests in 1.923s, startup 0.043s,
execution 1.923s, budget2.0s, exit0**. This is the final full affected fast gate.
Existing test module bytes before the new class are identical to the pinned
baseline. Provenance/omitted-link/standalone regressions remain enabled. It is a
measured bounded result, not a guaranteed future wall-clock budget.

The new capture host's targeted Mypy is GREEN. The affected test-only target retains
two existing errors at lines400/1111. The **combined affected targets with fresh
cache** reproduce six existing errors (below) in both current and pinned baseline
module trees; there are no new host errors after adding its genuine `set[str]`
annotation. No type-ignore/selection suppression or unrelated fix was introduced.

Reproduction:

```sh
MYPYPATH=src:. uvx --quiet --python 3.13 mypy@1.18.1 --python-version 3.11 \
  --check-untyped-defs --explicit-package-bases \
  scripts/capture_glade_independent_crdt.py tests/test_architecture.py \
  --cache-dir FRESH_TEMP_CACHE
```

For baseline, copy the app to a temporary tree (omit .git/venv/caches/artifacts),
extract `git show ca04499a360d910fbf8ee2540ed446facd051b35:tests/test_architecture.py`
**read-only** into that tree's same module path, and run the identical command/
MYPYPATH from that tree with another fresh cache. Keep the new typed host identical
so both commands use the same combined target list. The original test module SHA256
is `d8542559b86c046a7525dfe612710ec7fed513d8a753863623c66020495d4250`.
Current/baseline wall1.954s/1.662s, both exit1 with identical diagnostics:

```text
scripts/check_architecture.py:350: "object" has no attribute "__iter__" [attr-defined]
scripts/check_architecture.py:354: "object" has no attribute "items" [attr-defined]
scripts/test_fast.py:118: TestSuite assigned to str [assignment]
scripts/test_fast.py:120: str passed to TextTestRunner.run [arg-type]
tests/test_architecture.py:400: Any | None cleanup [union-attr]
tests/test_architecture.py:1111: annotation required for offered [var-annotated]
Found 6 errors in 3 files (checked 2 source files)
```

Ruff 0.13.0 check and format for scripts/tests/src pass; architecture policy remains
unchanged. Neither baseline type errors nor a later GREEN timing result is evidence
of successful admission or authority to weaken a gate.

## Scope and next gate

The contract file contains exact typed semantics/Gyld allocation and the intended
next GREEN **pure component** outcome. Its records host uses synthetic trusted facts
and volatile commit replies; its trace events model reconciliation. No T12 actual
node automatic mesh qualification, T14 physical interrupted journal/restart proof,
antirollback keys/time, canonical remote encoding/domains, real certificate/
requester/grant integration, version/seal migration, enrollment or production
activation is achieved. Owner deployment values remain unselected. No unrestricted
partition acceptance is installed behind an incomplete security/storage adapter.

After contract gate acceptance, IC-2 MUST make these meaningful behavioral consumers
GREEN with the real pure kernel, retain the scaffold architecture/source checks,
and keep live adapter qualifications separately named. The Gyld fast-budget failure is closed by scoped validated reuse; review must
also dispose of the unchanged baseline type limitations explicitly. No Git mutation was performed
by the drafter; baseline source reads and temporary artifact generation are read-only
with respect to repository/HEAD state.


## Drafter file inventory

Root workzone docs (new):

- `dev-docs/GladeIndependentCrdtAdmissionContract.md`
- `dev-docs/GladeIndependentCrdtAdmissionContract-Evidence.md`

Glade member (modified):

- `contracts/Cargo.toml`, `contracts/Cargo.lock`, `contracts/README.md`
- `contracts/architecture-policy.json` (existing entries retain their exact formatting/values)
- `contracts/check.sh`, `contracts/test-selection.sh`, `contracts/arch002-fixture.sh`

Glade member (new):

- `contracts/tests/test_tooling.py`
- `contracts/crdt-admission-core/Cargo.toml`
- `contracts/crdt-admission-core/src/lib.rs`, `src/types.rs`
- `contracts/crdt-admission-core/tests/records_host_contract.rs`, `representation.rs`, `source_boundaries.rs`, `support/mod.rs`
- `contracts/crdt-admission-core/examples/text_admission_trace.rs`
- `contracts/crdt-admission-core/check-text-contract.sh`

Glial member (new):

- `test/independent_admission_contract.mjs`
- `test/independent_admission_source.mjs`
- `test/fixtures/independent_concurrent_siblings.json`

External Gyld app (new):

- `examples/glade-independent-crdt.gyld.py`
- `examples/glade-independent-crdt-sources.json`
- `scripts/capture_glade_independent_crdt.py`

External Gyld app (modified):

- `examples/README.md`
- `tests/test_architecture.py` (new class only; original prefix unchanged)

No existing semantic design, DecisionLog, frozen base ledger/declarations,
public node/root/grant/sysdata/wire API or engine/evaluator is changed by this
IC-1 drafting tranche. Parent-owned checkpoint/commit/review remains next.
