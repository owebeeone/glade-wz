# Glade Raft Q3 Contract — SAFETY-AXIS REVIEW

**Review object:** Root diff `14c347612678e76affd561abd31728adac6a9043..ed243db983c485e46a27aa870ec745de16a56d7a`, principally DRAFT `dev-docs/GladeRaftConfigurationSnapshotContract.md` and compiling RED specifications under `proofs/raft-adoption`. Remediation round 1; contract gate only, no production activation or Q3 implementation qualification.

**Baseline:** Reviewed root `ed243db983c485e46a27aa870ec745de16a56d7a`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling root/member/external sources were inspected with pinned `git show`; scoped working-tree source/contract comparisons were empty. All four HEADs matched at start and end.

**Date:** 2026-10-03

**Axis:** Safety: attack permitted degraded paths, irreversible transitions, durable recovery, joint authority and consumer satisfiability. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero open P0, P1, P2 or P3 findings. The three initial Safety counterexamples are corrected for this contract/specification gate. This verdict permits staged implementation after the combined contract gate; it does not qualify the unimplemented algorithms.

---

## 0. Evidence base

Read root `AGENTS.md`, `AGENTS_GWZ.md`, the canonical renewed Safety prompt and the review-loop skill. The legitimate prior-round inputs were both initial reports and `GladeRaftQ3Contract-RemPlan-1.md`. No current peer prompt/report or SafetyClosure prompt/report was read.

Reviewed:

- Configuration/snapshot contract, lines 1–116, including all normative requirements and mandatory implementation witnesses.
- Entire q3-api/q3-spec package: public types and traits, compiler consumer, refusing providers, all membership/snapshot/store/joint-exit/replay conformance functions and all 19 behavioral consumer tests.
- Existing `proof/src/application.rs:92–147,150–282,292–312`, the new `proof/tests/q3_fixture_compatibility.rs`, manifests, lockfile changes, architecture inventory, source scanner, process-global inventory and README.
- BuildEntry; LibraryBoundaryAndTestingPolicy §§1–6; PackageArchitecture §§1–8; AdoptionContract §§1–4; QualificationPlan §§1–6; PersistenceContract and its crash-oracle/implementation amendments.
- Q3 evidence and the qualification evidence/review ledger, distinguishing accepted Q2 results from pending Q3.
- Architecture revision 3’s component allocation and A1–A5; pinned Gyld Admission, Policy, Records, StorageAdapter and NodeAssembly allocation.
- Canonical reconciliation clauses concerning discovery versus authority, physical checkout locks, creation-rooted governance, serving-hop disclosure and operator-approved placement; pinned member SubstrateV1/CrossNodeWritesPlan context.
- Local raft-rs 0.7.0 `raw_node.rs` configuration/advance APIs; `raft.rs:2564–2665,2743–2775`; storage snapshot/compaction operations and joint configuration grammar.

Independent permitted execution, with the specified PROTOC where applicable:

| Command/target | Result |
| --- | --- |
| q3-api/q3-spec `cargo test --locked --offline … --no-run` | PASS: all targets compile |
| q3-api tests | 2 PASS: exact Create fixture and exhaustive dyn replay consumer |
| adoption-proof `--test q3_fixture_compatibility` | 2 PASS: corrected Create and retained negative preconditions |
| q3-spec `--test configuration_snapshot` | 19 failures, all `NotQualified`; exit 101; none ignored |
| `proofs/raft-adoption/check.sh` | PASS: architecture, source boundaries, formatting; 21 owned source files, zero global exceptions |
| all-target Clippy with `-D warnings` | PASS |

Assertions beyond the first refusing-provider failure were inspected as specifications, not counted as executed safety evidence. No broad Q2 build or process-kill rerun was needed for this source-only correction. Nothing was edited.

## 2. Invariant analysis

### Prior-finding verification

| Initial finding | Independent verification on the corrected object | Disposition |
| --- | --- | --- |
| Safety P2-1: invalid shared Create preconditions | `conformance.rs:34–46` now supplies generation/home `0/0`, separately retaining selected `Action::Create.home`. The whole-command regression passes. The unchanged Application accepts the complete expected resource; nonzero generation/home still return `StaleGeneration`/`WrongHome`. Success helpers compare the complete Accepted resource. | Closed at contract/specification gate |
| Safety P2-2: outgoing home can become stranded through joint exit | Contract lines 41–43 require deterministic current-home validation at the actual LeaveJoint predecessor cut, retained `Refused(HomeInUse)`, unchanged entire configuration/version and no carrier configuration application. `joint_exit.rs` specifies direct and admission-race placement, joint restart, exact refused retry, qualified movement/retirement, stale-version refusal and a new successful exit intent. | Closed at contract/specification gate; actual replica execution remains mandatory |
| Safety P2-3: index replay cannot represent configuration results | `lib.rs:196–222` supplies complete typed application/configuration/noop results. The exhaustive dyn consumer compiles. `replay.rs` retains accepted and refused ConfigReceipts and original entries before checkpoint/install/restart, compares complete originals, rejects changed envelopes for every kind and distinguishes missing indexes. | Closed at contract/specification gate; actual retained-history replay remains mandatory |

**Joint authority and home eligibility.** I replayed the original outgoing-home sequence against the revised rules. Placement on outgoing-only node 3 remains allowed while joint, but the later exit must inspect that placement in its actual predecessor state and remain joint. Admission cannot bypass this ordered guard. Resolution cannot rewrite the old terminal refusal: a new authorized key is necessary. Learner evidence remains private, retained and checked against the complete predecessor cut; local availability cannot change deterministic application. Separate incoming/outgoing majorities remain required.

**Full history and typed replay.** The revised result shape represents every promised original result without abusing absence. Snapshot retention still includes complete commands, receipts, configuration envelopes, private readiness, noops, policy, names and tombstones. Missing/unapplied history is distinct from a real election noop. Full replay must match every materialized map and configuration; plausible independent lists cannot establish consistency. Current disclosure still governs historical outcome release.

**Publication and recovery.** Incoming snapshot state is reconstructed privately through its own cut before coherent publication. Synchronization precedes installation, application, message release and receipts. Ready/LightReady failure stops participation; post-write errors poison until reopen and remain unknown. Partial journal tails quarantine, committed history remains immutable, and legitimate uncommitted replacement remains legal. Historical same-term votes survive membership removal. Rollback detection honestly requires an independent floor.

**Boundaries and scope.** The new contract depends only on existing application/durability contracts. The explicitly proposed proof-to-q3-api development edge tests numerical fixture compatibility against the existing Application; it introduces no contract-to-implementation dependency or production composition. Store creation/open remain separate, exclusive and bound to validated authority; missing storage never grants genesis. The inventory additions preserve existing classifications and dependency restrictions. Source checks inspect owned Rust files regardless of disabled branches.

**Carrier compatibility.** raft-rs explicitly supports refusing a committed configuration change without calling `apply_conf_change`; pending configuration tracking follows applied progress. Snapshot recipient checks, stale rejection, fast-forward and fatal malformed-configuration paths justify the draft’s mandatory host validation and actual-carrier witnesses. Compiler success does not prove those witnesses.

## 3. Risks and next action

Implementation fidelity remains the principal risk. No Q3 host, V2 adapter or snapshot codec exists. GREEN fixture/compiler checks and RED refusing-provider tests prove neither dual quorum operation nor physical publication, semantic corruption rejection, compaction recovery or SIGKILL survival.

The mandatory implementation matrix retains those obligations, complete external original-result oracles, capacity refusal and the named process-crash limits. Complete-history growth and dependence on an external rollback floor remain explicit limitations.

The next action is to merge the independent renewed contract verdicts. Only combined GO permits staged Q3 implementation, beginning with actual adapter/host RED regressions and retaining the subsequent Code/State gate. The reviewed four-repository tuple remained unchanged.
