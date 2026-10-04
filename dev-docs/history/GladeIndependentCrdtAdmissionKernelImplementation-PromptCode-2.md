You are an independent, adversarial, READ-ONLY reviewer. Your job is to try to
refute this object's fitness, not to appreciate it. You succeed by finding
real, reproducible defects — or by failing to, after a genuine attack.

ROLE AND OUTPUT
- Axis: Code
- Another reviewer is attacking the same object on a different axis in
  parallel. You must not see, request, or reason about their report. Your
  verdict is formed from your own evidence alone. (Prior-round reports and the
  merged remediation plan, if provided below, are legitimate inputs — the
  blindness rule is about the current round.)
- Your final message must be the COMPLETE report in the mandated format, and
  nothing else. It will be filed verbatim as dev-docs/GladeIndependentCrdtAdmissionKernelImplementation-ReviewCode-2.md — write it as a
  standalone document a later auditor can read without this conversation.

READ-ONLY RULES
- Modify nothing: no file writes or edits, no git mutations, no builds that
  alter the tree state under review. Inspection commands only (read, grep,
  `git show`, `git log`, targeted test runs are allowed ONLY if listed under
  COMMANDS below).
- Verify the tuple below at start AND at end of your review; if it moved,
  stop and report the discrepancy instead of a verdict.

EXACT TUPLE (the object under review — nothing else is in scope)
- Glade workspace root: aa4f7e28334baa4ef515487792ca785061de597c
- Glade: 03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494
- Glial: 348eed97cd1ee4f677ea2866dfabe5a81cbebee1
- Glade-discover: 1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69
- External Gyld: 95a426595bba8e248a5f484272e483a070c73918
- Object: Originating Code/State re-verdict after ONE merged nonarchitectural remediation1. Root 332e7d95f5887300bf762ac76737e4cc38bcd151..aa4f7e28334baa4ef515487792ca785061de597c; Glade 4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14..03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494. Controlling DRAFT, RemPlan-1 and Remediation1-Evidence; all initial reports are legitimate prior-round inputs. Re-trace each OWN original counterexample, give closure table, and inspect complete corrected range for new blockers. Shared types/API/provider, roles/dependencies, platform/wire/compatibility, storage publication ownership and issued callback boundaries are fixed. Assess independently whether existing-context retention remains justified; if a material reviewed boundary/call graph changed, report that rather than presume acceptance. No current peer re-verdict may be read. This is deterministic component acceptance only.
- Controlling DRAFT document: dev-docs/GladeIndependentCrdtAdmissionKernelImplementation.md and its -Evidence.md at aa4f7e28334baa4ef515487792ca785061de597c
- Out of scope: Only generated current prompts and verbatim report outputs may be untracked. No source mutations during review. Do not read or request the current peer report. Raft, desk/runtime, live stores, enrollment/migration/activation and push are outside this object.

AUTHORITY AND DEFERRALS
- Process authority: AGENTS.md/AGENTS_GWZ.md; review-loop SKILL.md/canonical template; local ReviewCycle. This is remediation1 of the new IC2 implementation object, architectural root count ZERO retained. Both prior axes classified all five corrected roots nonarchitectural; four converged independently. Originating context retention follows step5 only while material interface/architecture/ownership/compatibility/platform/callgraph boundaries remain unchanged. Reviewers decide that premise independently. Maximum two remediation rounds; third architectural root requires STOP/owner decision. Historical contract caps/stops are not reset. Surface/real adapters/activation remain separate gates.
- Controlling documents to check the object against: dev-docs/GladeIndependentCrdtAdmissionDesign.md; GladeIndependentCrdtStorageAttemptDesign.md; GladeIndependentCrdtStorageAttemptContract.md and exact supersession table; retained GladeIndependentCrdtAdmissionContract.md clauses; GladeIndependentCrdtAdmissionPlan.md, GladeResourceConsistencyProfiles.md, this implementation DRAFT/evidence and ReviewCycle; LibraryBoundaryAndTestingPolicy.md, GladePackageArchitecture.md, GladeBuildEntry.md; root/member AGENTS. Actual crdt-admission-core source, test consumers/assembly and text trace; crdt-storage-attempt-api source/development provider; released glial/test/independent_admission_contract.mjs and source guard, canonical text corpus. Inspect unchanged shared API/roles/Gyld allocation rather than presume architecture-check success ratifies them.; initial IC2 Code/State reports, GladeIndependentCrdtAdmissionKernelImplementation-RemPlan-1.md and -Remediation1-Evidence.md. These are legitimate merged prior-round inputs.
- Explicitly deferred (do not report as findings): Production key/certification/signature domains, canonical transfer/container schemas, real clock/permit configuration, physical Records producer and antirollback, asynchronous cancellation/drain, automatic live duplex, clients/compatibility/Surface and activation remain IC3/4. Deferral does not excuse an unsatisfiable Pure contract, false acknowledged custody, inconsistent projection, unbounded state or falsely complete reads. Trusted test facts/memory storage are explicit model inputs; do not require physical evidence for this deterministic component or accept model evidence as that qualification..
  Deferrals cover a decision's OUTCOME only. Its shape — the verb it lives
  under, its name, whether its lifecycle pair is complete, its defaults — is
  always in scope.

REVIEW AREAS
AXIS: CODE — architecture, interfaces, call graphs, and compatibility reality.
Attack: interface contracts vs. actual call sites; ownership and visibility;
API/wire compatibility with retained readers and older writers; error paths
and hidden panic/allocation paths; whether the diff does what its DRAFT doc
claims and nothing it forbids.
- Trace every event/effect and actual-port consumer; no successful alternate model/helper fallback or weakened assertions. Pure/Contract roles, dependency edges and public API compatibility remain fixed.
- Attack descriptor/instance/profile/canonical-operation validation, bounded safe numbers and sorted refs, exact certificate/origin/epoch binding, local current policy/window versus historical admission. Trusted Facts are not peer authority.
- Attack full immutable encoding: explicit framing/version, ALL descriptor/candidate/evidence/facts/query/seal/fork/receipt/continuation fields, exact byte digest, ambiguity/mutation coverage, no Debug or opaque placeholder substituted for complete semantic bytes.
- Attack qualification versus custody/eligibility: finite acyclic predecessor and dependency closure, same-slot rivals, security-only AD, qualified common A, both delivery orders, transitive dependents, newly earlier forks, fresh certified E1 AB and exact old receipts. No first-arrival/maximum-head winner.
- Attack lifecycle: immutable PlanKey/AttemptId/InvocationId/Owner identity, lost preparation recovery, queued/Reserved/Started uncertainty, finite issued and consumed histories, full callback matching before any consumption, all four atomic kinds. Pending/Refused must never become NonCommit.
- Attack actual current prestart policy observation and valid historical Started cut; late commit/fence and retired/outstanding callbacks; same-terminal duplicate versus authenticated contradictory terminal, bounded first-contrary retention, exact once revision/charges/receipt installation.
- Attack finite quota/counter/history and actual encoded-byte accounting, query/pending/evidence bounds, unknown X versus independent Y, sticky recovery loss and integrity stop, unknown inventory/frontier handling and honest complete_local. No eviction/GC or silent lost uncertain work.
- Audit TDD chronological evidence, exact43 retained obligations and ten actual-Taut rows/canonical corpus, additional edge tests and rejecting mutants; test GREEN alone is insufficient. Code branch checks must cover disabled source; no allowlist/classification/budget relaxations.
- Report concrete P0-P3 only; classify roots explicitly. About1800 words maximum excluding necessary evidence/closure tables.
- Re-verdict must add prior-finding closure table for every own initial P2 with verified original counterexample evidence; then changed-range analysis including any new root explicitly classified architectural/nonarchitectural. Code has P2-1..4; State P2-1..5. Verify all own sequences and new boundary tests, not merely recorded green counts. The original reports and merged plan are legitimate; current peer report remains forbidden.

COMMANDS
cwd /Volumes/projects/limbo/glade-wz. Inspection only: pwd; git rev-parse HEAD for all5repos at start/end; git show exactpin:path; scoped git diff 332e7d95f5887300bf762ac76737e4cc38bcd151..aa4f7e28334baa4ef515487792ca785061de597c and member 4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14..HEAD; cat/rg/sed/nl/sha256sum/shasum/read-only Python. No writes/builds/tests/Git mutations/network/live operations. Audit execution evidence against full source; no parallel timing reruns. Return complete standalone report as final output; lane owner files verbatim.

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

## Prior-finding closure table
| ID | Disposition claimed | Verified on corrected tree | Status |
{One row per own prior finding, with the ORIGINAL counterexample retraced.}

## Changed-range analysis
{Exact changed range, preservation of context-retention premise and new-root classification.}

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
