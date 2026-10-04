# Glade Raft Q3 implementation — CODE-AXIS REVIEW

**Review object:** Q3 implementation and controlling DRAFT documents at root `ce0876423e921cdd066106af9190ae7c6ec5d781`, pending implementation acceptance; no production activation.

**Baseline:** Implementation range `ccab267c6b23bfec7471923098944048a7959563..ce0876423e921cdd066106af9190ae7c6ec5d781`; renewal range `b61197602e5594bdf89770bda069ce7d30fdb222..ce0876423e921cdd066106af9190ae7c6ec5d781`. Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Member controlling sources were read with pinned `git show`; root implementation reads were checked against an empty scoped working-tree diff.

**Date:** 2026-10-03.

**Axis:** Architecture, interfaces, call graphs, compatibility and qualification fidelity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 recovery defect blocks. I pre-commit to GO on a revision that resolves P2-1 as specified, passes its regression and the affected gates, and introduces no new blocking defect.

---

## Prior-finding closure table

This is fresh renewal evidence, not the originating reviewers’ testimony.

| Prior finding | Renewal assessment |
| --- | --- |
| Code P2-1 — poisoned direct compaction remains serving | Verified corrected. Central publication handling stops the voter on store errors and malformed successful responses. Actual direct-install/CatchUp tests cover all five faults, stopped replies/resources/queued traffic, physical reopen and exact original recovery or torn-tail quarantine. |
| Code P3-1 — foreign fixture internally mismatched | Verified corrected. A separate coherent foreign envelope is independently replayed, its complete materialized tail compared, and its binding normalized back to the exact supported checkpoint. Target refusal preserves files and originals. The older outer-only mismatch is accurately labelled. |
| Code P3-2 — nested-joint/nonjoint-leave witnesses absent | Verified corrected. Correctly versioned requests reach `JointInProgress` and `NotJoint`; complete states/files remain unchanged, rejected keys have no outcome, and subsequent legitimate configuration work succeeds. |
| State P2-1 — stale first voter strands restart | Original counterexample verified corrected by the actual-file partitioned joint-exit regression; disconnected-candidate coverage also passes. General recovery fitness remains blocked by the distinct joint candidate-domain counterexample below. |
| State P2-2 — future checkpoint silently succeeds | Verified corrected. Common committed history is compared before recipient selection. Conflicting future history quarantines; a nonconflicting unsupported future cut explicitly refuses. Exact files/originals remain unchanged; same-history compaction/restart succeeds. |

## Changed-range analysis

The renewal implementation diff changes ten files: 1,024 insertions and 22 deletions. Production-shaped changes are confined to central `Voter::publish` response/error handling, stopped-source/target routing and snapshot callbacks, manual campaign selection, and direct-install overlap/no-recipient validation. Five owning test modules add seven tests; the retained conformance fixture receives an explanatory comment.

No public trait, manifest, dependency, role, allowlist, physical format or test-selection change occurs in this renewal range. The altered shared publication/routing call paths justify fresh full-scope review.

**Root-cause classification:** P2-1 below is a newly identified, bounded implementation defect in candidate enumeration within the existing manual-recovery strategy. It needs no new interface, architectural boundary or authority contract. It is **not a NEW ARCHITECTURAL root cause** for the remediation cap. Its counterexample extends the candidate-selection failure surface addressed by prior State P2-1.

## 0. Evidence base

All four HEADs matched the exact tuple at both boundaries. Final scoped source/controller diff was empty. No source/report writes or Git mutations were performed; no current peer or originating-closure prompt/report was accessed.

Read process authority, both initial reports and RemPlan-1; BuildEntry, library/package policies, AdoptionContract, PersistenceContract, ConfigurationSnapshotContract §§1–7, ImplementationAllocation, QualificationPlan and relevant evidence/ledger sections. Architecture checks used revision-3 allocations and pinned Gyld Records/StorageAdapter/Admission/Policy/NodeAssembly declarations. Pinned Glade substrate/write documents supplied retained compatibility context.

Implementation inspection covered V2 lifecycle, framing, journal and transition validation; Q3 carrier grammar, encoding, complete replay, recovery and Storage; session routing, admission, lifecycle and public port; Voter Ready/LightReady ordering; semantic, fault, admission, recovery, snapshot and remediation witnesses; concrete consumer composition; Python kill oracle and Rust worker. Particularly relevant locations are:

- `proof/src/q3/session/lifecycle.rs:4–167`, especially candidate enumeration at 82–111.
- `session.rs:98–185` and 300–337; `session/port.rs:199–223`.
- `voter.rs:20–190`; `recovery.rs:75–164`; `machine.rs:77–310`.
- Owning remediation modules, complete-history semantic tests and snapshot install/fast-forward/reconciliation tests.
- Cached raft-rs 0.7.0 campaign/vote handling, `raft.rs:1261–1310`, 1460–1505 and 1516–1576; joint voter union/containment in `quorum/joint.rs`.

Executed commands used the specified cached PROTOC for Cargo test/Clippy:

| Command | Result |
| --- | --- |
| `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace` | **120 passed**, zero failed; four explicit tier ignores. Original Q3 consumers **19/19**, additional carrier consumers **5/5**. |
| Same manifest, `-p glade-raft-adoption-proof --lib -- --ignored` | **2 passed**, zero failed. |
| `proofs/raft-adoption/check.sh` | Architecture, explicit-source-boundary and formatting checks passed; process-global guard: **49 files, zero exceptions**. |
| Same manifest, `cargo clippy --locked --offline … --workspace --all-targets -- -D warnings` | Passed. |
| `python3 proofs/raft-adoption/process-crash.py --self-test` | **4 passed**. |
| Q3 runner `--self-test` | Passed ten recovered-field mutations and missing/changed parent originals. |
| Q2 runner with Cargo-reported `process_crash-304030f150fc974e` | `write-ack` and `write-cut` SIGKILL/fresh-process checks passed. |
| Q3 runner with Cargo-reported `q3_process_crash-57c67e4971c8507c` | `ack`, `joint-before-apply`, `snapshot-before-apply` passed complete APP/CONFIG/ENTRY lookup/retry/typed replay. |

Documented historical behavioral REDs were inspected, not independently rebuilt. The new counterexample is source-derived; prohibited custom test/source writes were not performed.

## 1. Findings

### [P2-1] Joint recovery excludes the fresh outgoing voters needed to elect a leader

**Location:** `proof/src/q3/session/lifecycle.rs:82–87`, used by both fresh `recover` and physical `Restart`.

**Root cause:** Candidate enumeration takes only the newest configuration’s incoming `voters`. The later filter recognizes incoming **or outgoing** local authorization, but an outgoing-only ID never reaches that filter or freshness ranking. During joint authority, outgoing voters remain authorized Raft voters and may be the only candidates with sufficiently fresh logs.

**Legal reproduction:**

1. Start the real V2/RawNode session: initial election noop at index 1.
2. Commit AddLearner4 at index 2 and perform actual CatchUp.
3. With no live resources yet, commit correctly versioned EnterJoint targeting `[4]` at index 3. Outgoing voters remain `[1,2,3]`. This target is nonempty, canonical and within the declared universe; no minimum target cardinality forbids it.
4. Create a resource with home 4 at index 4 and retain its complete Accepted receipt and original Entry. Node 1 remains an authorized outgoing leader.
5. Disconnect 4. Submit a valid home-4 mutation at index 5. The outgoing nodes durably retain the longer suffix, but the missing incoming majority prevents commitment; submission remains unknown.
6. **Drop the session before live Reconnect**, then physically open all four retained files and invoke fresh `Q3Session::recover`. Thus the old leader cannot repair node 4 before recovery.
7. Every node has applied joint configuration at index 3. Node 4’s last log index is 4; outgoing nodes have index 5 in the same log term.
8. Recovery campaigns only node 4. Actual raft-rs freshness checks make every outgoing voter reject its stale log. Its self-vote supplies the incoming majority, but cannot supply the required outgoing majority.
9. Repeated Restart selects 4 again. No election ticks or alternative campaign control repair the state.

Both authority majorities and complete committed application data are available. Vote rejection/commit information cannot supply the absent uncommitted Entry. Higher election terms do not improve node 4’s last-log term/index.

**Violated invariant and impact:** ConfigurationSnapshotContract §3/QM-006 and RA-010/011 require lawful joint restart and retained-history recovery; AdoptionContract’s progress conditions are satisfied here. The implemented manual path permanently prevents service and further progress despite an available joint quorum. Automatic-election deferral does not excuse this host selection defect.

**Required correction:** Enumerate the complete authorized incoming/outgoing voter union, deduplicate it, then apply availability, local completeness, durable-log freshness and deterministic tie-breaking. Preserve exclusion of learners and members removed after LeaveJoint. Use supported RawNode campaigns without fabricating term, log or authority.

**Closure test:** First observe behavioral RED for this exact actual-file sequence. Fresh reopen must establish an authorized leader, recover complete original create/configuration receipts and Entry bytes, preserve home 4/generation 1, lawfully resolve the unknown mutation through real reconciliation/commit, and accept new work. Repeat physical Restart; verify genuine loss of either joint majority still prevents acceptance. Retain the original stale-voter-removal and disconnected-candidate regressions.

## 2. Invariant analysis

The remaining publication-boundary attacks failed. V2 validates before append, retains an exclusive handle, synchronizes successful images, poisons uncertain publication, and validates every journal record on reopen. Torn tails quarantine without repair; complete uncertain writes are synchronized before admission. Same-term historical votes survive removal, committed bytes remain immutable, and legitimate uncommitted suffix replacement remains possible.

Ready and LightReady persistence precede application/message release. Snapshot candidates remain private until coherent publication succeeds. Stopped voters cannot release queued source traffic or receive snapshot-result callbacks. The legal asynchronous LightReady witness exercises actual commit-only publication.

Checkpoint restoration replays every original index and typed result, then compares complete configuration and materialized resources, reservations, permissions, policy frontier and retry evidence. Semantic mutations, conflicting committed histories and future-cut refusal are covered. Actual snapshot installation, fast-forward, stale rejection and recipient rejection are distinguished; a real later suffix reaches the learner after its saved join snapshot.

Learner vote/pre-vote suppression and restart admission derive from retained authority/history. Both joint-majority partitions refuse progress. Direct/queued outgoing-home exits retain identical refusals across replicas; movement/retirement permits a new exit without rewriting prior outcomes.

All exit-matrix witness classes execute successfully. The new finding demonstrates that their existing joint-restart coverage does not cover the complete allowed candidate domain.

## 3. Risks and next action

Full-history retention/startup cost, the 16 MiB refusal boundary and rollback undetectability without an independent floor remain explicit experimental limits. Process-kill evidence does not certify power loss, production crypto/transport, independent failure domains, automatic elections or legacy/effect activation.

The next action is bounded TDD remediation of P2-1, followed by independent verification on a new settled tuple. Q3 implementation acceptance remains pending.