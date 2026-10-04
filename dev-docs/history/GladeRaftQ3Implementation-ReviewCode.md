# Glade Raft Q3 implementation — CODE-AXIS REVIEW

**Review object:** Q3 implementation and controlling DRAFT documents at root `b61197602e5594bdf89770bda069ce7d30fdb222`, pending implementation acceptance; no production activation.

**Baseline:** Root diff `ccab267c6b23bfec7471923098944048a7959563..b61197602e5594bdf89770bda069ce7d30fdb222`. Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling documents and member sources were read with pinned `git show`; implementation working-tree reads had an empty scoped diff.

**Date:** 2026-10-03

**Axis:** Architecture, interfaces, actual call paths, compatibility and complete qualification fidelity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 blocks; two P3 coverage findings remain. I pre-commit to GO on a revision that resolves P2-1 as specified and supplies the explicitly required witnesses identified in P3-1 and P3-2.

---

## 0. Evidence base

All four HEADs matched the exact tuple at both review boundaries. No source/report files were changed, no Git mutations were performed, and no current peer prompt/report was accessed.

Process and architectural sources inspected included root AGENTS.md, AGENTS_GWZ.md, review-loop SKILL.md, BuildEntry, LibraryBoundaryAndTestingPolicy, PackageArchitecture, candidate architecture revision 3, and the pinned Gyld allocation. Contract review covered AdoptionContract, PersistenceContract, QualificationPlan, ConfigurationSnapshotContract §§1–7, ImplementationAllocation §§1–5, ImplementationEvidence, and relevant qualification evidence/ledger sections. Pinned member substrate, cross-node-write and registry contract sources supplied compatibility context.

Implementation inspection covered:

- V2 lifecycle/publication, codec, journal and validation modules, including complete-frame bounds and committed-prefix transitions.
- Q3 machine, encoding, carrier grammar, recovery, Storage and Voter modules.
- Session routing/admission, physical restart, direct compaction and public trait methods.
- The nineteen shared consumer bindings and underlying store, membership, snapshot and typed-replay specifications.
- Owning admission, recovery, semantic, Ready/LightReady, snapshot, recipient and reconciliation tests; combined carrier tests.
- The Python SIGKILL runner and Rust worker, including the independently constructed joint-before-apply result.
- Locally cached raft-rs 0.7.0 Ready/advance API obligations.

Executed results, using the prompt’s exact cached PROTOC:

| Command | Result |
| --- | --- |
| `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace` | 113 passed, zero failed, four explicit tier ignores; nineteen shared Q3 cases all passed |
| Same manifest, `-p glade-raft-adoption-proof --lib -- --ignored` | Two Q2 cases passed |
| `proofs/raft-adoption/check.sh` | Architecture, process-global, explicit-source-boundary and formatting checks passed; 44 Rust files, zero exceptions |
| `cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --workspace --all-targets -- -D warnings` | Passed |
| `python3 proofs/raft-adoption/process-crash-q3.py --self-test` | Passed ten recovered-field mutations and missing/changed parent evidence |
| Q3 runner with `target/debug/deps/q3_process_crash-57c67e4971c8507c` | ACK, joint-before-apply and snapshot-before-apply SIGKILL cuts passed |
| Q2 runner with `target/debug/deps/process_crash-304030f150fc974e` | Both SIGKILL cuts passed |
| `python3 proofs/raft-adoption/process-crash.py --self-test` | Four tests passed |

Historical RED-to-GREEN evidence was inspected; historical failing revisions were not rebuilt. The new counterexample below is derived from the actual call graph, not claimed as an executed custom regression: custom source writes were prohibited.

## 1. Findings

### [P2-1] Direct compaction leaves a poisoned voter eligible to serve

**Location:** `proof/src/q3/session.rs:300–327`, particularly `node.publish(image)?` at line 326; callers `session/port.rs:199–211` and `session.rs:283–284`. Serving eligibility is checked at `session.rs:68–91`.

**Violated invariant:** ConfigurationSnapshotContract §5 explicitly forbids serving from a poisoned memory mirror and requires publication failures to stop voter participation. QC-002/QC-007 require the same failure boundary across snapshot publication and host lifecycle.

**Counterexample:** Construct the real disk-backed session with a one-shot fault gate around leader 1’s injected CheckpointStore, following the existing LightReady test’s gate pattern. Acknowledge Create at index 2. Obtain its complete checkpoint, arm `PartialWrite`, then call `session.install(checkpoint)`.

Compaction calls `Voter::publish`; V2 publication sets `poisoned=true`, writes a partial record and returns `IoUnknown`. Neither `compact_node` nor `install` records `node.failure`. The node remains a RawNode leader with its prior machine and `failure=None`.

Immediately call `outcome(create.request)` or exact `submit(create)`. Both return the original receipt from memory without consulting the poisoned store. Configuration lookup/retry and resource reads have the same bypass. The leader therefore remains a serving instance although physical reopen will quarantine its torn journal.

`AfterSync` produces the same eligibility defect with a complete unknown publication rather than a torn tail. Automatic checkpoint regeneration inside CatchUp shares this call path. Ready processing correctly marks failure in `drain`; direct compaction bypasses that guard.

**Impact:** The advertised stop-until-recovery boundary depends on which caller published the image. A failed compaction continues releasing outcomes from a voter whose durable admission is unknown or quarantined.

**Required correction:** Centralize publication-failure handling or explicitly mark the affected voter failed on direct compaction errors. Preserve the original error and retained evidence; require physical reopen and validation before read/retry service resumes.

**Closure test:** Add an actual-session regression covering all five publication faults during direct install and CatchUp regeneration. After each failed publication, assert the affected voter cannot serve receipts/resources or emit protocol messages. Then verify physical restart yields the documented prior/full/quarantine result and exact original outcomes where recovery succeeds.

### [P3-1] The “valid foreign snapshot” witness is internally mismatched

**Location:** `q3-api/src/conformance/snapshot.rs:112–117`; named consumer `snapshot_valid_foreign_binding_corrupt_state_and_capacity_refuse`; ImplementationEvidence line 45.

QS-001 explicitly requires a semantically valid foreign-group checkpoint. The current fixture clones group 70’s checkpoint and changes only `checkpoint.binding.group` to 71. Its application envelope still declares `[7,70,2]`, and its retained configuration intents retain group 70. It is consequently inconsistent with its own outer binding.

The implementation rejects foreign outer bindings before semantic replay, so this review does not establish an unsafe acceptance. The concrete consequence is overstated qualification: the evidence claims the required valid foreign checkpoint case passed, but the fixture supplies a different negative class.

**Correction and closure:** Retain the existing mismatch test. Add a separately encoded, internally coherent foreign checkpoint, with matching outer/envelope/history bindings, original cut/term, configuration and materialized evidence. Feed it to the concrete target and assert refusal without publication, reset or state exposure. Identify that witness accurately in the evidence.

### [P3-2] Required nested-joint and nonjoint-leave carrier admission witnesses are absent

**Location:** `proof/src/q3/machine.rs:49–69`; ConfigurationSnapshotContract QM-005; ImplementationAllocation §5 host-admission row.

The contract requires actual nested/wrong-leave cases. Source searches found `JointInProgress` and `NotJoint` only in declarations and implementation, with no tests asserting either result. Existing leave tests exercise valid exit, home refusal or stale-version refusal; those do not reach these branches.

These guards matter because an unsuitable configuration proposal can be rewritten by raft-rs, losing the retained private intent. Their code appears correct, but nominal joint/leave success does not qualify the required prevention path.

**Correction and closure:** Against the real session, submit a fresh correctly versioned EnterJoint while already joint, and a fresh correctly versioned LeaveJoint while stable. Assert the specific admission errors, unchanged durable log/configuration, absence of a terminal outcome for the rejected key, and successful subsequent legitimate configuration work. Keep these distinct from stale-version tests.

## 2. Invariant analysis

The principal durability attacks otherwise held. V2 validates every journal record and transition, holds its OS lock through recovery, refuses known-empty/Q2/missing storage without reset, and synchronizes complete recovered writes before admission. Same-term nonzero votes cannot change; configuration removal does not reinterpret historical vote validity. Committed suffix bytes remain immutable while actual higher-term reconciliation can replace uncommitted entries.

The checkpoint codec reconstructs original entries and typed results, then compares configuration and complete application evidence. It does not merely accept a checksum or recognizable label. Semantic adversaries cover payload/home/generation, names/tombstones, permissions/frontier, original commands/results/indexes and cut/term. Common committed history is compared before incoming snapshot installation.

Ready and LightReady paths persist before returning messages or exposing replies. The LightReady witness returns the outstanding Ready through the supported asynchronous advance API before stepping responses. Actual snapshot installation, matching-term fast-forward, stale rejection and missing-recipient rejection are separately observed.

Learner readiness comes from actual durable/applied state and complete evidence equality. Incomplete learner vote/pre-vote admission and restart authority are gated. Joint partitions require both majority sets; queued home placement produces identical retained exit refusals across replicas. Movement/retirement permits a new exit while preserving the old refusal.

The combined witness receives a learner-add snapshot plus a later original suffix, then exercises joint recovery, current disclosure, retirement and explicit exit. External kill oracles retain complete originals; the before-apply joint result is independently specified.

## 3. Risks and next action

The experiment retains its stated limitations: process-crash evidence, logical data-bearing voters, trusted numeric authority, full-history retention and manual campaigns. No power-loss, production transport/crypto, automatic-election or legacy/effect activation conclusion follows.

The next action is one scoped remediation: close P2-1 through an actual failing regression, add the two missing mandatory witness classes, update the evidence, and return a newly pinned tuple for independent verification.
