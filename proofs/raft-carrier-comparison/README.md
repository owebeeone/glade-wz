# Private Q4-B0 compiler / behavioral RED scaffold

**DRAFT only.** All election operations in both named provider crates return
`NotQualified`. No actual carrier, runtime, source adaptation, election, disk,
production dependency or authority is present. See
[the allocation](../../dev-docs/GladeRaftB0Allocation.md) for proposed signatures,
upstream engineering feature/timing profile and still-open proof obligations.

The API is std-only. The spec normally depends only on API; its current std-only
refusing providers are dev dependencies. Their future engine replacement would
contaminate package unit loops through those dev edges: a reviewed runner/test
allocation must preserve the common consumers while moving real composition out
before installing engines. Source, future scheduling and queued-RPC modules here
are pure controlled test scaffolding, not claimed implementation providers.

From the adopting workspace root, compiler/fixture/oracle witnesses (45 passing: 41 spec and 4 API/provider):

```sh
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-api --test public_contract
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-raft-rs -p glade-carrier-openraft --test compiler_contract
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-spec --lib --test constructor_contract --test fixture_plumbing --test oracle --test rpc_reply --test endpoint_lifecycle --test scheduler_lifecycle
```

Ordinary behavioral RED command (20 failing assertions; no ignored/filtered cases):

```sh
cargo test --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-spec --test b0_election
```

Structural/source/global/format and lint commands (passing, separately from RED):

```sh
proofs/raft-carrier-comparison/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-carrier-comparison/Cargo.toml -p glade-carrier-api -p glade-carrier-spec -p glade-carrier-raft-rs -p glade-carrier-openraft --all-targets -- -D warnings
```

[Measurements](evidence/measurements.json) record exact commands, exit statuses,
machine/toolchain, execution-only, warm build-plus-RED and fresh-target build.
[Behavior assertions](evidence/behavior-red.log),
[structural results](evidence/structural.log), [lint](evidence/clippy.log),
[API](evidence/api-witness.log), [provider compiler](evidence/provider-witness.log)
and [fixture/compiler/oracle](evidence/spec-witness.log) are filed separately.
The correlated-reply fixture supplement has its own observed [RED](evidence/rpc-red.log)
and [GREEN](evidence/rpc-green.log) evidence; it implements only controlled fixture mechanics.
`measure.py --prefix <fresh-label>` replays this checkpoint with the RED failure recorded as failure;
recognizing its expected status does not make behavior pass or qualify a carrier.

Future real adapters MUST reuse the same exported scenario functions. The refusing
command remains explicit and RED; a separate reviewed composition runs real
adapters without changing these expected invariants or selecting fewer scenarios.

Remediation 1 preserves the original evidence above. Its regression-first
[RED log](evidence/rem1-red.log) records 12 compiling assertion failures; the
[overflow supplement](evidence/rem1-red-overflow.log) adds the thirteenth unique
regression before corrections. The separate
[authoring compile log](evidence/rem1-authoring-compile.log) is not behavioral RED.
[Initial GREEN](evidence/rem1-green.log) and
[expanded GREEN](evidence/rem1-green-expanded.log) retain fixture verification.
Remediation 1 [measurements](evidence/rem1-final-measurements.json) and
[inventory](evidence/rem1-files.sha256) describe the final scoped packet.

The footprint witnesses exercise only supplied source/work mechanics and an
external access/state oracle, without an election/engine surrogate. OpenRaft's
constructor-only entropy access remains distinct from raft-rs's documented leader
quorum-loss reset. A persistent task need not register again: a runtime registration
fault is asserted at an actual attempted registration, or the no-access cut remains
healthy. Concrete adapted clock/task/register cadence remains a later obligation.
The endpoint/scheduler regressions cover shared stop, replacement, unchanged stale
controls, selected-future stop, saved wake and reentrant future cleanup. Moving
saved RPC wakes after all borrows is a scoped cleanup refactor verified by existing
RPC tests; no safe direct Rc-endpoint reentrant std-Wake witness is claimed.
Both originating reviewers must verify their findings before closure. No B0
runtime or source-adaptation acceptance follows from these GREEN fixtures.

To reproduce with a fresh evidence prefix (prior logs are retained):

```sh
python3 -B proofs/raft-carrier-comparison/measure.py --prefix review-next
```

The evidence gate requires exactly 20 ordinary compiled B0 failures, zero ignored
or filtered cases, as well as passing independent GREEN and structural/lint gates.

Remediation 2 restores live caller-owned Timeout/Cancel independently of remote
peer liveness. Respond/Resolve and message delivery retain strict remote scope
checks. The [compiling RED](evidence/rem2-red.log) contains eight failed matrix
cells plus two corrected prior tests; the [first GREEN](evidence/rem2-green.log)
then passes those paths. The final packet has
[41 spec GREEN witnesses](evidence/rem2-final-spec-witness.log) plus 4 separate
API/provider compiler tests (45 total), with the unchanged
[20 ordinary provider RED cases](evidence/rem2-final-behavior-red.log).
The [mutant check](evidence/rem2-final-termination-mutant.log) restores the exact
peer-liveness defect in an isolated copied fixture and rejects all eight cells.
It also passes six unrelated/negative unit checks, with none ignored or filtered.

The eight cases cover held/consumed request × stopped/replaced peer × Timeout/Cancel.
They first poll the actual response Pending, check only the selected caller-owned
RPC is removed, require a wake without inline future execution, and verify the
correct typed error at the next selected poll. Read-only private inspection
verifies every originally issued byte remains identical; peer/replacement messages,
RPC and work inventories stay unchanged. The caller continues traffic. Foreign,
forged, replaced-caller and repeated terminal controls, as well as stale and live-
peer late replies, refuse unchanged. These are controlled fixture results only.

Current [measurements](evidence/rem2-final-measurements.json),
[exact changes](evidence/rem2-changes.json), and
[root-relative inventory](evidence/rem2-files.sha256) retain the new checkpoint
separately from all earlier evidence. The ownership clarification is in the
allocation; no public signatures, dependencies, providers, case labels or
comparison-contract text changed. This is the second architectural remediation
round. Original finders and fresh peer-blind full reviewers MUST verify the settled
object; no self-closure or third architectural patch is authorized.

Focused standalone mutant reproduction:

```sh
python3 -B proofs/raft-carrier-comparison/check-termination-mutant.py
```
