# Combined IC-3B physical-host/API cold consumer surface — SURFACE-AXIS REVIEW

**Review object:** SAME combined IC-3B genuine authentication/physical persistence component, focused Surface re-verdict after the second B correction. Controlling consumer guide: `dev-docs/GladeIndependentCrdtProductionIntegrationPersistenceUsage.md`, B candidate at workspace root `f0b1c325f9e0eff27b4eefadac8084bff3505e73`.

**Baseline:**

| Repository | Exact START and END SHA |
| --- | --- |
| Glade workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` |

Consumer guides and this reviewer’s prior report were read with pinned `git show`. The complete permitted consumer-document diff was inspected from root `c8d778e7c24b68690e27bfac8ca4457f647e747f` to the revised root. No implementation source was read.

**Date:** 2026-10-04

**Axis:** Surface: exact API discovery, eligibility versus retirement, result handling, ownership, lifecycle completeness, and cold failure/recovery paths. Independent, adversarial, read-only. Other axes run in parallel; nothing here relies on their reports. Filed verbatim by the lane owner.

**Verdict: GO** — own P3-1 is closed; no new P0–P3 findings. This is a retained-context Surface re-verdict, not new independent full-component acceptance or physical implementation qualification.

---

## Prior-finding closure table

| ID | Disposition claimed | Original counterexample verified | Status |
| --- | --- | --- | --- |
| P3-1 — Utility retirement is ordered but its public calls are unnamed | Document exact utility retirement calls, arguments/results, durable ordering, refusal/Unknown custody, and the concrete-consumer boundary | Repeated the original docs-only host-implementation walkthrough. PersistenceUsage line 66 now identifies `IngressAuthority::settle(&permit) -> Result<(),Fault>` and `IngressAuthority::abandon(&permit) -> Result<(),Fault>`, explains both outcomes and ordering, and prohibits retirement after Refused/Unknown publication. TypedUsage line 125 repeats the essential contract. The implementer no longer needs to guess the next utility call. | **Closed by this reviewer** |

P3-1 remains classified as a **nonarchitectural documentation omission**. Its closure does not create a separate package or increase the scope of acceptance. This report closes no finding belonging to another reviewer.

## Changed-range analysis

The complete permitted consumer-document range contains five changed locations:

| Location at revised root | Change | Surface consequence |
| --- | --- | --- |
| Authentication line 33 | Adds exact retained policy/full-interval authorization-barrier semantics; separates possession from append permission | Clarifies which authority applies after a second consultation |
| PersistenceUsage line 58 | Deadline and holder checks use the complete interval/policy retained at the barrier, including the upper bound | Prevents the reader from relying on an earlier narrower interval |
| PersistenceUsage line 64 | Adds exactly drained oversized-result handling through retained compact loss | Exposes the supported failure path without permitting oversized normal settlement |
| PersistenceUsage line 66 | Names utility retirement methods, arguments/results, ordering, and error handling | Resolves P3-1 |
| TypedUsage line 125 | Adds matching utility retirement and ownership summary | Makes the correction discoverable from the typed guide |

All three guides were read completely at the revised root. Their unchanged context was inspected around these additions, including construction, authentication, exact receive provenance, loss, retry, close, reopen, and compatibility.

Within the permitted documentation, these edits introduce no new factory, configuration field/default, root format, compatibility direction, ownership-transfer mechanism, recovery reset, or helper CLI. They clarify affected authorization and loss paths and complete the utility lifecycle description. Those affected paths received a fresh cold inspection below; the unchanged documentation conclusions from the prior full Surface review remain applicable.

This boundary conclusion is limited to consumer documentation. Implementation changes between Glade `d3fded040d6e3c459cd5f975d6c672873356cc23` and `a47691598df648eb8c9554b27f3d06b0cffcf596` were not inspected, as the canonical Surface prompt forbids source reading. No implementation-range proof is claimed.

No new concrete root was found. The prompt’s accounting is preserved: semantic accounting unchanged, typed two completed remediations unchanged, and B two completed remediations. The ledger was not read; this report does not independently certify its contents.

## 0. Evidence base

The canonical `PhysicalHost-PromptSurface-2.md` was read in full. This reviewer’s filed `PhysicalHost-ReviewSurface-1.md` was read at the revised root. Previously read workspace/member instructions and review-loop skill/template remained in context.

The complete revised consumer evidence was:

- `GladeIndependentCrdtProductionIntegrationTypedUsage.md`, lines 1–125.
- `GladeIndependentCrdtProductionIntegrationPersistenceUsage.md`, lines 1–95.
- `GladeIndependentCrdtProductionIntegrationAuthentication.md`, lines 1–33.

Commands were pinned `git show`, numbered `sed` reads, the scoped committed documentation diff, scoped working-tree `git diff --name-only`, and `git rev-parse HEAD` for all five repositories at START and END. Every HEAD matched the required tuple. Working-tree comparisons reported no changes to the three reviewed guides.

No files were changed. No builds, tests, network requests, live actions, implementation source, design, plan, evidence, or current peer reports were read or executed. Documentation links to excluded material were not followed.

PersistenceUsage line 3 still states that B adds no CLI. No helper CLI was introduced in the reviewed documentation, so no help execution was applicable.

## 2. Invariant analysis

### Original utility-discovery counterexample now fails

The original failure was specific: the cold host implementer could discover eligibility checks and their ordering, but could not name the mutating utility operation to call afterward.

PersistenceUsage line 66 now supplies the missing lifecycle pair:

| Stage | Documented utility operation | Ownership/result meaning |
| --- | --- | --- |
| Drained eligibility | `can_settle(&permit)` | Nonmutating; foreign, closed, or not-drained settlement refuses |
| Exact normal-result eligibility | `can_settle_received(&permit,&bytes)` | Also checks successful bounded completion and exact length/digest |
| Never-started eligibility | `can_abandon(&permit)` | Nonmutating; started abandonment refuses |
| Committed normal/loss retirement | `IngressAuthority::settle(&permit) -> Result<(),Fault>` | Ok retires the entry; Err refuses foreign, closed, or ineligible custody |
| Committed never-started retirement | `IngressAuthority::abandon(&permit) -> Result<(),Fault>` | Same explicit result handling, with never-started eligibility |

The borrowed `&permit` argument is visible. Utility retirement is distinct from session persistence operations that consume the owned permit. Concrete DiskSession consumers are expressly told to use session operations and not retire the private authority manually.

### Selection, retirement, refusal, and Unknown remain distinct

The corrected paragraph requires eligibility before I/O while leaving custody unchanged. Durable Committed selection precedes retirement. Refused or Unknown publication prohibits both retirement calls and keeps the exact continuation outstanding.

An unexpected retirement error after selection must be treated as Unknown rather than fabricated success. That instruction prevents a cold implementer from discarding the error merely because the durable operation appeared selected.

The surrounding results/retry section still preserves the registered scope and exact continuation, prohibits replacement permits/identities/guards, and limits retry to existing retained custody. The correction therefore closes discoverability without weakening uncertainty handling.

### Authorization changes expose the affected boundary

Authentication line 33 states that current authorization uses the exact retained signed policy and full conservative interval. An advancing second consultation cannot validate an earlier Reserved Begin decision. Protected Started history and original receipts retain their earlier meaning.

PersistenceUsage line 58 applies that distinction to receive deadline and holder checks, including the retained interval’s upper bound. It keeps the prior finite retry budget, refreshed guard-base binding, refusal-before-consumption rule, and explicit stop when inputs remain unsafe.

The cold reader is not authorized to borrow append permission from a successful challenge/response or select an earlier narrower time interval. These additions do not introduce a hidden new consumer configuration option or call-order requirement.

### Oversized completion has a named loss path

PersistenceUsage line 64 now covers an exactly drained result larger than the guard’s byte bound. Normal settlement remains prohibited. After the drained settlement continuation is retained, the consumer can call `retain_pending_ingress_loss(&guard_id)` on the same live session.

The guide specifies a compact digest-bound loss marker using reserved critical space. It prohibits installing the oversized data as inventory or interpreting it as empty input. It preserves the original continuation on failed loss publication; committed loss permits clean close while permanently keeping completeness false.

This is consistent with `received_digest()` supplying no normal settlement authority for above-bound completion and with conservative loss requiring drained eligibility. It does not authorize orphan-permit reconstruction, undrained cleanup, or bypass of unknown physical custody.

### Existing lifecycle and compatibility conclusions remain supported

Construction still requires explicit evidence, trusted policy/time, roots, registration, provision choice, owner, namespaces, and finite limits. Fresh provision remains distinct from reopen; full reopen hashes the exact complete trusted-floor bytes; narrow reopen restores the retained authoritative plan namespace.

Normal receive still requires the exact successful bounded bytes from its owned continuation. Error, substitution, timeout, and later empty inventories do not discharge an earlier obligation. Started cancellation still requires cooperative drain/join or retained honest uncertainty.

Combined close remains explicitly qualified. Pending retains the session and root locks; only Closed permits clean physical release. Orphan active guards and unresolved protected observation markers remain documented recovery stops. No deletion, owner replacement, forced reset, or unavailable-as-empty-store shortcut was introduced.

Fresh-root legacy marker ownership, supported independent-adapter reopen, and v1/v2 refusal direction remain unchanged. The profile still claims Unix process restart, independently trusted floor custody, and no arbitrary simultaneous rollback protection.

## 3. Risks and next action

The documentary retirement signatures and physical behavior were not checked against source or executed witnesses. That is outside this Surface re-verdict and remains an obligation of the relevant independent reviews and gates.

The next action is to file this report verbatim and carry its GO plus reviewer-verified P3-1 closure into the same-tuple verdict merge. Required originating closures, genuine accepted-B metadata, and the strict pre-remote gate remain separate requirements. No P3-only package or additional acceptance scope follows from this closure.