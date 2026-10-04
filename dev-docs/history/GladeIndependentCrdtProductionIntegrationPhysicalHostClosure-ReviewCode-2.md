# Combined IC-3B Physical Host Originating Closure 2 — CODE-AXIS REVIEW

**Review object:** The same combined genuine-authentication and physical-persistence component after its second bounded B correction. Reviewed Glade range: `d3fded040d6e3c459cd5f975d6c672873356cc23..a47691598df648eb8c9554b27f3d06b0cffcf596`. Controlling DRAFT: `dev-docs/GladeIndependentCrdtProductionIntegrationDesign.md` at root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`. This report closes originating Code P2-4 and verifies preservation of the three original closures.

**Baseline:** Root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`; Glade `a47691598df648eb8c9554b27f3d06b0cffcf596`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `400cedcf1fff74128f366af758a0c389a23435d7`. All five heads matched at START and END. Inspection used source reads, scoped Git diffs and read-only Python provenance/evidence checks.

**Date:** 2026-10-04

**Axis:** Code — originating counterexample closure, preservation of prior closures, and complete changed-range/helper-consumer analysis. Independent, adversarial, read-only. Parallel full and closure reviews supply no evidence here. Filed verbatim by the lane owner.

**Verdict: GO** — P2-4 is closed; P2-1–P2-3 remain closed. No new P0–P3 finding was established. This focused originating verdict does not replace fresh full acceptance of the changed callback ordering.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
|---|---|---|---|
| P2-1 — receive-result substitution | Issuer-owned successful bounded receive identity remains required before normal settlement | The utility witness and host comparison are unchanged. The final public suite again executes nonempty→different signed empty, malformed-prefix→signed empty and receive-error→signed empty refusals, their process-death/reopen preservation, and exact empty/nonempty positive controls. The loss change supplies no normal-settlement bypass. | **CLOSED, preserved** |
| P2-2 — retired Prepare recovery | Complete original live/retired issuance authenticates existing immutable attempt lookup | Original lookup and deduplication are unchanged. Final public replay passes before/after reopen; four-kind Reserved and terminal replay controls pass in the Records selection. Mutated/unissued requests refuse; image, counters, charges and phase remain unchanged by retrieval. | **CLOSED, preserved** |
| P2-3 — receive-only advancement | Durable current observation plus bounded guard-base retry, without append rights | The receive-only empty-grant policy/time advancement regression remains in the final public suite and passes. It retains the chained policy/time cut, reopens it, establishes ingress and still denies append challenge. Marker-establishment, failed-retention and denial controls remain passing. | **CLOSED, preserved** |
| P2-4 — earlier interval authorizes after different interval is retained | Ingress authorizes using the complete cut returned after the persistence barrier | The exact original sequence is reproduced publicly: selected `[20,100]`, first ingress observation `[20,21]`, persistence observation `[20,100]`, deadline 50. Compiling RED issued the unsafe permit; corrected source returns `Ownership` using the retained upper bound 100. Stable valid interval and changed-image retry controls pass. | **CLOSED** |

The recorded implementation results support closure only after independent source tracing. Writer disposition labels alone were not used as proof.

## Changed-range analysis

The exact second correction comprises 11 Glade files. I inspected every production/test change, all three new test files, the surrounding host methods, the shared helper’s consumers and the relevant pure-core transitions.

Four production files change:

- `records.rs` takes the authorization cut returned by `persist_observed`.
- `authentication.rs` returns the complete validated retained cut, selects any new Fence issuance accounting with the Observation image, then drains the selected transition’s effects.
- `storage.rs` additionally compares the final retained policy and full interval with the Reserved attempt’s sealed precondition.
- `loss.rs` converts an existing host-owned refused settlement into a fixed digest-bound loss record without requiring its oversized bytes to fit the normal receive bound.

The public utility representation, public ports, authority sources, disk v2 grammar, physical primitives, dependency declarations, roles and default routes are unchanged. The five consumer documents explain the exact retained authorization barrier, oversized-loss handling and actual retirement calls.

The private call graph does change materially:

`persist_observed → selected ObservePolicy effects → runtime::drain → original native Fence → exact FenceResolved core callback`.

This invalidates blanket reliance on the earlier callback-order proof. The merged plan correctly requires fresh full affected Code/State review alongside originating closures. My closure relies on retracing that path and its added crash witnesses, not treating it as an unchanged detail.

The helper now includes newly issued requests in Recovery invocation count/high-water and external invocation/effect floors before selecting the Observation image. This is necessary because process death can occur before the native Fence starts. Recorded intermediate crash failures exposed omitted accounting; the final source and three corrected process cuts verify its repair.

The Fence-winner fixture now locates its original request in live or retired custody because automatic draining can retire it earlier. Its terminal, delayed-Begin, contrary-callback, conflict and no-accepted-state assertions remain intact. This is an exact-request traversal adaptation, not another superseded availability guarantee. Remediation1’s two explicitly stronger refusal replacements remain unchanged.

No new root is established. P2-4 remains part of merged nonarchitectural B-R6; oversized loss remains the B-R4 edge. The documentation correction remains B-R7. Accounting is preserved:

| Object | Retained accounting |
|---|---|
| Semantic | Two architectural, one nonarchitectural; one completed remediation |
| Typed | Two architectural, five prior nonarchitectural; two completed remediations |
| Combined B | Zero architectural, six nonarchitectural; two completed remediations |

Closure does not subtract historical roots or reset caps. No third typed architectural root is classified. Original A2 originating closures remain owner-deferred to wider ABC.

## 0. Evidence base

I read the complete canonical closure prompt, my original report and own Closure-1 report, legitimate RemPlan-1/2, corrected evidence and relevant ledger sections. Retained controlling context includes the applicable instructions, review-loop/template, Build Entry, library/testing and package-boundary policies, Design/Plan/Typed/Authentication/Persistence contracts and original physical evidence. No current `-2` testimony was read.

Source tracing covered:

- `records.rs:264–300`, including preflight, marker, retained-cut authorization and guard establishment;
- complete `records/authentication.rs`, especially retained equality confirmation, Observation selection, request accounting and effect draining;
- complete native `storage.rs`, including original request authentication, Reserved start/fence and terminal resolution;
- complete `loss.rs`, normal settlement and retained-continuation retry context;
- runtime submit/drain/reattach, recovery-query replay and challenge/authenticate consumers;
- pure-core ObservePolicy/Fence issuance, full-cut matching, evidence callback validation and terminal callback handling;
- complete public `ic3_disk_remediation2` consumers, retained-cut crash tests, oversized-custody tests and modified Fence/source-selection tests.

Independent provenance checks established:

- All 65 combined source hashes match both working files and committed Glade source.
- All five current consumer-document hashes match working files and committed root.
- Git’s exact correction list matches the manifest’s 11 files.
- Unchanged prior combined source pins remain identical.
- The 45 retained compatibility files, 27 canonical codec files and 23 existing artifacts match their protected hashes.
- The prior five consumer documents match their original committed root, rather than being compared incorrectly with their current amended versions.

The remediation chronology is 71,711 uncompressed bytes, SHA-256 `4d95a09b37610a7e1908e0179c5eb3dd4f27d2d6c69881a44524c0e0c1d61476`; gzip SHA-256 is `28c07bef73a2b83848418998b2218a8017fea5037887d575ca08890779eb1c03`. I audited its command/result index and relevant complete RED, intermediate-failure, corrected-regression and final-gate records.

| Recorded selection | Audited result |
|---|---|
| Initial four public regressions | Compiled; all four failed at intended interval, stale-start or oversized-loss assertions |
| Corrected exact interval and loss controls | Passed; initial subsequent close failures remained recorded until Fence draining was corrected |
| Final public `ic3_disk` suite | 29 passed, three owned helpers ignored; 6.729 seconds wall |
| Final affected Records selection | 29 passed, six owned helpers ignored; 38.248 seconds wall |
| Three intermediate Fence process cuts | Corrected run passed after selected-image request accounting was added |
| Retained Fence/close/protected Started controls | Three passed; original assertions preserved |
| Auth, boundary and assembly | 16/6/30 passed |
| Contract production gate | Passed retained contracts, architecture, formatting and strict lint |
| Expanded disabled-source selection | Four passed; new consumer subtree included |
| Node architecture/process-global gates | Passed; 152 production files, unchanged three permanent entries and zero debt |
| Affected formatting and Clippy | Affected files passed; Clippy retained exactly nine old warnings |
| Canonical vectors/strict pre-remote | Candidate representation passed; strict pre-remote retained expected refusal |

The Records selection covers 151 affected cuts: 57 native, 65 ingress, 26 observation and three new Fence cuts. The unchanged 171 historical candidate/security/fork native cuts were not rerun. Their earlier evidence remains pinned; four-kind custody controls were rerun. This report does not relabel that retained evidence as a fresh 319-cut execution.

No writes, builds, tests, Git mutations, network or live actions were performed during this review.

## 2. Invariant analysis

The original interval attack now fails for the intended reason. The equality fast path returns the exact retained `[20,100]` cut; ingress’s deadline check uses its upper bound 100 and refuses deadline 50. A stable `[20,21]` control still establishes a guard. Selecting a changed cut still invalidates an old guard base and requires bounded reload/retry. Neither refusal grants a permit or receive-factory authority.

The related write-start path compares both the initially observed admission cut and the complete cut retained at the protected barrier against the sealed precondition. A changed policy or interval cannot produce Started. Genuine public revocation/expiry controls retain one original NonCommit, no accepted operation, and clean close/reopen. An already protected Started attempt bypasses fresh reauthorization and retains its original receipt semantics.

I audited every helper consumer. Challenge/response return possession results; they do not supply final append authorization. Submit obtains current observation before selecting its query. Verify/Seal retain observation before delivering replies, and the pure core checks the selected full policy/time against the original local query. New threshold controls exercise advancement at both callback boundaries and show that stale replies create no Prepare. Recovery retains original request/query custody and applies the same current-cut checks without synthesizing a new link.

ObservePolicy’s effects in this path are original Fence requests or bounded reports. Fence resolution publishes NonCommit for Reserved custody and delivers the exact callback; it does not consult policy again or remint verification. The selected image contains the request and its counters before execution. Three actual deaths—before Fence intent, after intent synchronization and after selection synchronization—reopen to one original NonCommit with no accepted receipt, no live storage invocation and clean close. Failed Observation selection runs no effects.

Oversized loss remains conservative. Normal settlement still refuses M+1. Pending conversion uses only the existing drained host-owned continuation; it preserves the boxed original Observation/Checkpoint and stores a fixed digest-bound marker. It installs no oversized inventory. Direct caller-prefix bounds remain enforced. Failed publication preserves the original capability, Active guard and Pending close; Unknown cannot be bypassed. Public process-death witnesses distinguish refused/failed loss from selected permanent loss, and restart cannot manufacture a live drained permit.

The unchanged stable-root, finite reservation, genuine-signature, native immutable-binding, historical-receipt and default-refuser proofs remain applicable within their recorded scope. No successful physical path was replaced with a MemoryHost fixture.

## 3. Risks and next action

This originating Code lane has no open finding at the settled tuple. P2-4 is closed and all three original closures are preserved.

The next action is to complete and file the separately required fresh full affected reviews and remaining originating closures at this same tuple. Only their actual results can support a committed B acceptance record. This focused GO supplies no automatic transport qualification or permission to bypass strict pre-remote.

Durability remains bounded Unix LocalProcessRestart. Power-loss/quorum durability, simultaneous trusted-floor/data rollback, automatic two-node exchange and live-store migration remain outside this result. Whole-node formatting debt remains the previously recorded 251 hunks; only affected-file formatting was claimed green here.