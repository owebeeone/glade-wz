# Glade Raft Bootstrap/Growth Design — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT `dev-docs/GladeRaftBootstrapGrowthDesign.md`, GDL-053 in `DecisionLog.md`, and clarification callouts in `GladeRaftProductionProfileDecisions.md` and `GladeRaftProductionIntegrationPlan.md`, at workspace `761f691a17d65a5b450bf5e8d502a3121e104bc1`, dated 2026-10-03.

**Baseline:** Workspace `761f691a17d65a5b450bf5e8d502a3121e104bc1`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Canonical sources were read with `git show <exact-pin>:<path>`.

**Date:** 2026-10-03.

**Axis:** Internal coherence, agreement with controlling contracts, amendment boundaries, and satisfiability of proposed evidence obligations. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This verdict accepts the consistency of the semantic proposal for subsequent selection and contract work. It does not accept production behavior, public interfaces, canonical bytes or activation.

---

## 0. Evidence base

Read the complete generated Consistency mandate. Its SHA-256 matched `c34b573a938a1856ca626059f6769ef4801c89884e90d18945f690b5858155a6`.

Read:

- `GladeRaftBootstrapGrowthDesign.md`, lines 1–299, and the restricted four-file diff from `9c0510690050e6edc928c09ecc7b414baba58ba1`.
- `GladeRaftAdoptionContract.md`, lines 1–97: RA-001–012, persistence promises, canonical reconciliation and allocation.
- `GladeRaftConfigurationSnapshotContract.md`, lines 1–118: bounded identities, lifecycle, complete readiness, ordered configuration refusals, both home checks, coherent recovery and actual-carrier gates.
- `GladeRaftQualificationPlan.md`, lines 1–209, and `GladeRaftProductionIntegrationPlan.md`, lines 1–116.
- `GladeRaftProductionProfileDecisions.md`, lines 1–84.
- `GladeAuthzModel.md`, lines 1–410; `GladeDiscoveryModel.md`, lines 1–328; `GladeWorkspaceDirectory.md`, lines 1–274.
- `GladeBuildEntry.md`, `LibraryBoundaryAndTestingPolicy.md`, `GladePackageArchitecture.md`, and `arch1/GladeArchitecture.md`, including their contract-first, classification, ownership and assurance requirements.
- Exact-pinned external `examples/glade-architecture.gyld.py`, relevant port, state and component declarations through line 420.
- Exact-pinned Glade `GladeSubstrateV1.md`, relevant model, session, acknowledgment and W1–W4 sections; `GladeCrossNodeWritesPlan.md` §3 and §8/CJ-1–4.
- `AGENTS_GWZ.md` and the review-loop skill.

`pwd` confirmed the designated workspace. All four `rev-parse HEAD` results matched the mandated tuple at both start and end. No files were modified. No builds, tests, network operations or Git mutations were performed. The counterexamples below are logical design attacks, not executed witnesses. No current peer report was read.

## 2. Invariant analysis

**Disconnected bootstrap and identity.** I attacked two empty participants using the same label, a participant remembering an existing scope but lacking its store, and a device holding a signed genesis after a lost creation reply. Lines 65–103 distinguish independent fresh roots from the same canonical identity, require retained exact genesis before transmission/store creation, and forbid replacing unknown initialization with altered group/configuration bytes. Empty discovery, expired claims, creator unavailability and elapsed election time provide no reset authority. This agrees with RA-001 and strengthens the known-scope behavior without claiming that cold-join CJ-1 already solves independent first boots.

The custody requirements are demanding but coherent: multiple signing devices need serialized issuance, and rollback recovery needs independently trusted evidence. Loss blocks same-identity recovery; it does not invent a recoverability guarantee. Conflicting signed roots quarantine rather than being selected by term or discovery ordering. Consequently, discovery cannot reconcile independently committed histories.

**One, two, three and five voters.** The stable-majority calculations are correct: 1/1, 2/2, 3/2, 4/3 and 5/3. An installation’s node count is kept separate from voter count and independent data domains. I attacked loss of either two-voter member, attempting to count a learner as the second voter, and replication of old singleton acknowledgments after growth. Lines 123–128, 180–207 and 230–236 reject all three shortcuts. The optional singleton-plus-learner installation explicitly preserves the singleton failure promise and lacks learner failover. Later verified coverage is a separately named claim, not a retroactive change to the original receipt.

**Growth, readiness and joint authority.** A discovery candidate cannot become a learner without ordered authorized admission; a learner cannot become a voter merely because a counter advanced. Transfer includes payloads, policy, outcomes, tombstones and configuration, with proof tied to actual durable/applied state, identity, incarnation and cut.

I considered an intervening application command or revocation after readiness capture, a stale proof from a previous incarnation, and interrupted promotion. Lines 115–121 and BG-006 require revalidation at the ordered predecessor cut and retained refusal for changed eligibility. The controlling Q3 contract additionally requires deterministic validation of the retained evidence rather than local availability observations; the draft expressly preserves those rules in its amendment table.

For `{A}`→`{A,B}`, the joint decision requires A and A+B. For `{A,B}`→`{A,B,C}`, A+B remains necessary; C cannot replace an unavailable outgoing voter. Explicit LeaveJoint, retained joint recovery and the prohibition on timeout rollback prevent stop-during-growth from becoming implicit shrink.

Lines 149–159 also distinguish a carrier’s entering-entry activation rule from an external receipt claiming joint data retention. A predecessor-config commitment cannot alone certify the stronger guarantee. The exact adapter boundary remains a mandatory later contract, rather than an invented host commit algorithm.

**Home removal and ordered refusal.** I attacked removal of A while a live resource remains homed there, and placement on an outgoing-only voter queued after exit admission. Lines 168–176 preserve both Q3 checks, including the actual LeaveJoint predecessor cut, unchanged configuration version and no carrier apply on refusal. Movement or retirement permits a new exit intent; exact retry of the refused intent preserves its original result. Joint membership therefore does not silently rehome resources, and the design does not introduce nonvoting homes.

**Retry, partitions and reads.** Full configuration intent identity and original outcomes are retained. A changed request or principal collision cannot alias the original; timeout and cancellation remain unknown. I attacked an isolated former leader receiving a new write, a lost reply retried after policy/configuration changes, and a stale route serving a “current” read. Lines 139–147 and 209–228 distinguish new authoritative acceptance, permitted historical outcome disclosure and current-state barriers. Recovery of an old committed result does not falsely assert current group state. Local disclosure enforcement and unseen-revocation limits agree with RA-004/006/007 and the authorization model.

**Recovery and evidence boundaries.** Missing, corrupt and rolled-back stores cannot regain authority from recognizable keys. New-incarnation recovery requires authorization and verified catch-up, without cloned active identity. Complete coherent snapshots and the independent rollback floor remain required by RA-011/Q3. Neither discovery epochs nor cached data provide substitutes.

BG-001–014 are satisfiable future obligations under separately specified carriers, custody and schemas. They require actual elections, persistence, interruption and complete outcome oracles where fixtures are insufficient. They do not claim those witnesses already exist. Q3’s numeric universe and controlled campaigns remain historical private evidence; five-voter and production authentication claims are explicitly unqualified.

**Controlling graph and amendments.** GDL-053 and both callouts consistently record variable cardinality as the owner’s requirement while keeping other profile choices provisional. The known first group remains an integration fixture; dynamic resources use ordered authenticated mappings rather than configured per-resource homes. Discovery retains routing/fold responsibilities, creation-rooted policy remains separate from consensus membership, and Records/StorageAdapter responsibilities match the proposed profile-host allocation. No new package, dependency, global-state exception or implementation approval follows.

Section 7 is a proposed amendment map, not a claim that canonical clauses have already been superseded. The exact canonical amendment and affected-consumer gates remain explicit.

## 3. Risks and next action

The principal residual risks are the concrete custody/antirollback mechanism, authenticated readiness and configuration evidence, and each carrier’s activation/retention boundary. Their eventual contracts must make the proposed semantics executable and preserve the complete RA trace, including current substrate and cold-join compatibility. This review supplies no production proof for them.

The next action is to combine the independent verdicts on this same tuple, then proceed to semantic option selection and exact canonical/consumer contract preparation if the combined gate permits it.