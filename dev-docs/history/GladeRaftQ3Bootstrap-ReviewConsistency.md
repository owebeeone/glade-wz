# Glade Raft Q3 learner-bootstrap clarification — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `c6beda7b2c94fef4060f7bc9a069b876fd2aa7d9..d4589feb02ad86b92f58686da35c8a216c370c44`: bounded learner-bootstrap amendment in `dev-docs/GladeRaftQ3ImplementationAllocation.md`, `GladeRaftQ3Allocation-RemPlan-1.md`, carrier-only `proofs/raft-adoption/proof/tests/q3_seeded_snapshot.rs`, and the corresponding qualification-ledger entry. Clarification pending renewed review; no algorithm acceptance or production activation.

**Baseline:** Root `c6beda7b2c94fef4060f7bc9a069b876fd2aa7d9`; unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Root controls were read with `git show d4589feb02ad86b92f58686da35c8a216c370c44:…`; Gyld controls with its pinned SHA. Uncommitted Q3 algorithms were excluded.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling contract, architecture, lifecycle and evidence graph. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This accepts the bounded clarification on this axis; it does not qualify learner implementation, application validation or physical persistence.

---

## 0. Evidence base

All four HEADs matched the required tuple at both review boundaries:

| Repository | Verified HEAD |
| --- | --- |
| Glade workspace root | `d4589feb02ad86b92f58686da35c8a216c370c44` |
| glade | `90dc1a60981185fa26ae5bfafbbb5377c12a413b` |
| glade-discover | `52ea2d118f45d9e7c3d9a789310dd5d669958851` |
| External Gyld | `ca04499a360d910fbf8ee2540ed446facd051b35` |

Read the supplied standing rules, pinned root `AGENTS.md`, `AGENTS_GWZ.md`, canonical Consistency prompt, review-loop skill, and QualificationPlan §6. No current-round peer prompt or report was accessed. Prior allocation Consistency/Safety reports were read as legitimate historical inputs, without adopting their conclusions.

Inspected:

- Allocation supplement, lines 1–119, especially revised lifecycle paragraphs 39–45, carrier obligations 55–64, and unchanged exit matrix 88–101.
- Entire RemPlan-1 and bounded diff, including the ledger’s bootstrap-remediation entry.
- Entire carrier regression, lines 1–55.
- ConfigurationSnapshotContract §§1–7, particularly lines 19–25, 35–43, 47–65 and its implementation exit matrix.
- QualificationPlan, AdoptionContract, PersistenceContract and amendments, QualificationEvidence, and relevant accepted-through ledger records.
- BuildEntry, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, architecture revision 3, and pinned Gyld ownership declarations.
- Pinned q3-api lifecycle/image declarations; refusing Q3Session and V2 store/factory controls; owning package manifests; shared combined snapshot/learner specification, especially `conformance/snapshot.rs:165–220`.
- Cached, manifest-pinned raft-rs 0.7.0 `raft.rs:2530–2650` and `storage.rs:242–278`, covering incoming snapshot handling, recipient membership, matching-term fast-forward, restoration and seeded storage state.

Used inspection commands only. No writes, Git mutations, builds, or mutable current implementation targets were run. The recorded behavioral RED and subsequent one-test PASS in RemPlan-1 were inspected as filed evidence, not independently re-executed. Local carrier source independently supports the asserted behavioral distinction.

## 2. Invariant analysis

**Counterexample and root-cause classification.** The original allocation required the newly created receiver already to contain the accepted join cut, while the combined witness required actual incoming snapshot installation at that cut. In the regression, seeding applies snapshot index 5/term 1 to MemStorage and sets applied 5. MemStorage sets commit 5 and retains that snapshot metadata. Incoming snapshot 5/1 contains receiver 4, passes recipient checking, matches the retained term, and reaches `Raft::restore`’s fast-forward return without creating a pending snapshot.

The unseeded receiver retains original voters `[1,2,3]`, starts at cut zero, and lacks the incoming snapshot’s term/cut. It instead reaches actual restoration and supplies snapshot index 5 in Ready. The test distinguishes these results rather than manufacturing an installed-snapshot counter.

This is the documented preimplementation **contract satisfiability defect**, not a demonstrated algorithm or durability defect. The amendment changes the permitted bootstrap mutation boundary and appropriately requires fresh review. No further root cause was found within this object.

**Agreement with admission and creation contracts.** ConfigurationSnapshotContract line 25 already permits a joining nonvoter to lack current state, while prohibiting voting or serving before complete independently validated restoration. Its lifecycle rule requires committed existing-group authorization before new-path creation or snapshot receipt. The amended supplement gives that incomplete receiver a precise original-genesis state and explicit host-owned admission evidence.

Original genesis ConfState is historical configuration input, not newly inferred group authority. The receiver cannot campaign, vote, contribute quorum/home readiness, serve or release outcomes. Unauthorized, wrong-group and uncommitted joins still cannot invoke create. These boundaries preserve RA-001 and RA-010 rather than replacing authenticated mapping with emptiness.

**Restart cannot become reset.** Revised line 41 requires renewed existing-group admission from retained group history and reopening the learner path. Inability to validate blocks participation. Line 45 continues to require dropping handles, physical open, retained identity/floors and complete validation before serving; open cannot create missing storage. Explicit new-path creation remains tied to validated committed admission. The amendment supplies no reset or replacement procedure.

**Complete history and actual transfer remain mandatory.** Full-log restoration and validated seeded initialization remain legal alternatives. Neither can falsely witness incoming snapshot installation at the already matching cut. The combined test still requires a fresh checkpoint containing learner 4 at the actual learner-add cut and a subsequent original suffix.

The unchanged shared specification requires a received snapshot cut at least the accepted learner-add index and strictly below current commit, followed by complete applied catch-up and joint restart. The clarification makes that witness satisfiable without weakening its assertions. Old-cut ConfState rewriting and caller-assigned readiness remain forbidden.

**Persistence and semantic obligations remain intact.** The private receiver exception concerns eligibility before restoration; it does not relax snapshot validation or publication. ConfigurationSnapshotContract still requires complete private replay validation before feeding a snapshot to RawNode, coherent checkpoint/applied/commit/configuration publication, synchronization before candidate exposure, and Ready/LightReady ordering before message or receipt release.

Committed-prefix immutability, legal uncommitted suffix replacement, historical same-term vote retention, whole-journal validation, partial-tail quarantine, poisoning after ambiguous writes and independent rollback floors remain unchanged. Joint majorities, deterministic readiness/home revalidation, complete outcomes, policy, tombstones and original typed replay remain implementation obligations.

**Allocation and evidence claims remain bounded.** No trait, package, manifest edge, third-party dependency, schema, classification or exception changes in this amendment. Pinned controls retain std-only contract values, separately allocated disk providers and development composition. The carrier test uses opaque bytes and explicitly disclaims application, authority and I/O qualification. Its braced conditional bodies satisfy the standing source rule; it introduces no conditional declarations.

The ledger preserves separate semantic-contract, allocation-remediation and future implementation gates. Historical Q1a/Q2 qualification remains attributed to its accepted tuples.

## 3. Risks and next action

The carrier regression establishes possibility of incoming restoration, not enforcement of the private follower boundary. Host authorization before create, suppression of voting/campaigning, restart admission, complete semantic replay, actual durable snapshot/suffix installation and fault/process recovery still require implementation regressions and the full exit matrix.

The next action is to merge this verdict with the independent Safety verdict. After required GO/GO, implement the clarified learner rule through actual host/adapter RED tests and retain the later settled-tuple Code/State acceptance gate.
