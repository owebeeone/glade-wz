# IC-2 Pure admission/reconciliation kernel, remediation 1 — CODE-AXIS REVIEW

**Review object:** Originating Code re-verdict after one merged remediation, Glade `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494`; controlling implementation DRAFT, RemPlan-1, and Remediation1-Evidence at workspace root `aa4f7e28334baa4ef515487792ca785061de597c`. Deterministic component acceptance remains pending.

**Baseline:**

| Repository | Exact reviewed HEAD |
| --- | --- |
| Workspace root | `aa4f7e28334baa4ef515487792ca785061de597c` |
| Glade | `03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were inspected directly and compared with pinned `git show` bytes. All five HEADs matched at start and end.

**Date:** 2026-10-04  
**Axis:** Architecture, interfaces, call graphs, and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — all four original Code findings are closed, but one newly identified P2 blocks. The new root is nonarchitectural; architectural root count remains zero. I pre-commit to GO on a revision that resolves P2-5 as specified while preserving the verified closures and reviewed boundaries.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Preserve terminal settlement against delayed nonterminal replies | Retraced held actual-port Begin/Started, terminal Inspect first, then delayed Started. `callbacks.rs:126–130` now retires the authenticated invocation and returns before phase downgrade or OutcomeUnknown. The new actual-port matrix covers four kinds, both terminal outcomes, and both delivery orders; it checks custody/charges, restoration, and no renewed recovery. Pending/Refused controls also preserve terminal state. | Closed |
| P2-2 | Enforce integrity stop throughout new-work continuations | Retraced verification and seal parked across an authenticated contradiction. `admission.rs:225–228` preserves the matched query and emits no new work. Already-qualified OfferFork stops at 433–436; staging additionally stops at 73–76. Controls preserve original receipts, independent Y publication, and genuine settlement of already owned Started work. | Closed |
| P2-3 | Bound frontier names and aggregate retained bytes | Retraced the million-byte unknown origin. `lib.rs:78–88` refuses before retaining/copying it into missing-slot effects, sets sticky loss, and preserves charges. `pending_fits` checks slot strings, fixed representation, committed pending charge, live queries, and unpublished pending reservations. Empty/exact/overflow names and aggregate byte boundaries have direct regressions. | Closed |
| P2-4 | Bound candidate-derived missing-slot union before emission/staging | Retraced the historically valid two-ref candidate with `pending_items=1`. `admission.rs:317–325` refuses before RequestMissing or Candidate staging. Staging checks prospective slots and final encoded pending charges. Tests cover exact-limit success, overlap, frontier/candidate combinations, unpublished reservations, and predecessor/ref emission deduplication. | Closed |

## Changed-range analysis

Reviewed Glade `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14..03d0fcd7629fcdbd5e5d44b9f9dcd9e32aec3494` and root `332e7d95f5887300bf762ac76737e4cc38bcd151..aa4f7e28334baa4ef515487792ca785061de597c`.

Production changes are confined to five existing private kernel files: admission, callbacks, dispatcher/frontier, projection, and staging. Six new test files add 16 regressions. Complete changed functions and their callers were inspected, including the shared pending helpers and neighboring busy-instance branches.

Originating-context retention remains justified. Public types, event/effect grammar, shared storage API/provider, publication ownership, callback issuance boundaries, encoder, lifecycle module, dependencies, roles, platform assumptions, and released consumers are unchanged. The new helpers alter private validation calls; they do not relocate a material architectural responsibility.

The newly identified P2-5 is an existing busy-path continuation defect exposed by inspecting the corrected staging range. It requires no interface or ownership change and is not an architectural root.

## 0. Evidence base

Read the complete generated Code-2 prompt; SHA256 matched `915ea1526e795de46feaee7577451321774fc6eda2301b4e7fcf78f0fd3ce1aa`. Read full RemPlan-1, Remediation1-Evidence, controlling DRAFT/evidence, and legitimate initial Code/State reports. No current peer re-verdict was accessed.

Inspected complete corrected production ranges:

| File | Inspected range |
| --- | --- |
| `admission.rs` | 1–457 |
| `callbacks.rs` | Authentication/terminal handling 1–160 and downstream installation/replay |
| `lib.rs` | 1–142 |
| `projection.rs` | 1–220 |
| `staging.rs` | 1–223 |
| Unchanged `lifecycle.rs` | Stage/resume/prepare call paths, especially 64–168 |

Read all new remediation tests: root 1–101, terminal 1–173, integrity 1–163, frontier 1–114, pending 1–137, exhaustion 1–66. Retained accepted contracts and prior complete-source/consumer inspection supplied the unchanged context.

Independent read-only comparisons verified all 41 protected-source entries against the preceding baseline and their recorded hashes. Forty-five core/API package files matched the new Glade pin; only the five declared production files changed among existing package files. Inspected Glial consumer/source/corpus bytes matched their unchanged pin.

Audited RED logs for terminal scheduling, integrity continuations, frontier bounds, prospective pending unions, emitted deduplication, and both historical counter-exhaustion points. Audited corrected terminal/integrity logs and final adopted results: 102 core tests, 34 API tests, architecture, formatting, Clippy, source guards, actual released text consumer, and canonical corpus. Process-global evidence reports 88 files, three permanent entries, and nothing new.

The terminal auxiliary fixture correction and initial helper compile failure are disclosed in the chronology. Initially passing controls were not treated as RED witnesses. Prior isolated mutants protect unchanged encoder and qualification predicates; the decision not to rerun them is consistent with the merged plan.

No tests, builds, mutations, or network operations were performed by this reviewer. Recorded timings include cache/concurrency and Cargo lock contention.

## 1. Findings

### [P2-5] Busy-instance diversion discards verified historical work and later permits a falsely complete read

**Location:** [staging.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:77), lines 77–83, reached after [admission.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:233), line 233 consumes the verification query. The same discard mechanism exists at the fresh-offer busy branch, admission lines 78–85.

**Violated invariant:** Retained admission contract §5 requires `complete_local=false` for unresolved observed history; §§3/6 require retained continuations or honest bounded refusal. Resuming one existing plan does not classify a different offered historical record.

**Credible sequence:**

1. On valid empty X with sufficient quotas, OfferReplica two distinct, historically admissible sequence-zero records A and B. Both verification queries are issued before either stages.
2. Resolve A’s exact query. Execute its Prepare/Begin through the actual memory port with publication held; deliver Started. A is now retained in `unknown_commits`; B’s query remains outstanding.
3. Deliver B’s exact Verified reply. Its historical admission and empty dependency closure pass. Admission removes B’s query, constructs its acceptance, and calls staging.
4. Staging sees A’s unresolved plan, calls `resume(X)`, and returns. Only A’s Inspect is issued. B is not retained in queries, candidates, plans, accepted custody, or any recovery-loss marker.
5. Publish A and deliver its authentic Committed Inspect result. A settles normally. With no other pending work, `projection::cut` returns `complete_local=true`, although the kernel discarded the verified, observed B.

This is a source-traced counterexample, not an executed regression claim. It uses ordinary permitted concurrent verification and the unchanged actual-port lifecycle.

**Impact:** An ordinary scheduling interleaving loses recovery work and falsely reports completion. No receipt for B is fabricated, but no retained continuation or refusal tells future recovery that B remains unclassified. Existing new tests park B until after A’s terminal settlement or across integrity failure; they do not exercise B’s successful verification while A remains unresolved.

**Required correction:** A busy-instance branch must preserve the distinct historical continuation within bounds, or explicitly refuse it and record sticky recovery incompleteness. Apply the rule to both fresh historical offers and already matched verification continuations. Keep A’s immutable reservation and its recovery unchanged; local intent refusal alone need not taint observed history. Existing types suffice.

**Closure/regression:** Add the exact two-query, held-Started sequence above through the actual port. After A settles, require either retained B recovery that subsequently installs its original identity or explicit bounded refusal with persistent incomplete status. Check Read, ResumeRecovery, validated restoration, unchanged A receipt/charges, and independent Y progress. Also cover a fresh historical offer during A’s unresolved interval and a local-intent refusal control.

**Root classification:** Nonarchitectural. The defect is failure to retain or record refused historical work at existing busy gates; no new lifecycle operation, schema, library, or ownership boundary is required.

## 2. Invariant analysis

The corrected terminal guard follows full request/cut authentication and invocation retirement. It leaves contradictory-terminal detection before the guard and prevents late nonterminal observations from reopening recovery.

Integrity checks now cover matched verification, sealing, direct qualified forks, and staging. Existing storage settlement remains separate, so the stop does not erase genuine late commits or old receipts.

Pending checks account for deduplicated slots from retained state, candidates, and unpublished/unknown Candidate plans. They use checked arithmetic and final encoded pending charges before issuance. Missing-request deduplication preserves qualification’s original dependency predicates; it does not select a fork winner.

Historical query and plan counter exhaustion now sets sticky recovery loss. Local-only exhaustion controls preserve known-history completeness. These corrected failure branches do not cover the busy diversion reported above.

Unchanged encoding, qualification/quarantine, fresh-origin continuation, shared interfaces, and actual released text-consumer behavior retain the prior evidence. No test expectation, selector, budget, classification, or allowlist was relaxed.

Unknown frontier digests still conservatively make recovery incomplete under the accepted Slot grammar. This remains an explicit progress limitation rather than a false reconstruction claim.

## 3. Risks and next action

The single next action is a scoped, test-first correction for P2-5, followed by required affected-package/consumer gates and originating review on another settled five-repository tuple. Remediation 1 closes the original four Code findings; this new finding remains open. Architectural root count remains zero, and historical review caps/stops remain intact.

This report does not qualify production crypto, physical persistence/antirollback, cancellation/drain, live duplex, migration, Surface, or activation.