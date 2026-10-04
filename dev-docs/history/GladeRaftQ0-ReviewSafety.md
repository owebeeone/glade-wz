# Glade Raft Q0 — SAFETY-AXIS REVIEW

**Review object:** Root diff `1808cb79e3ae0f3e3a2cae71584ccb54ca37709a..581ef60a65bfebda8b39645aeb122f12c23828ee`: `dev-docs/GladeRaft*.md`, GDL-052, and `proofs/raft-adoption`. Q0 draft contract/allocation checkpoint with intentionally failing refusing providers; not an implementation acceptance gate.

**Baseline:** Glade workspace root `581ef60a65bfebda8b39645aeb122f12c23828ee`; `glade` `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; `glade-discover` `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` `ca04499a360d910fbf8ee2540ed446facd051b35`. Reviewed committed sources through `git show` at these pins, excluding dirty member copies and unrelated files.

**Date:** 2026-10-03

**Axis:** Safety—attack degraded paths, irreversible transitions, disclosure, recovery, proof boundaries, and achievable qualification schedules. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks; zero P0, P1, or P3 findings. I pre-commit to GO on a revision that resolves **P2-1** as specified.

---

## 0. Evidence base

The four HEADs were checked at both review start and review end. All matched the exact tuple above; none moved. A scoped working-tree diff for the reviewed Raft documents and proof sources returned empty.

Read:

- Process authority: root `AGENTS.md`, `AGENTS_GWZ.md`, and `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`.
- `GladeRaftAdoptionContract.md`, lines 1–97; `GladeRaftQualificationPlan.md`, lines 1–149; `GladeRaftImplementationEvaluation.md`, lines 1–76; `GladeRaftQualificationEvidence.md`, lines 1–44; GDL-052, `DecisionLog.md`, lines 73–83.
- Proof API, `api/src/lib.rs`, lines 1–143; refusing providers, `proof/src/lib.rs`, lines 1–120; qualification tests, lines 1–356; API consumer tests, lines 1–73.
- Proof workspace/package manifests, architecture policy, process-global allowlist, `check.sh`, `check-source.py`, and README in full; lockfile dependency entries through line 280.
- `GladeBuildEntry.md`, lines 1–88; `LibraryBoundaryAndTestingPolicy.md`, lines 1–108; `GladePackageArchitecture.md`, lines 1–160.
- Buy/build fixed decisions and relevant rows D-06, R7/R9/R16, and Q12.
- `GladeDiscoveryModel.md`, lines 1–328; `GladeWorkspaceDirectory.md`, lines 1–274; `GladeAuthzModel.md`, lines 1–283.
- Pinned Glade `GladeSubstrateV1.md`, core model and session answers R1–R8; pinned `GladeCrossNodeWritesPlan.md`, particularly §3 W1–W6 and its admission/receipt context.
- `arch1/GladeArchitecture.md`, lines 1–178; pinned external Gyld declaration’s Binding, Policy, Admission, Records, supplier, and NodeAssembly contracts and allocations.
- `GladeOwnershipMechanismEvaluation.md`, lines 1–197, and its prior accepted `ReviewCycle.md`, lines 1–154.
- Downloaded `raft-0.7.0/src/raft.rs`, around lines 2798–2816: confirmed the documented `rand::thread_rng()` timeout-reset call.

Only inspection commands were executed. No builds, tests, writes, or Git mutations occurred. The recorded API test passes, eleven qualification failures, and structural gate passes are evidence supplied by `GladeRaftQualificationEvidence.md`, not independently rerun results. Inspection confirms that the refusing providers explain the reported behavioral failures.

## 1. Findings

### [P2-1] The principal-namespace success witness reuses an already consumed administrator request ID

**Location:** `proofs/raft-adoption/proof/tests/qualification.rs`, lines 135–146; helper definitions at lines 9–39. Controlling invariant: RA-004, `GladeRaftAdoptionContract.md`, line 45.

**Violated invariant:** Reusing the same complete retry identity with changed command bytes MUST fail. The test instead requires that reuse to succeed.

**Reproduction:**

1. Line 137 commits `create(1)`. Its request identity is:

   ```text
   scope=7, resource=100, incarnation=1, principal=1, sequence=1
   ```

2. Lines 138–142 exercise user principal 10’s sequence 1 mutation and changed retry.
3. Line 143 constructs `mutate(1, 25)`. Line 144 changes its principal to 1.
4. That mutation now has exactly the request identity consumed by the create in step 1. Its generation, home, action, and payload differ from the retained create command.
5. Line 145 nevertheless requires `Accepted`; line 146 requires payload 25.

A conforming implementation MUST return `RetryConflict` for this administrator mutation and preserve the original create outcome. The proposed test cannot pass under RA-004. This conclusion follows directly from the fixture values; no protocol timing or implementation assumption is needed.

**Impact:** Q0 hands Q1 an unsatisfiable success specification. Making it green by partitioning retry history by action, discarding create outcomes, or allowing changed-command reuse would weaken the declared exact-retry boundary. The current refusing harness conceals this contradiction because execution fails earlier.

**Required correction:** Keep the namespace test, but give its setup create an administrator sequence other than 1—for example, `create(2)`. Then principal 10/sequence 1 and principal 1/sequence 1 are distinct, previously unused mutation identities. Do not alter RA-004 or add action kind to the retry namespace.

**Closure/regression test:** Assert explicitly that the setup create identity differs from the administrator mutation identity. Also add a separate negative witness that deliberately reuses the create identity for a mutation and requires `RetryConflict`, unchanged payload, and preservation of the original create receipt.

Q0 closure requires corrected compiling specifications and continued honest behavioral RED against the refusing provider. It does not require implementing the application to obtain GREEN.

## 2. Invariant analysis

**M3-D orders the real application history.** The attempt to reinterpret Raft as a metadata preflight fails against contract §1 and RA-003/005/007. Complete protected payloads and outcomes belong to the ordered history; independent authoritative projection writes are forbidden. The fixture’s numeric payload is explicitly complete fixture data, without claiming complete Glade encoding.

**Leadership does not grant resource ownership.** RA-002 distinguishes terms from home/generation, and the leader-change test compares home, generation, and payload across failover. No inspected amendment grants takeover through discovery epochs or a leader observation.

**Unknown does not manufacture noncommit.** `Proposal::Unknown`, trusted `lookup()` returning `None`, and RA-004 consistently preserve uncertainty. The refusing Q0 provider creates no retained application outcome and therefore cannot falsely acknowledge or terminally reject an application mutation.

**Retry recovery and disclosure have separate boundaries.** The committed-machine API is expressly trusted host access. Client-facing `reply()` and `outcome()` must enforce current disclosure permission. The revocation schedule retains historical retry semantics while withholding both accessors after disclosure revocation. The changed-retry response is expressly transient and must preserve the original retained outcome. P2-1 concerns the contradictory fixture identity, not this separation.

**Movement readiness is bounded and unqualified beyond the fixture.** The plan requires complete successor application through the committed frontier, including policy and retry history. The fixture’s readiness input is explicitly harness-trusted, not client-minted evidence. Its lag/catch-up test checks refusal before readiness and acceptance afterward. Production cut certificates and separate BeginMove/Activate remain unqualified. Q1 implementation review must examine delayed readiness observations against the actual ordered cut; no present code claims to verify them.

**External effects and legacy overlap remain excluded.** RA-008 does not convert an ordered intent into an exclusive shell/build/Git effect. RA-012 requires legacy writer exclusion and a retained rollback fence before production activation. The amendment proposals preserve current contracts for unactivated identities and identify the existing admission and receipt clauses requiring reconciliation.

**Architecture and evidence boundaries remain honest.** The API has meaningful required `apply`/`lookup` operations and no implementation dependencies or exposed Raft types. The proof is classified as a standalone harness with bounded dependency edges and an explicit re-review condition before production reuse or added adapters. Its local check does not claim transitive dependency compliance. The documented RNG defect is acknowledged even for construction/state transitions, and manual campaigns are not described as injected randomness.

**RED is behavioral, but not proof of specification correctness.** The evidence distinguishes missing declarations and protoc failures from executable assertion failures. That distinction holds. P2-1 demonstrates why an entirely RED checkpoint still needs semantic review: a refusing provider cannot reveal whether later assertions are mutually compatible with the contract.

## 3. Risks and next action

No production durability, cryptography, authenticated genesis, metadata-witness profile, dynamic membership, snapshot/compaction, real sink, migration, or automatic-election qualification follows from this review. These remain explicitly deferred rather than falsely closed.

The next action is a bounded correction of **P2-1**: repair the namespace fixture and add the create-to-mutate retry-conflict witness, then submit the revised exact tuple for focused re-verdict before Q1 implementation.
