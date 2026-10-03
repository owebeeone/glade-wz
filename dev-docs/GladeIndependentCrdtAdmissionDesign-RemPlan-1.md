# Independent CRDT admission design — remediation 1

Date: 2026-10-04. Status: **correction planned; findings remain open until reviewer
verification**. Initial reviewed root `bba04ad27311db50e3e6aedddb4d91780e4e4483`,
Glade `c65a6e87f0c257c15de8db080c29d365a883af85`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.

Both initial axes returned NO-GO. Three blocking findings identify two distinct
root causes; the canonical-origin defect was found independently by both axes.
Reports are filed verbatim. No implementation has begun.

| Finding | One disposition | Closure witness |
| --- | --- | --- |
| Consistency P2-1 | Define a fresh never-reused canonical `Op.origin` for each certified epoch; use it consistently through certificates, refs, retry, chain/quarantine and text writer identities. Same display label may repeat. | Retain an old valid prefix, recover under a distinct origin, preserve old/new eligible operations and their exact released text result under both orders; reject new epoch reusing the old canonical origin before mutation. |
| Safety P2-2 | Same canonical-origin correction as Consistency P2-1; one shared patch, not a second mapping. | Originating Safety reviewer retraces the old/new epoch collision and verifies the new witness. |
| Safety P2-1 | Require historically qualifying admission evidence for both rivals before a signed same-slot conflict can invalidate admitted history. Separate attributable unauthorized rival evidence from projection quarantine; do not use current revocation as a retrospective veto. | Valid O and dependent text remain eligible when a revoked/expired writer signs O' without qualifying admission, in either evidence order. Two genuinely qualifying competing admissions still quarantine both branches/dependents equally. |

Update design §§2/3/5/7 and ICD-T06/T07/T09 with exact requirements and expected
results. Qualification of a fork rival MUST be defined independently of derived
fork quarantine so the correction does not introduce circular eligibility.
Retain all existing receipt/history, current serving policy, bounded pending/
evidence and no-synthetic-adapter boundaries. No new profile family or stronger
custody promise is introduced.

One merged semantic patch closes both root causes. Originating reviewers verify
their original counterexamples. Because this refines security eligibility and
canonical identity, use fresh peer-blind Consistency/Safety reviewers for the
full revised gate as well; stale broad proofs are not reused. This is the first
architectural remediation round on this object. The two-round cap remains.
No finding is self-closed by the drafter or lane owner.
