# Glade Raft Q0 draft — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `1808cb79e3ae0f3e3a2cae71584ccb54ca37709a..581ef60a65bfebda8b39645aeb122f12c23828ee`: `dev-docs/GladeRaft*.md`, DecisionLog GDL-052 and `proofs/raft-adoption`. Q0 draft contracts and intentionally refusing harness; no implementation acceptance claimed.

**Baseline:** Workspace root `581ef60a65bfebda8b39645aeb122f12c23828ee`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` at `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling sources were read with `git show` at these exact commits. Dirty member copies and unrelated working files were not implementation evidence.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling document graph, contract semantics, package policy and satisfiability of executable specifications. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks; zero P0, P1 or P3 findings. I pre-commit to GO on a revision that resolves **P2-1** as specified, provided the remaining reviewed object is unchanged.

---

## 0. Evidence base

All four HEAD checks matched the prescribed tuple at both the start and end of review. No files were written, builds or tests executed, Git state mutated, subagents launched, or other current-round reviewer reports/prompts read.

The evidence inspected comprises:

- The dispatched `dev-docs/GladeRaftQ0-PromptConsistency.md`, root `AGENTS.md`, `AGENTS_GWZ.md`, and `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`.
- The scoped root diff summary and complete GDL-052 addition.
- `GladeRaftAdoptionContract.md`, lines 1–97, including RA-001–012, proposed canonical reconciliation and architecture allocation.
- `GladeRaftQualificationPlan.md`, lines 1–149, including the Q0 gate, Q1a limitations, requirement trace, verification obligations and local process binding.
- `GladeRaftImplementationEvaluation.md`, lines 1–76; `GladeRaftQualificationEvidence.md`, lines 1–44.
- Fixture README, both package manifests, workspace manifest, lockfile package/version/dependency inventory, architecture policy, process-global allowlist and both gate scripts.
- API source, lines 1–143; refusing proof source, lines 1–120; API consumer tests, lines 1–73; qualification tests, lines 1–356.
- `GladeBuildEntry.md`, lines 1–88; `LibraryBoundaryAndTestingPolicy.md`, especially LBT-001–012 and §§3–5; `GladePackageArchitecture.md`, lines 1–160.
- `GladeBuyBuildMatrix.md`, lines 1–168, particularly D-06, R7, R9, R16 and Q12.
- `GladeDiscoveryModel.md`, lines 1–328, particularly §§0, 3 and 7; `GladeWorkspaceDirectory.md`, particularly §4, lines 115–139, and WD-8.
- `GladeAuthzModel.md`, §§1, 3a, 3b, 4, 4a and 7a, including authenticated private scope and operator-approved placement.
- Pinned Glade `GladeSubstrateV1.md`, particularly §2, §6 R1/R2/R7 and W1–W8; pinned `GladeCrossNodeWritesPlan.md`, particularly §3, lines 193–236.
- `arch1/GladeArchitecture.md`, lines 1–178; pinned external `examples/glade-architecture.gyld.py`, particularly ports, state ownership and components through NodeAssembly.
- `GladeOwnershipMechanismEvaluation.md`, particularly M3, deployment profiles, EM journeys and §7 canonical impact; its prior `ReviewCycle.md`.
- Pinned architecture-check implementation and negative-test inventory; pinned process-global checker documentation and scan structure.
- Locally cached `raft-0.7.0` dependency source for `RawNode` proposal/Ready/advance obligations and `raft.rs` election-timeout RNG. No web documentation was independently fetched in this restricted review.

Recorded build/test outcomes in `QualificationEvidence.md` were inspected, not rerun. The source supports the reported distinction between compiler RED and behavioral RED: the refusing providers compile but cannot produce the required applied receipts, and the direct application specification expects `Ok(None)` while the provider returns `NotQualified`.

## 1. Findings

### [P2-1] The principal-namespace witness reuses the administrator’s existing Create identity

**Location:** `proofs/raft-adoption/proof/tests/qualification.rs`, lines 135–146; identity constructors at lines 9–39. The contradictory requirements are RA-004 in `GladeRaftAdoptionContract.md`, line 45, and exact command equality in `api/src/lib.rs`, lines 18–29.

**Violated invariant:** Changed command bytes under an already used complete retry identity MUST fail, while a genuinely distinct principal namespace MUST remain independent. Q0 consumer specifications must be satisfiable without weakening that invariant.

**Reproduction:** Trace the test without supplying any implementation behavior:

1. Line 137 commits `create(1)`. Its request identity is `(scope=7, resource=100, incarnation=1, principal=1, sequence=1)`.
2. Lines 138–142 exercise user 10’s mutation and changed retry under sequence 1.
3. Line 143 constructs `mutate(1, 25)`. Line 144 changes its principal to 1.
4. The resulting request identity is exactly the identity already used by `create(1)`.
5. Its generation, home and action differ from the retained Create command.
6. Line 145 nevertheless requires acceptance, and line 146 requires payload 25.

A conforming implementation must return `RetryConflict` for step 4’s changed command, preserve the original Create outcome and leave payload 23. It cannot satisfy this test’s final assertions.

**Impact:** The Q0 refusal masks an impossible future GREEN condition because the test currently fails at its initial Create setup. During Q1, satisfying the namespace witness as written would require ignoring an existing retry identity, partitioning identity by action contrary to `RequestId`/RA-004, or overwriting the original outcome. Thus this is a concrete contract/test contradiction, not an expected failure caused by the intentionally refusing provider.

**Required correction:** Reserve the administrator’s Create request sequence. Give the two principal-namespace mutations the same sequence that is unused in both namespaces—for example, use sequence 2 for the original user mutation, its changed retry and the administrator mutation. Preserve the changed-byte refusal and original-outcome checks.

**Closure/regression test:** Add an explicit identity assertion showing that the two namespace mutations differ only in principal and that neither aliases the Create request. Retain assertions that changed bytes conflict, trusted retained history preserves the original outcome, and the second principal’s fresh mutation succeeds. At Q0, confirm consumer compilation and continued honest behavioral RED; implementation GREEN is not required to close this fixture defect. At Q1, a conforming provider must pass the corrected schedule, and a provider that overwrites a used request identity must fail it.

## 2. Invariant analysis

**Scoped amendments remain proposals.** Contract lines 63–66 and GDL-052 explicitly preserve current production contracts until reviewed activation. The reconciliation identifies the relevant discovery/no-consensus clauses, physical-copy lock limitations, live-claim admission, unclaimed fallback and receipt semantics. The cited phrases and rule meanings agree with the pinned controlling sources. In particular, existing forwarded `Ok` covers holder and forwarder storage under the process-crash profile; the draft does not silently upgrade it to durable Raft quorum acceptance.

**M3-D orders actual fixture data.** The API command carries the complete numeric payload, and `Resource` retains it as data. This is a bounded numeric application profile, not a digest substituted for application state. RA-003, RA-005 and RA-007 exclude metadata preflight followed by independent authoritative append. External effects remain excluded under RA-008.

**Leadership and home are distinct.** RA-002 and the leader-change schedule preserve home, generation and payload after an election. The old-leader minority schedule requires Unknown while isolated, then separately exercises ordered movement and stale-generation rejection. No discovery epoch or Raft term is presented as application authority.

**Trusted apply and client disclosure are separated.** `CommittedMachine` receives already committed history and does not claim to establish consensus or ingress authentication. Its trusted lookup returns retained history; serving-hop accessors separately enforce current disclosure permission. The changed-retry response accessor is explicitly transient and does not replace the original retained outcome. This boundary distinction survives inspection, apart from P2-1’s conflicting test identity.

**Unknown is not noncommit.** Q0 `propose` returns only Unknown. Missing lookup/reply data is not asserted to prove abort. The contract and API documentation preserve uncertainty after lost replies and distinguish applied terminal outcomes from host ordering errors.

**Successor readiness is bounded honestly.** `Move` readiness is described as harness-derived complete applied-frontier evidence, not client authority. The lag/catch-up test creates an actual missed application suffix in its proposed schedule. Atomic fixture movement is explicitly partial; two-stage movement and production verification remain open. No production readiness claim follows from its numeric witness.

**Library roles and dependency direction are coherent for this experiment.** The API has meaningful required operations and zero dependencies. The harness is outside production paths, has a named owner/rationale and retirement conditions, and declares its candidate dependencies explicitly. The local gate adopts the existing checker rather than claiming another repository’s adoption covers this workspace. Structural passes are not presented as behavioral conformance.

**Evidence maturity is not overstated.** The proof implements neither RawNode nor application behavior. The evidence distinguishes declaration RED, behavioral RED and toolchain failures. Memory retention, trusted policy fixtures, logical voters, manual campaigns and source inspection remain distinct from disk durability, cryptography, independent failure domains and production integration. The dependency RNG defect is explicitly acknowledged, including construction/state-transition calls; manual campaigns are not called injected randomness or full dependency compliance.

## 3. Risks and next action

This verdict concerns Q0 document and consumer fitness only. It provides no approval for production deployment, library selection, real cryptography, genesis/membership, metadata witnesses, physical durability, staged movement, bounded retention, migration, external sinks or automatic elections. Those obligations remain explicitly unqualified.

Future acceptance also requires measured feedback timings, actual Ready/LightReady processing, reusable behavioral conformance against the implemented provider, and the declared counterexample mutants. None is established by this read-only review.

The next action is one bounded fixture correction for **P2-1**, followed by renewed verification of its identities, consumer compilation and recorded behavioral RED. Do not implement the proposed boundary to work around the contradictory assertion.
