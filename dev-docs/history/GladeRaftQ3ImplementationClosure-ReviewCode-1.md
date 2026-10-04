# Glade Raft Q3 implementation remediation 1 — CODE-AXIS CLOSURE REVIEW

**Review object:** Corrected Q3 implementation and evidence at root `ce0876423e921cdd066106af9190ae7c6ec5d781`; remediation round 1, pending independent closure and fresh full-scope renewal. No production activation.

**Baseline:** Original reviewed implementation `b61197602e5594bdf89770bda069ce7d30fdb222`; allocation baseline `ccab267c6b23bfec7471923098944048a7959563`. Unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were inspected through the exact original-to-corrected diff and working-tree reads verified by an empty scoped diff.

**Date:** 2026-10-03

**Axis:** Originating Code reviewer’s focused verification of original counterexamples, changed call paths and qualification fidelity. Independent, adversarial, read-only. Other reviews run in parallel; nothing here relies on them. Filed verbatim by the lane owner.

**Verdict: GO** — Code P2-1, P3-1 and P3-2 are closed. No new P0–P3 finding or new architectural root cause was identified in this closure review. This verdict fulfills the original conditional pre-commit; it does not replace the separately required fresh full-scope gate.

---

## Prior-finding closure table

| Original finding | Disposition | Independently verified closure |
| --- | --- | --- |
| Code P2-1: direct compaction leaves a poisoned voter serving | **Closed** | Central publication handling stops the voter on store errors and malformed successful responses. Actual direct-install/CatchUp tests cover all five faults, block retained replies/resources and queued source influence, then physically reopen for exact recovery or quarantine. |
| Code P3-1: “valid foreign” witness internally mismatched | **Closed** | Separate foreign fixture consistently binds outer/envelope group 71, independently validates complete original history and materialized evidence, then verifies concrete rejection without publication. Original outer-only mismatch remains accurately labelled. |
| Code P3-2: nested-joint/nonjoint-leave witnesses absent | **Closed** | Fresh correctly versioned requests produce the specific admission errors; complete durable states/files remain unchanged, rejected keys have no outcome, and subsequent legitimate membership work succeeds. |

State’s two original findings and the merged plan were legitimate prior-round inputs. Their changed ranges and tests were inspected and executed for regression assessment; originating State closure remains that reviewer’s responsibility.

## Changed-range analysis

The original-to-corrected proof/evidence diff changes eleven files. It introduces five owning test modules and seven test functions. Implementation changes are confined to publication failure handling, message routing, manual campaign selection and direct checkpoint installation.

`proof/src/q3/voter.rs:28–50` now combines store publication and returned-state validation under one failure handler. Binding, revision and image equality remain required. A returned error, invalid successful response or checked revision failure records the exact error in `failure` before returning. Cached state changes only after successful validation.

`proof/src/q3/session.rs:115–124,172–184` excludes stopped sources and targets from delivery and excludes stopped sources from snapshot-result callbacks. This closes previously queued traffic as a continuation path after publication failure.

`session/lifecycle.rs:82–117` selects an available, eligible manual recovery candidate by actual durable last-log term/index, with lowest-ID tie-breaking. It retains membership, local applied coherence and terminal-term guards; it invokes real RawNode campaign.

`session/port.rs:199–222` compares incoming checkpoint history against retained committed evidence before filtering recipients. An unsupported future cut with no eligible publication now returns `InvalidImage`; conflicting overlap returns `Quarantined`.

The shared snapshot specification gains only an accurate explanatory comment. Evidence distinguishes historical counts, behavioral RED observations, coverage additions and final verification. Controlling contracts, accepted allocation, architecture policy, manifests and lockfile are unchanged.

These changes affect shared failure, mutation and routing call paths. Fresh full-scope renewal is therefore appropriate. They enforce existing obligations rather than introduce a new public boundary. No dependency, classification, process-global exception, conditional-compilation exception or production edge was added.

## 0. Evidence base

The complete canonical closure prompt, initial Code/State reports, merged RemPlan-1 and revised ImplementationEvidence were read. The intact initial review supplied previously inspected contract and implementation context. Changed implementation ranges and all five new owning test modules were inspected directly, including their assertions and physical reopen paths.

All four HEADs matched the corrected tuple at the beginning and end. Final scoped working-tree diff was empty. No source/report writes, Git mutations or current fresh-review/other originating-closure prompts or reports were accessed.

Commands used:

`PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64`

with locked/offline Cargo and the stipulated manifest.

| Verification | Independent result |
| --- | --- |
| Proof library, `--lib q3::` | 28 passed; zero failed/ignored; six unrelated tests filtered |
| Default `--workspace` | 120 passed; zero failed; four explicit tier ignores; zero filtered |
| Original configuration/snapshot consumers | 19/19 passed, default-selected |
| Proof library, `--lib -- --ignored` | Two Q2 cases passed |
| Worker `--test process_crash --test q3_process_crash --no-run` | Compiled; exact executable paths reported |
| `proofs/raft-adoption/check.sh` | Architecture, process-global, explicit-source and formatting checks passed; 49 files, zero exceptions |
| Workspace/all-target Clippy, `-D warnings` | Passed |
| Q2 oracle self-test | Four tests passed |
| Q3 oracle self-test | Passed ten recovered mutations and missing/changed parent evidence |
| Actual Q2 SIGKILL runner | Both cuts passed |
| Actual Q3 SIGKILL runner | ACK, joint-before-apply and snapshot-before-apply passed |

Reported and executed workers were `process_crash-304030f150fc974e` and `q3_process_crash-57c67e4971c8507c` under `proofs/raft-adoption/target/debug/deps`.

The evidence records actual pre-correction REDs for publication, queued-source influence, recovery freshness and future installation. Historical RED executions were not independently replayed; corrected counterexamples were independently executed.

## 1. Findings

No open or new finding remains within this originating Code closure.

## 2. Invariant analysis

**Original poisoned-compaction counterexample:** The new regression retains an acknowledged Create, complete application/configuration receipts and original Entry bytes, then arms genuine V2 publication faults during direct install or CatchUp checkpoint regeneration. `PartialWrite` now returns `IoUnknown` and records voter failure. Outcome and exact retry return unknown instead of the retained memory receipt; resource/configuration serving is unavailable. The cached state remains unchanged.

This result holds across all ten direct/CatchUp combinations. BeforeWrite physically reopens the exact prior state/file. Complete uncertain publications reopen the coherent checkpoint. PartialWrite quarantines. Recoverable cases reconstruct original typed application/configuration results and exact retry outcomes; CatchUp can subsequently complete real restoration.

The additional malformed-success test genuinely publishes through V2 before corrupting returned binding, revision or image. These responses stop service without installing malformed memory state. Its fourth case isolates host revision arithmetic with a deliberately altered cached revision; it does not claim physical V2 revision wrapping. Physical reopen recovers the valid published checkpoint and original outcome.

Queued failed-source traffic cannot influence another voter: the injected higher-term heartbeat leaves the follower’s durable state unchanged, and the queue drains. Failed snapshot senders also receive no lifecycle callback. Public compaction targets exclude failed voters, leadership selection excludes them, and Ready processing remains gated.

**Foreign checkpoint witness:** Both outer binding and encoded portable group become 71. The two-entry noop/Create history has no configuration-intent group field left at 70. Independent parsing checks every original Entry/result, checkpoint term, configuration and complete materialized tail. Normalizing the two binding declarations returns the exact original valid checkpoint. Concrete installation rejects `WrongBinding` while preserving files, images, view, receipt and typed replay. This satisfies the missing coherent-foreign class without adding foreign runtime support.

**Membership admission witnesses:** Stable LeaveJoint uses the current configuration version and returns `NotJoint`. Nested EnterJoint similarly returns `JointInProgress`. Complete State values—including suffix and revision—and physical files remain unchanged. Both rejected keys remain absent after legitimate AddLearner, EnterJoint and LeaveJoint operations. Stale-version refusal cannot explain these passes.

The remaining exit matrix was rechecked against retained source analysis and current executions:

| Matrix area | Corrected evidence |
| --- | --- |
| Lifecycle/compatibility | Exclusive create/open, locks, real Q2 preservation, missing/empty/wrong-bound refusal and coherent foreign rejection pass |
| Learner/joint authority | Actual catch-up, incomplete vote/pre-vote/restart exclusion, separate majorities and home-exit refusals pass |
| Configuration interruption | Pending unknown, original typed retry, joint/leave restart and durable-freshness recovery pass |
| Snapshot semantics/suffix | Full replay/map adversaries, actual install/fast-forward/rejection, later suffix and real uncommitted reconciliation pass |
| Publication/crash | Ready/LightReady and direct-compaction faults, physical recovery, full external SIGKILL oracles pass |
| Bounds/rollback/combined | Capacity and term guards, independent floors, combined learner snapshot/suffix/joint/policy/retirement/exit pass |

The future-install regression additionally verifies conflicting ahead history quarantines before publication, nonconflicting unsupported future installation refuses explicitly, and ordinary same-history compaction/restart still succeeds. Recovery-freshness regressions preserve complete originals and home/generation, restore progress through lawful campaigns and retain unknown behavior under genuine quorum loss.

## 3. Risks and next action

The existing experimental limits remain: APFS process-crash evidence, logical voters, trusted numeric authority, retained full history, manual campaigns and rollback detection only with independent floors. Production crypto/transport, power-loss certification, automatic-election compliance and legacy/effect activation remain outside this verdict.

The next action is to file this closure verbatim and merge it with the separate originating State closure and fresh Code/State renewal on the same corrected tuple. Q3 acceptance remains the lane owner’s aggregate decision.