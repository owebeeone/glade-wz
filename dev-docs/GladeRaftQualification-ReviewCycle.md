# Glade Raft qualification — review ledger

Date: 2026-10-03. Status: **Q0, initial Q1a and Q2 private disk/process-crash proof accepted; Q2 source `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5` after Code/State GO**.

Q1a accepted at root `31bbea0cf1da3c6ae437cf482cb744d561693c08`, with the
unchanged member/Gyld tuple below, after
[Code re-verdict](GladeRaftQ1a-ReviewCode-1.md) and
[State re-verdict](GladeRaftQ1a-ReviewState-1.md) returned GO. This accepts only
the fixed-group memory implementation and its partial RA witnesses; production
contracts, crate selection, durable qualification and activation remain open.

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

## Final Q1a merge and accepted-through record

| Gate | Source root | Verdict | Disposition |
| --- | --- | --- | --- |
| Q1a remediation1 | `31bbea0cf1da3c6ae437cf482cb744d561693c08` | Code GO / State GO | Originating Code reviewer independently closes P2-1; State reruns and confirms the correction without expanding movement authority. |

Accepted-through source is exactly the tuple above. Final filing changes only
review/status documentation; proof sources remain at that reviewed revision.
Fresh original Code/State prompts were generated from the same canonical template
and archived as [Code](GladeRaftQ1a-PromptCode.md) and
[State](GladeRaftQ1a-PromptState.md). Same agents performed focused re-verdicts
against one merged patch; no public interface, authority, persistence or dependency
boundary changed. All reports are verbatim.

Final verification: **22 tests pass**, architecture/source/format/process-global
checks pass, Clippy with warnings denied passes. Qualified cases and measurement
revision/limits are in [the evidence](GladeRaftQualificationEvidence.md).

| Discovery phase | Distinct defects | Outcome |
| --- | ---: | --- |
| Draft specification review | 1 P2; blind convergence on retry namespace collision | Corrected before implementation, both axes closed. |
| Implementation acceptance review | 1 P2; Code-only public replay/envelope mismatch missed by initial GREEN suite | Regression RED then GREEN; Code closes, State verifies. |
| Production escape | 0 observed | No production integration or activation took place. |

Q0 used one remediation round; Q1a used one. Zero open acceptance findings.
This is not closure of unqualified production requirements. No push was made;
unrelated member/handoff/research/scratch changes were preserved.

## Historical next qualification work after Q1a

Proceed next with a separately reviewed Q2 disk/persistence contract and fault
witnesses, then Q3 authenticated group/configuration/snapshot work and Q4 Glade
integration/legacy exclusion. Automatic elections require the explicit clock/
randomness dependency treatment already recorded. Production failure domains,
post-failure receipt guarantees, real readiness/crypto and external sinks need
profile decisions/evidence. The first proof cannot substitute for these gates.

## Q2 contract initial gate

Source root `0d2649369b91e21ddb8ed557b28032c89334fa9a`, unchanged member/Gyld
pins: fresh Consistency GO with P3-1; Safety NO-GO with P2-1 and P3-1. Reports
are [Consistency](GladeRaftQ2Contract-ReviewConsistency.md) and
[Safety](GladeRaftQ2Contract-ReviewSafety.md), verbatim. Combined gate NO-GO.
Both axes independently found the same lint/diagnostic issue (one distinct P3).
Safety additionally found the crash oracle could accept a changed recovered
receipt (one distinct P2). [Remediation1](GladeRaftQ2Contract-RemPlan-1.md)
corrects both in one specification patch before disk/integration implementation.
Storage interfaces, authority, roles and persistence design remain unchanged.

## Q2 contract accepted-through

Accepted at root `db2db3bba1bdbd931468949fffbd81d444044a3a`, unchanged
member/Gyld pins, after [Consistency](GladeRaftQ2Contract-ReviewConsistency-1.md)
and [Safety](GladeRaftQ2Contract-ReviewSafety-1.md) GO/GO. One remediation round,
one distinct P2 and one distinct P3, zero open contract findings. Same reviewers
verified their original counterexamples on one merged patch. No storage boundary,
architecture or authority changed. The generated canonical prompts remain archived;
focused dispatch added the Python oracle self-test to allowed commands.

Both reports note a late redundant working-tree evidence appendix, excluded from
the pinned object. The owner removed it; the contract was restored byte-for-byte
to the reviewed revision before implementation. Historical RED/GREEN evidence
was already committed in QualificationEvidence. No unreviewed clause was adopted.

## Q2 implementation initial gate and remediation 1

Settled root `ac69bbcc325c0946bbf215309bcce5edd3210db6`, unchanged member/Gyld
pins: fresh [Code](GladeRaftQ2Implementation-ReviewCode.md) and
[State](GladeRaftQ2Implementation-ReviewState.md) both report **NO-GO**.
Reports are verbatim; canonical generated prompts are archived as
[Code](GladeRaftQ2Implementation-PromptCode.md) and
[State](GladeRaftQ2Implementation-PromptState.md).

There are two distinct P2 findings, no blind convergence: Code found an
unsupported outstanding-Ready interleaving in the LightReady witness; State
found a live campaign could publish a reserved terminal term that startup refused.
The initial passing LightReady schedule is withdrawn as qualification evidence.
[One merged remediation](GladeRaftQ2Implementation-RemPlan-1.md) maps both findings
to observed-RED regressions and bounded corrections. Ordinary host orchestration,
public contracts, dependency/journal/production boundaries remain unchanged.
Originating reviewers re-verdict their own counterexamples on the new settled
checkpoint before any acceptance. Q2 contract and implementation round counts
are separate; implementation is in remediation round 1, with both findings open
until their reviewers verify closure.


## Q2 implementation accepted-through

| Gate | Source root | Verdict | Disposition |
| --- | --- | --- | --- |
| Q2 implementation initial | `ac69bbcc325c0946bbf215309bcce5edd3210db6` | Code NO-GO / State NO-GO | Two distinct P2 defects: unsupported LightReady witness lifecycle and reserved-term live/restart disagreement. |
| Q2 implementation remediation 1 | `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5` | Code GO / State GO | Originating reviewers independently verify their own regression and close both findings on the same settled tuple. |

Accepted-through tuple: root `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`; Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.
Re-verdicts are [Code](GladeRaftQ2Implementation-ReviewCode-1.md) and
[State](GladeRaftQ2Implementation-ReviewState-1.md), filed verbatim. Focused
prompts were generated from the same canonical template, role and revised tuple:
[Code](GladeRaftQ2Implementation-PromptCode-1.md),
[State](GladeRaftQ2Implementation-PromptState-1.md). The merged plan was a legitimate
shared remediation input; neither reviewer read the peer current-round report.

No public interface, authority, journal, architecture, dependency or ordinary
host call graph changed. The test-only async schedule was corrected, and the
already reserved terminal term was fenced before live publication. Same reviewers
therefore performed focused closure with their contexts intact. One merged
implementation remediation round was used; zero open P0–P3 findings. The Q2
contract object's separate count is also one round. Final filing changes only
review/status documents; accepted implementation bytes are those at the tuple above.

Final independently verified evidence: **49 default Rust tests + 2 explicit disk
unit cases = 51**, **4 Python oracle tests**, **2 actual SIGKILL/fresh-process
cycles**; architecture/source/format/process-global and all-target Clippy pass.
Original historical LightReady execution/measurement is explicitly disqualified
as ordering evidence. [QualificationEvidence](GladeRaftQualificationEvidence.md)
records the supported schedule, RED/GREEN closure and exact profile limits.

| Discovery phase | Distinct defects | Final outcome |
| --- | ---: | --- |
| Q2 contract review | 1 P2, 1 P3 | Oracle fidelity and diagnostic/Clippy defects closed before implementation. |
| Q2 implementation review | 2 P2 | Term/restart boundary and test lifecycle defects closed after RED → GREEN. No blind convergence. |
| Production escape | 0 observed | No production integration/activation occurred. |

Q2 acceptance is the private APFS/process-crash experiment only. Next is a
separately reviewed **Q3 configuration/snapshot contract**, compiling consumers
and RED witnesses for authorized learner catch-up/joint transitions, lost
configuration replies, snapshot installation/compaction and complete retained
retry/policy/retirement replay. Q4 remains the production crypto/bootstrap,
transport, failure-domain and legacy/effect exclusion gate. Automatic-election
randomness and power-loss claims still require their recorded profile decisions
and evidence. No push or desktop rebuild is part of this landing; unrelated
member, handoff, research and scratch work was preserved.


## Q3 contract review — package preparation

The owner requested continuation of reviews. The next object is the DRAFT
[configuration/snapshot contract](GladeRaftConfigurationSnapshotContract.md),
its separately compiled contract consumers and deliberately refusing provider.
[Q3 evidence](GladeRaftQ3ContractEvidence.md) records the expected behavioral RED,
compiler witnesses, affected Q2 checks and tier measurements. This is contract
review, not Q3 implementation acceptance or production ratification.

The review tier is dual **Consistency/Safety** before implementation; a later
dual **Code/State** gate follows actual carrier/disk/fault qualification. New
package roles and exact dependency edges are proposals in this gate, with no
relaxation of existing boundaries or process-global exceptions. Use fresh,
read-only peer-blind reviewers on one committed tuple, archive canonical
prompts and file reports verbatim. Merge findings into one remediation patch;
originating reviewers verify closure, subject to the bound two-round cap.

The Q2 accepted source/member tuple remains the baseline. Existing member
modifications and untracked handoff/research/scratch work are outside the object.
No production consumer, desktop build, push or activation is included.
