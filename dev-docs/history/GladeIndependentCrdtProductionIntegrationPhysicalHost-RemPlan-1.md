# Combined IC-3B physical host — merged remediation 1

Status: **NO-GO; one test-first correction patch authorized.** This plan closes no
finding. Reviewed root `db29535fe162356df93fbdc28c052185cfd9a0f7`, Glade
`9dbc677a25d93af0c561817571ad7eb1990ddadc`, Glial
`348eed97cd1ee4f677ea2866dfabe5a81cbebee1`, discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`, external Gyld
`400cedcf1fff74128f366af758a0c389a23435d7`.

The complete PhysicalHost ReviewCode/ReviewState/ReviewSurface reports are filed
verbatim alongside this plan. Code and State each returned NO-GO with three P2
findings; Surface returned GO with one nonblocking P3. The lane owner merges all
findings into the following five roots, four blocking. No finding is disputed.

## Dispositions and closure witnesses

| Root | Findings | Disposition | Required closure evidence |
| --- | --- | --- | --- |
| B-R1: receive-result provenance; architectural typed root | Code P2-1; State P2-1 | Correct the issuer-owned completion capability so normal settlement MUST match the exact successful received bytes. Authenticating a separately supplied observation is insufficient. Failed, partial, unclassifiable or unretainable input MUST remain guarded or enter conservative retained loss. Caller-authored flags/digests MUST NOT supply authority. Preserve nonmutating eligibility, original continuation custody on refusal/Unknown, and retirement only after durable selection. | Compiling behavioral RED through external public DiskHost/session consumers for genuine nonempty→different signed empty, malformed prefix→signed empty and receive error→signed empty. GREEN MUST refuse normal substitution and preserve incompleteness, including actual kill/reopen. Exact empty and nonempty controls MUST still settle. Both originating reviewers verify their original counterexamples; fresh full axes inspect the changed capability. |
| B-R2: omitted durable current-observation step; nonarchitectural B root | State P2-2; Code P2-3 | Retain every validated monotonic policy/time observation from enclosing challenge, response and ingress operations, including their refusal paths. Use conservative floor-ahead recovery when full-image publication cannot finish; failed retention MUST leave the host unavailable/uncertain and MUST NOT allow restart to restore older authority. Receive-only ingress MUST durably advance its own observation without acquiring append rights. A changed guard base requires a bounded reload/retry that converges under stable valid input before any consumption. | Public challenge/response/ingress denial with advanced time and chained revocation; actual child death/reopen MUST refuse lower time/older policy. Genuine receive-only policy with empty append grants MUST establish ingress after normal time and policy advancement with bounded retry; restart, regression, revocation, capacity and publication-failure controls MUST return no consumable permit when unsafe. Preserve historical receipts, exact retry and original protected Started cuts. |
| B-R3: retired original prepare custody; nonarchitectural B root | Code P2-2 | Authenticate the complete exact issued Prepare request against live OR retired issuance custody. Recover the existing immutable PlanKey/AttemptBinding before fresh-allocation revision checks. Mutated, foreign and unissued requests MUST still refuse. AttemptId-only/high-water authorization and duplicate allocation are forbidden. | All four native kinds: original prepare/recover_plan before and after callback retirement, unresolved and terminal attempt, and process reopen. Assert original AttemptBinding, unchanged counters/charges and one attempt. Mutated/unissued negative controls; originating Code verifies. |
| B-R4: public retained-continuation loss discharge; nonarchitectural B root | State P2-3 | Provide a host-owned public route from an existing drained refused settlement continuation to conservative loss, preserving the original guard/permit custody. An explicit retained-operation conversion or automatic permanent-refusal transition is permitted. No reminting, clearing undrained guards, inference from absence or bypass of unresolved physical publication is permitted. | Public API only: aggregate-capacity and malformed-observation refusal → retained loss → Closed → reopen with permanent loss and complete_local=false. Failed loss publication MUST retain the original authoritative guard and continuation. Private pending-map extraction is not a closure witness. Originating State verifies. |
| B-R5: fresh-root legacy compatibility discoverability; nonarchitectural B documentation root, nonblocking | Surface P3-1 | Explain the fresh disposable root's legacy-store.sealed marker, ownership, expected old-binary refusal and supported reopen direction alongside provision/reopen. Marker deletion MUST NOT be represented as qualified downgrade. Do not imply existing-store migration. Include in the same patch; no standalone P3 package/round. | Cold documentation-only create/reopen/old-binary walkthrough answers marker ownership, refusal and supported direction without source knowledge. Surface assesses the revised consumer guide. |

Code and State independently converged on B-R1. Their ingress findings also
identify the same omitted durable-observation step: State shows restart can
forget an advanced cut; Code shows a legitimate receive-only scope cannot
advance. B-R2 merges that implementation omission while retaining BOTH distinct
counterexamples and required tests. It does not merge unrelated native prepare
or retained-loss custody defects.

## Controlling-object accounting and caps

The semantic object remains **two architectural roots, one nonarchitectural
root, one completed merged remediation**. Its exact-observation and monotonic
floor requirements already cover these failures; no semantic root is added.

The typed object now has **two architectural roots and five prior
nonarchitectural roots, one completed remediation**. B-R1 is the reviewers'
second typed architectural root, distinct from prior generic physical-type
identity. This correction consumes the **second typed remediation** when
completed; a B filename MUST NOT reset that accounting. Original A2 R1–R6
originating closure remains owner-deferred to the wider ABC review.

Combined B now has **zero architectural roots, four nonarchitectural roots**
(B-R2–B-R5), **zero completed remediation rounds**. This authorizes its first
merged remediation. There are four blocking roots across the controlling
objects, five total. No third architectural root has been classified. A
reviewer-classified third typed architectural root MUST STOP the lane before
another correction and be reported for the owner's redesign-or-accept decision.
The review-loop's bounded nonarchitectural exception is not permission for
additional architecture work. Historical caps and stopped objects remain intact.

## Single patch, evidence and gates

The same sole drafter MUST implement one scoped patch after compiling behavioral
RED for each blocking root. Compile errors, zero-selected tests and writer claims
do not close findings. Update actual API contracts and cold usage for changed
capability/lifecycle shape. Preserve every original assertion, released operation
and corpus byte, identity binding, receipt guarantee, dependency pin,
classification, process-global allowance and owner-approved five-second budget.
Do not relax checks, selectors or allocation bounds to make tests pass.

Run affected contract/utility consumers and actual physical host consumers first,
then relevant native/ingress kill/reopen matrices, original authentication and
assembly controls, adopted architecture/negative fixtures, disabled-scope/source,
affected formatting, Clippy and process-global gates. Repeat broader checks only
where the shared capability/publication change invalidates prior evidence.
Record whole-node formatting/Clippy baseline debt honestly. Preserve all failures
and chronological RED/GREEN commands in new remediation evidence and source
pins; do not rewrite initial B evidence, source pins, run log or reports.

Parent owns GWZ settlement. The drafter MUST NOT stage, commit, push or mutate
workspace metadata. Existing stores/keys/floors, defaults/launcher, desk rebuild,
activation, Raft and owner-excluded BOM work remain outside scope. Orphan Active
guards after unclean death remain conservative pending/incomplete; no timeout,
lease, drop or restart supplies drain/absence proof.

## Corrected checkpoint and independent closure

The corrected capability and public lifecycle move shared boundaries. Parent
MUST freeze a clean exact tuple, then dispatch fresh peer-blind full Code/State
and cold Surface reviews of the entire combined component. Fresh prompts MUST
be generated from the canonical template; full-review verdicts alone MUST NOT
stand in for originator closure.

The original B Code and State reviewers MUST each independently verify their
own three counterexamples and regression evidence on that identical corrected
tuple, with complete prior-finding closure tables and changed-range analysis.
Their legitimate merged plan is a re-review input; current peer testimony is
excluded. Surface P3 correction is assessed within the same wider surface gate,
without creating a separate package. File every complete report verbatim.

Only actual GO and required originating blocking closures allow accepted B
qualification metadata. The strict pre-remote gate MUST remain refusing until
that real committed acceptance record and pinned genuine evidence exist.
Automatic two-node history exchange (C) remains next, after this gate; no C
execution or acceptance is claimed here.
