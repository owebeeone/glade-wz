# IC-2 Pure admission kernel, remediation 1 — STATE-AXIS REVIEW

**Review object:** Originating State re-verdict after one merged nonarchitectural remediation, Glade `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494`; controlling implementation DRAFT, RemPlan-1 and Remediation1-Evidence at workspace root `aa4f7e28334baa4ef515487792ca785061de597c`. Deterministic component acceptance only.

**Baseline:**

| Repository | Exact reviewed HEAD |
| --- | --- |
| Workspace root | `aa4f7e28334baa4ef515487792ca785061de597c` |
| Glade | `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were read directly and checked against committed bytes using read-only inspection and `git show`. All five HEADs matched at the beginning and end.

**Date:** 2026-10-04  
**Axis:** Closed recovery grammar, terminal finality, bounded retention, integrity stops and honest completeness. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — all five original State P2 findings are closed; no new P0–P3 finding established. All corrected roots remain nonarchitectural. Architectural root count remains zero for this implementation object.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1: delayed Started downgrades terminal | Authenticate and retire the late invocation without reopening settlement. | Retraced held actual-port Begin/Started, publication or fence, terminal Inspect, then delayed Started. The new callbacks guard returns before phase mutation or OutcomeUnknown. The regression matrix covers four kinds, both terminal outcomes and both delivery orders; checks unchanged custody/charges, terminal identity, validated restoration and no renewed lookup. Fully issued late Pending/Refused controls also preserve settlement. | **Closed** |
| P2-2: parked verification bypasses integrity stop | Guard evidence continuation, direct fork and staging paths while preserving owned settlement. | Retraced simultaneous A/B queries, A committed, authenticated contrary terminal, then B’s parked Verified or Sealed reply. Exact query matching precedes the stop check; the query remains retained and no Seal/Prepare/Begin follows. Direct already-qualified OfferFork is stopped. Controls preserve original exact retries, independent Y publication and genuine late commitment of an already-owned Started attempt. | **Closed** |
| P2-3: candidate missing slots exceed `pending_items` | Bound the prospective deduplicated union before emission and staging. | Retraced historical sequence-zero candidate with two absent refs and `pending_items=1`: evidence returns Capacity, records sticky loss and emits no missing request or Prepare. Exact-limit and overlapping candidates succeed. Candidate/frontier combinations include unpublished candidate-plan reservations. Duplicate predecessor/reference coordinates now emit one slot. | **Closed** |
| P2-4: arbitrarily large frontier origin | Validate names and aggregate retained footprint before cloning into state/effects. | Retraced the million-byte unknown origin: name validation returns Capacity/sticky incompleteness before construction of missing slots. Empty names are Invalid without state change. Exact name limit succeeds; limit+1 and large-name controls refuse. Aggregate exact/overflow cases include existing slots, charged candidate custody and parked verification bytes. No false charge or custody eviction occurs. | **Closed** |
| P2-5: historical counter refusal becomes falsely complete | Set existing recovery-loss flag at query and plan allocation failure. | Retraced `next_effect_id=MAX` historical offer and `MAX-1` offer followed by verification: each refuses without wrap, retains prior custody/charges and sets sticky incompleteness. Read, ResumeRecovery and validated fixture restoration remain incomplete. Local-intent exhaustion controls preserve complete known history. | **Closed** |

## Changed-range analysis

Reviewed the complete Glade range `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14..03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494` and root range `332e7d95f5887300bf762ac76737e4cc38bcd151..aa4f7e28334baa4ef515487792ca785061de597c`.

Production changes are confined to five existing private files:

- `admission.rs`: continuation/fork stop guards, prospective pending checks and historical allocation bookkeeping.
- `callbacks.rs`: terminal monotonicity after authenticated invocation retirement.
- `lib.rs`: frontier name and aggregate capacity validation.
- `projection.rs`: deduplicated missing slots and private reservation/footprint helpers.
- `staging.rs`: stop guard, prospective bounds, exact encoded pending-charge check and historical allocation bookkeeping.

Six new test files add sixteen regressions. No original test or assertion was changed.

Originating-context retention is justified. Public State/event/effect types, storage API/provider, physical publication ownership, callback issuance/authentication, package roles/dependencies, platform/wire assumptions and consumer paths remain unchanged. Added private helper calls implement bounds inside that existing boundary; they do not introduce a materially different ownership or integration call graph. No new architectural or nonarchitectural root was established.

## 0. Evidence base

Read the complete canonical State-2 prompt; SHA256 matched `854c2a6f173c6d2f7dd05923ccc75de5df8680b1f5a2ffda22ea63a957fed73e`.

Read full RemPlan-1, Remediation1-Evidence, initial Code/State reports and the updated ReviewCycle sections. Retained originating evidence includes the controlling admission/storage contracts, supersession rules, full kernel, assembly/provider and retained consumers. Independently confirmed the controlling DRAFT, original implementation evidence, admission/storage designs/contracts and library/package policies are unchanged.

Read corrected production files completely: admission 1–457, callbacks 1–251, lib 1–142, projection 1–220 and staging 1–223. Read all new remediation tests: root 1–101, terminal 1–173, integrity 1–163, frontier 1–114, pending 1–137 and exhaustion 1–66. Rechecked relevant issuance and reconstruction support.

Independently compared all 41 protected files against both baseline committed bytes and current committed/live bytes: all matched, including original 43 obligations, support/provider, public types, encoder, lifecycle, released text/corpus and selectors. Protected-source artifact SHA256 matched `c6ca4f64a06722465efb91c68e7194c8f6311b675f7b53e61775946f1c7dc066`. All eleven changed Glade files also matched committed bytes. Architecture policy, process-global checker and allowlist remained unchanged.

Audited existing execution logs without running tests or builds:

| Evidence | Audited result |
| --- | --- |
| Terminal RED / final terminal GREEN | Genuine Started-versus-Terminal failure; corrected two-test suite passes. |
| Integrity RED / GREEN | Parked continuation and direct-fork failures; three tests pass after guards. |
| Frontier and pending RED / bounded GREEN | Concrete ingress, aggregate and reservation failures; twelve interim tests pass. |
| Duplicate-slot RED | Emitted length two versus expected one. |
| Exhaustion RED | Both historical allocation points fail sticky assertion; local control passes. |
| Final all-targets and adopted gates | Core 102 and API 34 pass; architecture, source guards, formatting and Clippy recorded successful. |
| Released text/corpus | Exact ten retained text rows and three-order canonical corpus pass. |
| Process/source guards | 88 Rust files, three permanent entries, zero debt/nothing new; JavaScript compound bodies pass. |

Chronology explicitly discloses correction of an auxiliary new terminal fixture and an intermediate helper compile failure. Neither substitutes for the genuine production regression RED. No retained assertion was weakened.

## 2. Invariant analysis

The terminal guard executes after full request/attempt/cut validation and retirement, while contradictory-terminal authentication remains earlier. Consequently, a late nonterminal can finish its issued invocation without changing settled truth; an unissued assertion still cannot acquire authority.

Integrity checks precede query removal and new work. They retain finite parked continuations and original evidence. Storage resolution remains available for already-owned attempts, so the fix does not turn a stop into lost custody or fabricated NonCommit.

Prospective slot accounting includes current pending slots, retained candidate closure and unpublished/unknown Candidate plans. Frontier events cannot consume reservations required by later publication. Missing-slot deduplication applies to emission as well as reservation. Checked byte arithmetic includes slot representation and owned strings, charged pending custody, live candidate/evidence bytes and unpublished pending charges. Final staging rechecks actual encoded pending charges before Prepare.

Historical allocation failures now preserve the observed refusal through the existing sticky field. No sentinel, fabricated charge, eviction or clearing operation was introduced.

Unchanged qualification, quarantine, complete encoding and original receipt behavior retain their previously reviewed consumer path. Recorded GREEN supports these conclusions; the retraced transitions and corrected guards establish closure.

## 3. Risks and next action

Unknown frontier digests still produce permanent conservative incompleteness under the accepted Slot grammar. This remains acceptable Pure safety behavior and does not qualify eventual exact-cut reconstruction.

The next action is to file this originating State GO alongside the independently formed Code re-verdict and let the lane owner record the dual outcome. This report does not authorize push or live activation, qualify physical persistence/antirollback or production authentication, or replace the later adapter, compatibility and Surface gates. Historical review stops, caps and this object’s remediation-round count remain intact.