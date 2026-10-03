# IC-1 internal contract — remediation1 evidence

Date: 2026-10-04. **Pending originating verification and fresh full review.**
This records the one merged correction in
[RemPlan-1](GladeIndependentCrdtAdmissionContract-RemPlan-1.md).
The drafter does not close any finding or authorize successful kernel behavior.
The original [IC-1 evidence](GladeIndependentCrdtAdmissionContract-Evidence.md)
and frozen Gyld source/evidence ledger remain historical records, unchanged.

Reviewed baseline: root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`,
Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`, Glial
`5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`64666e8b1caadde8922b9d42163afbab90655c65`.
The parent records the corrected committed tuple after this handoff.

## Correction and test-first evidence

The new external Rust consumer was written before types/helpers changed.
`cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test recovery_contract --no-run`
initially exited101 with17 compile errors in0.483s: missing BatchCommitted,
lookup_id, qualified_forks and BatchRetained, and the mandatory Digest versus
optional retry target. This demonstrates the missing interface, **not** the final
behavioral RED. After the type changes, all consumers compile; final RED below
comes from executed behavior assertions against the unchanged refusing step.

The JS required-row/post-fork assertions were extended before the Rust trace.
`sh glade/contracts/crdt-admission-core/check-text-contract.sh` first exited1 in
1.368s at `ICD-017 exact real-text witnesses cannot be omitted or duplicated`,
showing the two absent AB-buffer rows. After trace extension, the exact ten-row
inventory passes and all ten rows fail actual kernel behavior assertions.

Consistency P2-1: LookupRequest/Reply carry fresh lookup_id separately from the
immutable plan_id; State.lookups is keyed by lookup_id. Unknown retires only that
invocation. The new consumer issues L1, resolves Unknown, issues L2 and supplies
all three stale L1 answers; each must preserve L2 and reservation. Exact L2
committed recovery returns original receipts without Seal/Commit. Counter
exhaustion must refuse without wrap. The restoration case explicitly injects a
trusted preserved high-water mark and invalidates transport continuations; it is
not disk restoration qualification.

Safety P2-1: BatchCommitted binds the original batch digest, achieved revision and
storage class plus the exact complete staged admission receipt list. Candidate
and SecurityEvidence plans/answers have no admission receipts; bare rival and
unresolved candidate evidence both have admission=None. QualifiedFork retains
explicit independently qualifying pairs, with genuine historical acceptance
continuations where needed. BatchRetained is batch custody, never application
admission. The new matrix contains committed/known-absent/still-unknown paths for
all four kinds, including uncertain direct barriers, once-only charges, original
custody preservation, duplicate refusal, candidate pending/common prefix,
security nonconviction, fork common prefix and independent-instance local progress.
Another consumer mutates committed batch digest/revision/storage/receipt list.
The looped matrix stops at its first assertion on this scaffold; these are compiled
future obligations, not12 successful recovery demonstrations.

Consistency P2-2: Rust and released-text AB now combine a genuine pair of E0 seq1
qualified rivals, floor1 quarantine and eligible E0 seq0 A with fresh E1 seq0 B
referencing A. Both rival arrival orders require AB, exact eligible identities,
retained old custody/receipts/exact retries, persistent old quarantine, fresh local
receipt and reused E0 rejection. Two separate AB-buffer rows preserve B-before-A
replay coverage. The prior A/AD/ABC/isolation rows remain required. During IC-2, a
source mutant refusing every fresh-origin local edit whenever quarantine is
nonempty MUST fail these combined Rust and actual-text witnesses. No successful
kernel or mutation framework is introduced at this gate.

Consistency P3-1: Normative callback refusal now applies to retired/mismatched
continuations, not elapsed delay. The original delayed prior-cut committed and
unknown recovery assertions remain, with the corrected exact batch reply.
Existing trusted staged fixtures now retain a counter above their preloaded IDs;
all original21 test functions/domain assertions remain.

## Reproducible checks

Run from `/Volumes/projects/limbo/glade-wz`. Wall times use Python monotonic timing
around captured subprocesses; test runner durations are noted separately. Normal
package dependencies and classifications are unchanged. Warm local Cargo cache
is used; this patch does not change selector/budget/dependency policy.

| Command | Result | Wall seconds |
| --- | --- | --- |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --no-run` | GREEN, exit0 | 1.037 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test recovery_contract` | intentional behavioral RED, exit101 | 0.477 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test records_host_contract` | intentional behavioral RED, exit101 | 0.434 |
| `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | intentional behavioral RED, exit1 | 0.501 |
| `cargo test --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --test representation --test source_boundaries` | GREEN, exit0 | 0.872 |
| `cargo fmt --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core -- --check` | GREEN, exit0 | 0.090 |
| `cargo clippy --manifest-path glade/contracts/Cargo.toml -p glade-crdt-admission-core --locked --offline --all-targets -- -D warnings` | GREEN, exit0 | 0.377 |
| `cargo run --quiet --locked --offline --manifest-path glade-discover/tools/architecture-check/Cargo.toml -- glade/contracts` | GREEN, exit0 | 0.533 |
| `python3 -B glade/scripts/checks/check_process_globals.py` | GREEN, exit0 | 0.497 |
| `node glial/test/independent_admission_source.mjs` | GREEN, exit0 | 0.229 |
| `python3 -B -m unittest discover -s glade/contracts/tests -p test_tooling.py` | GREEN, exit0 | 3.860 |
| `sh glade/contracts/arch002-fixture.sh` | GREEN, exit0 | 3.424 |
| `sh glade/contracts/test-selection.sh` | GREEN, exit0 | 0.088 |
| `sh glade/contracts/check.sh crdt-admission` | intentional behavioral RED, exit101 | 0.778 |

Final no-run compiles all targets, including the new consumer. Separately executing
the original records consumer reports **0 passed/21 failed**, and the new recovery
consumer **0 passed/6 failed**; both runners report0.00s. Failure examples are
`first lookup invocation`, the missing unknown reservation, CallbackMismatch and
Capacity assertions, and `ICD-011 expected explicit local read cut`. No missing
symbol/compile error is used as the settled RED evidence.

Released `@owebeeone/taut-shape`0.9.1 passes all three pinned reference corpus
orders, then reports **10 behavioral failures**: nine unavailable kernel cuts and
missing isolated B/C local receipts for ABC. Exact inventory includes AB/AB-buffer/
AD/A in both orders, ABC and isolation. These traces never substitute expected
eligible operations or emulate admission/merge.

GREEN representation/source checks report2+3 passed, source runner0.04s.
JS braced-body guard passes. Architecture passes. Process-globals reports81 files,
3 unchanged permanent items,0 debt, nothing new. Tooling regression reports2 passed
in3.815s, including the synthetic additional member. ARCH002 retains its positive
control and exact framework refusal for all11 classified libraries. The selected
normal runner passes architecture then exits101 at the original21 behavioral
assertions; Cargo stops before the additional RED test binary, so it was executed
separately above. This is deliberately not a successful feature gate.

## Exact scope and preserved provenance

Only these eight files are authored by this remediation:

- `dev-docs/GladeIndependentCrdtAdmissionContract.md`
- `dev-docs/GladeIndependentCrdtAdmissionContract-Remediation1-Evidence.md`
- `glade/contracts/crdt-admission-core/src/types.rs`
- `glade/contracts/crdt-admission-core/tests/support/mod.rs`
- `glade/contracts/crdt-admission-core/tests/records_host_contract.rs`
- `glade/contracts/crdt-admission-core/tests/recovery_contract.rs` (new)
- `glade/contracts/crdt-admission-core/examples/text_admission_trace.rs`
- `glial/test/independent_admission_contract.mjs`

Read-only comparison against the reviewed member commits confirms the refusing
lib.rs, architecture policy, Cargo.lock, Gyld supplemental declaration and frozen
supplemental ledger are byte-identical. Their SHA256 values are:

| Preserved file | SHA256 |
| --- | --- |
| core src/lib.rs | `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539` |
| contracts/architecture-policy.json | `9cec3766559206118c1f311ce7bf45fbde4917324045c4d3a47582fdb29f755f` |
| contracts/Cargo.lock | `837760babc118987b19a6da018c15691b3d84073a602dd941c0c982233e4128b` |
| Gyld supplemental source ledger | `600ca40082f78372446565f820250a7fe9df8cc2d0200f6d67682baffb09d901` |
| Gyld supplemental declaration | `6ab485da04265f92912fb6853908b548fd9580c94a3622b7ae6cfc513759aa49` |

Gyld allocation/requirements mapping is unchanged; no Gyld files are modified and
no new Gyld result is claimed. Existing full171 timing and six baseline Mypy errors
remain in the historical evidence, without rewriting frozen metadata. Canonical
remote schema, crypto/clock/custody trust, physical commits/restart, live duplex,
client lifecycle, process RSS/physical quota and activation remain later gates.
The volatile host and synthetic state/evidence prove none of them. No Git mutations
or successful kernel implementation occurred. Findings remain open for their
originating reviewers and fresh full axes.
