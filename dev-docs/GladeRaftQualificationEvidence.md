# Glade Raft qualification evidence

Date: 2026-10-03. Status: **initial Q1a memory proof accepted at root
`31bbea0cf1da3c6ae437cf482cb744d561693c08` after Code/State GO/GO**.
Q0 draft/consumer acceptance is recorded in [the review ledger](GladeRaftQualification-ReviewCycle.md).
The [plan](GladeRaftQualificationPlan.md) and
[contract](GladeRaftAdoptionContract.md) control claims. Exact reviewed source
revisions and reports will be recorded in the review-cycle ledger.

## TDD and build evidence

1. The API consumer test was written against absent declarations: expected
   E0432 missing imports. This was declaration RED only.
2. Minimal std-only API declarations and an intentionally refusing harness were
   added. API `public_contract` compiles and passes **2 tests**; its recorder is
   a substitution/type witness, not a conforming application implementation.
3. `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml
   -p glade-raft-adoption-proof --test qualification` with the compatible PROTOC
   compiles and produces **0 passed, 11 failed**. Assertions fail because no
   retained applied response exists, create never accepts and the direct noop
   returns `Err(NotQualified)` instead of `Ok(None)`. This is behavioral RED.
   No application or RawNode driver is implemented in the Q0 checkpoint.
4. `proofs/raft-adoption/check.sh` passes its syntax/metadata architecture gate,
   two-source empty-allowlist process-global scan, token-aware conditional scope
   check and Rust formatting check. These structural passes do not mean the
   behavioral contract passes.

Build setup failures are separate evidence: offline dependency cache initially
lacked protobuf; missing protoc and the newer major-32 compiler also failed
before running tests. The bundled protobuf-build 0.14.1 macOS x86-64 compiler
prints `libprotoc 3.9.0` and runs on this arm64 machine. The exact supplied path
and repeatable commands are in [the fixture README](../proofs/raft-adoption/README.md).
Rust toolchain is 1.96.0. No build failure is counted as behavioral RED.

## Initial executable specifications

`proof/tests/qualification.rs` names RA-linked schedules for: ordered competing
creates; leadership versus home; dropped reply/retry after failover; changed
retry and principal namespaces; old-leader minority and post-move suffix fencing;
isolated successor lag and catch-up; ordered revocation/current disclosure;
retirement/exact historical retry; wrong scope/unsupported effect; two-voter
loss; contiguous application/noop/exact replay/gap/conflicting replay.

These are intended assertions pending implementation, not passing results. The
full plan retains unqualified bootstrap, crypto, metadata witnesses, separate
BeginMove/Activate, capacity, disk and production integration obligations.

## Q0 remediation evidence

The two independent reviews at root `581ef60a65bfebda8b39645aeb122f12c23828ee`
converged on one namespace-fixture collision. Remediation1 changes only the
qualification test source: Create sequence1 is reserved, namespace mutations
use sequence2 with explicit identity assertions, and an intentional Create-ID
reuse test requires conflict while retaining the original receipt.
The corrected specifications compile; API remains **2 passed** and the refusing
qualification provider remains behavioral RED: **0 passed, 12 assertion failures**.
Formatting passes. This is a test repair, not implementation or reviewer closure.

## Q1a executable results

The actual `raft 0.7.0` RawNode carrier now drives fixed voters1/2/3 (and the
negative two-voter fixture). Ready persists complete entries/hard state to the
named memory store before releasing messages; committed entries/noops are applied
in order. LightReady updates only hard-state commit, applies further entries and
advances application. No automatic ticks, sockets, sleeps, snapshots or real disk.

The deterministic Application implements the trusted CommittedMachine order/retry
contract. Movement additionally consumes a **private fixture readiness envelope**
inside the driver/application harness: the public numeric field cannot mint it,
exact command bytes remain unchanged, and the witness must match the preceding
application cut. First-time public apply without that witness refuses Move. This is not a
production movement port or proof of signed readiness; its boundary and call path
are in scope for the Code review.

The implementer added five readiness/validation/counterexample specifications and
observed **17 behavioral assertion failures** against the refusing application
before implementation. Two private codec tests also observed assertion RED against
an empty refusing codec. They now pass. Lane-owner rerun confirms **21 passing
tests**: 2 API consumer/type witnesses, 2 private-codec tests, 17 behavioral tests.

| Qualified slice | Concrete assertions |
| --- | --- |
| RA-002 partial | Competing known-scope creates produce one name binding/conflict; leader change preserves home, generation and payload. |
| RA-003/004 partial | Memory quorum/application before receipt; isolated minority no outcome; dropped reply plus failover/exact retry returns original receipt/index; changed bytes and deliberate Create-ID reuse refuse without overwriting history; principal namespaces independent. |
| RA-005 partial | Majority move increments generation; delayed old-generation command refuses; pre-cut exact retry preserves outcome; missing successor suffix refuses then real catch-up permits; forged future/current cut cannot mint readiness; queued intervening write invalidates captured readiness. |
| RA-006 partial | Ordered fixture revocation denies delayed new mutation, permits authorized historical disclosure, then withholds reply/outcome after disclosure revocation. Wrong scope, incarnation, generation, home, policy frontier and unauthorized fixture commands refuse. |
| RA-008/009 partial | Opaque external effect refuses; retirement fences new Create/Mutate/Move, retaining exact historical Create outcome. No real sink or durable tombstone claim. |
| RA-010/011 partial | Two voters cannot progress after either isolation; application handles noops/exact replay and rejects gaps/conflicting replay. This is not restart/disk/configuration qualification. |
| Counterexample | Executable check-then-local-append mutant admits old-generation work after a move; the real ordered path rejects the analogous delayed work. It is a test-only mutant, not a runtime mutation mode. |
| Private codec | Complete commands/readiness round-trip; truncated, trailing, unknown-version/action and invalid boolean frames refuse decode. Invalid committed fixture bytes stop the harness, not recovery/quarantine qualification. |

No full RA requirement is closed by these partial witnesses. Still unqualified:
authenticated bootstrap/conflicting mapping/private scope; metadata-only witnesses;
remote revocation freshness; bounded retention; separate BeginMove/Activate;
actual disk/restart/power loss; automatic elections and dependency RNG compliance;
learners/joint membership/snapshot/compaction; linearizable reads; genuine Glade
crypto/receipts, source/effect sinks and legacy activation/rollback.

## Verification and measured feedback

Commands from workspace root (PROTOC as in README):

```sh
cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml
proofs/raft-adoption/check.sh
cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings
```

All pass in the lane-owner run. The local architecture check covers two classified
libraries, declared manifest edges/traits/targets; the process-global check scans
5 owned Rust sources with **0 allowlist entries**; token checking/formatting pass.
No dependency allowlist, classification or process-global exception was loosened.
These checks do not certify transitive dependency code; the documented raft RNG
issue remains open. No unrelated member suite was required for this independent
proof with no production consumer edits.

Measured on Apple M3 Pro arm64, macOS26.6.2, Rust1.96.0
`ac68faa20 2026-05-25`; monotonic Python perf_counter around subprocesses:

| Measurement | Wall seconds | Profile |
| --- | ---: | --- |
| Cold target build plus tests | 9.349 | New empty CARGO_TARGET_DIR; existing downloaded registry and OS caches; locked/offline. Not a clean-machine/network measurement. |
| Warm Cargo build plus tests | 0.181 | Existing proof target, same test command. |
| Prebuilt test execution | 0.011 | Four executable test targets, sequential `--quiet`, including zero-test API unit target; no compilation/doc-test build. |

The cold command adds CARGO_TARGET_DIR pointing to a fresh temporary directory;
prebuilt paths were obtained via the same Cargo test command with
`--no-run --message-format=json`. These are single measurements at the pre-remediation 21-test revision
`5e81483f80b23a64c7914986fc4d31e1c3d9fd4f` of the bounded
fixture, not production latency/throughput or a feedback budget guarantee.

## Q1a remediation evidence

At root `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`, Code found P2-1:
public replay of a driver-attested Move compared absent private readiness against
the retained envelope, causing ConflictingReplay for an exact public command.
State independently returned GO; combined gate remained NO-GO.

The new `cluster::tests::public_replay_of_driver_attested_move_recovers_original_receipt`
first failed with `Err(ConflictingReplay { index: 3 })` versus the original accepted
Move receipt. The minimal public adapter now recovers existing-index canonical
command replay from retained history. It still rejects changed commands; new
indexes still use the witness-free public path and refuse unwitnessed Move.
Private complete-envelope conflicting-replay checks are unchanged.

Lane-owner rerun: **22 tests pass** (2 API, 2 codec, 1 actual-driver/public-interface
replay regression, 17 qualification cases). Clippy and local architecture/source/
process-global/format gates pass. Signatures, dependencies, readiness verification,
Ready ordering and all unqualified profiles are unchanged. The [Code re-verdict](GladeRaftQ1a-ReviewCode-1.md) independently closes the
counterexample and [State re-verdict](GladeRaftQ1a-ReviewState-1.md) verifies the
same revised tuple. The [ledger](GladeRaftQualification-ReviewCycle.md) records
bounded acceptance and the open production gates.

## Q2 contract checkpoint — behavioral RED

The proposed [persistence boundary](GladeRaftPersistenceContract.md) adds std-only
contract and disk implementation crates. Before adapter/integration implementation,
the reusable contract model passes one semantic test. The concrete disk scaffold
returns `NotQualified`: **8 compiling tests fail behaviorally**. The actual-driver
recovery target compiles: **1 negative passes, 6 positive/fault/reopen tests fail**;
the negative is initially vacuous against the refusing constructor and does not
prove quarantine. Existing 22 memory tests still pass.

The separate process-crash worker compiles. Its runner exits with the expected
failure before the acknowledged cut because `DiskStore::create_new` returns
`NotQualified`; no actual kill/recovery PASS is claimed at this checkpoint.
Commands use the README PROTOC, `-p glade-raft-disk --test conformance`,
`-p glade-raft-adoption-proof --test recovery`, and `--test process_crash --no-run`
plus `process-crash.py --worker <reported executable>`. Build naming errors were
corrected before behavioral RED and do not count as tests failing on semantics.
The structural architecture/process-global/source/format gates pass with no new
exception entries. Implementation awaits dual contract acceptance.

## Q2 contract remediation 1 — oracle RED/GREEN

Safety's crash-receipt counterexample was reproduced before correcting the oracle:
three Python oracle tests ran, two failed because the old index/payload/post-restart
agreement check admitted altered accepted home/generation and accepted-to-rejected
substitution. The corrected runner retains a complete canonical numeric original
receipt in the parent before SIGKILL and compares fresh-process lookup and retry
against it. The before-apply cut instead uses a separately defined complete expected
fixture receipt. **Four oracle/parser tests now pass** with `process-crash.py --self-test`.

Recovery Result diagnostics are retained directly. Warnings-denied all-target
Clippy and structural gates pass. Behavioral scaffold RED remains: disk 8 failures,
recovery 1 negative pass/6 failures on NotQualified, process worker refusal before
the kill cut. This corrects specification/oracle fidelity, not implementation.
