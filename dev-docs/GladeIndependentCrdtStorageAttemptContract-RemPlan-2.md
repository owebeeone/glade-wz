# Storage-attempt typed checkpoint — remediation 2

Date: 2026-10-04. Status: **planned; fresh-review blockers OPEN. Kernel remains
refusing. This is remediation round 2 on the redesigned typed object.**

Reviewed root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`; Glade
`346d963f09089a0636a01fac8a257f067908147d`; unchanged Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`.

Originating [Code closure](GladeIndependentCrdtStorageAttemptContract-ClosureCode-1.md)
and [State closure](GladeIndependentCrdtStorageAttemptContract-ClosureState-1.md)
verified their original counterexamples at this tuple. Their scoped GO is not
aggregate acceptance. Fresh full [Code](GladeIndependentCrdtStorageAttemptContract-ReviewCode-2.md)
and [State](GladeIndependentCrdtStorageAttemptContract-ReviewState-2.md) both returned
NO-GO, with six P2 IDs representing four distinct defects. They blindly converged
on the residual historical lookup and revision-finality defects. Both explicitly
classify every new finding non-architectural; the architectural root count remains
one on this object. The original IC-1 failed object's separate three-root stop is
unchanged. No finding is self-closed.

## One merged correction

| Finding | Disposition | Required regression/closure |
| --- | --- | --- |
| Code-2 P2-1 / State-2 P2-1 | Complete the original delayed prior-cut recovery fixture's Inspect40 registration through the existing full-request helper before advancing the counter. This is a residual instance of original Code P2-3; its broad migration remains incomplete despite focused closure. | Preserve ExactRetry with original receipt after revocation from the correctly issued fixture. Pair the otherwise identical unissued callback with unchanged state/reservation and CallbackMismatch. Assert both maps and issuance floor; inspect every manually injected lookup, including multiline inserts, for coverage. Kernel behavior remains compiling RED. |
| Code-2 P2-2 / State-2 P2-3 | Reject a new incompatible expected application revision before queued/Reserved preparation can be exposed. Preserve the full exact dedup/recover path for existing attempts and unresolved work; rejected expectation MUST NOT create a terminal with an invented revision. This uses the existing Refused grammar and changes no shared DTO. | Ahead/stale expectations for all four kinds through Prepare/Begin and Prepare/Fence paths leave custody/revision unchanged and recover/restore consistently. Include valid-current publication and exact dedup/recovery controls; rejected work cannot replace uncertain owned work. |
| State-2 P2-2 | Validate unique complete AttemptIds and per-instance unresolved exclusivity across queued preparations and retained Reserved/Started attempts before accepting recovery. Multiple historical terminals remain legal. | Duplicate-ID, distinct-ID overlapping active, queued-plus-active and multiple queued-owner negatives refuse. Positive recovery has historical terminals plus one current unresolved attempt and independent Y progress. Reject inconsistent images without reconstructing or deleting history. |
| Code-2 P2-3 | Validate the complete retained ledger, bindings/history and critical reservations against requested limits before open mutates owner/generation floors. Incompatible reductions MUST refuse without deleting retained data or consuming ownership. | Close/reopen with each relevant bound below retained consumption, require refusal and unchanged state/floors, then reopen compatibly at the same proposed generation. Outcomes, identities and reservations survive and authentic recovery/restore succeeds. |

All regressions MUST fail meaningfully before fixture fixes. Preserve the current
26 API and 11 fixture/source checks, 42 kernel behavior tests, original assertions
and all ten text rows. Add focused coverage; no successful `step`, test selection/
budget/classification/allowlist relaxation, live operation or unrelated refactor.
Shared API, dependency/role, wire/platform, assembly seams and accepted lifecycle
remain unchanged. Use one merged patch under the sole drafter. Update Contract
with precise refusal/validation obligations and file separate remediation evidence.
Run relevant affected checks; avoid unchanged external Gyld reruns.

The current full Code/State reviewers MUST independently re-verdict their own
findings and inspect the corrected range on one settled tuple, retaining context.
Under review-loop step5, ordinary validation corrections behind unchanged shared
interfaces use originating re-reviews. If drafting changes a shared interface,
architecture, assembly call graph, mutation boundary, compatibility or platform
assumption, the owner MUST instead dispatch fresh full axes. No such change is
planned. Kernel implementation remains stopped until both complete GO verdicts.

This is the second merged remediation round. The skill's bounded process still
controls: any subsequent architectural defect cannot be hidden in an extra patch;
its escalation rules and the third-architectural-root stop remain explicit.
No push or activation is authorized.
