# Storage-attempt typed contract / RED evidence

Date: 2026-10-04. Status: **DRAFT, compiling contract/RED checkpoint only**.
[The controlling contract](GladeIndependentCrdtStorageAttemptContract.md) defines
scope and exact supersession. Its inspection tuple and accepted design sources
apply here; the owner MUST settle the edited root/Glade/external-Gyld tuple before
fresh independent Code/State review. No successful kernel implementation, physical
host, wire/API freeze or live qualification is claimed. The historical failed
IC-1 contract, evidence and review-cycle files remain unchanged.

## 1. Chronology, including the initial TDD deviation

The first draft moved shared values and declared the API **before** writing its
specific new Rust regression consumers. The original 27 RED suite had run, but it
did not cover those new public declarations. That chronology was a TDD deviation;
it is not rewritten as test-first, and no owner waiver is inferred.

On the owner's audit instruction, implementation stopped. The authored API/core
scaffolds were saved outside the tracked tree under `/tmp/sta-bootstrap`.
Read-only `git show` restored only those authored type/manifests/policy/lock paths
to the inspection baseline. New tests and unrelated/Gyld work were preserved;
no Git mutation was used. The following restarted sequence then executed:

| Order | Specific regression before implementation | Observed failure and following smallest step |
| --- | --- | --- |
| 1 | Nine public API consumers and eleven kernel STA001–011 consumers written against the restored baseline | API package missing; core consumers had63 missing-type/field errors. Reintroduced only needed declarations/shared reexports, leaving refusing fixtures and `step`. |
| 2 | Same consumers rerun with compiling signatures | API 9/9 behavioral failures and core 11/11 behavioral failures. These, rather than the preceding compiler errors, establish the new behavior RED. |
| 3 | Expanded actual-port host journeys: lost preparation acknowledgment/queued registration, Started worker and both fence races, finite exhaustion | All 12 host journeys failed against refusing port fixture before development memory behavior was written. |
| 4 | Source identity/owner/callback/finality and disabled associated/foreign/field fixtures | One new shape regression failed (four prior checks passed); implemented syntax-aware required shape checks, then all five passed. |
| 5 | Both affected selector lists, all 12 count and synthetic 13th ARCH002/source regressions | Three tooling tests failed. Fixed only selected API+core lists and replaced forbidden out-of-line path declaration with inline test include. All three then passed; policy roles/edges remained review proposals. |
| 6 | Separately pinned Gyld lifecycle consumers before capture implementation | Three capture consumers failed for absent lifecycle host; implemented app-owned overlay capture, then exact ownership/pin/standalone allocation checks passed before successful development storage behavior. |
| 7 | Qualified immutable semantic-capture reuse regression | Four tests errored on absent `original_baseline`; implemented exact source/ledger/identity provenance validation before reuse. |
| 8 | Qualified immutable lifecycle-capture reuse regression | Three errors among five tests on absent `lifecycle_baseline`; implemented exact lifecycle source/ledger/identity provenance validation. Corrupt pins, changed declaration, changed semantic provenance and removed ownership still refuse. |
| 9 | Started cut survives revocation/delayed begin acknowledgment | One executed behavioral failure: repeated begin incorrectly rechecked the current cut. Preserved the retained Started cut before any current authorization check; regression passed. |
| 10 | Validated model restoration | One executed failure before adding the restoration fixture. It now retains mappings/phases/namespaces/revisions/floors; external antirollback/clone qualification remains absent. |
| 11 | Host checks its own serialized observation, not caller `current` | One executed behavioral failure before injecting explicit trusted host observations/checking them; regression passed. |
| 12 | Historical AcceptedBatch retains original receipt without local prestart | One executed failure before allowing historical `prestart=None`; local prestart checks remain intact. |
| 13 | Terminal contradiction evidence, explicit limits and consumed full-request history | Each added field first failed compilation (`terminal_conflicts`, `storage_limits`, `retired_storage_invocations`). Only representation was added; kernel behavior stayed refusing. Genuine bound contradiction plus six unissued/foreign/changed request mutations compile and remain RED. |
| 14 | Caller-owned session driver consumer | Missing `drive_with_port` failed compilation before extracting the generic explicit-session driver. Its actual-emission/continuation assertions now compile and remain RED. |
| 15 | Finite trusted observation injection | Initially failed on missing Result shape, then a compiling helper returning Ok for all injections failed behaviorally on the second instance. Added finite instance/name checks; the seventeenth host test passed. |

Saved local diagnostic logs include `/tmp/sta-restarted-red.log`,
`sta-host-red.log`, `sta-source-red.log`, `sta-selector-red.log`,
`sta-startcut-red.log`, `sta-restore-red.log`, `sta-hostcurrent-red.log`,
`sta-historical-red.log`, the three field-shape RED logs,
`sta-driver-shape-red.log`, `sta-observation-shape-red.log` and
`sta-observation-behavior-red.log`. These ephemeral logs are diagnostic aids;
the committed consumer sources and exact commands below reproduce the checkpoint.
The initial deviation remains disclosed regardless of the restarted sequence.

## 2. Files and boundary decisions

Root additions are the controlling Contract and this Evidence. Glade adds the
proposed Contract package [API](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-storage-attempt-api/src/lib.rs),
its public-port journeys, test-only bounded host, six-mutant probe and source
checks. Core [types](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/types.rs)
reexport only genuinely shared values and add complete lifecycle continuations;
its [STA consumers](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/tests/storage_attempt_contract.rs)
remain RED. The original 21 Records and six recovery consumers retain their
assertions and loop cases with typed request/terminal plumbing; the shared driver
executes only actual public effects through the supplied port. The convenience
wrapper creates a scoped fresh fixture; future successful multi-call tests MUST
retain the explicit session or inject qualified recovery.

Cargo member/lock/manifests, README, narrowly edited architecture policy and both
selectors/tooling tests are affected. The policy diff adds the new Contract role,
required methods and minimal sha2/dev-syn edges, replacing core's SHA edge with
that Contract edge; existing unrelated entries/formatting are preserved. Pure core
is still Pure. No classification, allowlist or framework exception was loosened.

External Gyld adds a [declaration](/Volumes/projects/limbo/gyld-wz/gyld/examples/glade-storage-attempt.gyld.py)
and separately frozen [lifecycle ledger](/Volumes/projects/limbo/gyld-wz/gyld/examples/glade-storage-attempt-sources.json),
with the existing app-owned supplemental capture host and architecture tests.
Canonical accepted lifecycle design text is frozen by source-qualified root/path
and SHA256, reviewed at `d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`, accepted at
`01602d89a7130df9cc09c6f4ba889d2b7ab4a4bc`. The old 31 allocations/124 obligations
remain; the new overlay is 34/135, allocating STA001–009 to Records with
StorageAdapter cooperation, STA010 to existing Pure decision owner and STA011 to
existing conformance owner. There is no Gyld engine/evaluator/base-policy change,
mutable sibling runtime source, scoring expansion or satisfaction claim.

## 3. Settled intentional RED and GREEN checks

Commands run from root unless the table names another working directory. Cargo
commands use the contracts workspace, `--locked --offline`. Warm wall measurements
use Python monotonic elapsed time around subprocess execution; tests are selected
API plus affected core, not an unaffected whole-workspace loop.

| Command / selected target | Result | Warm wall seconds |
| --- | --- | ---: |
| API `cargo test -p glade-crdt-storage-attempt-api --all-targets` | GREEN:17 public journeys, one six-mutant probe and five source tests; final run after observation bound | 0.711 final run |
| core `cargo test -p glade-crdt-admission-core --test representation --test source_boundaries` | GREEN:2 representation and3 source checks | 1.243 |
| core `cargo test -p glade-crdt-admission-core --test records_host_contract` | Intentional RED:0pass/21fail, exit101 | 1.061 |
| core `cargo test -p glade-crdt-admission-core --test recovery_contract` | Intentional RED:0pass/6fail, exit101 | 1.067 |
| core `cargo test -p glade-crdt-admission-core --test storage_attempt_contract` | Intentional RED:0pass/13fail, exit101 | 0.703 |
| API+core `cargo test --all-targets --no-run` | GREEN:all selected library/test/example targets compile | 0.777 final incremental run |
| API+core `cargo fmt -- --check` | GREEN | 0.235 final run |
| API+core `cargo clippy --all-features --all-targets -- -D warnings` | GREEN; rerun after last helper change | not timed |
| contracts `sh check.sh crdt-storage-attempt` | Architecture PASS then intentional core RED, exit101; later stages separately executed | 1.005 |
| contracts `sh test-selection.sh` | GREEN:both selectors exact API+core, unchanged other selectors, all 12 | 0.114 |
| contracts `python3 -m unittest tests.test_tooling` | GREEN:3 tests including synthetic 13th positive/negative/source checks | 4.567 |
| contracts `sh arch002-fixture.sh` | GREEN:untouched copied workspace positive control; exact ARCH002 framework refusal for each of 12 classified libraries | 4.083 |
| `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | Intentional RED:all exact ten mandatory rows; released canonical corpus positive control PASS in three orders | not timed |
| `node glial/test/independent_admission_source.mjs` | GREEN:unchanged JS compound-body guard | not timed |
| `python3 glade/scripts/checks/check_process_globals.py` | GREEN:82 files,3 permanent allowlisted occurrences,0 debt,nothing new; unchanged allowlist | not timed |
| Gyld `python3 -B scripts/check_architecture.py` | GREEN | not timed |
| Gyld pinned Ruff0.13 check/format on modified host and architecture test | GREEN | not timed |
| Gyld `MYPYPATH=src:. uvx --quiet --python 3.13 mypy@1.18.1 --python-version 3.11 --check-untyped-defs --explicit-package-bases scripts/capture_glade_independent_crdt.py` | GREEN:one modified host source | not timed |

The 40 Rust behavioral failures are assertions against compiling unchanged
refusing `step`, not missing symbols or a private successful admission model. The
original 27 test bodies keep their success/failure/edge assertions; once charges,
all four kinds, full original receipts, start cuts, fork/old retry AB and Y
isolation remain. KnownAbsent/negative-commit fixtures become fenced terminal
NonCommit; separate Pending journeys cover temporarily absent registration and
late publication. STA005 authenticates full issued/consumed requests before
contradiction evidence; attacker-changed terminal DTOs cannot create integrity
stop. STA010 is sticky with no clearing event/witness. API model success does not
satisfy these kernel assertions or any physical qualification.

Runner-only timings (warm already-built executables, excluding Cargo/build):
public journeys0.0035s; mutant probe0.0035s; API source0.0215s; core
representation0.0036s/source0.0376s; Records RED0.0044s; recovery RED0.0037s;
STA RED0.0039s. These precede only the final observation-map regression addition.
A selected fresh CARGO_TARGET_DIR build of both packages' all-targets/no-run took
4.696s before the last helper regression and **6.181s on the final fixture**, exit0
in both cases; dependencies were cached locally. An earlier fully cached warm
all-targets/no-run was 0.249s; the final incremental no-run was 0.777s. This is cold selected compilation,
not a clean-download benchmark or runner timing. No workspace-wide minor loop
or unaffected Mypy rerun was substituted.

## 4. Gyld timing chronology and unchanged baseline limits

The full affected command is **`python3 -B scripts/test_fast.py --affected architecture`**.
Its selector and2.0s execution budget are unchanged. All assertions passed on every
reported timing run; budget failures are reported as failures, not hidden.

| Chronological stage | Exact count | Startup seconds | Execution seconds | Budget result |
| --- | ---: | ---: | ---: | --- |
| Initial lifecycle capture added | 174 | 0.062 | 2.545 | FAIL,+0.545 |
| Qualified semantic reuse regression/implementation | 175 | 0.044 | 2.113 | FAIL,+0.113 |
| Removed redundant annotation work in qualified check | 175 | 0.042 | 2.051 | FAIL,+0.051 |
| Exact frozen-source fingerprint check | 175 | 0.048 | 2.066 | FAIL,+0.066 |
| Qualified lifecycle reuse regression/implementation, concurrent work | 176 | 0.049 | 4.306 | FAIL,+2.306 |
| Isolated next run | 176 | 0.069 | 2.105 | FAIL,+0.105 |
| Diagnostic per-test timing (same 176 assertions, different diagnostic runner) | 176 | excluded | 1.973 | Diagnostic only; not claimed official budget gate |
| Official runner concurrent with Cargo compilation | 176 | 0.049 | 4.292 | FAIL,+2.292 |
| Final isolated official affected runner | **176** | **0.046** | **1.958** | **PASS,2.0s unchanged** |

Immutable fixture reuse is app-host provenance checked, not stale acceptance:
changed semantic ledger/source, changed lifecycle source and removed requirement
ownership regressions refuse. The shared tests retain the historical 31/124 source
ledger; the new lifecycle ledger is separate. Final timing has only 0.042s margin
and the recorded host-contention runs show why it is not a scheduling guarantee.
Known six baseline Mypy errors from historical wider checks remain limitations;
this tranche checks the new host and does not rewrite or rerun unaffected wider
workspaces to claim they disappeared.

## 5. Preserved bytes and stop boundary

Read-only comparison with the inspection pins verifies these files are identical:

| File | SHA256 |
| --- | --- |
| core refusing src/lib.rs | `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539` |
| Glial released text consumer | `c87faeee23302f37d5279d061e526ccfb6b29fd0698f0b68330f5334ed81b87f` |
| Glial source guard | `f91df76172152fa746bc3fbed2e5e967f44808166e39b57dc04e90deb444818e` |
| Canonical sibling corpus | `d2b65f182b66028e05e7d9615cb7b27d791dcbf55bbc6a11e450503b81fd731c` |
| Historical Gyld semantic ledger | `600ca40082f78372446565f820250a7fe9df8cc2d0200f6d67682baffb09d901` |
| Historical Gyld supplemental declaration | `6ab485da04265f92912fb6853908b548fd9580c94a3622b7ae6cfc513759aa49` |

The digest-pinned canonical sibling corpus bytes are unchanged; the unchanged
consumer verifies their payload/ref/origin identity before reporting its
three-order positive control. Historical failed root docs/evidence, accepted
semantic design, discovery, Glial and process-global allowlists are unchanged.

Synchronous model operations prove no live async cancellation/drain, physical
barrier, process kill/reopen, quota reservation, rollback/clone exclusion,
cryptography or disk-outage independence. The critical metadata sizing/formats,
real Records scheduling and external ownership floors need later qualified hosts.
Package role/edge and typed contract changes remain proposals until fresh dual GO.
No Git mutation, push or live operation was performed by the drafter. **The core
MUST remain refusing until this typed checkpoint passes its required reviews.**
