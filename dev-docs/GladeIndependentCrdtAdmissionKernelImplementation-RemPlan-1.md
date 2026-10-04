# IC-2 kernel — merged remediation 1

Date: 2026-10-04. Status: **one test-first correction authorized; findings open**.
Both independent axes reviewed root `332e7d95f5887300bf762ac76737e4cc38bcd151`,
Glade `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14`, with unchanged Glial,
discovery and external Gyld pins. [Code](GladeIndependentCrdtAdmissionKernelImplementation-ReviewCode.md)
and [State](GladeIndependentCrdtAdmissionKernelImplementation-ReviewState.md)
both returned NO-GO. Four defects were found independently by both axes;
State found one additional failure-bookkeeping defect. Nine finding IDs map to
five roots. Both reviewers classified every root **nonarchitectural**.

This is remediation round1 of the new IC-2 implementation object. Architectural
root count remains zero; all earlier object histories/caps remain intact.
No finding is self-closed. The same source drafter MUST produce one merged patch
and execute each regression RED before its correction.

| Finding IDs | Root and disposition | Required closure evidence |
| --- | --- | --- |
| Code P2-1; State P2-1 | Accept. Late authenticated nonterminal observations MUST NOT downgrade a retained terminal attempt or return it to recovery issuance. | Actual-port held Begin/Started followed by committed or fenced terminal Inspect, then delayed Started; both delivery orders, all four kinds. Check terminal phase/result, original custody/charges, validated fixture restoration and no renewed lookup. Include Pending/Refused late observations where applicable. |
| Code P2-2; State P2-2 | Accept. Enforce existing integrity stop at every continuation/fork/staging path that creates new work. | Park verification and sealing across an authenticated terminal contradiction; no new Seal/Prepare/Begin/admission or charge afterward. Already-qualified OfferFork must refuse new retention. Preserve genuine old owned-work settlement, original receipts/reservations/evidence, exact retries and independent Y progress. |
| Code P2-3; State P2-4 | Accept. Bound frontier names and aggregate retained/emitted slot bytes before cloning into persistent state/effects. | Empty/name-limit/limit+1/very-large names and exact/overflow aggregate byte cases with pre-existing pending work. No oversized retained state/effect, false charge, eviction or false complete read. Use existing name/byte limits; record sticky observed recovery loss on bounded refusal. |
| Code P2-4; State P2-3 | Accept. Check prospective deduplicated missing-slot union from candidates, frontiers and current pending state before staging or emitting recovery requests. | Two absent refs with pending_items1 refuses; exact-limit succeeds; overlap deduplicates; multiple candidates and candidate/frontier combinations stay within bounds at every transition. Preserve custody/charges/heads on refusal and sticky loss. |
| State P2-5 | Accept. Historical effect-ID exhaustion MUST record sticky recovery loss at offer and plan-allocation boundaries. | next_effect_id MAX and MAX-1 historical journeys refuse without wrap or false completion; sticky survives ResumeRecovery/validated fixture continuation. Prior custody/charges stay unchanged; local-intent-only exhaustion control need not taint history. |

The accepted shared API, State/event/effect fields, package roles/dependencies,
physical mutation boundary, provider/assembly, original43 domain assertions and
released text/corpus remain fixed. No new interface, clearing witness, metadata
namespace, selector or allowlist exception is authorized. The conservative
unknown-frontier-digest limitation remains explicit; both reviews found no
additional blocker there. This is not a live reconstruction qualification.

Run the affected core/API all-targets and adopted architecture/test/fmt/clippy
gate, exact ten released-text/corpus checks, source/process checks and whitespace
after correction. Existing rejecting mutants need rerun only if their protected
logic materially changes; record that decision and preserve prior failures.
No whole-workspace substitution or redundant unchanged Gyld run.

The lane owner MUST settle a new complete five-repository tuple and file evidence.
Originating reviewers will retrace their own original counterexamples and inspect
the complete corrected range with closure tables. Context retention is appropriate
only if interfaces, architecture, compatibility/platform assumptions, ownership
and previously reviewed production call graph remain materially unchanged. If the
patch changes those boundaries, new fresh full axes are required instead. The
reviewers MUST independently assess that premise. Source stays fixed during review.
No push, live change, component acceptance or activation is authorized by this plan.
