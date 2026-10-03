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

### Q3 initial contract verdict merge

Source root `fa1ff8b9fd0be53932300730ff925d0e41c76b1a`, with unchanged member/Gyld
pins recorded in canonical [Consistency](GladeRaftQ3Contract-PromptConsistency.md)
and [Safety](GladeRaftQ3Contract-PromptSafety.md) prompts.
The independent reports are filed verbatim:
[Consistency GO](GladeRaftQ3Contract-ReviewConsistency.md),
[Safety NO-GO](GladeRaftQ3Contract-ReviewSafety.md).

Safety identified three distinct P2 defects: invalid Create fixture preconditions,
live-home eligibility unprotected through joint exit, and an application-only
index replay result unable to represent configuration receipts. No blind
convergence occurred. [Remediation 1](GladeRaftQ3Contract-RemPlan-1.md) accepts all
three for one bounded contract/specification patch and maps each to closure
specifications. Compiler success and initial NotQualified failures did not expose
these subsequent assertion/transition defects; no Q3 implementation has shipped.

The typed replay correction changes a shared interface and the exit rule changes
a reviewed mutation boundary, so the corrected committed package requires fresh
dual Consistency/Safety reviewers under review-loop §5.5. The originating Safety
reviewer also verifies its original counterexamples. Remediation round 1 is in
progress; Q3 contract remains NO-GO pending completed verdicts, and Q3
implementation/Q4 remain open.


## Q3 contract accepted-through

| Gate | Source root | Verdict | Disposition |
| --- | --- | --- | --- |
| Q3 contract initial | `fa1ff8b9fd0be53932300730ff925d0e41c76b1a` | Consistency GO / Safety NO-GO | Three distinct Safety P2 findings: invalid Create fixtures, incomplete joint-exit home protection, application-only index replay. |
| Q3 contract remediation 1 | `ed243db983c485e46a27aa870ec745de16a56d7a` | Fresh Consistency GO / fresh Safety GO / originating Safety closure GO | All original counterexamples independently verified; no new/open P0–P3 findings. |

Accepted-through tuple: root `ed243db983c485e46a27aa870ec745de16a56d7a`;
Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`;
Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`;
external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`.

Fresh [Consistency](GladeRaftQ3Contract-ReviewConsistency-1.md) and
[Safety](GladeRaftQ3Contract-ReviewSafety-1.md) reports and the
[originating Safety closure](GladeRaftQ3Contract-ReviewSafetyClosure-1.md) are
verbatim. Canonical generated prompts are archived as
[Consistency](GladeRaftQ3Contract-PromptConsistency-1.md),
[Safety](GladeRaftQ3Contract-PromptSafety-1.md) and
[originating closure](GladeRaftQ3Contract-PromptSafetyClosure-1.md).
Fresh contexts were required because typed replay changed a shared interface
and the LeaveJoint rule changed the reviewed mutation boundary. Both initial
reports and the merged plan were legitimate prior-round inputs; no reviewer
read a peer current-round prompt/report. The originating reviewer separately
verified its own counterexamples on the same frozen source.

One merged contract remediation round used. No blind convergence. The three
initial findings were discovered during contract review, before Q3 implementation.
Zero production escapes observed; no Q3 production integration occurred.

Acceptance is limited to the private Q3 contract/specifications, new contract and
harness roles/normal edges, plus the explicitly justified dev-only
proof-to-q3-api fixture compatibility edge. No existing role, normal dependency
boundary, process-global exception or application/Q2 implementation was loosened.
The checker verifies declared boundaries; independent reviews approve their
allocation within this experiment. The accepted source includes no actual V2
store, configuration driver or snapshot codec.

Current evidence: API/compiler and exact fixture **2 PASS**, real unchanged
Application compatibility **2 PASS**, **19 intentional NotQualified RED**;
original selected Q2 packages **49 PASS**, with compatibility cases making **51**
in that selected command, plus **2 explicit Q2 disk cases PASS**. Architecture,
format, explicit source boundaries, process-global scan (21 files, zero
exceptions) and all-target Clippy PASS. Existing Q2 SIGKILL evidence remains at
its prior accepted tuple; it is not new Q3 evidence. Full evidence/commands and
measurements are in [Q3 contract evidence](GladeRaftQ3ContractEvidence.md).

Next: Q3a actual V2 lifecycle/fault and authorized uncompacted learner/joint host
RED tests before implementation; Q3b full-history codec/semantic replay and actual
snapshot/suffix/compaction tests. Both slices and every mandatory exit-matrix row
must pass before dual Code/State Q3 implementation acceptance. Q4 and production
activation remain open. Final filing changes documentation/status only; accepted
source remains pinned above. No push or desktop rebuild is included. Unrelated
member, handoff, research and scratch work is preserved.

## Q3 implementation allocation gate — preparation

The owner directed implementation after Q3 contract acceptance. The
[allocation supplement](GladeRaftQ3ImplementationAllocation.md) declares the
exact new provider normal/development edges and injected create/open lifecycle,
with compiling refusing providers and actual consumer RED cases. No algorithms
are included at this checkpoint. Library-boundary policy and BuildEntry require
fresh Consistency/Safety review before implementing these additions. The
accepted Q3 semantic contract remains the controller; no production integration,
new GWZ member, third-party dependency, process-global exception, push or desktop
rebuild is included. The baseline is root
`e1c260f6cf992f5d890c2ca454e2323b0d8e78b1`, unchanged member/Gyld pins. Canonical
prompts bind the new committed tuple; reports are filed verbatim. Q3a/Q3b actual
implementation remains pending allocation acceptance and its TDD/evidence gates.


## Q3 implementation allocation accepted-through

Source root `ccab267c6b23bfec7471923098944048a7959563`; unchanged Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`.
Fresh independent [Consistency](GladeRaftQ3Allocation-ReviewConsistency.md) and
[Safety](GladeRaftQ3Allocation-ReviewSafety.md) both GO, zero P0–P3, zero
remediation rounds. Reports are verbatim. Canonical generated prompts:
[Consistency](GladeRaftQ3Allocation-PromptConsistency.md),
[Safety](GladeRaftQ3Allocation-PromptSafety.md). Both verified the tuple and empty
scoped working-tree diff at start/end; neither accessed peer current-round
prompt/report. No blind convergence or production escape observed.

This accepts the paired StoreLifecycle create/open contract and exactly the
allocation supplement's normal/development edges for V2DiskStore/V2StoreFactory,
Q3Session and dev-composed Q3 consumer tests. No new package/third-party
dependency, classification relaxation or process-global exception. Compiler
consumer 3 PASS, nineteen shared + eight lifecycle cases deliberately RED,
51 preserved selected regressions PASS, structural/format/Clippy PASS (23 owned
files/zero exceptions). The historical refusing stage is not Q3 qualification.

Owner directive now proceeds through Q3a/Q3b TDD and the complete accepted exit
matrix before fresh dual Code/State implementation review. New boundary/edge
recomposition still requires review. Source qualification, real faults/SIGKILL,
external original oracles and measured tiers remain open. This filing changes
review/status documentation only; accepted allocation source is pinned above.


### Q3 allocation remediation 1 — bootstrap clarification pending

Before implementing learner creation, the drafter identified a concrete
satisfiability conflict: seeded receiver at learner-add S cannot demonstrate
actual installation of the required same-cut snapshot. [One bounded correction](GladeRaftQ3Allocation-RemPlan-1.md)
allows the externally authorized but locally incomplete private catch-up follower,
with campaign/vote/quorum/home/serving blocked until appropriate validated
restoration. Actual RawNode RED -> GREEN carrier counterexample is filed;
application/authority/disk catch-up still requires full implementation tests.
Fresh Consistency/Safety review of the pinned supplement amendment precedes the
corresponding learner algorithm. All in-progress uncommitted Q3 implementation
is outside this document-review object and uses no unreviewed learner rule.
This is allocation remediation round 1, distinct from the accepted semantic
contract's round count and the upcoming implementation Code/State object.


### Q3 bootstrap clarification accepted-through

Source root `d4589feb02ad86b92f58686da35c8a216c370c44`, unchanged member/Gyld tuple
above. Fresh [Consistency](GladeRaftQ3Bootstrap-ReviewConsistency.md) and
[Safety](GladeRaftQ3Bootstrap-ReviewSafety.md) both GO, zero open/new findings.
Canonical generated prompts are [Consistency](GladeRaftQ3Bootstrap-PromptConsistency.md)
and [Safety](GladeRaftQ3Bootstrap-PromptSafety.md); reports filed verbatim.
Both independently trace the seeded-cut counterexample and amended boundary;
only pinned document/carrier-test sources were reviewed. Uncommitted Q3 algorithms
and the carrier-test initializer rewrite were excluded; no algorithm or actual
physical/application catch-up acceptance is inferred. All four HEADs verified
start/end. One allocation remediation round; semantic-contract and final
implementation-object round counts remain separate. The contract satisfiability
issue escaped the initial allocation review and was caught before implementing
learner construction, with no production escape. No blind convergence on a new
finding occurred in renewal.

The learner rule is now authorized for TDD implementation. Actual incomplete
follower vote/pre-vote suppression, restart admission/no-reset, complete real
snapshot-plus-suffix catch-up and all other matrix rows remain mandatory before
Code/State implementation acceptance. This filing touches documentation only,
not the in-progress algorithms. Production profile/activation remains unchanged.

## Q3 implementation review — first gate, NO-GO

Frozen source root `b61197602e5594bdf89770bda069ce7d30fdb222`, unchanged member/Gyld
tuple above. Concrete Q3a/Q3b algorithms and [implementation evidence](GladeRaftQ3ImplementationEvidence.md)
passed independent owner verification: 113 default tests, two explicit Q2
library cases, two Q2/three Q3 actual SIGKILL cuts, strict gates; 44 owned Rust
files, zero process-global exceptions. Every original nineteen consumer remained
default-selected with no filter. These passing tests are not review acceptance.

Fresh peer-blind [Code](GladeRaftQ3Implementation-ReviewCode.md) and
[State](GladeRaftQ3Implementation-ReviewState.md) both NO-GO, reports verbatim.
Generated canonical prompts: [Code](GladeRaftQ3Implementation-PromptCode.md),
[State](GladeRaftQ3Implementation-PromptState.md). Both verified all four HEADs
and clean scoped bytes at start/end. No current peer prompt/report accessed.
Code found one P2 failure to exclude poisoned direct-compaction callers and two
P3 mandatory witness gaps. State found two P2 defects: stale first-voter restart
selection and silently successful ahead-of-local checkpoint installation.
The axes did not converge on a shared defect; no production escape occurred.

[One merged remediation plan](GladeRaftQ3Implementation-RemPlan-1.md) maps every
finding to a disposition and exact closure. Implementation remediation round 1
is now authorized for TDD; all five findings remain open. The contract/allocation
round counts are separate. Q3 implementation and Q4/production activation remain
pending. This review filing does not change the reviewed algorithms, member
sources, unrelated work, push status or desktop runtime.

## Q3 implementation remediation 1 — original closures, renewal NO-GO

Corrected source root `ce0876423e921cdd066106af9190ae7c6ec5d781`, unchanged
member/Gyld tuple above. Owner independently verified 120 default PASS, two
explicit Q2 cases, both oracle self-tests, all five actual SIGKILL cuts and strict
gates; 49 owned Rust files, zero exceptions. The original nineteen remain enabled.
The source/evidence/merged patch was committed before reviews.

Originating [Code](GladeRaftQ3ImplementationClosure-ReviewCode-1.md) and
[State](GladeRaftQ3ImplementationClosure-ReviewState-1.md) both GO, independently
closing all five original findings. Fresh full-scope peer-blind
[Code](GladeRaftQ3Implementation-ReviewCode-1.md) NO-GO /
[State](GladeRaftQ3Implementation-ReviewState-1.md) GO. All reports are filed
verbatim from the reviewers' completed Markdown; no current peer/closure
reports were shared during review. Canonical prompts use the same source tuple:
[Code](GladeRaftQ3Implementation-PromptCode-1.md),
[State](GladeRaftQ3Implementation-PromptState-1.md), originating
[Code](GladeRaftQ3ImplementationClosure-PromptCode-1.md)/
[State](GladeRaftQ3ImplementationClosure-PromptState-1.md).

New renewal Code P2-1 identifies outgoing-only voter exclusion from manual
campaign enumeration. Legal incoming[4]/outgoing[1,2,3], a partitioned unknown
entry on the outgoing suffix, and physical reopen permanently campaign stale4.
The reviewer classifies this as a bounded implementation defect under the same
manual recovery strategy, not a new architectural root cause. State did not
independently converge on it; no production escape occurred.
[Remediation 2](GladeRaftQ3Implementation-RemPlan-2.md) accepts the counterexample
and authorizes one TDD correction with continued independent re-verdicts.
Q3 acceptance and Q4 production gates remain open.

## Q3 implementation accepted-through — remediation 2

**Accepted algorithm/test source:** root `468fa7725bcd7cf025b4889c37f697f2bfc3d2d2`; unchanged Glade
`90dc1a60981185fa26ae5bfafbbb5377c12a413b`, glade-discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`. Final peer-blind
[Code](GladeRaftQ3Implementation-ReviewCode-2.md)/[State](GladeRaftQ3Implementation-ReviewState-2.md) both **GO**, zero open/new P0–P3 and no mandatory witness gap.
Canonical continued-review prompts: [Code](GladeRaftQ3Implementation-PromptCode-2.md),
[State](GladeRaftQ3Implementation-PromptState-2.md). Completed reports are filed
verbatim from the reviewers' exact final Markdown. Both verified all four HEADs
and clean scoped source/controller bytes at start/end. No current peer report
was shared. This final filing is documentation only; algorithm/test bytes stay
at the accepted source.

Code-2 closes its original outgoing-only candidate counterexample through the
exact actual-file Drop-before-live-Reconnect sequence. State-2 independently
checks the correction, both-majority authority and original closures/prior GO.
The five initial findings were previously closed by originating Code/State on
`ce087642`; final re-verdicts preserve those closures. **Six total implementation
findings closed; two bounded remediation rounds; zero open findings**. No new
architectural cause or third architectural round was identified. Initial axes
found distinct defects; the renewed outgoing-only defect was Code-only, without
blind convergence. All were caught during qualification before production escape.

The three-line enumeration correction uses the deduplicated incoming/outgoing
union before unchanged filters, real-log ranking and legal campaign. No interface,
call graph, publication/mutation boundary, platform, dependency, role, allowlist
or test-selection relaxation was added. Continued intact reviewers were therefore
appropriate; fresh renewal had already covered the prior shared failure/routing
changes. The object was committed and independently verified before dispatch.

Owner and both final reviewers executed **121 default Rust PASS, zero failures,
four explicit tier ignores, zero default filtering**; two explicit Q2 cases PASS;
both parent-oracle self-tests PASS; **two Q2 and three Q3 actual SIGKILL cuts PASS**.
Original nineteen concrete Q3 consumers remain default-selected. Strict Clippy,
architecture, formatting, token/source and process-global gates pass: **49 owned
Rust files, zero exceptions/debt/permanent entries**. Evidence and observations
are recorded in [implementation evidence](GladeRaftQ3ImplementationEvidence.md).

This accepts **Q3a authorized membership/joint recovery and Q3b full-history
checkpoint/snapshot/suffix recovery in the private APFS/process-crash harness**.
Q0/Q1a/Q2 remain accepted. Q4 is next: reviewed Glade production integration,
canonical authority/transport, deployment/failure domains, automatic-election
compliance and complete legacy writer/effect exclusion. Production crate/profile
selection and activation remain open; no power-loss or physical-reclamation
claim follows. No push or desktop rebuild is included in this landing.

## Q4-A production integration preparation

Authorized 2026-10-03: proceed with Glade production integration of **Raft**.
Plan: [Q4](GladeRaftProductionIntegrationPlan.md); first internal store-lifecycle
object: [legacy Store seal](GladeRaftLegacyStoreSealContract.md). Source audits
are fact maps, not verdicts. GWZ local clone `raft-production` preserves all
inherited dirt, which is outside this object. No production activation, push,
start-script change or desk rebuild is included.

Contract tuple root `6a35216a6d97aa22e9d53b54256f7395c306da3c`, Glade
`19a269dd12b5f109d03e2361e3af4108d4b1fcbc`, discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35` received peer-blind
[Consistency GO](GladeRaftQ4ASealContract-ReviewConsistency.md)/
[Safety GO](GladeRaftQ4ASealContract-ReviewSafety.md).
No P0/P1/P2. Consistency P3-1 shared non-Unix recognition coverage was corrected
before implementation and [closed by its finder](GladeRaftQ4ASealContract-ClosureConsistency.md)
at root `3bf731994c8f18e4e27ed568e123bed099dca2f0`, Glade
`638cca4b2784cc3e51c1e47b1fea47d026b57734`. This contract gate accepted
compiling behavioral RED, not an implementation.
Exact evidence and honest fixture-interference results are in
[Q4-A evidence](GladeRaftQ4ASealEvidence.md). No architectural remediation round
has been consumed. Production freeze/activation will need their own reviews.


### Q4-A implementation accepted-through

Exact reviewed root `aafb14a663fe130db5ef56cb002b2bb9b9399b11`, Glade
`c8c0613f645dd4b6aaf546f586d77cfdb76a0c87`, unchanged discover
`52ea2d118f45d9e7c3d9a789310dd5d669958851` and external Gyld
`ca04499a360d910fbf8ee2540ed446facd051b35`; additional affected-consumer
pins are enumerated in both reports. Fresh peer-blind
[Code GO](GladeRaftQ4ASealImplementation-ReviewCode.md)/
[State GO](GladeRaftQ4ASealImplementation-ReviewState.md), zero findings.
The lane owner accepts only the internal whole-Store retirement interlock for
participating builds on a stable root in the exercised macOS/APFS
process-interruption profile. Exact committed-source full gate: nine components
PASS, 507 cases per composition root across 20 binaries. Each reviewer separately
reran the full gate and actual seal/process-kill targets; old fixed-name Store
fixtures required serialized test windows. Inherited cold-join dirt is excluded.

| Discovery phase | Findings/disposition | Architectural remediation rounds |
| --- | --- | --- |
| Contract | One P3 platform-selection coverage defect, original finder closed | 0 |
| Implementation acceptance | No P0–P3 findings | 0 |
| Broader production | Q4-B/C/D/E, RA-012, authority, carrier and failure domains remain open | No acceptance claimed |

Test-driver and fixture failures remain recorded in evidence; passing reruns did
not erase them. No released/activated production profile exists to support an
escaped-defect claim. No baseline/policy/allowlist was relaxed. The accepted code
was already committed in the local clone; this acceptance filing records the
verdicts without changing source. It does not merge/push, seal live storage,
rebuild the desk or alter the two-node development launcher.
