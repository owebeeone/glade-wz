You are an independent, adversarial, READ-ONLY reviewer. Your job is to try to
refute this object's fitness, not to appreciate it. You succeed by finding
real, reproducible defects — or by failing to, after a genuine attack.

ROLE AND OUTPUT
- Axis: Safety
- Another reviewer is attacking the same object on a different axis in
  parallel. You must not see, request, or reason about their report. Your
  verdict is formed from your own evidence alone. (Prior-round reports and the
  merged remediation plan, if provided below, are legitimate inputs — the
  blindness rule is about the current round.)
- Your final message must be the COMPLETE report in the mandated format, and
  nothing else. It will be filed verbatim as dev-docs/GladeIndependentCrdtAdmissionContract-ReviewSafety.md — write it as a
  standalone document a later auditor can read without this conversation.

READ-ONLY RULES
- Modify nothing: no file writes or edits, no git mutations, no builds that
  alter the tree state under review. Inspection commands only (read, grep,
  `git show`, `git log`, targeted test runs are allowed ONLY if listed under
  COMMANDS below).
- Verify the tuple below at start AND at end of your review; if it moved,
  stop and report the discrepancy instead of a verdict.

EXACT TUPLE (the object under review — nothing else is in scope)
- Glade workspace root: 7c5ac428a92c4087cf0ed1fff9e399d6f06c5850
- Glade: 885249a2a093e082aad6e1dc9936a7fd5c54052b
- Glade-discover: 1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69
- External Gyld: 64666e8b1caadde8922b9d42163afbab90655c65
- Glial: 5fd46ba5180051eb20d7b5547f59f52c0f3ebe06
- Object: IC-1 typed internal contract, Gyld allocation and compiling behavioral RED gate at 7c5ac428a92c4087cf0ed1fff9e399d6f06c5850; root diff from 4d735893c8db0a9ac9b4de9cde01600873b20ce3. Controlling doc dev-docs/GladeIndependentCrdtAdmissionContract.md and committed evidence identify exact Glade/Glial/Gyld baseline ranges and allowed commands. Review typed event/effect/continuation/commit/recovery semantics, genuine released text RED consumers, package role/dependencies, selection and source-boundary fixtures, inherited source-pinned Gyld allocation. Kernel is deliberately safe-refusing: no successful admission implementation or live feature is claimed.
- Controlling DRAFT document: dev-docs/GladeIndependentCrdtAdmissionContract.md at 7c5ac428a92c4087cf0ed1fff9e399d6f06c5850
- Out of scope: Generated current prompts and peer reports. No other mutations authorized while reviewing. Read no current peer report. Raft provider, desktop runtime, live seals/enrollment/activation, new preference merge profiles and unrestricted offline genesis are outside this lane.

AUTHORITY AND DEFERRALS
- Process authority: AGENTS.md and AGENTS_GWZ.md; review-loop skill and canonical template. GladeRaftQualificationPlan.md §6 provides the existing local process binding; this workspace has no GWZ CurrentProgramCheckpoint/AgentProcessRules/GwzProcessOptimization documents. This new lane has its own design/plan/review ledger, dual design and durable acceptance tiers, at most two architectural remediation rounds. No current user-facing interface is frozen by a semantic proposal; Surface is separately mandatory before that freeze.
- Controlling documents to check the object against: dev-docs/GladeResourceConsistencyProfiles.md, GladeIndependentCrdtAdmissionPlan.md, DecisionLog.md GDL054-057; LibraryBoundaryAndTestingPolicy.md, GladePackageArchitecture.md, GladeBuildEntry.md, arch1/GladeArchitecture.md and external Gyld architecture declaration; canonical dev-docs/glade/GladeAuthzModel.md, GladeDiscoveryModel.md, GladeWorkspaceDirectory.md; glade/dev-docs/GladeSubstrateV1.md §6 W1-W7, GladeCrossNodeWritesPlan.md, GladeCrdtAdapter.md, GladeShapeDispatch.md, GladeZones.md; TautShapeCatalogAdoption.md; existing ReplicaSync/BindingResolver and Store/mesh/signing/Glial text code at member pins; GladeRaftAdoptionContract.md, GladeRaftProductionIntegrationPlan.md, GladeRaftLegacyStoreSealContract.md for coexistence.; dev-docs/GladeIndependentCrdtAdmissionContract.md and its committed evidence; semantic design accepted at dcc8 with original closure and fresh GO/GO records; exact affected package READMEs/policies and frozen standalone Gyld supplemental source ledger/declaration/capture tests.
- Explicitly deferred (do not report as findings): Carrier/library selection, exact machines/key issuance/custody/deployment, new payload-profile families beyond existing text, unrestricted offline resource creation, production migration/activation and genuine adapter qualification remain later mandatory gates where explicitly deferred by the design. Deferral is not permission for incoherent proposed semantics, an impossible test, authority invention, unsafe production admission or claims that synthetic verification/storage qualifies real adapters. Owner authorized general CRDT design/review/implementation, not a waiver of canonical amendment or architectural requirements..
  Deferrals cover a decision's OUTCOME only. Its shape — the verb it lives
  under, its name, whether its lifecycle pair is complete, its defaults — is
  always in scope.

REVIEW AREAS
AXIS: SAFETY — what the text permits to go wrong.
Attack: degraded and mixed-version paths; irreversible steps and their
preconditions; disclosure/privacy scale; stuck states reachable under the
text's own rules; whether "never worse than the status quo" claims survive
concrete interleavings; scope creep that widens blast radius.
- Attack separate canonical instances vs same names, first-observed shape/profile, immutable profile negotiation/version, private principal and authentic origin/requester vs authenticated forwarding node.
- Attack two isolated authorized replicas accepting concurrently without holder/quorum, same valid set/deterministic merge, actual bidirectional app-history sync vs directory-only reconnect, loss of the sole admitted copy.
- Attack disconnected policy freshness/permits, authentic issuance/attestations/time bounds, revoke/expiry/partition/heal, immutable historical receipt vs provisional projection, fork/gap quarantine, invalid ancestors and foreign refs.
- Attack exact idempotent retry, lost acknowledgement/unknown outcome, restart/custody/rollback, durable atomic acceptance/evidence/outbox boundaries, revalidation, partial I/O and capacity/rejoin/retention floors.
- Attack exact superseded W/Authz/canonical clauses; no reinterpretation of unactivated legacy/strong bindings, whole-store seal interaction, mixed-version/rollback exclusions.
- Attack requirement-to-test traceability, real Taut/Glial profile reuse, satisfiable bounded first implementation tranche, Gyld allocation/library roles and source/global rules. No numeric or fixture proof may be relabeled a live feature. Report about 1500 words maximum; concrete counterexamples, no padding.
- Contract-specific: trace every asynchronous candidate/verification/seal/commit/unknown lookup continuation; immutable actual byte binding vs duplicated asserted fields; stale/foreign callback no mutation; reservation and receipt identity; qualification independent of derived quarantine; common-prefix frontiers with forks; current-serving policy separated from historical eligibility.
- Attack behavioral RED: tests must compile and fail assertions against safe-refusing behavior, actual consumer must take kernel output (no fabricated fallback); exact ABC/AD/AB cases and isolation. Discriminate pending/fork/security evidence, known failure/unknown, time/limits.
- Attack Gyld preservation: all original 24 allocations and 107 source-qualified obligations/links retained (raw generated IDs may differ across overlay lineage), all17 ICD requirements/journeys explicit with frozen source pin; no mutable sibling runtime fixture or Gyld engine/evaluator expansion. Package policy change is proposed reviewed Pure role/minimal deps, not weakening existing checks; ARCH002 positive control and exact negative refusal for actual inventory remain. Disabled source branches inspected.
- Verdict accepts internal semantics/RED allocation only. No user-facing wire/API freeze: Surface required later. Label evidence strengths and remaining prerequisites exactly.

COMMANDS
cwd /Volumes/projects/limbo/glade-wz. Read-only pwd/git rev-parse HEAD for root and each member; git show exactpin:path; scoped git diff 4d735893c8db0a9ac9b4de9cde01600873b20ce3..7c5ac428a92c4087cf0ed1fff9e399d6f06c5850; cat/rg/sed/nl/hash/read-only Python. Verify all five HEADs at start/end and stop if moved. No file writes/Git mutations/network/live actions. Design gate: no builds/tests. Contract gate: inspection only; audit committed RED/GREEN evidence against actual source/consumers. Do not rerun timing gates in parallel or build tests for this document gate. Code gate: only exact targeted commands recorded in committed implementation evidence. Contract object is dev-docs/GladeIndependentCrdtAdmissionContract.md plus its evidence, typed refusing scaffold/tests/selection/policy adoption and frozen standalone Gyld overlay/capture. Inspect changed Glade/Glial/Gyld files against the recorded baseline; successful kernel behavior is deliberately RED, not an escaped implementation failure. Read the whole generated prompt and object; no current peer material.

SEVERITY AND VERDICT CONTRACT
- Findings use IDs P0-n / P1-n / P2-n / P3-n:
  P0 = active corruption, data loss, credential exposure, or false composition.
  P1 = likely destructive or unrecoverable release blocker.
  P2 = concrete correctness, recovery, compatibility, parity, or
       diagnosability defect.
  P3 = bounded robustness, coverage, maintainability, or documentation defect
       with a concrete consequence.
- Verdict is GO or NO-GO. NO-GO while any P0, P1, or P2 is open.
- Each finding: ONE root cause, exact location, violated invariant, credible
  reproduction or state/interleaving sequence, impact, required correction,
  and a closure/regression test. Separate independent root causes.
- Style preferences and speculative unease are not defects. Do not pad.
  Interface shape is not style: wrong command placement, a misleading name,
  a missing half of a lifecycle pair, or an option without a default is a
  finding (P2 or P3), on every axis.
- If your verdict is NO-GO but every blocking finding has a bounded,
  text-or-code-fixable remedy, you may pre-commit: "I pre-commit to GO on a
  revision that resolves {IDs} as specified." This makes the re-verdict cheap
  and is encouraged when honest.

Use this complete final report template:

# {OBJECT} — {AXIS}-AXIS REVIEW

**Review object:** {object at exact SHA / doc path + status + date}
**Baseline:** {per-repo SHAs; note how sources were read, e.g. `git show HEAD:`}
**Date:** {date}
**Axis:** {one line: mandate}. Independent, adversarial, read-only. The other
axis runs in parallel; nothing here relies on it. Filed verbatim by the lane
owner.

**Verdict: {GO | NO-GO}** — {counts, e.g. "two P1 and three P2 findings
block"}. {If NO-GO and honest: pre-commit-to-GO clause naming the finding IDs.}

---

## 0. Evidence base
{What was actually read/run: files with line ranges, documents with sections,
commands with results. This section is what makes the verdict auditable.}

## 1. Findings
### [P1-1] {one-line root-cause title}
{Location · violated invariant · reproduction or state sequence · impact ·
remedy · closure test.}
{… one subsection per finding, severity-ordered. Omit section if none.}

## 2. Invariant analysis
{The invariants attacked and the evidence they held — attacks that FAILED are
part of the result; they are what a GO rests on.}

## 3. Risks and next action
{Residual risks below the finding bar; the single next action this verdict
implies.}
