# IC-2 Pure admission kernel, remediation 2 — STATE-AXIS REVIEW

**Review object:** Originating State confirmation after the second merged nonarchitectural correction, Glade `37dff286ce1eb9690204d4a7d14c940333396a30`; controlling implementation DRAFT, RemPlan-2 and Remediation2-Evidence at workspace root `104d91cab7fde2c036ff962d979b56aadca6534a`. Deterministic component acceptance only.

**Baseline:**

| Repository | Exact reviewed HEAD |
| --- | --- |
| Workspace root | `104d91cab7fde2c036ff962d979b56aadca6534a` |
| Glade | `37dff286ce1eb9690204d4a7d14c940333396a30` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected directly and compared with pinned committed bytes using read-only inspection and `git show`. All five HEADs matched at both start and end.

**Date:** 2026-10-04  
**Axis:** Closed recovery grammar, custody, terminal monotonicity, bounded retention and truthful completeness. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — prior State GO confirmed on this tuple. All five original State closures remain valid; the busy-history correction independently withstands the State counterexample. No new P0–P3 finding established. Architectural root count remains zero.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| State P2-1; initial Code P2-1 | Preserve terminal settlement against delayed nonterminal callbacks. | Retraced actual held Begin/Started, terminal Inspect first, then delayed Started. The unchanged authenticated-retirement guard prevents phase downgrade and OutcomeUnknown. New busy bookkeeping cannot alter attempts or terminal results. The retained four-kind/two-outcome/both-order matrix checks custody, charges, restoration and absence of renewed lookup. | **Closure preserved** |
| State P2-2; initial Code P2-2 | Stop new work through parked verification, sealing, forks and staging. | Retraced A’s authenticated terminal contradiction followed by B’s parked Verified or Sealed reply. Integrity checks still precede query removal and the corrected busy branches. Direct qualified forks remain stopped; already-owned storage settlement, exact retries and independent Y remain available. | **Closure preserved** |
| State P2-3; initial Code P2-4 | Bound prospective candidate-derived missing slots. | Retraced a historical candidate with two absent refs and `pending_items=1`. The unchanged evidence bound refuses before missing-work emission or staging. Exact-limit, overlap, candidate/frontier and unpublished-reservation controls retain their source and assertions. The new busy refusal cannot install an overflowing candidate. | **Closure preserved** |
| State P2-4; initial Code P2-3 | Bound frontier names and aggregate retained bytes. | Retraced the million-byte unknown origin. Frontier validation still refuses before missing-slot retention/emission, sets sticky incompleteness and preserves charges. Name and aggregate-footprint helpers are byte-identical to the prior reviewed tree. | **Closure preserved** |
| State P2-5 | Record historical counter-exhaustion refusal. | Retraced historical offer at MAX and verification-to-plan allocation at MAX−1. Existing failure branches still set sticky loss, preserve custody/charges and cannot wrap. Busy refusal adds another truthful loss branch without clearing either marker. Resume/Read/restoration and local-only controls remain unchanged. | **Closure preserved** |
| Code-2 P2-5 — State confirmation | Explicitly refuse distinct busy historical work and retain incompleteness. | Independently retraced fresh historical B and already-matched Verified B during held Started A. Both corrected branches report Capacity, set sticky incompleteness and resume only A. Actual A settlement, Read, ResumeRecovery and validated continuation cannot report complete history afterward. | **Verified on State axis; originating Code reviewer owns formal closure** |

These checks preserve the nine original finding closures without treating the prior State verdict as proof of the new correction.

## Changed-range analysis

Reviewed the complete root range `aa4f7e28334baa4ef515487792ca785061de597c..104d91cab7fde2c036ff962d979b56aadca6534a` and Glade range `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494..37dff286ce1eb9690204d4a7d14c940333396a30`.

Production changes comprise fourteen added lines in two existing branches:

- [admission.rs:78](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:78): a fresh busy offer reports Capacity and marks historical input incomplete before the existing recovery call.
- [staging.rs:77](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:77): busy staging applies the same rule after matched verification, using the existing local precondition to distinguish local intent.

One new 253-line test file adds three regressions. Root changes document the conservative refusal behavior and review chronology; they do not alter accepted contracts.

Originating-context retention remains justified. Public types/events/effects, storage API/provider, immutable plans, publication ownership, callback authority, roles/dependencies, wire/platform assumptions and consumer paths remain fixed. Existing recovery calls are retained; no responsibility or material ownership boundary moved.

The busy-history root remains **nonarchitectural**: the correction uses the accepted sticky field and existing refusal outcome. No additional architectural or nonarchitectural root was established.

## 0. Evidence base

Read the complete canonical State-3 prompt; SHA256 matched `f70ea9e9380f0ab6b30d287962663a6a114b86454a94389dea8f3434e3b58a4a`.

Read full RemPlan-2, Remediation2-Evidence and legitimate prior Code-2 report; retained the originating State-2 review context. Inspected DRAFT and ReviewCycle changes, including the explicit absence of a deferred queue or marker-clearing witness. No current peer round-three report was accessed.

Read complete current `admission.rs` lines 1–464, `staging.rs` lines 1–230 and `remediation2.rs` lines 1–253. Rechecked terminal authentication/retirement, pending helpers, frontier bounds and lifecycle inspection issuance. The previously reviewed unchanged source, accepted contracts, provider and consumer context remains applicable.

Independent read-only comparison verified all **47 protected files** against baseline committed, current committed and live bytes. All matched their recorded hashes. Witness artifact SHA256 matched `db25882ed50b22a7f7638574f4474229709677109846efad83576e43b6c01c72`. This includes all 41 prior witnesses and six remediation-one test files.

Separately confirmed callbacks, dispatcher/frontier, projection, architecture policy, process-global checker/allowlist and controlling admission/storage contracts remain unchanged. All three changed Glade files match the reviewed commit.

Audited existing execution evidence; no tests or builds were run by this reviewer:

| Evidence | Audited result |
| --- | --- |
| Initial compiling RED | Three failures on missing explicit busy refusal. |
| Refined preimplementation RED | Two historical cases reach A’s actual terminal settlement and fail `complete_local=true` versus expected false; local-only control passes. |
| Focused GREEN | All three regressions pass. |
| All-targets/adopted gates | Core 105, API 34; architecture, formatting, Clippy and source checks recorded successful. |
| Released text/corpus | Exact retained ten-row consumer and canonical three-order corpus pass. |
| Process/JavaScript guards | 88 Rust files, three permanent entries, zero debt/nothing new; compound bodies pass. |

The chronology discloses refinement of only the new regression before implementation. No original assertion or fixture was weakened. Recorded timings are cached/concurrent evidence. The unchanged isolated-mutant decision is consistent with the scoped patch; no new mutant execution is claimed.

## 2. Invariant analysis

The new actual-port sequence issues both A/B queries before staging A, delivers A’s Started reply with publication held, then delivers B’s exact successful historical verification. Staging now records an explicit refusal rather than silently forgetting B. The fresh-offer variant reaches the same honest outcome before verification.

Both paths preserve A’s exact unknown plan, attempt and charges. They issue no new Verify/Seal/Prepare/Begin for refused B. The tests execute the original Pending Inspect before advancing the provider’s invocation floor through Y, then use a newly issued exact Inspect to settle A. This preserves callback authority and caller-owned session continuity.

After settlement, A retains its original identity, receipt, revision and charges; B receives no custody or receipt. Sticky incompleteness survives Read, ResumeRecovery and validated fixture continuation. Local-only refusals leave known history complete after A settles. Independent Y commits while A remains unresolved.

Terminal monotonicity, integrity stops, bounded pending reservations, exact encoded charging, historical counter loss, qualification/quarantine and fresh-origin behavior retain their previously reviewed paths and unchanged regressions. The correction introduces no eviction, fabricated accounting, terminal inference from Pending, or clearing operation.

## 3. Risks and next action

Refused busy historical input is not queued. A later retry may retain its operation but cannot clear the sticky marker. Unknown frontier digests retain the same conservative limitation. These are explicit Pure safety limits, not promises of eventual exact-cut reconstruction.

The next action is to file this State GO with the independently formed originating Code closure and let the lane owner record the dual outcome. Remediation count remains two; historical stops/caps remain intact. This report authorizes neither another correction nor push/live activation, and does not qualify production authentication, physical persistence/antirollback, live duplex, compatibility or Surface.