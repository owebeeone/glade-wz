# Glade Raft Q3 contract — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `14c347612678e76affd561abd31728adac6a9043..fa1ff8b9fd0be53932300730ff925d0e41c76b1a`, principally `dev-docs/GladeRaftConfigurationSnapshotContract.md` and the compiling RED specifications under `proofs/raft-adoption`. Status: DRAFT contract gate; no Q3 implementation qualification or production ratification.

**Baseline:** Reviewed root `fa1ff8b9fd0be53932300730ff925d0e41c76b1a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` `ca04499a360d910fbf8ee2540ed446facd051b35`. Root controlling documents were inspected through pinned `git show` and working-tree reads checked against an empty scoped diff. Member controlling sources and the external declaration were read using their pinned SHAs.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling contract/design graph, internal invariants, boundary shape and specification satisfiability. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This verdict accepts the scoped DRAFT contract and specification package for implementation; it does not qualify membership, snapshots, V2 persistence or production behavior.

---

## 0. Evidence base

The four tuple HEADs matched the specified revisions both before and after review. The final scoped working-tree diff was empty. No source, contract or repository state was modified.

Read:

- Root `AGENTS.md`, `AGENTS_GWZ.md`, the supplied Consistency prompt and `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`.
- `GladeRaftConfigurationSnapshotContract.md`, lines 1–114, including all requirement and implementation-exit rows; `GladeRaftQ3ContractEvidence.md`, including commands, RED results and measurements.
- `GladeRaftAdoptionContract.md`, §§1–4; `GladeRaftPersistenceContract.md`, §§1–7 and its oracle/implementation amendments; `GladeRaftQualificationPlan.md`, §§1–6; the qualification evidence and review ledger, including the Q3 preparation entry.
- `GladeBuildEntry.md`; `LibraryBoundaryAndTestingPolicy.md`, §§1–6; `GladePackageArchitecture.md`, §§1–8; architecture revision 3, particularly its responsibility table and A1–A5.
- The pinned Gyld allocation, particularly Records, Admission, Policy, StorageAdapter, NodeAssembly and responsibility allocations.
- Canonical reconciliation clauses in Buy/build D-06/R7/R9/R16/Q12, WorkspaceDirectory §4/WD-8, DiscoveryModel §§0/3/7, Authz §§1/3a/3b/4a/7a, and pinned member SubstrateV1/CrossNodeWritesPlan sources.
- All new q3-api/q3-spec declarations, compiler consumer, refusing providers and membership/snapshot/store conformance functions; manifests, inventory and source-scan changes.
- Locally pinned raft-rs 0.7.0 `raw_node.rs` proposal/application/advance APIs; `raft.rs` pending-configuration handling, restore and configuration application; storage snapshot/compaction behavior and ConfState grammar.

Executed independently:

```sh
PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64 cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml -p glade-raft-q3-api -p glade-raft-q3-spec
proofs/raft-adoption/check.sh
```

The API compiler consumer passed. All 15 behavioral specifications compiled and failed at the deliberately refusing provider’s `NotQualified`, exit 101. Their subsequent assertions were inspected as specifications, not counted as executed evidence. The architecture, process-global, explicit-source-boundary and formatting gate passed: 19 owned production Rust sources, zero allowlisted items. Existing broader Q2 regression/process-crash evidence was inspected rather than unnecessarily rerun.

## 2. Invariant analysis

**Authority and canonical allocation.** I attacked the possibility that Q3 converts discovery, empty storage or voter eligibility into creation/configuration authority. Sections 1–3 explicitly prevent each conversion. The trusted principals are bound numerical fixtures, not unsigned production governance. Learner creation requires a validated existing-group intent, while genesis remains confined to initial assembly. The adopted M3-D proposal already identifies its qualification of Records’ source-commit exclusion; Q3 does not silently ratify that amendment or move external effects into Records. Storage retains physical publication responsibility and does not authenticate intents.

**Boundary and lifecycle satisfiability.** The proposed contract depends only on two existing contract crates. Public values contain no carrier, runtime or concrete disk types. `CheckpointStore` supplies meaningful coherent publication operations; the private `QualificationSession` represents replaceable experiment composition and declares its trusted controls. Separate concrete `create_new`/`open` lifecycle operations, exclusive creation, lock ownership, missing-store refusal and version refusal are specified. New inventory entries add reviewed proposals without relaxing existing roles or edges. The consumer and refusing implementations establish that the declared dyn boundaries compile.

**Committed history versus replaceable suffix.** Sections 2 and 4 distinguish immutable committed evidence from legitimate uncommitted suffix replacement. Checkpoint omission must be covered by retained complete history; the checkpoint and suffix form one coherent image. Original Entry bytes, application command/result pairs, configuration intents/results and noops survive compaction. Consequently, latest-state lookup alone cannot satisfy the contract. Structural store admission is explicitly insufficient for serving: complete semantic replay and overlapping committed-history comparisons remain host obligations. The opaque storage fixtures do not falsely claim to exercise that replay.

**Configuration and joint authority.** The two partition specifications isolate complementary failures: `[1,3]` can supply the outgoing majority alone; `[1,4]` can supply the incoming majority alone. Neither may return an application outcome during joint authority. Learner catch-up cannot be asserted by caller counters. A logged readiness envelope is checked against the actual predecessor state, so an intervening application entry produces a retained deterministic refusal. raft-rs expressly permits rejecting a committed configuration change without calling `apply_conf_change`; its pending-change condition follows the applied frontier. The draft’s refusal rule is therefore compatible with the inspected carrier lifecycle.

**Retry, removal and historical voting.** Configuration identity includes scope, group, principal and sequence, and exact retries precede stale-cut/readiness rejection. Changed intent bytes preserve the original outcome. Application admission’s optional early `RetryConflict` is explicitly distinguished from the existing committed-machine terminal rejection. Removing a current live resource home refuses until qualified movement or retirement; ordinary membership changes do not alter home/generation. The same-term historical vote exception preserves voting monotonicity after removal without treating that vote as membership authority.

**Snapshot installation and recovery.** The incoming-snapshot candidate resolves the otherwise impossible `checkpoint=S, applied=A<S` image: it is privately reconstructed through S before publication and exposed only after successful synchronized publication. Ready and LightReady ordering retains Q2’s persistence-before-release requirements. A snapshot excluding a newly admitted learner must be regenerated at an authorized cut containing that receiver, not have its old ConfState rewritten. The combined specification requires both an observed received snapshot cut and a later suffix. Full replay comparison covers resources, tombstones, names, policy, outcomes and configuration; valid-format cross-map adversaries remain mandatory implementation witnesses.

**Claim limits.** The draft accurately preserves unknown write outcomes, poisoning, conservative partial-tail quarantine, externally trusted rollback floors, the 16 MiB complete-record limit and reserved terminal term. It distinguishes actual SIGKILL from synthetic I/O faults and power loss. No RED failure, opaque roundtrip or compiler success is promoted into behavioral acceptance.

## 3. Risks and next action

The principal residual risk is implementation fidelity. Assertions after the initial refusal have not executed, and actual ConfChangeV2/snapshot handling, V2 framing, fault recovery and combined process-crash behavior remain unqualified. Complete-history snapshots also retain explicit storage/startup growth and capacity-refusal limits.

The next action is for the lane owner to merge the independent verdicts. Only a combined contract GO permits the staged Q3 implementation, beginning with actual adapter/host RED tests and retaining every mandatory exit-matrix witness before Code/State acceptance.
