You are an independent, adversarial, READ-ONLY reviewer. Your job is to try to
refute this object's fitness, not to appreciate it. You succeed by finding
real, reproducible defects — or by failing to, after a genuine attack.

ROLE AND OUTPUT
- Axis: Surface
- Another reviewer is attacking the same object on a different axis in
  parallel. You must not see, request, or reason about their report. Your
  verdict is formed from your own evidence alone. (Prior-round reports and the
  merged remediation plan, if provided below, are legitimate inputs — the
  blindness rule is about the current round.)
- Your final message must be the COMPLETE report in the mandated format, and
  nothing else. It will be filed verbatim as dev-docs/history/GladeIndependentCrdtProductionIntegrationPhysicalHost-ReviewSurface-1.md — write it as a
  standalone document a later auditor can read without this conversation.

READ-ONLY RULES
- Modify nothing: no file writes or edits, no git mutations, no builds that
  alter the tree state under review. Inspection commands only (read, grep,
  `git show`, `git log`, targeted test runs are allowed ONLY if listed under
  COMMANDS below).
- Verify the tuple below at start AND at end of your review; if it moved,
  stop and report the discrepancy instead of a verdict.

EXACT TUPLE (the object under review — nothing else is in scope)
- Glade workspace root: c8d778e7c24b68690e27bfac8ca4457f647e747f
- Glade: d3fded040d6e3c459cd5f975d6c672873356cc23
- Glial: 348eed97cd1ee4f677ea2866dfabe5a81cbebee1
- Glade-discover: 1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69
- External Gyld: 400cedcf1fff74128f366af758a0c389a23435d7
- Object: Combined IC-3B physical-host/API cold consumer surface, including new can_settle/can_abandon utility eligibility queries; exact settled tuple below. Do not read implementation source or controlling design/plan. Fresh full cold surface review after first B/second typed merged correction, including changed receive provenance and retained loss lifecycle. Surface reads docs only, not remediation plan, source or evidence.
- Controlling DRAFT document: dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md; cold consumer surface only at c8d778e7c24b68690e27bfac8ca4457f647e747f
- Out of scope: Only generated current prompts and verbatim reports may be untracked. Source and docs are frozen. Do not read/request current peer testimony. IC-3C automatic exchange and IC4/browser/live migration/enrollment/existing-store seals/activation, desk, Raft and push are excluded. Fresh disposable-root compatibility marker and actual old-executable refusal witness are in scope.

AUTHORITY AND DEFERRALS
- Process authority: Root/member AGENTS/AGENTS_GWZ; review-loop skill /Users/owebeeone/.claude/skills/review-loop/SKILL.md and canonical template. This is the SAME combined B object after its first merged correction; semantic2architectural/1nonarchitectural/1completedremediation unchanged; typed2architectural/5priornonarchitectural, this is its SECOND remediation, never reset at B naming; B0architectural/4nonarchitectural, this is its FIRST remediation. Third new reviewer-classified typed architectural root STOP before further patch/owner decision. Initial B Code/State findings are not self-closed: fresh full axes and original B Code/State own-counterexample closure are required at this same tuple. Original A2 R1-R6 independent closures remain owner-deferred to wider ABC. Fresh disposable-root marker authorized; no existing-store migration. Follow committed ledger for actual completed-round status. Strict pre-remote gate remains closed until genuine accepted B evidence and committed acceptance record. Corrected source committedd3fded040d6e3c459cd5f975d6c672873356cc23 and controlling rootc8d778e7c24b68690e27bfac8ca4457f647e747f. Ledger now records typed TWO completed remediations, B ONE completed remediation; these are corrections not finding closure. Semantic counts unchanged. Read only the specified cold consumer documentation/help, not implementation source, design/plan, evidence or current peer testimony. Preserve the recorded controlling-object accounting; classify concrete roots; cap applies.
- Controlling documents to check the object against: ONLY dev-docs/GladeIndependentCrdtProductionIntegrationTypedUsage.md, dev-docs/GladeIndependentCrdtProductionIntegrationAuthentication.md, dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md at the settled root. No design/plan, source, evidence, original or current peer reports. HEAD verification and cold doc reads only.
- Explicitly deferred (do not report as findings): Original typed re-review/originating closure remains owner-deferred to wider ABC; this is the new combined B consumer shape, not original R1-R6 re-verdict. IC3C automatic exchange and IC4 activation/dynamic bootstrap, live roots/keys, existing migration/sealing, desk, Raft and push excluded. Owner-excluded BOM case excluded. Deferrals never hide actual API lifecycle/default/error shape..
  Deferrals cover a decision's OUTCOME only. Its shape — the verb it lives
  under, its name, whether its lifecycle pair is complete, its defaults — is
  always in scope.

REVIEW AREAS
AXIS: SURFACE — the interface as the person using it meets it.
You read NO code and NO design or plan document: only the object's `--help`
output at every level, its user-facing docs pages, and the existing command
families' `--help` for comparison. Attack: where each command sits against
the families that already exist (would a user look for it there?); names
and one-line summaries read cold (do they say what the thing does, to
someone who does not know the design?); lifecycle pairs (every install has
an uninstall, every create a remove, every write an undo — present, named
symmetrically, documented together); every option has a stated default;
then do the first-day walkthrough from `--help` alone — install it, use it
once, undo it, and report every point where you had to guess, could not
find the next command, or found no command at all. A defect here is what
ships forever; file it as P2 when it will need a compatibility break to
fix after release, P3 otherwise.
- Cold first-use path: configure genuine evidence, select fresh provision versus reopen and explicit trusted root/floor/registration/owner/namespaces/limits, obtain actual combined or narrower host/session and preserve identity on reopen. Inspect documented defaults and mismatches without source.
- New nonmutating utility eligibility queries versus retirement: returned eligibility is not durable settlement; after checked I/O, Committed retirement versus Unknown owned retention; foreign/not-drained/closed/started-abandon errors, receiver cancellation/drain and close Pending handoff all discoverable cold.
- No hidden utility call order, destructive retry, owner replacement, implicit fresh auth from old custody, root deletion while pending, or receipt guarantee beyond the named qualified profile. Document narrow StorageAttemptHost creation versus existing authoritative namespace restoration and compatibility with ReplicaOpen creation.
- This API does not expose an actual CLI at B; if final docs introduce a helper CLI, inspect its full cold help by approved read-only command before verdict. No source/code/design reading, no writes/builds.
- Cold successful receive/result settlement versus malformed/error/substituted result; refused and Unknown ownership; public retained loss/cancellation/close/reopen path discoverable without private state access; fresh legacy compatibility marker ownership/refusal/supported direction. No source/design/plan or current testimony.
- Cold docs-only walk-through by inspection; no execution/builds/source. Report concrete P0-P3, root classification, credible consequence and doc closure check; complete standalone report; aim for about2000words without omitting necessary findings or evidence.

COMMANDS
cwd /Volumes/projects/limbo/glade-wz. Read-only inspection: pwd; git rev-parse HEAD for all5repos START and END; git show pinned:path; scoped git diff; rg/cat/sed/nl/shasum/read-only Python. No writes/Git mutations/builds/tests/network/live actions. Audit exact source and recorded evidence. Final output is complete standalone mandated report, parent files verbatim.

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
| ID | Disposition claimed | Original counterexample verified | Status |
{One row per own prior finding; writer claim is not closure.}

## Changed-range analysis
{Complete range and independent context-retention validity; new-root classification.}

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
