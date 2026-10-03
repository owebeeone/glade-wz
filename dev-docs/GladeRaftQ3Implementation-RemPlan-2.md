# Q3 implementation remediation 2

Date: 2026-10-03. Status: **planned; renewal Code P2-1 open; Q3 acceptance pending**.
Reviewed root `ce0876423e921cdd066106af9190ae7c6ec5d781`; unchanged Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`, glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.

Fresh [Code renewal](GladeRaftQ3Implementation-ReviewCode-1.md) is NO-GO with one
P2; fresh [State renewal](GladeRaftQ3Implementation-ReviewState-1.md) is GO with
zero findings. The five original findings are closed by their originating
[Code](GladeRaftQ3ImplementationClosure-ReviewCode-1.md) and
[State](GladeRaftQ3ImplementationClosure-ReviewState-1.md) reviewers on this tuple.
All four complete reports are filed verbatim. There was no blind convergence on
the new defect. It was caught during qualification, before production escape.
This is implementation remediation round 2; semantic/allocation rounds remain
separate. The finding reviewer explicitly classifies it as bounded candidate
enumeration under the existing strategy, **not a NEW ARCHITECTURAL root cause**.

## One disposition and closure

**Renewal Code P2-1: joint recovery excludes fresh outgoing-only voters. Accept.**
The manual candidate domain MUST include the deduplicated incoming/outgoing
voter union of the newest validated configuration. Existing availability,
nonfailure, local completeness, actual durable log freshness, deterministic
tie-break and terminal-term checks remain. Learners and removed post-exit members
remain excluded. This changes no public boundary, role, dependency, authority
contract or supported target domain; no election tick or fabricated term/log is
permitted.

The drafter MUST first observe a compiling behavioral RED for the exact actual
V2/RawNode sequence: noop1; AddLearner4@2 plus real catch-up; EnterJoint incoming
[4]/outgoing[1,2,3]@3 without existing live resources; accepted home4 Create@4;
Disconnect4; valid home4 mutation@5 remains unknown while outgoing nodes retain
its complete uncommitted Entry. **Drop before any live Reconnect**, then physically
open all four files into fresh recover. The original code repeatedly campaigns
stale4 and strands available joint authority. Reconnect before dropping would
let the old live leader repair4 and invalidate the counterexample.

Closure MUST recover an authorized fresh leader through real RawNode voting,
preserve complete original create/configuration receipts and Entry bytes plus
home4/generation1, resolve the unknown original mutation through genuine
reconciliation/commit, accept new work, and repeat physical restart with exact
original retries/replay. Genuine loss of either joint majority MUST still prevent
acceptance. Keep all previous regressions enabled. Record full expected values;
a presence check or counter assignment is insufficient.

## Verification and re-verdict

One bounded patch: owning regression, smallest candidate enumeration correction,
and accurate evidence. No unrelated source, interface, policy, third-party,
process-global exception, test selection or production change. TDD and GREEN
before any cohesive file movement remain mandatory.

Run focused regression, all owning Q3 cases and nineteen concrete consumers;
then complete private workspace, explicit Q2 tier, check.sh, strict all-target
Clippy, both oracle self-tests and all two Q2/three Q3 actual SIGKILL cuts. Owner
independently verifies and freezes the combined patch at one exact tuple.

Continue the fresh Code/State reviewers with intact contexts for independent
re-verdicts: Code MUST re-run/re-trace its original counterexample; State MUST
check the correction, joint quorum obligations and prior GO preservation. Require
closure tables and changed-range analysis. This bounded enumeration edit uses
the same existing campaign/filter/ranking path; it does not introduce a new
interface, mutation boundary or call graph requiring another fresh renewal.
If the actual patch exceeds that classification, the owner MUST reconsider the
review tier before acceptance. No finding self-closes. The two-round cap and
reviewer classification of new architectural causes remain controlling.
