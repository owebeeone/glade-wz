# Glade Raft Q3 implementation — STATE-AXIS REVIEW

**Review object:** Root diff `ccab267c6b23bfec7471923098944048a7959563..ce0876423e921cdd066106af9190ae7c6ec5d781`; actual Q3 implementation, V2 disk, RawNode membership/snapshots, full-history recovery and Implementation gate. Acceptance pending; no production activation.

**Baseline:** Original implementation `b61197602e5594bdf89770bda069ce7d30fdb222`; corrected root `ce0876423e921cdd066106af9190ae7c6ec5d781`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling documents and member references were read through pinned `git show`; implementation working-tree reads were checked against empty scoped diffs.

**Date:** 2026-10-03.

**Axis:** Durable-state semantics, filesystem ordering, restart legality and adversity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero open or newly identified P0–P3 findings on this axis. This verdict accepts only the named private Q3 process-crash qualification profile.

---

## Prior-finding closure table

IDs are namespaced by originating axis. This fresh renewal independently verifies the corrections; it does not substitute for the separately required originating-reviewer closure reports.

| Prior finding | Renewal disposition and evidence |
| --- | --- |
| State P2-1: stale first voter strands restart | **Verified resolved.** `restart_freshness_tests.rs:1–218` executes the original home-2, learner4, joint234/123, disconnect2, leave-through1/3/4 sequence using actual files. Fresh reopen chooses voter3 rather than stale2 or removed1. Complete original application/configuration receipts and Entry bytes survive; home2/generation1 remains unchanged; a new mutation and repeated physical restart succeed. Genuine quorum loss produces no leader or mutation. The additional disconnected-candidate case preserves progress through available voters2/3. |
| State P2-2: ahead checkpoint bypasses overlap validation | **Verified resolved.** `future_install_tests.rs:1–110` constructs actual sessions with payload11 versus payload99 at index2 and advances the source checkpoint to3. Conflict returns Quarantined before publication. A coherent nonconflicting future checkpoint returns InvalidImage. Files, view, original receipt and Entry remain unchanged; ordinary same-history compaction and physical restart still succeed. |
| Code P2-1: failed direct compaction remains serving | **Verified correction effective on State axis.** `Voter::publish` now retains publication/response-validation errors as the voter’s failure. `install_fault_tests.rs:1–357` exercises direct install and CatchUp regeneration across all five actual V2 faults, plus malformed successful returns. Retained memory replies/resources and queued failed-source influence stop. Physical reopen establishes exact prior/full recovery or PartialWrite quarantine. |
| Code P3-1: foreign fixture is internally mismatched | **Verified required witness added.** `foreign_checkpoint_tests.rs:1–127` supplies matching foreign outer/envelope bindings, complete normal-entry history/results and materialized evidence. Its two-entry history contains no configuration-intent group field requiring translation. Target refusal preserves files and state. The previous outer-only mismatch remains separately labelled. |
| Code P3-2: nested-joint/nonjoint-leave witnesses absent | **Verified required witnesses added.** `configuration_admission_tests.rs:1–130` uses fresh correctly versioned intents, obtains NotJoint and JointInProgress, compares complete files/states/view, checks absent terminal outcomes and subsequently completes legitimate membership work. |

All owning cases above passed in this reviewer’s execution.

## Changed-range analysis

The original-to-corrected production changes are confined to four Q3 host modules. `session/lifecycle.rs:76–121` selects an available, nonfailed, locally coherent incoming candidate by actual durable last-log `(term,index)`, with deterministic lowest-ID ties. `session/port.rs:199–225` validates common committed history before recipient selection and refuses an unsupported future cut with no recipients.

`voter.rs:20–55` centralizes publication and malformed-success failure handling without installing an invalid returned State. `session.rs:113–184` suppresses delivery from/to stopped voters and snapshot-result callbacks to stopped sources. Five owning test modules and one explanatory consumer comment accompany these changes; evidence and remediation documents record the renewed call graph.

No public interface, manifest edge, dependency, classification, format, allowlist or test-selection relaxation changed in this remediation. Q1a/Q2 implementations remain preserved. The shared failure/routing changes justify this fresh full-scope renewal. **No NEW ARCHITECTURAL root cause was identified.**

## 0. Evidence base

All four HEADs matched the exact tuple at both review boundaries. Initial and final scoped working-tree diffs were empty. No source/report writes or Git mutations were performed. No current peer or originating-closure prompt/report was read.

Process sources included AGENTS.md, AGENTS_GWZ.md, the complete canonical State prompt and review-loop SKILL.md. Controlling reads included BuildEntry; LibraryBoundaryAndTestingPolicy; PackageArchitecture; AdoptionContract §§1–4; PersistenceContract §§1–7 and amendments; ConfigurationSnapshotContract §§1–7; ImplementationAllocation §§1–6 and bootstrap disposition; QualificationPlan §§1–6; qualification/implementation evidence and relevant review-ledger history. Pinned architecture revision3, Gyld allocation, Glade substrate receipt/write clauses, cross-node-write rules and discovery registry contracts supplied allocation and compatibility context.

Implementation inspection covered:

- V2 lifecycle/publication, bounded codec, journal recovery and transitions.
- Q3 Machine, carrier grammar, encoding, recovery, Storage and Voter modules.
- Session routing, admission, compaction, create/open/restart and all corrected ranges.
- Nineteen shared consumers and concrete composition; semantic, admission, recovery, snapshot/recipient/reconciliation, LightReady and remediation witnesses.
- Q3 SIGKILL parent/worker and Q2 oracle results.
- Cached raft-rs 0.7.0 Ready/advance APIs and snapshot restore/fast-forward/recipient checks.

Relevant inspected ranges include `disk/src/v2.rs:1–202`, codec `1–270`, journal `1–38`, validation `1–156`; `proof/src/q3/machine.rs:1–357`, recovery `1–174`, storage `1–119`, voter `1–207`, session `1–360`, lifecycle `1–167`, port `1–235`; snapshot tests `1–460`, recovery tests `1–329`, and Q3 worker `1–288`.

Cargo commands used exactly:

`PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64`

| Executed command/result | Independent outcome |
| --- | --- |
| `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace` | **120 PASS**, zero failed, four explicit tier ignores, zero filtered in the default targets |
| Same manifest, `-p glade-raft-adoption-proof --lib -- --ignored` | **2 PASS**, zero failed; 32 deliberately unselected default cases |
| `proofs/raft-adoption/check.sh` | Architecture PASS; process-global guard: **49 files, 0 allowlisted items, 0 debt, 0 permanent**; explicit-source PASS; formatting exit0 |
| `cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace --all-targets -- -D warnings` | Exit0 |
| Same test manifest, `-p glade-raft-adoption-proof --test process_crash --test q3_process_crash --no-run` | Both worker executables reported |
| `python3 proofs/raft-adoption/process-crash.py --self-test` | **4 tests, OK** |
| `python3 proofs/raft-adoption/process-crash-q3.py --self-test` | **PASS: 10 recovered mutations, missing/changed parent originals** |
| Q2 runner with reported `process_crash-304030f150fc974e` | **write-ack, write-cut SIGKILL/fresh-process PASS** |
| Q3 runner with reported `q3_process_crash-57c67e4971c8507c` | **ack, joint-before-apply, snapshot-before-apply PASS**, complete APP/CONFIG/ENTRY lookup/retry/typed replay |

Workers were under `proofs/raft-adoption/target/debug/deps/`. Historical compiling behavioral RED evidence and corrected GREEN records were inspected; historical failing revisions were not rebuilt. No custom regression source was written.

## 2. Invariant analysis

**Closed recovery grammar and single writer.** Exclusive creation never overwrites retained storage. Open never creates missing files, holds the OS lock during whole-journal validation, and synchronizes recovered file/directory before admission. Empty, torn, corrupt, incompatible and invalid-transition evidence refuses without repair. BeforeWrite preserves the adapter’s prior usable state; attached host failure nevertheless stops participation. Complete uncertain records recover only after validation; partial records quarantine. Trusted-floor tests explicitly distinguish detectable rollback from undetectable valid-old replacement without an independent floor.

**Monotonic durable history.** Term, commit, applied, configuration version and checkpoint cannot regress. Same-term nonzero votes cannot change or clear, including votes for removed voters. Committed original suffix bytes and same-cut checkpoints remain immutable. Uncommitted suffix replacement is legal and is demonstrated through actual higher-term MsgAppend reconciliation after a checkpoint.

**Persistence before exposure.** Ready publishes entries/HardState/private snapshot candidate coherently before memory installation, application or message release. LightReady commit-only changes are separately published before application. Derived applied/configuration state is durable before successful lifecycle return. Actual snapshot and LightReady tests exercise all five faults; direct publication now shares the stop boundary. Queued routing cannot revive a stopped voter.

**Semantic completeness.** Snapshot restore replays every original index and compares complete typed outcomes, configuration and materialized resources, names/tombstones, permissions/frontier, commands and retry evidence. Validly framed single-field adversaries quarantine. Common committed history is checked before direct compaction and protocol snapshot admission. Actual installation, matching-term fast-forward, stale refusal and recipient refusal are distinguished.

**Membership and home legality.** Retained bound joins authorize creation; an empty learner cannot campaign, vote/pre-vote, serve or establish home readiness. Restart reconstructs its admission. Promotion uses observed complete durable/application evidence and deterministic retained predecessor proofs. Both joint-majority partitions refuse commitment. Direct and queued outgoing-home exits retain identical refusals across replicas; qualified movement/retirement permits a new exit while preserving the old refusal.

**Crash and combined evidence.** Genuine SIGKILL tests compare fresh recovery against parent-held complete originals. Joint-before-apply uses an independently specified complete expected configuration result. The combined witness receives learner-add snapshot3 plus original suffix4 before promotion, then preserves policy, retirement and exact retry through joint restart and explicit exit. Preserved Q1a/Q2 regressions passed.

## 3. Risks and next action

Full-prefix application retention and append-only physical journals retain their stated storage/startup costs and 16 MiB refusal boundary. The evidence covers named process-crash/fault cuts, not exhaustive OS interleavings or power loss. Logical voters, trusted numeric authority, manual campaigns and the recorded production crypto/transport/randomness/legacy/effect deferrals remain unchanged.

The next action is the lane owner’s verdict merge on this exact tuple, including the separately required originating closures and fresh Code renewal. This State GO alone does not authorize production activation.