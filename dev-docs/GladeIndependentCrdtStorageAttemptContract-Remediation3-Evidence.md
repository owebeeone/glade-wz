# Storage-attempt contract — nonarchitectural correction 3 evidence

Date: 2026-10-04. **DRAFT; State-3 P2-1 OPEN; kernel remains refusing.**

This one confined patch follows [RemPlan-3](GladeIndependentCrdtStorageAttemptContract-RemPlan-3.md)
and the complete [State-3](GladeIndependentCrdtStorageAttemptContract-ReviewState-3.md)
report; [Code-3 GO](GladeIndependentCrdtStorageAttemptContract-ReviewCode-3.md) is
legitimate merged input. Prior six IDs were independently closed. Input root
`939ace96077f529fc66ee5986e772352f7190a15`, Glade
`c6c4239beecb129aa0585fe74006cc287dff3b87`; unchanged Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. The owner MUST settle the corrected
review tuple; these are input pins. The drafter does not self-close findings.

## 1. Exact correction

Only the existing development `MemoryHost::restore` validator changes. Its finite
per-instance committed-revision set rejects distinct commits at one revision.
Existing expected+1 and terminal<=current checks ensure each committed revision
is positive and bounded by the head. Comparing the set cardinality with that head
therefore rejects missing head, gaps and unbacked positive heads under the already
required full-history/no-GC grammar, without iterating a caller-carried scalar
range. Conversion to u64 is checked. No import/base-revision field or rollback
exception is added. NonCommit must equal its binding's unchanged expected revision;
Started/Committed original cuts must fit all their immutable windows. Reserved
may still carry a cut that a later Begin refuses, and historical starts do not
undergo current-policy reauthorization.

Five added public regressions cover the exact authentic two-commit mutation,
six missing-history/head cases, legal sequential1/2 and repeated NonCommit at2,
independent X/Y current revision1, four Started/Committed window mutations, and
an altered NonCommit revision. Authentic recovery/reopen/inspection repeats all
original terminal bindings/outcomes. No history, identity, binding or head is
rewritten to make an inconsistent image acceptable.

Glade changes are exactly `contracts/crdt-storage-attempt-api/tests/public_contract.rs`
and `tests/support/mod.rs`; root changes are the controlling Contract and this
new evidence. Shared API, refusing kernel, assembly, mutation boundaries,
architecture, dependencies, roles, limits and selectors remain unchanged.
No boundary problem or architectural root was found while enforcing these
existing-field consistency rules. Any such root in this third round MUST stop
for owner decision; this correction does not reset that cap.

## 2. Test-first chronology

Before validator edits, tests were added and each executed from `glade/contracts`
with `cargo test --locked --offline -p glade-crdt-storage-attempt-api --test public_contract`
and the following filter:

| Filter | Executed initial result |
| --- | --- |
| `authentic_two_commit` | Compiling RED: authentic X commits1/2, then only second binding expected1→0 and Committed2→1 changed; restore accepted the contradictory image. `/tmp/sta-rem3-duplicate-red.log`. |
| `full_retained_commit` | Compiling RED: removed authentic revision3 head terminal, left current3; restore accepted. `/tmp/sta-rem3-gap-red.log`. |
| `sequential_commits_repeated` | GREEN before implementation: sequential1/2, repeated negative at2, independent X/Y each at1, compatible reopen/terminal repetition. `/tmp/sta-rem3-positive-before.log`. |
| `restored_started_and_committed` | Compiling RED: authentic protected Committed cut20–21 retained while its window starts22; restore accepted. Test also contains Started and upper-window-bound mutations. `/tmp/sta-rem3-cut-red.log`. |
| `restored_noncommit` | Compiling RED: authentic expected1 NonCommit1 altered to revision0; restore accepted. `/tmp/sta-rem3-noncommit-red.log`. |

The phase-payload tests followed the owner's explicit adjacent audit instruction,
using existing accepted grammar. Every negative regression was present and had
executed behavioral RED before the corresponding validator edits. No compiler
failure or source substitution is counted as behavioral RED. Minimum validator
checks were added together; all28 public journeys then passed, including every
mutation row and positive control (`/tmp/sta-rem3-public-first-green.log`).
Prior disclosed TDD history remains unchanged; this evidence does not rewrite it.

## 3. Focused final verification

Monotonic wall seconds include Cargo startup/build/test. All Cargo checks use
`--locked --offline`; fmt selects only API/core. The development provider is
included by affected core consumers, so both packages are selected.

| Command/target | Exact result | Wall seconds |
| --- | --- | ---: |
| API `cargo test --all-targets` | GREEN:28 public +1 six-mutant probe +5 source =34; all prior29 retained | 0.898 |
| core `cargo test --test fixture_composition --test representation --test source_boundaries` | GREEN:7+2+3 =12, unchanged obligations | 1.597 |
| core `cargo test --test records_host_contract --test recovery_contract --test storage_attempt_contract --no-fail-fast` | Intentional compiling RED:22+6+15 =43; all retained | 2.265 |
| selected API/core `cargo test --all-targets --no-run` | GREEN:all selected targets compile | 0.222 |
| selected API/core `cargo fmt -- --check` | GREEN | 0.173 |
| selected API/core `cargo clippy --all-features --all-targets -- -D warnings` | GREEN | 0.326 |
| architecture checker `cargo run --quiet --manifest-path ../../glade-discover/tools/architecture-check/Cargo.toml -- .` | Boundaries PASS | 0.545 |
| root `sh glade/contracts/crdt-admission-core/check-text-contract.sh` | Intentional RED:exact ten rows; canonical three-order positive control PASS | 1.070 |

Final domain failures are compiling assertions against refusing step, not failed
fixture construction or compiler errors. No core test/assertion/loop, text row or
canonical corpus file changes. API/core source checks inspect the existing
conditional/compound-body boundaries. Production sources are byte-identical;
unchanged process-global/JS/selection/tooling negatives and Gyld checks rely on
preserved prior evidence and were not redundantly rerun. No allowlist, budget,
classification, framework refusal or frozen allocation/source was relaxed.

Direct warm executable seconds (separate from Cargo): public0.0038,
mutant0.0027, API source0.0303; core fixture0.0041, representation0.0029,
source0.0430; intentional Records RED0.0045, recovery RED0.0036, STA RED0.0038.
Fresh-target selected all-target compilation was4.606s (GREEN), with dependency
cache present; final incremental compilation was0.222s. No clean-download or
physical-host performance claim is made. Logs/timing JSON under `/tmp/sta-rem3-*`
are diagnostic aids; committed tests/commands reproduce this checkpoint.

## 4. Preserved stop and review boundary

Shared API and refusing kernel remain byte-identical to Glade input:
API SHA256 `86cf4baf80aa532848a9a016e3c18a34a8896fd39a266919b84cf24ca06ac03d`;
kernel SHA256 `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.
No successful admission, physical host, import/encoding, external rollback proof,
crypto/disk/restart, async cancellation, duplex or activation is qualified.
No Git mutations, root plan/report/ledger writes, external Gyld writes, push or
live operations occurred. Both root/member whitespace checks pass.

State-3 P2-1 remains OPEN for its originating reviewer to retrace on the settled
tuple. Code must confirm that final tuple and unchanged invariants before the
aggregate gate. Architectural count remains one, with no new interface/field or
architecture; this is the expressly confined third nonarchitectural correction.
