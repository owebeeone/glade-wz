# Q3 implementation — STATE-AXIS REVIEW

**Review object:** Root diff `ccab267c6b23bfec7471923098944048a7959563..b61197602e5594bdf89770bda069ce7d30fdb222`, actual Q3 implementation and Implementation gate; pending acceptance, no production activation.

**Baseline:** Root `b61197602e5594bdf89770bda069ce7d30fdb222`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling documents and member references were read through pinned `git show`; implementation reads were checked against an empty scoped working-tree diff.

**Date:** 2026-10-03.

**Axis:** Durable-state semantics, filesystem ordering, restart legality and adversity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, passes their regressions and relevant gates, and introduces no new blocking defect.

---

## 0. Evidence base

Process authority read: root AGENTS.md, AGENTS_GWZ.md, complete generated State prompt and review-loop SKILL.md. Controlling sources inspected: BuildEntry; LibraryBoundaryAndTestingPolicy; GladePackageArchitecture; Q3 ImplementationAllocation, ConfigurationSnapshotContract §§1–7, QualificationPlan, AdoptionContract, PersistenceContract, QualificationEvidence, ImplementationEvidence and qualification ledger. Architecture allocation was checked against root architecture notes, pinned external Gyld declarations and pinned Glade Substrate receipt/write clauses.

Implementation inspection covered V2 publication, codec, transition validation and whole-journal recovery; Q3 carrier grammar, snapshot encoding, deterministic Machine, startup recovery, Storage, Voter Ready lifecycle, session admission, lifecycle and controls; concrete consumer composition; lifecycle/parser, semantic, snapshot, LightReady, admission and recovery tests; both crash runners and Q3 worker. Particularly relevant locations are:

- `proof/src/q3/session/lifecycle.rs:4–149`.
- `proof/src/q3/session/port.rs:199–212`.
- `proof/src/q3/voter.rs:27–177`.
- `proof/src/q3/recovery.rs:45–160`.
- `proof/src/q3/machine.rs:77–310`.
- `disk/src/v2.rs:25–166` and its codec/journal/validation modules.

Local raft-rs 0.7.0 election, vote rejection and commit-by-vote code was inspected directly. No current peer prompt/report was accessed.

Commands used the specified cached PROTOC. Independent results:

| Verification | Result |
| --- | --- |
| Locked/offline default workspace tests | 113 PASS; zero failures; four explicit ignores |
| Explicit proof-library ignored tier | 2 PASS |
| Original configuration/snapshot consumers | 19/19 PASS, default-selected, concrete providers |
| Additional carrier consumers | 5 PASS |
| `check.sh` | Architecture/source/format PASS; 44 Rust files, zero process-global exceptions |
| Workspace/all-target Clippy, warnings denied | PASS |
| Q2 oracle self-test | 4 PASS |
| Q3 oracle self-test | PASS; ten recovered mutations and missing/changed parent evidence rejected |
| Q2 external SIGKILL runner | `write-ack`, `write-cut`: PASS |
| Q3 external SIGKILL runner | `ack`, `joint-before-apply`, `snapshot-before-apply`: PASS |

Cargo reported workers `process_crash-304030f150fc974e` and `q3_process_crash-57c67e4971c8507c`; those exact executables were used. All four HEADs matched at both boundaries; the final scoped diff was empty. Historical behavioral RED evidence is documented at refusing checkpoints; it was not independently rerun here.

## 1. Findings

### [P2-1] Restart repeatedly campaigns a stale first voter and cannot recover an available quorum

**Location:** `proof/src/q3/session/lifecycle.rs:76–101`, called by recovery at lines 72–74 and Restart at lines 147–149.

**Root cause:** Candidate selection uses the first eligible ID in the newest configuration. It checks local applied coherence but never compares candidates’ durable last-log term/index. After that candidate loses, the session provides neither an alternative campaign nor an election tick. Repeated Restart selects the same candidate.

**Credible reproduction:**

1. Start the actual V2 session; create the resource with home 2.
2. Add learner 4, complete actual catch-up, then enter joint incoming `[2,3,4]`, outgoing `[1,2,3]`.
3. Disconnect node 2.
4. Commit LeaveJoint through nodes 1, 3 and 4. They provide outgoing majority `{1,3}` and incoming majority `{3,4}`. Resource home 2 remains authorized.
5. Reconnect, then Restart or reopen all four genuine files.
6. The newest configuration selects candidate 2. Its original log lacks the committed leave entry, while nodes 3 and 4 have the longer log.
7. Actual RawNode vote checks reject candidate 2’s stale log. Removed node 1 cannot supply the needed authority. Drain finishes without a leader.
8. Subsequent Restart chooses 2 again. Reconnect has no leader to ping; CatchUp requires a leader; public controls expose no alternative campaign.

Increasing election terms does not cure the missing log. Commit-by-vote cannot supply the absent Entry. The existing removed-leader regression synchronizes every recipient and therefore misses this state.

**Violated invariant and impact:** Q3 contract §3 and RA-010/011 require lawful configuration/restart recovery. The selected manual-election profile must remain operable when sufficient complete authorized voters survive. This legal durable state strands retained outcomes and all further progress despite an available quorum. Automatic-election RNG deferral does not excuse the implemented manual restart path.

**Required correction:** Make deterministic recovery campaign a suitable authorized voter using actual durable log freshness and supported RawNode transitions. Handle failed attempts without leaving an inaccessible permanent state. Preserve explicit minority unknown/refusal behavior and terminal-term guards.

**Closure test:** Add the exact partitioned leader-removal sequence above against V2 stores and RawNodes, observe RED first, then require fresh reopen to establish an eligible leader, recover the complete original create/configuration receipts and Entry bytes, preserve home 2/generation 1, and accept a new mutation. Also verify repeated restart and genuine quorum loss without fabricated authority.

This counterexample is source-derived; no custom test source was written during this read-only review.

### [P2-2] An ahead-of-local checkpoint reports successful installation without validating its overlap

**Location:** `proof/src/q3/session/port.rs:199–212`.

**Root cause:** `install` checks internal checkpoint consistency, then filters recipients by `local_applied >= checkpoint.index`. If every local node is behind, the target list is empty and the method returns `Ok(())`. Comparison against retained local history occurs only inside `compact_node`, which is never called.

**Credible reproduction:** Session A acknowledges payload 11 at index 2. A separate valid same-group fixture B acknowledges payload 99 at index 2, then applies another ordinary entry at index 3. B’s checkpoint is completely valid under Machine::restore, including its original history and materialized maps. Calling `A.install(B_checkpoint)` returns success: A’s nodes are at index 2, so all are filtered out. The conflicting original index-2 history is never compared, and no checkpoint is published.

**Violated invariant and impact:** QS-005 and contract §4 require valid conflicting replay histories to quarantine; §5 requires durable recoverability before successful compaction. Internal self-consistency does not establish agreement with existing acknowledged history. This boundary silently accepts a conflicting candidate and reports installation success without performing installation.

The actual incoming MsgSnapshot path performs stronger overlap validation; the defect is in the separately exposed session installation boundary.

**Required correction:** Validate candidate overlap independently of recipient eligibility. Reject unsupported ahead-of-local installation explicitly, or perform a qualified actual installation. Never return successful installation when no eligible publication occurred.

**Closure test:** Use two concrete sessions with the sequence above. Require conflicting history to return Quarantined while preserving A’s exact files, original receipt and Entry. Separately test an internally valid, nonconflicting future checkpoint: require explicit refusal or verified durable installation, never silent success. Retain successful same-history compaction and restart coverage.

This is also a source-derived counterexample, not an executed custom regression.

## 2. Invariant analysis

The durability ordering attack otherwise failed. V2 validates before append, holds an exclusive handle throughout recovery/service, synchronizes successful publications, and poisons after publication begins. BeforeWrite preserves the prior usable image. Partial records quarantine without repair; complete uncertain records are revalidated and synchronized on reopen. Checkpoint, configuration, suffix and cursors occupy one frame.

Ready first publishes entries/HardState and any private snapshot candidate. LightReady commit-only publication precedes application and message release. Derived configuration/applied state is published before successful lifecycle return. Failure stops participation; it does not reinterpret uncertainty as noncommit. The LightReady witness uses the supported asynchronous append lifecycle.

Checkpoint validation replays every original index, compares complete typed outcomes, reconstructs configuration and compares materialized resources, reservations, permissions, frontier and retry evidence. Semantic adversaries exceed checksum or opaque-roundtrip checks. Committed common prefixes remain immutable; actual higher-term reconciliation replaces only an unapplied suffix.

Learner admission derives from retained accepted joins. Incomplete learner vote/pre-vote suppression survives reopen. Actual snapshot installation is distinguished from matching-term fast-forward, stale rejection and recipient exclusion. The combined case transfers the learner-add snapshot and later real suffix before promotion.

Both joint-majority partitions refuse progress. Direct and queued outgoing-home exits retain identical refusals across replicas; movement/retirement permits a new exit without rewriting the old outcome.

The SIGKILL parent retains complete original evidence. Joint-before-apply uses an independently specified complete expected receipt. These are genuine process-termination witnesses, with appropriately bounded claims.

## 3. Risks and next action

Append-only physical history, full-prefix startup costs, the 16 MiB refusal limit, and rollback undetectability without an independent floor remain explicit experimental limitations. Production crypto, transport, failure domains, automatic-election randomness and power-loss certification remain deferred.

The next action is one scoped TDD remediation patch for P2-1/P2-2, followed by this reviewer’s verification of both original counterexamples and the relevant Q3/Q1a/Q2 gates. Q3 implementation acceptance must remain pending.
