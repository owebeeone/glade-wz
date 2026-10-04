# Glade Raft Q3 implementation remediation 2 — STATE-AXIS REVIEW

**Review object:** Root diff `ccab267c6b23bfec7471923098944048a7959563..468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; actual Q3 implementation, V2 disk, RawNode membership/snapshots, full-history recovery and Implementation gate. Remediation round 2; acceptance pending, no production activation.

**Baseline:** Previous full-scope renewal `ce0876423e921cdd066106af9190ae7c6ec5d781`; corrected root `468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Pinned sources and the complete substantive correction diff were inspected; working-tree reads had empty scoped diffs.

**Date:** 2026-10-03.

**Axis:** Durable-state semantics, authorized joint recovery, restart legality, publication ordering and preservation of prior State guarantees. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero open or newly identified P0–P3 findings on this axis. The outgoing-only candidate correction preserves the prior State GO and satisfies its additional recovery obligations. Acceptance remains limited to the named private process-crash profile.

---

## Prior-finding closure table

IDs identify their originating report; the two Code P2-1 entries are distinct findings.

| Prior finding | Current State assessment |
| --- | --- |
| Renewal Code P2-1: joint recovery excludes fresh outgoing-only voters | **Verified resolved on State axis.** Independently executed the actual-file counterexample. Fresh outgoing voter1 wins through RawNode, original unknown Entry5 becomes committed, complete receipts/Entries survive two genuine reopen cycles, new work succeeds, and loss of either joint majority prevents acceptance. Originating Code closure remains that reviewer’s responsibility. |
| Initial State P2-1: stale first voter strands restart after joint exit | **Closure preserved.** The original home2, partitioned joint-exit regression passes. Fresh authorized voter3 recovers complete originals and progress; removed1 remains outside subsequent authority. The disconnected-candidate regression also passes. |
| Initial State P2-2: future checkpoint bypasses overlap validation | **Closure preserved.** Conflicting future history quarantines before publication; unsupported nonconflicting future installation explicitly refuses. Exact files/originals remain unchanged; ordinary compaction/restart succeeds. |
| Initial Code P2-1: failed direct compaction remains serving | **Closure preserved.** Direct install/CatchUp across all five actual V2 faults and malformed-success responses still stop service and queued source influence until physical reopen. |
| Initial Code P3-1: foreign checkpoint witness internally mismatched | **Closure preserved.** The coherent foreign-envelope/history witness and its unchanged-file rejection pass; the older outer-only mismatch stays separately labelled. |
| Initial Code P3-2: nested-joint/nonjoint-leave witnesses absent | **Closure preserved.** Correctly versioned requests reach JointInProgress/NotJoint, preserve complete durable state/files and produce no terminal outcome; subsequent legitimate work succeeds. |
| Prior State renewal GO | **Preserved.** No correction changes its publication, snapshot, replay, admission or compatibility boundaries; their full matrix was rerun successfully. |

The initial reports, originating closure reports, fresh round-1 reports and merged plans were legitimate prior-round inputs. No current Code-2 prompt/report was accessed.

## Changed-range analysis

The implementation delta since `ce087642` changes only `session/lifecycle.rs` and its existing `restart_freshness_tests.rs`. At lifecycle lines 87–90, outgoing voters are chained into incoming voters and deduplicated through BTreeSet before the existing filters/ranking. The correction adds three lines; it retains availability, nonfailure, local applied coherence, durable last-log term/index ranking, lowest-ID tie-breaking and terminal-term refusal.

The owning regression at `restart_freshness_tests.rs:220–455` adds the exact stale-incoming/fresh-outgoing counterexample, complete expected values, unknown-entry reconciliation, continued work, repeated physical reopen and independent majority-loss fixtures. It does not reconnect the old live leader before dropping it.

Other changes are historical report filing, RemPlan-2, evidence and ledger updates. Controlling contracts, package/dependency policies, manifests, lockfile, V2 implementation, deterministic Machine, snapshot codec, routing/publication paths and crash oracles are unchanged. No supported nonempty target cardinality was narrowed.

**Architectural classification:** This is a bounded candidate-enumeration correction within the existing manual campaign/filter/ranking path. No public boundary, authority contract, call graph or platform assumption changes. **No NEW ARCHITECTURAL root cause was identified.** The implementation object has used remediation round 2; this verdict does not authorize another architectural remediation round.

## 0. Evidence base

The canonical State-2 prompt was read completely before review. The intact previous review supplied process and full-scope source context: AGENTS.md, AGENTS_GWZ.md, review-loop authority, BuildEntry, LibraryBoundaryAndTestingPolicy, PackageArchitecture, AdoptionContract, PersistenceContract, ConfigurationSnapshotContract §§1–7, ImplementationAllocation/bootstrap disposition, QualificationPlan §6, architecture revision3 and pinned Gyld/member references. A scoped committed comparison confirmed those controlling sources unchanged.

Current inspection covered the complete implementation delta, RemPlan-2, prior Code renewal and originating closures, updated ImplementationEvidence and ledger. Relevant current ranges are lifecycle `1–170`, restart-freshness tests `1–455`, session serving/routing `68–192`, cached raft-rs campaign `raft.rs:1250–1315` and `quorum/joint.rs`. Previous inspection of V2 framing/recovery/transitions, complete semantic replay, Ready/LightReady, admission and all exit-matrix witnesses remains applicable to their unchanged bytes.

The saved compiling RED log `/tmp/glade-q3-rem2-outgoing-red.log` was inspected: the owning regression failed with `Err(NoQuorum)` versus `Ok(1)` after fresh recovery. This was behavioral assertion failure, not compiler failure. Historical source was not rebuilt; corrected behavior was independently executed.

All Cargo commands used:

`PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64`

| Executed verification | Independent result |
| --- | --- |
| Locked/offline test manifest `proofs/raft-adoption/Cargo.toml`, `-p glade-raft-adoption-proof --lib outgoing_only_fresh_voter` | **1 PASS**, zero failed/ignored; 34 unrelated cases filtered |
| Same manifest, `--workspace` | **121 PASS**, zero failures; four explicit tier ignores; zero default-target filtering |
| Original shared Q3 consumers within workspace | **19/19 PASS**, default-selected, zero ignored/filtered |
| Same manifest, `-p glade-raft-adoption-proof --lib -- --ignored` | **2 Q2 PASS** |
| `proofs/raft-adoption/check.sh` | Architecture/source/format PASS; **49 owned files, 0 allowlisted items, 0 debt, 0 permanent** |
| Locked/offline Clippy, same manifest, `--workspace --all-targets -- -D warnings` | Exit0 |
| Same test manifest, `-p glade-raft-adoption-proof --test process_crash --test q3_process_crash --no-run` | Both exact executables reported |
| `python3 proofs/raft-adoption/process-crash.py --self-test` | **4 tests, OK** |
| `python3 proofs/raft-adoption/process-crash-q3.py --self-test` | **PASS: 10 recovered mutations, missing/changed parent originals** |
| Q2 runner using reported `process_crash-304030f150fc974e` | **write-ack/write-cut SIGKILL and fresh-process complete-original PASS** |
| Q3 runner using reported `q3_process_crash-57c67e4971c8507c` | **ack/joint-before-apply/snapshot-before-apply PASS**, complete APP/CONFIG/ENTRY lookup/retry/typed replay |

Worker paths were under `proofs/raft-adoption/target/debug/deps/`. All four HEADs matched at start and end; final scoped source/controller diff was empty. No source/report writes, custom test additions or Git mutations occurred.

## 2. Invariant analysis

**Authorized candidate union.** Joint voter authority includes incoming and outgoing members; the carrier’s joint configuration uses their union for membership/campaign messages while requiring both majority results. The new BTreeSet only deduplicates candidate enumeration. It does not alter quorum weights, ConfState, log evidence or voting decisions. Learners never enter this domain. After LeaveJoint, outgoing membership disappears, so removed members remain excluded. Local membership/completeness checks still prevent an incomplete receiver from becoming a candidate.

**Original counterexample and unknown recovery.** The owning sequence commits learner4@2 and joint incoming `[4]`/outgoing `[1,2,3]`@3 before creating home4 at index4. Disconnect4 leaves its log at4 while outgoing nodes retain complete uncommitted mutation Entry5; no applied receipt is returned. Drop/open checks that imbalance before carrier construction. Candidate1 has the freshest durable log and lawfully wins both voting majorities. New-term protocol work commits the exact original Entry5, with the independently specified complete index5/home4/generation1/payload23 receipt. No counter or term assignment manufactures progress.

Complete create/configuration/unknown receipts and Entry bytes are compared through lookup, exact retry and typed replay. Every replica retains Entry5/result at a committed cut. New payload77 work succeeds, and a second genuine drop/open/recover preserves its complete original receipt/Entry and resource home/generation.

**Joint authority remains conjunctive.** Separate complete fixtures disconnect4 or disconnect2/3. The former removes the incoming majority; the latter removes the outgoing majority. New payload99 remains unknown, commit stays unchanged and every materialized resource retains payload77/home4/generation1. The original two shared joint-partition consumers also pass. An outgoing candidate’s eligibility therefore cannot substitute for either required data-bearing majority.

**Prior full matrix.** Actual create/open/locking, missing/empty/Q2/wrong-bound refusal and coherent foreign rejection remain passing. Publication faults preserve coherent checkpoint/configuration/suffix images, stop failed voters and produce justified prior/full/quarantine recovery. Ready/LightReady persistence precedes application/messages; malformed success cannot install an invalid memory mirror.

Full-history snapshot replay still compares typed originals and all materialized maps. Actual install, fast-forward, stale/recipient refusal, learner-add snapshot plus later suffix and legitimate uncommitted reconciliation remain distinct passing witnesses. Pending/lost configuration replies, deterministic stale-readiness/home refusals, movement/retirement resolution, current disclosure and explicit exit remain preserved.

All five genuine SIGKILL cuts passed complete external-original comparisons. Joint-before-apply retains its independent expected-result oracle. Preserved Q1a/Q2 regressions passed alongside the complete Q3 matrix.

## 3. Risks and next action

Full-prefix retention/startup cost, append-only physical history, the 16 MiB refusal boundary and rollback detection requiring independent trusted state remain explicit limits. Manual campaigns and APFS process-crash evidence do not establish automatic-election compliance, power-loss survival, independent physical failure domains or production crypto/transport/legacy/effect readiness.

The next action is the lane owner’s merge of this State GO with the independent Code-2 re-verdict on the same tuple, then recording the bounded qualification disposition. No production activation follows from this review.