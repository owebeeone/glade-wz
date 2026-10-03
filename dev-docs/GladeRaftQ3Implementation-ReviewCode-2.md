# Glade Raft Q3 implementation — CODE-AXIS REVIEW

**Review object:** Actual Q3 implementation, V2 disk, RawNode membership/snapshots, full-history recovery and implementation evidence at root `468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; implementation remediation round 2, pending aggregate acceptance. No production activation.

**Baseline:** Full implementation range `ccab267c6b23bfec7471923098944048a7959563..468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; this re-verdict’s delta is `ce0876423e921cdd066106af9190ae7c6ec5d781..468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`. Unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Pinned member sources and exact root diffs supplied source context; working-tree reads were verified against empty scoped diffs.

**Date:** 2026-10-03.

**Axis:** Architecture, interfaces, call graphs, compatibility and qualification fidelity; originating verification of the renewed Code counterexample. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — renewal Code P2-1 is closed; zero remaining or newly identified P0–P3 findings on this axis, and no mandatory witness gap remains. This fulfills my prior conditional pre-commit for the named private Q3 process-crash profile.

---

## Prior-finding closure table

IDs are namespaced by originating review.

| Prior finding | Disposition and independent evidence |
| --- | --- |
| Renewal Code P2-1 — joint recovery excludes fresh outgoing-only voters | **CLOSED.** The exact physical-reopen counterexample passes against real V2 stores and RawNodes. The complete authorized voter union participates in candidate selection; original receipts/Entries, unknown mutation recovery, new work, repeated physical reopen and both joint-majority losses are verified. |
| Initial Code P2-1 — failed direct compaction remains serving | **Closure preserved.** Central publication handling, stopped-source/target routing and malformed-success validation are unchanged. Direct-install/CatchUp tests covering all five faults and physical recovery passed again. |
| Initial Code P3-1 — foreign fixture internally mismatched | **Closure preserved.** The coherent foreign-envelope/history witness and unchanged-file refusal passed again; the older outer-only mismatch remains accurately labelled. |
| Initial Code P3-2 — nested-joint/nonjoint-leave witnesses absent | **Closure preserved.** Correctly versioned requests reach the specific admission guards, preserve complete files/state and permit subsequent legitimate configuration work. |
| Initial State P2-1 — stale first voter strands restart | **Correction preserved and independently exercised.** The original partitioned joint-exit/reopen and disconnected-candidate regressions pass. The broader candidate-domain defect raised during renewal is now corrected without weakening their checks. |
| Initial State P2-2 — future checkpoint silently succeeds | **Closure preserved.** Conflicting overlap quarantines before publication; unsupported nonconflicting future installation explicitly refuses. Exact originals/files and successful same-history compaction/restart remain covered. |

The initial reports, originating round-1 closures, fresh round-1 reports and merged plans were legitimate prior-round inputs. No current State-2 prompt/report was accessed.

## Changed-range analysis

The substantive implementation delta contains **three added lines** in `proof/src/q3/session/lifecycle.rs:87–90`: incoming voters are chained with outgoing voters and deduplicated through `BTreeSet` before the existing filters and ranking.

Availability, failure, local authorization/completeness, actual durable last-log freshness, deterministic lowest-ID tie-break and terminal-term guards remain intact. The same supported `RawNode::campaign` call is used. No election ticks, fabricated terms/logs/counters or target-cardinality restriction were introduced.

One 237-line owning regression extends the existing `restart_freshness_tests.rs` module to 455 lines. Evidence and RemPlan-2 describe the correction; the other root delta consists of prior-round filing and review-ledger history.

There is no change to public contracts, call graph, mutation/publication boundaries, V2 format, deterministic replay, Storage, crash runners, manifests, lockfile, classifications, dependency policy, process-global exceptions, platform scope or default test selection.

**Architectural classification:** This remains the bounded candidate-enumeration correction identified in my previous review. **No NEW ARCHITECTURAL root cause was identified.** The patch does not exceed the continued-review classification or trigger the architectural remediation cap.

## 0. Evidence base

The complete canonical Code-2 prompt was read before work. Intact prior review context supplied the controlling contracts, architecture allocations and full-scope implementation analysis. This re-verdict inspected RemPlan-2, prior filed reports, the complete substantive delta, updated implementation evidence, review-ledger delta and historical behavioral RED output.

Important ranges inspected are:

- `session/lifecycle.rs:76–120`, including voter-union enumeration and unchanged ranking/campaign.
- `session/restart_freshness_tests.rs:220–455`, including exact setup, pre-recovery durable imbalance, retained complete originals, two genuine reopen cycles and independent majority-loss executions.
- Preserved central publication handling in `voter.rs:20–55` and direct-install validation in `session/port.rs:199–223`.
- Previously reviewed V2 framing/journal/transitions, complete Machine/recovery/Storage paths, Ready/LightReady, semantic/snapshot/admission witnesses, concrete consumers and external kill oracles.

Controlling documents—including ConfigurationSnapshotContract §§1–7, ImplementationAllocation, AdoptionContract, PersistenceContract, QualificationPlan and architectural policies—are unchanged across this delta. Their prior pinned source analysis remains applicable.

The RED log `/tmp/glade-q3-rem2-outgoing-red.log` records a compiling behavioral failure: fresh recovery returned `Err(NoQuorum)` instead of `Ok(1)` at the exact outgoing-only recovery assertion. I inspected that record; I did not rebuild historical failing source or claim an independent historical RED execution.

Cargo commands used the canonical cached PROTOC and `--locked --offline --manifest-path proofs/raft-adoption/Cargo.toml`.

| Executed verification | Independent result |
| --- | --- |
| Proof library filter `outgoing_only_fresh_voter` | **1 passed**, zero failed/ignored; 34 intentionally filtered |
| Default `cargo test … --workspace` | **121 passed**, zero failed; four explicit tier ignores; zero default-target filtering |
| Original configuration/snapshot consumers | **19/19 passed**, concrete providers, default-selected |
| Additional carrier consumers | **5/5 passed** |
| Proof library `--lib -- --ignored` | **2 Q2 cases passed** |
| Worker `--test process_crash --test q3_process_crash --no-run` | Both exact executables reported |
| `proofs/raft-adoption/check.sh` | Architecture/source/format passed; **49 Rust files, zero process-global exceptions** |
| `cargo clippy … --workspace --all-targets -- -D warnings` | Passed |
| Q2 oracle `--self-test` | **4 tests passed** |
| Q3 oracle `--self-test` | Passed ten recovered-field mutations and missing/changed parent originals |
| Q2 external runner | **write-ack and write-cut SIGKILL/fresh-process checks passed** |
| Q3 external runner | **ack, joint-before-apply and snapshot-before-apply passed** complete APP/CONFIG/ENTRY lookup/retry/typed replay |

The executed workers were Cargo-reported `process_crash-304030f150fc974e` and `q3_process_crash-57c67e4971c8507c` under `proofs/raft-adoption/target/debug/deps`.

All four HEADs matched at start and end. Final scoped source/controller diff was empty. No source/report writes or Git mutations were performed.

## 2. Invariant analysis

**Original outgoing-only counterexample.** The owning regression reproduces the precise legal sequence: noop1; accepted AddLearner4@2 and real catch-up; accepted joint incoming `[4]`/outgoing `[1,2,3]`@3 before any live resource; complete Accepted home4 Create@4; disconnect4; valid mutation@5 remaining unknown while outgoing nodes durably retain its original Entry.

The session is dropped **before live Reconnect**. Actual file loads establish that outgoing logs end at 5 and incoming4 ends at 4 before constructing recovered carriers. Thus the test cannot pass by allowing the former leader to repair the stale incoming voter.

Union enumeration selects fresh outgoing-only voter1. Actual RawNode voting and new-term work establish authority through both majorities and commit the exact original unknown Entry5. Complete create/configuration receipts and Entries match lookup, exact retry and typed replay. The mutation’s full expected receipt was specified before dropping the session; recovery cannot invent its own oracle. Every replica retains the original Entry5/result at a committed cut.

New accepted payload77 preserves home4/generation1. A second genuine drop/open/recover preserves all original receipts/Entries and the new receipt/replay. Separate executions disconnect4, losing the incoming majority, or disconnect2/3, losing the outgoing majority. Both leave subsequent mutation unknown, preserve the committed cut and retain every materialized Resource. Candidate union therefore restores eligibility without weakening joint commitment.

**Authority and lifecycle attacks.** Selection remains restricted to the newest validated incoming/outgoing union and then to locally authorized, nonfailed, available, complete instances. Learners and post-exit removed voters are excluded. Existing same-target joint coverage exercises overlapping voter sets; deduplication does not alter authority. Terminal-term refusal and actual-log freshness remain enforced.

**Preserved full matrix.** Lifecycle/compatibility tests retain exclusive create/open, held locks, missing/empty/Q2/wrong-bound refusal and coherent foreign rejection. Learner tests retain real catch-up, incomplete vote/pre-vote/restart admission and home-readiness exclusion. Configuration interruption retains pending unknown, original accepted/refused outcomes and typed noop/configuration replay.

Snapshot semantic validation still reconstructs every original index and compares complete outcomes, configuration and materialized maps. Actual install, fast-forward, stale/recipient rejection, saved learner-add cut plus later suffix and real uncommitted reconciliation remain distinct passing witnesses.

Publication and Ready/LightReady fault tests preserve coherent cuts and stop messages/replies until physical recovery. Bounds, historical removed-voter votes and independent-floor rollback honesty remain passing. All five genuine SIGKILL cuts compare fresh recovery with parent-held complete originals; joint-before-apply retains its independently specified expected result. Combined snapshot/suffix/joint/policy/retirement/exit and all preserved Q1a/Q2 regressions passed.

These source and executed checks substantiate closure; the drafter’s GREEN assertion alone was not used as acceptance.

## 3. Risks and next action

The experiment retains full-history storage/startup costs, append-only physical journals, the 16 MiB refusal boundary and rollback detection requiring independent trusted state. Named process-kill/fault witnesses do not certify power loss or exhaustive OS interleavings. Production crypto/transport, independent failure domains, automatic-election compliance and legacy/effect activation remain deferred.

The next action is the lane owner’s Code/State verdict merge on this exact tuple and filing of accepted-through evidence if both axes GO. This verdict authorizes no production activation.