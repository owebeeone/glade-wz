# Glade Raft Q0 remediation 1

Date: 2026-10-03. Reviewed source: root
`581ef60a65bfebda8b39645aeb122f12c23828ee`; unchanged member/Gyld pins in
both original reports. Status: **one merged bounded fixture correction**.

Both peer-blind axes independently found the same root cause: the namespace
success case reused the administrator's Create request identity. This convergence
is recorded as one defect, not two independent defects.

| Finding | Disposition | Closure evidence |
| --- | --- | --- |
| Consistency P2-1 | Correct test only. Reserve Create sequence1; use sequence2 for both principals' mutations and the changed user retry. Assert the identities differ only by principal and neither aliases Create before exercising setup. | Inspect exact RequestIds; compile specs and observe continued refusing-provider RED; Consistency reviewer verifies original counterexample. |
| Safety P2-1 | Same correction plus separate intentional Create-to-Mutate reuse case. Require RetryConflict, unchanged payload, original retained Create receipt. No action discriminator or weakened retry rule. | Corrected positive identities and explicit negative reuse assertions; compile/RED; Safety reviewer verifies counterexample. |

Safety suggested changing Create's sequence; changing both fresh mutation sequences
instead is equivalent namespace separation and satisfies the requested assertion
and separate negative witness. API, proposed architecture, manifests, canonical
amendments and implementation boundary are unchanged. No implementation is added.
One revision contains the complete correction. Focused re-verdicts use the same
reviewers, per skill, because no shared interface/architecture changed.
