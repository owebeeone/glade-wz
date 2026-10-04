# Combined IC-3B Physical Host Originating Closure 1 — CODE-AXIS REVIEW

**Review object:** Originating Code closure of the three initial P2 findings, including the complete remediation range `9dbc677a25d93af0c561817571ad7eb1990ddadc..d3fded040d6e3c459cd5f975d6c672873356cc23`. Controlling DRAFT: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `c8d778e7c24b68690e27bfac8ca4457f647e747f`. This is the same combined B object after its first merged correction and the typed object’s second correction.

**Baseline:** Root `c8d778e7c24b68690e27bfac8ca4457f647e747f`; Glade `d3fded040d6e3c459cd5f975d6c672873356cc23`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `400cedcf1fff74128f366af758a0c389a23435d7`. All five heads matched at START and END. Inspection used scoped Git diffs, source reads and read-only Python archive/hash analysis.

**Date:** 2026-10-04

**Axis:** Code — independent verification of originating counterexamples and complete changed-range analysis. Independent, adversarial, read-only. Parallel full and closure reviews supply no evidence here. Filed verbatim by the lane owner.

**Verdict: NO-GO** — all three original counterexamples are closed, but one new P2 correctness finding blocks the corrected candidate. This focused closure does not replace fresh full acceptance.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| P2-1 | Private issuer-owned successful receive witness; normal settlement compares exact bounded bytes before I/O | Retraced nonempty→different signed empty, malformed prefix→signed empty and receive error→signed empty through public DiskHost/session consumers. Recorded compiling RED reproduced all three original assertion failures. Corrected comparison refuses each; actual child death/reopen retains Active guards and incompleteness. Exact empty and nonempty controls still settle. | **CLOSED** at this tuple |
| P2-2 | Authenticate complete original Prepare against live or retired issuance before existing-plan recovery | The original public retired request now reaches retained PlanKey/Binding deduplication. Recorded RED returned `CallbackMismatch`; GREEN returns the original AttemptBinding before and after reopen. Four genuine native kinds cover live/retired Reserved and terminal requests; replay changes no image, counters, charges or phase. Mutated/unissued negatives refuse. | **CLOSED** at this tuple |
| P2-3 | Receive-only ingress durably advances current policy/time without append permission; changed guard base requires bounded reload/retry | Retraced empty-append-grant policy with time advancement and a valid chained policy. The final public regression converges within three attempts, retains generation 1/time floor 22, reopens that exact cut and establishes another ingress round. Local challenge remains denied. Failure/revocation controls provide no unsafe permit. | **CLOSED** for the original liveness counterexample; new P2-4 below concerns authorization under two different observations |

These closures result from source tracing and audited executed evidence, not the writer’s disposition claims.

## Changed-range analysis

The correction changes exactly 24 Glade files. I inspected the complete production and test diff, all new consumer files, the copied prior decoder, and surrounding call paths. The original combined-component inspection remains applicable to unchanged source: committed hashes match the full 62-file manifest, and the exact changed-file list matches Git.

The production changes introduce four connected behaviors:

1. `IngressAuthority` privately records successful bounded receive length/digest. Normal host settlement invokes `can_settle_received`; drained eligibility remains available for conservative loss.
2. A closed `Ic3DiskFloor/v2` observation marker precedes current-source consultation. Pending observation recovery requires a complete authenticated next image; missing-next recovery refuses instead of forgetting learned authority.
3. Prepare lookup includes unchanged retired requests.
4. A public concrete method converts the host’s existing drained refused settlement continuation to retained loss, preserving its original permit/observation/checkpoint through failure.

The marker materially changes recovery availability. The merged plan explicitly supersedes two original assertions: missing-next Observation can no longer yield usable reopen, and learning an oversized policy can no longer leave cached authority usable. Revised tests preserve selected-image, critical-space and advanced-floor guarantees while asserting stronger refusal. This is an explicit lifecycle amendment, not an unrecorded weakening.

Root documents update these capabilities, the concrete loss route, bounded ingress retry and supported compatibility direction. The private v2 grammar rejects v1 rather than silently migrating. The copied v1 decoder is the exact original production prefix and rejects v2. GWZ changes record the corrected Glade member commit and settlement metadata. No dependency, role, process-global allowance, released operation/corpus or default route change was introduced.

The new finding is a **nonarchitectural B implementation root**: the ingress call path authorizes with one trusted interval while its persistence helper consults and retains another. Existing contracts already provide the required current interval and durable cut. No semantic or typed architecture change is needed to correct that mismatch.

Accounting therefore remains attached to the controlling objects:

| Object | Accounting after this report |
|---|---|
| Semantic | Two architectural, one nonarchitectural; one completed remediation |
| Typed | Two architectural, five prior nonarchitectural; two completed remediations; original A2 closures remain deferred |
| Combined B | Zero architectural, five nonarchitectural roots including new P2-4; one completed remediation |

No third typed architectural root is classified. Completed correction rounds are not acceptance records.

## 0. Evidence base

I used the complete closure prompt, my original filed Code report and the legitimate merged remediation plan. Applicable process and architecture instructions, canonical review-loop instructions, controlling Design/Plan/Typed/Authentication/Persistence documents and original evidence remain the baseline. I read the corrected consumer contracts and relevant current-cut, guard, storage and publication clauses. No current full-review, peer closure or other current testimony was read.

Changed source inspection included:

- `crdt-recovery-api/src/ingress.rs`, public utility regressions and expanded disabled-source coverage;
- complete corrected physical metadata and paired publication/recovery paths;
- complete Records authentication, runtime and recovery-query paths;
- ingress establishment, settlement, retained-loss conversion and native prepare/start paths;
- all new external remediation consumers, four-kind replay controls, observation-marker process cuts and modified original tests.

I independently checked all 62 combined source hashes against both working files and the committed Glade revision, all five consumer-document hashes against working files and committed root, and the exact 24-file remediation list. All matched. The 45 retained compatibility files, 27 canonical codec pins and nine initial B artifacts also matched their manifest hashes.

The correction log is 581,222 uncompressed bytes, SHA-256 `b276e0f2349a9536b2eae265b7b0bb4ac91d5433715ab51fe705135232b98f5b`; gzip SHA-256 is `fdb8fa4f9631e8299f0846cf72c72f7e128c2d2046b895f54c79f69d7e9c3862`. I inspected its chronological command/result index and relevant complete failure, regression and gate records. Compilation failures and zero-selected runs were not treated as behavioral evidence.

Audited results include:

| Recorded command or selection | Result and relevance |
|---|---|
| Initial public substitution regressions | Compiled; three expected assertion failures, exact nonempty control passed |
| Initial exact retired Prepare regression | Compiled; original `Refused(CallbackMismatch)` failure |
| Initial receive-only observation regression | Compiled; bounded convergence failed at its intended assertion |
| Corrected public disk suite | 22 passed, two owned child helpers ignored |
| Final chained-policy/time receive-only selection | One selected test passed, 2.344 seconds wall |
| Four genuine kinds; live/retired Reserved controls | Both scoped selections passed |
| Final Records selection | 28 passed, five owned helpers ignored; 121.675 seconds wall |
| Process-cut matrices | 228 native, 65 ingress, 26 observation cuts; five public substitution/loss deaths plus chained-revocation death |
| Contract production gate | Passed; affected contract consumers, architecture, format and strict lint |
| Expanded disabled-scope/source checks | Four passed after final fixture changes |
| Authentication/boundary/assembly | 16/6/30 retained controls passed |
| Physical foundation and exact legacy prerequisite | Corrected selections passed |
| Architecture and forbidden-edge controls; process globals | Passed without allowance relaxation |
| Formatting/Clippy | Affected format passed; whole-node format retained 251 baseline hunks; Clippy retained nine old warnings |
| Strict pre-remote | Expected refusal; accepted review record remains null |

No builds, tests, writes, network or live actions were performed during this review.

## 1. Findings

### [P2-4] Ingress checks an earlier time interval after retaining a different current interval

**Location:** `node/src/independent/records.rs:280–300`; `node/src/independent/records/authentication.rs:10–31`.

`begin_ingress` obtains `(cut, policy)` at line 281. It then calls `authentication::persist`, which consults `PolicyClock::current()` again. The later deadline/window check nevertheless uses the first `cut`. Its equality guard compares policy digest and the evidence object’s scalar time floor, which now belongs to the second consultation; it does not compare the two complete time intervals.

Usually a changed second interval selects a new image and the subsequent `guards::fresh` check refuses the old guard base. The stable-cut optimization creates a concrete exception.

**Violated invariant:** Current authorization must use the provider’s conservative interval, including its upper bound. The persisted observation and authorization decision must agree. The corrected Persistence contract requires exact learned policy/time custody; the implementation explicitly requires the current interval’s `latest_ms` not to exceed the guard deadline.

**Credible public sequence:**

1. Provision a genuine authorized holder policy with selected trusted interval `[20,100]`, scalar floor 20, and sufficient capacity.
2. Load its image and construct an otherwise exact fresh guard with deadline 50.
3. Configure the trusted injected `PolicyClock` to return `[20,21]` for the first ingress consultation, then `[20,100]` for the persistence consultation and subsequent calls. Policy and injected floor remain unchanged.
4. Both observations satisfy the implemented monotonic checks: earliest time remains 20; no policy regression or malformed signature occurs.
5. Persistence finds its second interval exactly equal to the cached interval and clears the marker through the equality fast path. Image generation/digest therefore remain unchanged, so the guard’s base passes `guards::fresh`.
6. Policy digest and scalar floor comparisons pass. The deadline check uses the first interval’s upper bound 21, then durably establishes the guard and returns a usable permit.

The host has just consulted and retained `[20,100]`, which straddles deadline 50, yet returns consumption authority using `[20,21]`. This does not require caller-authored trust or private-field access; a legal sequence-returning `PolicyClock` suffices.

**Impact:** A receive can start despite the retained current conservative interval failing its own deadline condition. The fixed-source regression fixtures do not exercise this sequence. This finding is established by source/state tracing; I did not execute it.

**Required correction:** Couple authorization and persistence to one exact validated observation, or return the exact retained observation from persistence and re-evaluate authorization against it. Do not compare only the scalar lower floor. Preserve the marker’s fail-closed publication behavior and bounded guard-base retry.

**Closure test:** Add an external public DiskHost/session regression with initial `[20,100]`, ingress consultation `[20,21]`, subsequent `[20,100]`, and deadline 50. No permit or receive factory may be obtained. Include a consistently valid interval control and changed-image retry control. Audit the corresponding shared helper call sites for the same two-observation mismatch.

**Root classification:** New nonarchitectural B call-path root, distinct from the original absence of durable observation persistence. Correcting it needs no new contract or typed representation. It consumes no architectural-root cap.

## 2. Invariant analysis

Receive-result substitution now fails before authentication or physical publication. The private witness is derived inside the owned future, records no success for error/over-bound results, and has no caller setter. Eligibility remains nonmutating. Refused settlement retains the original capability and durable Active guard; conservative loss cannot become completeness.

Retired prepare replay now authenticates the full original issued value before recovering its immutable existing attempt. The deduplication remains ahead of fresh revision/allocation checks. Audited controls preserve image, counters, charges, receipts and phase.

The protected marker closes the original restart-forgetting window: establishment failure consults no source; post-consultation failure blocks usable load, recovery, reconstruction and close. Unrelated publication preserves the marker. Complete next Observation recovery validates and re-syncs before selection; missing-next refusal leaves advanced custody intact. Actual child-process evidence supports these distinctions.

Receive-only advancement now converges for stable valid cuts without borrowing append permission. P2-4 limits that result when the helper observes a different interval within the operation.

Prior stable-lock, finite physical reservation, exact native binding, immutable terminal, historical receipt, genuine cryptography and default-refuser conclusions remain applicable where unchanged. Recorded reruns cover the affected publication and capability paths. They establish Unix LocalProcessRestart behavior, not machine-power-loss or automatic duplex transport.

## 3. Risks and next action

The three originating findings are closed at this tuple. The corrected component remains NO-GO because P2-4 is open.

The next action is a bounded second B remediation that binds ingress authorization to the exact retained trusted observation and adds the public regression above. Preserve semantic/typed accounting and the two explicit lifecycle amendments. Fresh full acceptance remains separately required; this originating closure cannot supply it.