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

## Q2 implementation — real disk, replay and controlled crash evidence

The corrected contract is accepted at `db2db3bba1bdbd931468949fffbd81d444044a3a`
after Consistency/Safety GO/GO. Implementation source awaits a fresh Code/State
settled-tree gate; no production adoption follows from GREEN.

The disk adapter observed **12 behavioral RED tests before implementation**:
the original eight and four additional malformed/checksummed-history cases.
All twelve now pass against actual files. Its four cohesive modules own locking/
publication, bounded CRC64 full-image codec, journal recovery and transition
validation. Every successful publish calls file sync_all; genesis and reopen
synchronize the parent directory, and reopen resynchronizes the complete recovered
file before admission. Partial records quarantine; post-write errors poison;
committed bytes/term/vote/commit invariants hold; uncommitted suffix replacement
remains legal. There are no added third-party dependencies.

The injected host's six initial positive/fault tests were RED before integration;
they now pass. Twelve recovery tests additionally exercise nonvacuous invalid
binding/bytes/prefix rejection, actual-disk malformed commands at committed and
uncommitted indexes, cross-voter committed conflicts, policy/tombstone/outcome
replay, private Move evidence/fences, old-leader suffix reconciliation, and real
AfterSync uncertainty. In the latter test an I/O error returns no receipt; the
persisted uncommitted proposal can later commit after restart, while prior receipts
stay identical. An error is demonstrably not proof of noncommit.

Two new capacity regressions observed RED: valid recovered u64::MAX term was
admitted, and a live campaign at that term panicked in the carrier's increment.
Initial exact-MAX refusal covered only the destination state. State review found
that campaigning from MAX-1 could publish the reserved term and prevent restart.
Remediation 1 closes that transition as documented below; general production
capacity and automatic-election behavior remain unqualified.

The live driver persists complete Ready entries/HardState before its memory
mirror. It uses advance_append to obtain LightReady without prematurely advancing
application, persists any commit-only change while preserving term/vote, then
applies both batches and advances the actual applied frontier. Failures halt the
voter's participation/serving and return no failed batch messages. Startup validates
all configured images/private entries and their common committed prefixes, replays
all committed history, and sets Config.applied before constructing the usable cluster.

The initial ignored real-disk LightReady case passed, but Code review found its
leader step occurred before returning the outstanding Ready, contrary to the
carrier contract. That passing schedule is withdrawn as qualification evidence.
The supported replacement and its RED/GREEN closure are recorded below.

The existing Voter declarations were moved as complete parser-identified items:
`rust-split explode` reconstruction was byte-identical; scoped memory tests stayed
GREEN after the move and before extending persistence. Orchestration, live storage
ordering and restart validation now have separate source files. No legacy/upstream
conditional compilation migration or unrelated refactor is claimed.

### Historical verification at initial implementation checkpoint ac69bbcc325c0946bbf215309bcce5edd3210db6

**48 default Rust tests pass**; the separately selected real-disk LightReady case
passed, for **49 executed Rust tests**. Code review subsequently disqualified
that LightReady witness; these are historical execution results, not acceptance. The default run intentionally ignores that case and
the externally driven process worker. **Four Python oracle/parser regressions pass.**
**Two actual SIGKILL/fresh-process recovery cycles pass**, with complete receipt
lookup/retry equality against the parent's pre-kill original, or the independent
before-apply fixture. All original 22 Q1a tests remain passing.

Architecture, token-aware explicit boundaries, format, process-global inventory
and all-target Clippy with warnings denied pass. Four libraries are classified;
13 owned source files are scanned with zero exception entries. The scanner does
not certify dependency code; raft-rs timeout RNG remains an open production gate.
Commands/tier separation are in the [README](../proofs/raft-adoption/README.md).

Machine/profile: Apple M3 Pro arm64, macOS26.6.2 build25G83, Rust1.96.0
(ac68faa20 2026-05-25). `diskutil info /Volumes/projects` identifies external
APFS disk5s1 (disk4s2 physical store); `df /private/var` and
`diskutil info /System/Volumes/Data` identify internal APFS disk3s1 (disk0s2).
Disk/LightReady fixtures use the project target on the former; tempfile host/kill
fixtures use the latter. No power interruption, controller-cache certification,
independent-machine quorum or production performance claim is made.

Partial RA trace: RP-001–012 supply bounded RA-003/004/005/006/009/010/011 witnesses.
Full RA-011 rollback protection without independent trusted state, production
failure domains/receipt promises, snapshots/compaction/configuration Q3, authentic
bootstrap/crypto, automatic elections/RNG and Glade legacy/effect exclusion Q4
remain open. Parser checksums do not authenticate malicious storage. The journal
retains unbounded records, limited to 16 MiB per complete image record.

### Q2 feedback measurements

Monotonic perf_counter around subprocesses, one run each, successful final
initial implementation source bytes at `ac69bbcc325c0946bbf215309bcce5edd3210db6`.
The LightReady measurement belongs to the unsupported historical schedule and
MUST NOT be used as accepted ordering evidence. “Cold” means fresh CARGO_TARGET_DIR with the existing
registry and OS caches; it does not mean a clean machine or registry download.

| Selected tier | Wall seconds |
| --- | ---: |
| Durability API cold build + model test | 1.584 |
| Durability API warm build + model test | 0.077 |
| Durability API prebuilt model execution | 0.004 |
| Full proof workspace cold build + 48 default tests | 11.527 |
| Disk warm build + 12 tests | 0.191 |
| Disk prebuilt 12-test execution | 0.051 |
| Host recovery warm build + 12 tests | 0.589 |
| Host recovery prebuilt 12-test execution | 0.467 |
| Explicit LightReady build + disk case | 0.116 |
| External two process-kill cycles | 0.212 |

No achieved latency budget is inferred from these samples. The small std-only
contract can be built/tested without Raft or disk; broader actual-I/O assurance
remains an explicitly selected tier. Metrics do not include the architecture
checker build. The implementation-review checkpoint pins these source bytes;
any later remediation must identify whether its measurements were repeated.

## Q2 implementation remediation 1 — revised acceptance witness

The initial Code/State gate at root
`ac69bbcc325c0946bbf215309bcce5edd3210db6` found two distinct P2 defects.
Both are accepted in [one remediation plan](GladeRaftQ2Implementation-RemPlan-1.md).
This section records verification, not self-closure: originating reviewers MUST
verify their own counterexamples on the revised settled tuple in the ledger.

**State RED → GREEN.** `q2_disk_campaign_cannot_publish_a_term_that_restart_refuses`
starts actual disk files with retained committed receipts, raises their legal
term to `MAX-1`, campaigns and reopens all files. Before correction it failed
with `CapacityExhausted` at startup. After correction it recovers each identical
prior receipt and leaves the persisted term at `MAX-1`, for both fixed voter
configurations. `q2_real_disk_reserved_term_is_refused_before_publication`
steps a real incoming terminal-term heartbeat: RED returned a heartbeat response
at `MAX`; GREEN returns `CapacityExhausted` before changing the stored image or
returning messages, and all files reopen. This fences peer-learned terms as well
as manual campaigning. Existing exact-MAX startup/live refusal tests still pass.

**Code RED → GREEN.** A test-only lifecycle guard inserted into the original
LightReady witness failed at its leader step with `state mutation while Ready
outstanding`. The corrected schedule persists the first Ready into disk and the
memory mirror, returns it using `advance_append_async`, then delivers exactly
one follower's persisted data acknowledgement. It delays the local persistence
notification, legally pings/collects a subsequent Ready, then returns that Ready
through the live `finish_ready`/`advance_append` helper. Ordered persistence
notification drains both records and produces an actual commit-only LightReady.
The pinned carrier documents this lifecycle in `raw_node.rs:471–475,617–645,
669–698` and `raft.rs:1033–1058` (raft 0.7.0 cached source).

The guard checks both refusal and permission for step/propose/campaign around
advance, and protects controlled mutations. Exactly one commit-only persist is
observed with unchanged term/vote and the new commit. Success applies the complete
independently expected receipt and recovers it after actual disk reopen. Failure
returns `Err(Io)` instead of any outgoing message vector, leaves application and
receipt unchanged, and retains the old stored commit. These are real RawNode and
disk paths; no fabricated LightReady substitutes for the carrier.

**Revised verification:** 49 default Rust tests pass; the two explicit ignored
real-disk unit cases pass (**51 executed Rust tests** total). Four Python oracle
self-tests and both actual SIGKILL/fresh-process recovery cycles pass. Architecture,
source-boundary, formatting, process-global checks and all-target Clippy with
warnings denied pass; 13 owned source files, zero allowlisted exceptions. The
normal run intentionally ignores the two explicit disk unit cases and the
external worker. The host recovery tier now contains 13 tests. No dependencies,
public interfaces, journal format, production consumer or allowlist changed.

The earlier cold/warm samples remain attributed to the initial checkpoint and
were not remeasured for this remediation. The revised acceptance counts and
ordering/crash evidence above supersede the unsupported historical LightReady
claim. Named machine/filesystem and process-crash limits remain unchanged.
