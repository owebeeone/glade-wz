# Glade Raft Q1a remediation 1

Date: 2026-10-03. Reviewed source root
`5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`; member/Gyld pins unchanged.
Status: **one regression-first replay correction; reviewer closure pending**.

Initial verdicts: Code NO-GO (one P2), State GO. This is one interface-conformance
defect discovered in implementation review, not an authority/readiness failure.

| Finding | Disposition | Closure regression |
| --- | --- | --- |
| Code P2-1 | Public apply recovers retained result for an existing index and exact canonical Command regardless of absent private evidence. Changed canonical content still fails. New indexes still require the existing private readiness path to accept Move; no public numeric witness becomes authority. Preserve private full-envelope conflicting-replay checks. | Real RawNode driver successfully applies Move; on its actual Application replay exact index/command through dyn CommittedMachine and require original receipt. Changed command must ConflictingReplay. New unwitnessed Move must remain IncompleteSuccessor; existing forged/lag/stale tests retained. |

Write/run the regression before the implementation branch. One merged patch,
unchanged public signatures/manifest/dependencies/authority and Ready processing.
Focused Code re-verdict independently closes the original counterexample; State
also rechecks the correction and returns a verdict on the same revised tuple.
No production port, persistence or movement profile is expanded.
