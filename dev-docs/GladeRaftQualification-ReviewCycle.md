# Glade Raft qualification — review ledger

Date: 2026-10-03. Status: **Q0 accepted for isolated Q1a implementation only**.

Q0 accepted at root `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee`, Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`, after
[Consistency re-verdict](GladeRaftQ0-ReviewConsistency-1.md) and
[Safety re-verdict](GladeRaftQ0-ReviewSafety-1.md) returned GO.
This accepts draft contract/allocation and compiling behavioral specifications,
not production amendments, implementation conformance or deployment.

## Review process and result

The review-loop skill and canonical template are bound in QualificationPlan §6.
Skill SHA-256 `bbe0c21347c8961429d4304f334741b45f170da5e64c0982f67d9d4976797d0f`;
template `c2f5eb15549609e1972ef41ba73a8f1a8707bf956b2ef82f3784ef98417c24a9`.
[Consistency prompt](GladeRaftQ0-PromptConsistency.md) and
[Safety prompt](GladeRaftQ0-PromptSafety.md) were generated from that body, their
respective role and the exact settled tuple. Fresh read-only agents had no
current-round peer reports. Reports are filed verbatim.

| Gate | Source root | Axes | Findings and disposition |
| --- | --- | --- | --- |
| Q0 initial | `581ef60a65bfebda8b39645aeb122f12c23828ee` | [Consistency](GladeRaftQ0-ReviewConsistency.md), [Safety](GladeRaftQ0-ReviewSafety.md): NO-GO/NO-GO | Both independently found one shared P2 request-identity collision in the namespace test. |
| Q0 remediation1 | `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee` | GO/GO | [Merged correction](GladeRaftQ0-RemPlan-1.md): fresh sequence2 for both mutation principals, exact identity assertions, separate deliberate Create-ID reuse refusal. Both reviewers verified the original counterexample was corrected. |

Same reviewers performed focused re-verdicts because no interface, architecture,
manifest or implementation boundary changed. One bounded remediation round was
used. Defect counts: one P2 discovered during specification review; none escaped
into implementation; zero open Q0 findings. Broader unqualified gates are not
reported as closed findings.

## Next gate

Q1a requires real RawNode/application implementation, GREEN for the corrected
specifications, meaningful additional failure/counterexample witnesses and the
local lint/architecture/process-global gates. A fresh peer-blind Code/State gate
will review an exact implementation checkpoint. Memory persistence and trusted
numeric evidence remain explicit; Q2–Q4 remain open.

## Q1a initial implementation review

Source root `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`, same member/Gyld
pins: fresh [Code](GladeRaftQ1a-ReviewCode.md) returned NO-GO with one P2;
[State](GladeRaftQ1a-ReviewState.md) independently returned GO. Code alone found
an exact public replay mismatch after driver-attested movement. The existing
21-test GREEN did not cover that interface composition. Combined gate was NO-GO.

[Remediation1](GladeRaftQ1a-RemPlan-1.md) adds an observed-RED actual-driver/dyn-trait
regression and restores retained canonical replay without weakening new-movement
readiness. Final suite is 22 tests; review closure is still pending. No shared
interface, authority or persistence boundary changed; both original reviewers will
return focused verdicts on the same revised tuple. One remediation round is used.

Standing user control-flow/cfg instructions supplemented both implementation
review prompts verbatim, before verdicts; the source tuple did not move. They
require compound control-flow bodies and explicit conditional compilation boundaries,
including disabled branches. No broader upstream/legacy migration is claimed.
