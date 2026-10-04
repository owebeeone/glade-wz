# Glade Raft Q2 implementation — merged remediation 1

Date: 2026-10-03. Initial object: root
`ac69bbcc325c0946bbf215309bcce5edd3210db6`, with unchanged member/Gyld pins
recorded in the [review ledger](GladeRaftQualification-ReviewCycle.md).
Combined verdict: **NO-GO**. Two distinct P2 defects, one per axis; no blind
convergence. This is implementation remediation round 1, separately counted
from the accepted contract object's one remediation round.

| Finding | Disposition | Closure witness |
| --- | --- | --- |
| State P2-1: live campaign admits a term that startup refuses | Accept. Reserve `u64::MAX` consistently: refuse campaigning from its predecessor and reject terminal-term images before store publication/messages. Preserve history; do not rewrite/reset terms. | Actual-disk campaign from `MAX-1` with prior receipt, reopen every store and recover exact outcome in both `[1,2]` and `[1,2,3]`. Actual incoming terminal-term heartbeat must return `CapacityExhausted`, publish no image/message and leave all files recoverable. Existing exact-MAX refusal stays green. |
| Code P2-1: LightReady test steps a node with an outstanding Ready | Accept. Replace the unsupported sequence with the carrier's documented asynchronous advance lifecycle before stepping the node. Keep genuine LightReady through the live persistence/application helpers. | A lifecycle guard MUST fail RED on the old sequence. Corrected real-RawNode/real-disk success/failure cases MUST observe a commit-only LightReady, preserve term/vote, persist before application, and return no failed-batch messages/outcome. Successful disk reopen MUST recover its complete receipt. |

The owner combines the fixes into one checkpoint. There is no shared public
interface, journal format, dependency, production mutation boundary or ordinary
driver call-graph change. The corrected alternate witness changes its test-only
schedule. Originating Code/State reviewers MUST each verify their own original
counterexample at the new settled tuple before acceptance. Reports remain verbatim.

Observed State RED: the campaign/reopen regression returned
`CapacityExhausted` at recovery; incoming heartbeat returned `Ok` with a
terminal-term response instead of refusal. Both became GREEN after the bounded
campaign/publication guards. Code RED/GREEN and the corrected schedule are
recorded in [the evidence](GladeRaftQualificationEvidence.md).

Rerun locked/offline workspace tests, both explicit disk unit tiers, Python
oracle self-tests, both actual process-kill cycles, architecture/source/format/
process-global checks and all-target Clippy with warnings denied. Historical
measurements MUST retain their checkpoint attribution. This accepts only the
private APFS/process-crash experiment; Q3/Q4 and production selection stay open.
