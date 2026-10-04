# Independent CRDT production integration — IC-3 review and delivery ledger

Date: 2026-10-04. Status: **owner authorized IC-3A, IC-3B and IC-3C; IC-3A semantic design accepted; typed gate and IC-3B/C remain open**.

The owner directed “Proceed IC-3ABC” after the deterministic IC-2 kernel was
accepted and its source/checkpoint history pushed. This authorizes design,
review and implementation of real scoped evidence, process-restart persistence
and automatic two-node application exchange under accepted contracts. It does
not authorize desk activation, existing-store migration or stronger storage
receipts. A known authenticated resource and text profile bound the first real
witness; they do not bound the general multiwriter capability.

Baseline: workspace `8c8332ad94ba42b43144830b6c3809c45d0b7bbd`, Glade
`37dff286ce1eb9690204d4a7d14c940333396a30`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`95a426595bba8e248a5f484272e483a070c73918`. IC-2 reviewed tuple, closures and
limitations remain in [its ledger](GladeIndependentCrdtAdmission-ReviewCycle.md)
and [archived component record](history/GladeIndependentCrdtAdmissionKernelImplementation.md).
Moving documentation did not amend any contract; historical references remain
unmodified as the owner requested. New references MUST name the actual location.

## Recorded gates and ownership

| Checkpoint | Recorded tier | Acceptance requirement |
| --- | --- | --- |
| IC-3A semantic production-adapter design/amendment | Fresh dual Consistency/Safety | Exact changed clauses, implementable evidence/storage/reconstruction/exchange contracts, all blocking findings closed on one committed tuple. |
| IC-3A typed boundaries/allocation and behavioral RED | Fresh dual Code/State | Meaningful interfaces, justified roles/dependencies and source-qualified Gyld allocation; actual compiling behavioral RED and retained consumer controls. No implementation before acceptance. |
| IC-3B authenticated physical host/recovery component | Fresh dual Code/State | Real signatures, disk atomicity/coupled outcomes, process kill/reopen and authoritative continuation proof; scoped gates and all original consumers. |
| IC-3C automatic duplex real-node aggregate | Fresh dual Code/State | Real separate processes authenticate, edit while partitioned, exchange without client ferrying, converge valid operation sets/text, survive restart/retry and reject wrong scopes/profiles; focused and aggregate evidence. |
| A user-facing interface/CLI/file-format freeze, if introduced | Additional Surface | Cold help/docs lifecycle/defaults inspection; IC-4 browser/compatibility and activation remain separate. |

Review-loop skill and canonical template control. One source/design drafter owns
the packet; lane owner owns this ledger, git, prompt generation, verbatim report
filing and verdict merge. Reviewers are fresh, independent, read-only and peer
blind each round, and verify exact HEADs at start/end. Strongest tiers inherit the
parent session; no cross-family selection. Completed prompts/reports/remediation
evidence SHOULD be filed under dev-docs/history for this cleanup convention.

Each new object starts at zero remediation rounds. No earlier stopped contract
object or IC-2 count is reset. At most two merged remediation rounds; the skill's
exception permits a third confined nonarchitectural correction, and any
architectural root in that third correction stops. A reviewer-classified third
new architectural root requires STOP and owner redesign-or-accept decision.
Originating reviewers alone close findings; writer GREEN is evidence, not closure.
Material boundary changes require fresh full axes.

## Immediate action

Source-ground the IC-3A design in current admission/storage API and actual
node signing, persistence, ownership, carrier and lifecycle seams. Explicitly
resolve retained observation and authoritative full-cut reconstruction before
claiming a sticky completeness marker can clear. Choose bounded injectable
development trust/time/storage/transport inputs; do not invent production keys,
quotas or live configuration. Tests precede implementation. Normal independent
package loops MUST remain fast; no role, dependency, selector, budget or
process-global allowance is loosened to make checks pass.

## IC-3A semantic packet settled for initial review

One source drafter completed the
[design](GladeIndependentCrdtProductionIntegrationDesign.md) (450 lines; SHA256
`e96d4ffc0bf8a8a213d4bb0d58d9f5e595604e80ca4ff71925dd4df5904eafc0`) and
[plan](GladeIndependentCrdtProductionIntegrationPlan.md) (195 lines; SHA256
`3cd2255bbc0e124bb7ffae77db41bba9b26abadfa977fe29585d40a355b96f1c`).
New links resolve and whitespace passes; source, archives, old references,
manifests and classifications remain unchanged. This is design/source inspection
evidence only, no crypto/disk/network execution or compiling typed gate.

Packet selects the bounded genuine direct-root authorization profile, complete
versioned per-instance images plus independently trusted floor handshake, same
logical-owner process reattachment under physical lifetime ownership, complete
retained callback/outcome/custody state, actual production node routing/lifecycle
and real Iroh symmetric full-history exchange. A narrow combined cut can complete
only durably retained adapter obligations; the accepted kernel sticky flags have
no clearing event and remain untouched. Those choices are DRAFT, not self-approved.

The first fresh dual Consistency/Safety review MUST verify exact source-qualified
contracts and implementation feasibility on a committed five-repository tuple.
Prompts are generated from the canonical skill template after commitment; exact
root SHA appears there. Other pins remain the baseline above. During review only
current generated prompts and verbatim report outputs may be untracked. Zero
remediation rounds/architectural roots initially; neither implementation nor
interface creation is authorized by writer completion. No push or live changes.

## Initial IC-3A semantic review — dual NO-GO

Both read-only reviewers verified the root `4d641dd179e5b0bd94e84df9cb334d7a21dae371`
and all four unchanged source pins at start and end. Their complete testimony
is filed verbatim in history:

- [Consistency](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewConsistency.md),
  SHA256 `ae3dc3e72e9cbc858982324f85e9e173649943e838a5a05e72c66eb061a60bdf`:
  NO-GO, three P2 findings.
- [Safety](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewSafety.md),
  SHA256 `7dda23b87d73a36eac9db41f8aaed2bfc3c180a616a68ffcad58231db69b5f46`:
  NO-GO, one P2 finding.

Four IDs represent three distinct roots. Consistency P2-1 and Safety P2-1
independently converge on the same architectural root: history can be consumed
before a durable ingress discriminator exists, and failed observation plus
failed loss-marker writes allow false completeness after restart. Consistency
P2-2 is a second architectural root, incomplete authoritative declaration/schema
identity composition. Consistency P2-3 restores the required pre-remote-use
Rust/TS/Python canonical-vector gate and is nonarchitectural. There are two
unique architectural roots, not four; no third root is established.

[Remediation 1](history/GladeIndependentCrdtProductionIntegrationDesign-RemPlan-1.md)
maps every ID to a correction and closure witness. One drafter edits the design
and plan as one patch. Completed remediation count remains zero until that
patch settles; round 1 is authorized. The original reviewers MUST verify their
own counterexamples on the corrected tuple. Because durable consumption and
authenticated identity boundaries change materially, fresh full
Consistency/Safety review is also required on that same tuple. This remains
the same semantic object and preserves the two-root count and all historical
caps. A reviewer-classified third architectural root requires STOP before
another patch. No source/interface implementation is accepted or begun.

The aggregate IC-3C gate also requires fresh Consistency/Safety verification
of the semantic contract in addition to the recorded Code/State review, as
specified by the delivery plan; this adds evidence and does not replace an axis.

## Semantic remediation 1 — settled, not self-accepted

The sole drafter froze one two-document patch: design 659 lines, SHA256
`c476dd10dab4618710ba02620ea71a75f20b11570c373bb685ea13320b3909f2`;
plan 232 lines, SHA256
`81566df8b4eec445fe3fd1fd163af4534f8240c0a823cc348ae577510ee62f62`.
The patch adds IC3-GUARD-001–003 durable pre-consumption receive lifecycle and
crash grammar, IC3-ID-001 complete authenticated namespace mapping with production
IdentityBinding, and IC3-CANON-001 mandatory independently pinned Rust/TS/Python
vectors before remote use. Operator-visible configuration/API shape expressly
requires Surface even when called private. Existing source, old tests, canonical
Op bytes, archived files/references and classifications remain unchanged.
New links and whitespace pass; there is no physical execution evidence.

Completed merged remediation count is now ONE. The same semantic object retains
TWO unique architectural roots and ONE nonarchitectural root; no finding is
self-closed. Fresh full Consistency/Safety reviewers and the originating reviewers
independently checking their own counterexamples MUST use the identical newly
committed five-repository tuple, compare the full change range from initial root
`4d641dd179e5b0bd94e84df9cb334d7a21dae371`, remain peer blind for current testimony,
and classify any new roots against the existing root set. Exact root appears in
generated prompts. Third new architectural root requires STOP without another
patch. No typed/API/provider implementation begins before semantic acceptance.

## IC-3A semantic acceptance

Status: **semantic design accepted at root
`06b16c9e17ff5507268a5823fad9a0be70a36790`, with unchanged Glade/Glial/discovery/Gyld
pins above, after fresh full Consistency/Safety GO and both originating closure
GO; this accepts semantic production-adapter design and exact amendments ONLY**.

| Required review | Result | Verbatim artifact |
| --- | --- | --- |
| Originating Consistency | GO; P2-1/P2-2/P2-3 closed semantically | [Closure](history/GladeIndependentCrdtProductionIntegrationDesign-OriginatingClosure-ReviewConsistency-1.md), SHA256 `ece3feec09507615c01f6dce98de0fb3bdc146a7a941c30c3b2b5c171f1c6c9a` |
| Originating Safety | GO; P2-1 closed semantically | [Closure](history/GladeIndependentCrdtProductionIntegrationDesign-OriginatingClosure-ReviewSafety-1.md), SHA256 `c9c321c8cfa4c4a930a54107989c22c9fc9af7e0ab97e05f7b8c091e67164609` |
| Fresh full Consistency | GO; zero new P0–P3 | [Review](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewConsistency-2.md), SHA256 `756d2e83deeb9c1366fecfb3d3e8a650aa61cf728d28d48e9eb387fdc3fca9e9` |
| Fresh full Safety | GO; zero new P0–P3 | [Review](history/GladeIndependentCrdtProductionIntegrationDesign-ReviewSafety-2.md); byte-exact testimony filed, hash recorded below |

All four reviewers independently verified the same five HEADs at start and end,
read the complete changed context, performed read-only inspection and excluded
current peer testimony. No typed, cryptographic, physical disk or actual network
execution is claimed. Semantic closures require the specified later executable
regressions. Two architectural roots and one nonarchitectural root were found
before acceptance; one merged remediation round closed them. No third root or
escaped defect is established. Historical stopped objects/counts remain intact.

Next: authorized A2 contract/data/structural-codec/allocation tranche with tests
first, protected existing consumers, actual production refusing-provider behavioral
RED and mandatory independent canonical-vector gate. The newly typed boundary is
a separate recorded Code/State checkpoint; this does not reset the semantic
object's retained root/cap record or authorize evading a newly discovered semantic
counterexample. No successful physical/auth provider or duplex starts before its
typed gate. IC-3ABC delivery remains open; no push/live/desk changes.

Fresh full Safety testimony SHA256: `49dbc20b4f92c9bdf7b46a05d17256129f884560ea3bf9a0ca24897c149de325`.

## IC-3A typed checkpoint — drafting released

Semantic acceptance landing root is
`3bff20351895b82e2f805e7f8939dab6b3296604`; four source pins remain the accepted
IC-2 baseline. The same sole drafter is released to A2 only: exact data extraction,
required evidence/recovery/guard interfaces, bounded structural codec and three
independent canonical consumers, production-bound refusing provider consumers,
source-qualified external Gyld overlay and focused gate adoption. The parent owns
this ledger, checkpoint staging and review dispatch. Test-first source/evidence is
required. Successful real auth/disk/duplex adapters remain behind typed acceptance.

The compiler-facing object begins at zero remediation rounds and zero discovered
roots. This records its separate initial gate, not a reset of the accepted
semantic object's two architectural/one nonarchitectural roots or historical
stops. A new semantic counterexample MUST retain the appropriate controlling
object's cumulative accounting. Fresh dual Code/State is the recorded typed gate;
Surface is additionally required for any actual operator-facing freeze. All
original consumers/assertions, roles, dependency direction, budgets and process
global restrictions remain protected. IC-3ABC completion remains open.

The new compiler-facing contracts expose public consumer APIs for open, owned
receive, settlement/abandonment and close. At A2 freeze an additional independent
Surface review MUST inspect their cold consumer usage/lifecycle documentation,
without implementation or design-plan context, alongside the dual Code/State gate.
This resolves the already recorded conditional API-surface requirement before
freeze; it adds an axis and does not replace either core reviewer. It establishes
no browser/CLI activation surface. Actual B/C operator configuration will receive
its own Surface assessment at the appropriate freeze.

### Owner-approved Gyld fast-test budget adjustment

On 2026-10-04, the owner explicitly directed “up it to 5 seconds” after the
179-test architecture selection passed its assertions but exceeded Gyld's
existing two-second execution budget (2.978 seconds). The A2 patch MUST change
only the single-selection execution threshold from 2.0 to 5.0 seconds. The
multi-selection 10.0-second threshold, separately measured startup, and separate
I/O suite remain unchanged. This is an owner-authorized project-specific budget
adjustment, not an agent relaxation to hide a failed gate. The prior failed run
MUST remain recorded; an executable threshold regression MUST precede the edit,
and the revised gate MUST pass before freeze. The change is included in the
settled typed review object and does not change Glade's budgets or classifications.

## IC-3A2 typed implementation — frozen source, acceptance open

The sole drafter froze 58 Glade implementation files and four controlling/consumer
documents. The lane owner independently checked every recorded SHA256 against
the working bytes: zero mismatches. Glade's implementation checkpoint is
`7d26ba6e6d133db650a37ff49eba71644ea370a6`, committed through GWZ. Excluded vector
pin metadata and external Gyld sidecar qualification MUST now name that exact
source commit; a later metadata-only commit MUST preserve all 58 source hashes.
This is a review checkpoint, not acceptance or physical qualification.

Recorded scoped evidence includes four new crate suites, protected original core
and storage consumers, unchanged released text/corpus checks, contract adopter
tests/format/Clippy, 16 architecture-refusal fixtures, node architecture and actual
assembled refusing consumers, disabled-branch syntax checks, and unchanged
process-global allowance checks. Three independent representation consumers cover
21 semantic positive cases and 18 malformed/closed-profile negatives. Symbolic
signature bytes qualify encoding only. The strict pre-remote prerequisite MUST
remain refusing until committed IC3-B1 genuine-signature qualification exists.

Gyld preserves its original 34 allocations/135 obligations and adds five
allocations/25 requirements, giving 39/160. Its owner-approved threshold regression
checks both sides of five seconds, preserves the ten-second multi-selection tier
and separate startup/I/O behavior. The first revised 181-test run failed at 5.251
seconds during concurrent Rust checks; a subsequent isolated final run passed at
2.860 seconds. Diagnostic selections measured 176 existing tests at 2.081 seconds
and five additions at 1.092 seconds. No cache or test-selection change was made.
Both failed runs MUST remain in the chronological evidence. Pinned affected Ruff
checks pass. Mypy still reports six errors; an exact unchanged HEAD baseline has
the identical six errors, with no new errors attributed to this patch. This debt
is recorded explicitly rather than calling the type gate green.

Next: finalize provenance-only metadata, settle the full five-repository tuple,
then fresh peer-blind Code/State and independent cold docs-only Surface reviews.
The typed object has zero discovered roots and zero remediation rounds before
those reviews. A2 acceptance requires every required verdict; the drafter's tests
and these checks cannot substitute for reviewer testimony. IC3-B/C successful
providers, real crash/restart and two-process exchange remain unimplemented.

Provenance-only final Glade HEAD is
`8b0595551dd90f32da4698fff9358aa30dbf2548`; the sole difference from implementation
`7d26ba6e6d133db650a37ff49eba71644ea370a6` is the vector pin metadata. All 58
implementation hashes match both commits and the working bytes. External Gyld's
six-file checkpoint is `03428fb36649d71541fd76f2471533e384be8209`, settled through
GWZ with its parent lock captured. Glial and discovery remain at the unchanged
accepted pins. Final evidence is
[TypedEvidence](history/GladeIndependentCrdtProductionIntegrationTypedEvidence.md),
with 91 chronological attempts and the byte-exact compressed raw output; original
45-file compatibility hashes and the frozen source manifest are filed alongside.

Surface scope additionally includes the new source-qualified Gyld capture CLI's
cold `examples/README.md` section and its `--help`, alongside the standalone
TypedUsage document. The reviewer MUST NOT read implementation or design/plan;
the full tuple is identical to Code/State. This additive scope records the actual
helper command surface before freeze and does not introduce production activation.
Generated canonical prompts will name the exact root checkpoint. No current peer
testimony may be read or requested. Successful real B/C implementation remains
blocked on this acceptance gate, not on renewed owner permission.

## IC-3A2 initial typed verdict merge — NO-GO

All three reviewers independently verified the settled tuple at start/end.
Code reports two P2 and two P3; State reports three P2; Surface reports GO with
two P3. Complete unchanged testimony is filed in history: TypedContract-ReviewCode
SHA256 `5d54cc16e21dd347d5035a2312ec57a54e6830c9a48f46706c73a2b18d92594b`,
TypedContract-ReviewState SHA256
`dd4645bb0bb565c28bdfa68055e7cff2bdc6aecdf41aa2edff3d20dc3319cb8c`, and
TypedUsage-ReviewSurface SHA256
`761d5c6feaab380fa0f502f2b06b0ffba54629cbf02066d907d060cd4e269122`.

Blind convergence establishes the generic physical type-erasure and embedded-Op
budget roots. Both axes also identify standalone refuser assertions mislabeled
as assembled consumers; State's P2 severity governs that shared root. The merge
contains THREE unique blocking roots, ONE architectural typed root and FIVE
nonarchitectural roots total, with zero completed typed remediation rounds.
No new semantic root or third architectural root is established. Initial source
GREEN and this ledger's prior assembly evidence attribution do not close the gap.

[RemPlan-1](history/GladeIndependentCrdtProductionIntegrationTypedContract-RemPlan-1.md)
disposes every finding into one authorized test-first correction patch, including
bounded help/usage fixes without separate P3 packages. Existing reports/evidence
remain intact; new evidence MUST explicitly correct the original assembly claim.
The same sole drafter is released to that patch only. New shared format boundary
requires fresh full Code/State, originating blocking-finding closures and the
updated docs-only Surface verdict at one new settled tuple. Successful real B/C
implementation is still withheld pending typed acceptance.

## Owner directive — defer corrected typed review to wider review

On 2026-10-04 the owner explicitly directed: “defer the review as part of a wider
review - proceed as accepted/”. This overrides the standalone typed re-review
and originating-closure dispatch specified by RemPlan-1 and the review-loop skill.
The current correction decision is owner-accepted. The sole drafter MUST finish
the one mapped patch and its test-first/scoped verification; the lane owner MUST
record the exact resulting committed tuple before releasing successful B/C work.
There is no renewed approval request or separate typed re-review at that point.

The record MUST distinguish **owner acceptance with independent closure deferred**
from reviewer GO. Original NO-GO testimony and historical evidence remain intact.
R1–R6 dispositions and regressions travel into the wider IC-3ABC aggregate review;
original P2 findings are not described as independently closed until that review
verifies their counterexamples. Typed accounting remains one architectural/five
nonarchitectural roots and first merged correction in progress; semantic accounting
is unchanged. The directive does not waive implementation tests, genuine crypto,
mandatory independent canonical consumers, physical kill/restart evidence or
actual two-node qualification, and does not authorize activation or push.

Next after the completed corrected tuple and checks: proceed with IC-3B genuine
authenticated persistence/recovery and IC-3C actual automatic exchange. The wider
review MUST include the deferred corrected type boundary, cumulative embedded-Op
budgets, actual assembly consumers, argument/help/usage surface and their original
closure checks alongside the new production integration. No reviewer GO or
cryptographic/durable qualification may be fabricated from this owner directive.

## IC-3A2 corrected checkpoint — owner accepted; closure deferred

The mapped first typed remediation is complete. Implementation Glade commit
`3bf66efecc59fd24261e13787e4c8fc3f822ca57` is followed only by vector pin metadata
at settled Glade HEAD `c69e6416f5f5155d4bb570bf272e796b2deae0a3`. All 59 source
hashes match both commits and working bytes. The four active document hashes and
four external Gyld file hashes match the new Remediation1SourcePins manifest.
External Gyld HEAD is `400cedcf1fff74128f366af758a0c389a23435d7`, with its workspace
lock captured at external root `9765f891f4b5b1ba9d1fc8752f6ab9e5ca64a6d1`.
Glial remains `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery remains
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`. This ledger's root commit settles
the corrected tuple; no push or production activation is included.

Parent verification confirmed all seven initial typed evidence/review artifacts
byte-exact against root `934bf93b45e97065ada9d971400e0a3b6ca3b586`. New source,
compatibility, evidence and chronological raw output are filed separately as
`history/GladeIndependentCrdtProductionIntegrationTypedRemediation1*`. The source
patch binds recursively concrete physical type identities, enforces cumulative
embedded-Op budgets before construction, exercises actual assembled refusers,
closes qualification argument parsing and clarifies recovery/capture usage.
Mapped regressions and affected checks pass. Gyld single-selection execution is
2.896 seconds within the owner-approved five seconds; Mypy retains the identical
six baseline errors and is not claimed green. Future successful-provider behavior
remains explicit compiling RED. The strict remote gate still refuses absent
genuine B1 evidence; symbolic corpus agreement is not cryptographic qualification.

Decision: **OWNER ACCEPTED; INDEPENDENT CLOSURE DEFERRED** by the explicit directive
above. No new reviewer GO is claimed. Typed accounting is one completed merged
remediation, one architectural and five nonarchitectural roots; original NO-GO
testimony and deferred R1–R6 closures remain in the wider review's scope. Semantic
accounting remains unchanged. Successful IC-3B/C implementation is now released
to the same sole drafter under the accepted contracts and test/evidence gates.
Any standalone review scheduling overridden by this directive MUST remain distinct
from genuine provider qualification and the wider aggregate review.
