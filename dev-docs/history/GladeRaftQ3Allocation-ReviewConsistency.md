# Glade Raft Q3 implementation allocation — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `e1c260f6cf992f5d890c2ca454e2323b0d8e78b1..ccab267c6b23bfec7471923098944048a7959563`, particularly `dev-docs/GladeRaftQ3ImplementationAllocation.md`, DRAFT dated 2026-10-03, the injected lifecycle boundary, provider scaffolds, concrete consumer bindings and proposed dependency inventory. No algorithms or production activation.

**Baseline:** Root `e1c260f6cf992f5d890c2ca454e2323b0d8e78b1`; unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling documents and scoped sources were read with `git show` at their pinned revisions. Scoped working-tree diffs were empty.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling contract, architecture, dependency and evidence graph. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This verdict accepts the proposed allocation and compiling specifications on this axis only; it does not qualify Q3 behavior.

---

## 0. Evidence base

The four HEADs matched the exact review tuple at both start and end:

| Repository | Verified HEAD |
| --- | --- |
| Glade workspace root | `ccab267c6b23bfec7471923098944048a7959563` |
| glade | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| External Gyld | `ca04499a360d910fbf8ee2540ed446facd051b35` |

Read the supplied root rules, `AGENTS_GWZ.md`, the canonical Consistency prompt and `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`. No current-round peer prompt or report was accessed.

Reviewed:

- Allocation supplement, lines 1–112, including the exact package table, lifecycle ownership, carrier constraints, TDD selections, exit witnesses and checkpoint evidence.
- Accepted `GladeRaftConfigurationSnapshotContract.md`, §§1–7; qualification plan, §§1–6; adoption contract, §§1–4; persistence contract, §§1–7 and its crash-oracle/implementation amendments.
- `GladeBuildEntry.md`, library-boundary policy including LBT-001–012, package architecture, and architecture revision 3.
- Pinned Gyld declaration, including Records, StorageAdapter, Admission, Directory, ConformanceHarness and NodeAssembly ownership.
- Qualification evidence and review ledger, including accepted Q2 limits, Q3 accepted-through record and allocation-gate preparation.
- Entire scoped implementation diff: manifests, lockfile, architecture inventory, lifecycle declaration/compiler consumer, both new provider modules, eight disk specifications and all nineteen concrete consumer bindings.
- Shared store, membership, snapshot, joint-exit and typed-replay specifications.
- Cached raft-rs 0.7.0 source around configuration proposal/application, pending configuration eligibility, snapshot restore/fast-forward, MemStorage snapshot construction, Ready advancement and snapshot reporting.

Executed only allowed commands, using the specified cached `PROTOC` for carrier builds:

| Selection | Result |
| --- | --- |
| `-p glade-raft-q3-api` | 3 PASS |
| `-p glade-raft-disk --test v2_lifecycle` | 8 intentional failures against refusing providers; none ignored or filtered |
| `-p glade-raft-q3-spec --test configuration_snapshot` | 19 intentional NotQualified failures during provider construction; none ignored or filtered |
| Existing API, durability, disk, qualification, recovery and Create compatibility targets | 47 PASS |
| Adoption-proof library tests | 4 PASS; 2 existing explicit disk cases ignored |
| `proofs/raft-adoption/check.sh` | Architecture, explicit source boundaries, process-global and formatting PASS; 23 production Rust files, zero allowlist entries |
| All-target Clippy with `-D warnings` | PASS |

Together the preserved selected regressions total 51 PASS, matching the supplement. The two explicitly ignored Q2 disk witnesses and external process-kill tier were not rerun. No source files or Git state were changed; permitted test artifacts were confined to the existing ignored target area.

## 2. Invariant analysis

**Allocation and dependency direction.** The new normal disk-to-q3-api edge supplies the contract its V2 providers implement. The normal proof-to-q3-api edge supports its library-level session implementation. Concrete disk selection remains a development dependency of the carrier harness, and concrete Q3 composition lives under q3-spec integration tests. The contract package imports no carrier, filesystem provider or runtime implementation type. The graph remains acyclic, with no new package or third-party dependency.

The accepted contract’s earlier dev-only compatibility edge is explicitly replaced in the supplement’s exact dependency table. The supplement identifies the inventory changes as proposals requiring this gate; structural PASS is not represented as policy approval. No existing role is reclassified or behavioral guarantee weakened.

**Lifecycle completeness and authority.** `StoreLifecycle` contains both required halves, create and open, returning the existing store abstraction through a dyn-compatible boundary. Concrete paths remain owned by the adapter. Root construction cannot create directories or infer authority. The storage factory performs structural lifecycle work; host validation of initial fixture authority or an accepted committed existing-group learner join precedes creation. Unauthorized, wrong-group and uncommitted joins must produce neither a create call nor a learner file.

Restart requires dropping old handles, physically reopening with retained instances and independently trusted floors, validating every required image and reconstructing state before serving. Missing storage cannot silently become genesis. The optional floor preserves the accepted limitation: a wholly valid old journal is not intrinsically detectable without independent trusted input.

**Durable-history and recovery promises.** The supplement preserves one coherent image containing HardState, checkpoint, configuration, applied cut and complete suffix. Its exit matrix carries forward immutable committed history, legitimate uncommitted replacement, same-cut checkpoint immutability, checked bounds and revision arithmetic. Historical same-term votes remain valid after removal and cannot be cleared merely because current membership changed.

Whole-journal validation, partial-record quarantine, pre-write usable-prior behavior and post-write poisoning remain required. Real old-Q2 compatibility evidence is explicitly outstanding; the labelled incompatible byte fixture is not promoted into canonical compatibility proof.

**Carrier and application ordering.** Local carrier source supports the supplement’s distinction between accepted configuration application and refusal. Empty ConfChangeV2 is leave-joint; it is not a refusal mechanism. Invalid overlapping proposals can lose their original context when the carrier rewrites them, justifying admission checks before proposal.

The text retains persistence before application, message release and receipts for Ready and LightReady, privately validated incoming snapshot candidates, and prohibition of state mutation while Ready is outstanding. It also requires actual restoration, matching-term fast-forward and stale/recipient rejection as separate witnesses. MemStorage’s generated snapshot cannot alone supply the historical application checkpoint required by the contract.

**Joint authority, home independence and complete snapshots.** Both data-bearing majorities remain necessary during joint consensus. Promotion requires complete observed durable/application readiness retained in private evidence, with deterministic predecessor-cut revalidation. Membership does not rehome resources or advance generations. LeaveJoint rechecks current outgoing-only homes, retains refusals, and cannot retroactively accept an old key after movement or retirement.

The complete original applied prefix, configuration envelopes, typed outcomes, resources, names, tombstones and policy must survive snapshots and replay validation. The learner-add checkpoint must contain the recipient at its actual historical cut; later suffix replay remains mandatory. No discovered membership or plausible materialized map substitutes for retained history.

**Evidence honesty and test satisfiability.** Shared semantic functions and all nineteen names are unchanged; only their provider bindings changed. The new disk cases compile against concrete APIs. Their early NotQualified failures leave later assertions unexecuted, exactly as reported. The supplement requires additional actual carrier, semantic adversary and process-kill witnesses before acceptance and preserves the external complete-original oracle. It claims neither working algorithms nor Q3 qualification.

## 3. Risks and next action

The principal remaining risk is implementation fidelity: most normative assertions are currently unreachable behind refusing constructors. This is an explicit unfinished implementation state, not a hidden evidence gap at this allocation gate. Full-history retention, 16 MiB refusal, physical journal growth and startup costs remain experimental limits. Production crypto, transport, randomness, independent failure domains and power-loss certification remain deferred.

The next action is to merge this independent verdict with the other axis. Only after the required GO/GO allocation gate should Q3a begin actual adapter and host regression RED, followed by implementation and the complete staged evidence gates.
