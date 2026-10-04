# IC-2 Pure admission/reconciliation kernel, remediation 2 — CODE-AXIS REVIEW

**Review object:** Originating Code closure after the second merged nonarchitectural correction, Glade `37dff286ce1eb9690204d4a7d14c940333396a30`; controlling implementation DRAFT, RemPlan-2, and Remediation2-Evidence at workspace root `104d91cab7fde2c036ff962d979b56aadca6534a`. Deterministic component acceptance only.

**Baseline:**

| Repository | Exact reviewed HEAD |
| --- | --- |
| Workspace root | `104d91cab7fde2c036ff962d979b56aadca6534a` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected directly and compared with pinned `git show` bytes. All five HEADs matched at start and end.

**Date:** 2026-10-04  
**Axis:** Architecture, interfaces, call graphs, and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — Code-2 P2-5 is closed; all four original Code closures remain intact. No new P0–P3 finding was established. Corrected roots remain nonarchitectural; architectural root count remains zero.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Original Code P2-1 | Prevent delayed nonterminal callbacks from reopening terminal attempts | The authenticated terminal guard and actual-port scheduling matrix are byte-identical to the verified remediation-1 tree. Held Started after terminal Inspect still returns before phase downgrade or OutcomeUnknown. Four kinds, both terminal outcomes/orders, restoration and no renewed lookup remain covered. | Closed |
| Original Code P2-2 | Enforce integrity stop on continuations, forks and staging | Evidence matching still precedes the stop, which preserves parked queries; direct forks and staging retain their guards. New busy bookkeeping runs after the staging stop and cannot bypass it. Original retry, independent Y and already-owned settlement controls are unchanged. | Closed |
| Original Code P2-3 | Bound frontier names and aggregate retained bytes | Name validation, checked pending footprint and boundary regressions are unchanged. The million-byte origin is refused before persistent missing-slot construction; charges and custody remain preserved. | Closed |
| Original Code P2-4 | Bound prospective candidate/frontier missing-slot union | Deduplicated union/reservation helpers, pre-emission checks and final encoded pending-charge checks are unchanged. Two missing refs with a one-slot cap still refuse before staging; exact-limit, overlap and unpublished-reservation controls remain intact. | Closed |
| Code-2 P2-5 | Explicitly refuse busy historical input and preserve incomplete status | Retraced both issued A/B queries: A reaches actual held Started; B’s exact historical verification reaches busy staging. The corrected branch sets `recovery_incomplete` and reports Capacity before resuming A. Actual A publication and Committed Inspect install only A, preserving its identity, receipt and charges; Read remains incomplete through ResumeRecovery and validated fixture continuation. Fresh historical offers take the corresponding corrected offer branch. Local-only fresh and verified/sealed controls remain complete after A settles; Y commits through the same port while A is unresolved. | Closed |

The fifth original State closure—historical query/plan counter exhaustion—also remains intact: its production bookkeeping and all three exhaustion regressions are unchanged and included in the protected-source verification. Thus this correction preserves all nine original finding closures.

## Changed-range analysis

Reviewed Glade `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494..37dff286ce1eb9690204d4a7d14c940333396a30` and root `aa4f7e28334baa4ef515487792ca785061de597c..104d91cab7fde2c036ff962d979b56aadca6534a`.

The complete production correction is fourteen added lines in two existing branches:

- [admission.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:78), lines 78–92: fresh busy historical offers set sticky incompleteness; all busy offers receive Capacity before the existing recovery call.
- [staging.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:77), lines 77–90: busy historical continuations receive the same bookkeeping and explicit refusal before recovery.

The only new test file is `tests/remediation2.rs`, 253 lines. The DRAFT adds an explicit description of conservative busy-history refusal, without claiming a deferred queue or clearing witness.

Originating-context retention remains justified. Shared types/API/provider, public event/effect grammar, package roles/dependencies, platform/wire assumptions, publication ownership and issued-callback boundaries are unchanged. Both corrections execute within the existing private call graph and retain its recovery operation. No material boundary change or new architectural/nonarchitectural root was established.

## 0. Evidence base

Read the complete canonical Code-3 prompt; SHA256 matched `84e831e04192150507a2dacc02590c4e53784d70c22dcf38e82c08e10267fd06`.

Read full RemPlan-2 and Remediation2-Evidence, the updated DRAFT range, legitimate prior Code/State reports, and the complete member diff. Inspected both corrected branches with their preceding validation/retry/stop conditions and following recovery paths. Read `tests/remediation2.rs` completely, including actual preparation, held Begin, pending Inspect, independent Y publication, terminal settlement, exact retry and validated continuation.

Independent read-only comparisons verified all 47 protected witnesses against the preceding Glade/Glial pins and recorded hashes, with no discrepancy. These include original domain assertions, assembly/provider, public types, encoder, lifecycle, selectors, released consumer/corpus and all remediation-1 tests. All 46 core/API package files matched the new committed Glade bytes.

Audited existing execution evidence without running builds or tests:

| Evidence | Audited result |
| --- | --- |
| Exact counterexample RED | Two historical cases execute A’s terminal settlement and fail `complete_local`: actual true versus expected false; local-only control passes. |
| Focused GREEN | All three remediation-2 tests pass after correction. |
| Final all-targets/adopted gates | Core 105 and API 34 pass, including all sixteen remediation-1 regressions; architecture, formatting, Clippy and source guards recorded successful. |
| Released text/corpus | Actual released consumer and three-order canonical corpus pass; unchanged consumer retains its exact ten-row assertions. |
| Process/source checks | 88 Rust files, three permanent entries, nothing new; JavaScript compound-body guard passes. |

The chronology distinguishes the initial missing-refusal RED from the refined falsely-complete-read RED. The refinement changes only the new test sequencing before implementation; it uses genuine issued Inspect requests and avoids inventing callback authority after Y advances the provider’s invocation floor.

Prior isolated encoder/fresh-origin mutants protect unchanged predicates. The recorded decision not to rerun them is consistent with RemPlan-2; no new isolated-mutant execution is claimed. Timings are cached/concurrent evidence, not independent cold-build measurements.

No files, repository state, network or live systems were modified. No current peer round-3 report was accessed.

## 2. Invariant analysis

The original A/B attack now fails at the intended safety boundary. B may be consumed as an explicit historical refusal, but the kernel retains its unclassified-history status through the existing monotonic marker. Settling A cannot clear that marker. No B receipt or charge is fabricated.

A’s immutable plan, attempt and uncertainty survive refusal. Recovery still obtains authentic Pending and terminal observations; committed installation preserves A’s original receipt, operation identity, revision and charges once. Independent Y remains operational through the same caller-owned memory session.

Fresh historical offers and successfully matched historical verification cover both entry paths that previously discarded B silently. Local intent remains distinguishable: its busy refusal alone does not assert lost observed history. Exact accepted retries occur before fresh-offer busy handling and retain their original receipts.

The correction does not weaken terminal monotonicity, integrity-stop enforcement, finite pending reservations, immutable encoding, historical qualification, quarantine or fresh-origin recovery. Protected byte comparisons and inspected ordering establish preservation; recorded GREEN supplies additional consumer evidence.

Permanent conservative incompleteness is an accepted limitation of this confined correction. Retrying B later may retain its operation, but no clearing event exists. This satisfies the permitted explicit-refusal remedy and does not claim eventual exact-cut reconstruction. The unchanged unknown-frontier-digest limitation has the same explicit safety boundary.

## 3. Risks and next action

The next action is to file this Code GO alongside the independently formed State confirmation and record the dual outcome for the exact tuple. This closes Code’s remaining finding after the second remediation; historical stops, review caps and architectural count remain intact.

This report qualifies the Pure deterministic component only. Production authentication, physical persistence/antirollback, cancellation/drain, live duplex, compatibility, Surface and activation remain separate gates. No push or live activation is authorized by this verdict.