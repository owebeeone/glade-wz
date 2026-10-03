# IC-1 internal contract — remediation 1

Date: 2026-10-04. Status: **planned, all findings open pending originating
verification; no successful kernel implementation authorized yet**.

Reviewed tuple: root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`; Glade
`885249a2a093e082aad6e1dc9936a7fd5c54052b`; Glial
`5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`; discovery
`1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld
`64666e8b1caadde8922b9d42163afbab90655c65`.

Both axes returned NO-GO. Reports are preserved verbatim:
[Consistency](GladeIndependentCrdtAdmissionContract-ReviewConsistency.md),
SHA256 `9cea2abf9d2698bb0ea7dcdd44f7099623839c4b290bb5bda9b70503e9b0e890`,
and [Safety](GladeIndependentCrdtAdmissionContract-ReviewSafety.md), SHA256
`ffbe0aace9804eefff5ea0e06d05eb8b496bd37afb0089e9de20310642f33442`.
Three P2 findings have three distinct root causes; Consistency additionally has
one P3 wording finding. There is no blind convergence to claim in this round.
Safety explicitly classifies retention-only recovery as an architectural interface
root cause. This is the first remediation round for the typed-contract object,
distinct from the accepted semantic-design remediation. The hard cap remains.

## One merged correction

| Finding | Disposition | Required executable closure |
| --- | --- | --- |
| Consistency P2-1 | Add an independently allocated monotonic lookup invocation identity, distinct from immutable commit-plan identity; bind it in request/reply and outstanding state. Define retirement after Unknown, new invocation on retry, and safe restoration/invalidation without identity reuse. | Issue L1, resolve Unknown, issue L2; duplicate L1 responses MUST yield CallbackMismatch and preserve L2/reservation. Correct L2 committed response recovers the original outcome without resealing/recommitting. Counter exhaustion MUST not wrap. |
| Safety P2-1 | Make committed lookup describe the atomic batch for every CommitKind: exact batch identity, revision and achieved storage class. Keep retention-only state/outcomes separate from application admission receipts. Define installation and reservation release for Candidate, SecurityEvidence, QualifiedFork and AcceptedBatch. | Compiling RED unknown→committed/known-absent/still-unknown for all four kinds. Candidate and bare rival have no admission. Retain correct state/charges once, preserve common-prefix/pending/independent-instance progress, never emit AcceptedLocal for retention-only work. AcceptedBatch retains its original exact receipt. |
| Consistency P2-2 | Combine the actual post-fork recovery journey: two historically qualifying E0 seq1 rivals produce quarantine, then fresh E1 seq0 references eligible E0 seq0. Exercise opposite orders through actual kernel-returned eligible operations and released text. | Exact eligible identities E0:0/E1:0 and AB, retained old receipts/retries, reject E1 reusing E0. Keep buffering/delivery-order coverage. At IC-2 a mutant refusing any fresh-origin edit when the instance has quarantine MUST fail. |
| Consistency P3-1 | Replace blanket "delayed" callback rejection with retired/mismatched invocation rejection. A delayed matching callback remains legal under the specified physical-start/policy ordering. | Reconcile normative text; retain delayed prior-cut committed/unknown-recovery regression and callback mismatch tests. |

The new batch-recovery value MUST NOT overload Receipt.admission with synthetic
evidence or treat retained security evidence as an admitted operation. Application
receipts MUST stay exactly bound to their staged/retained original admission.
Unknown reservations and original commit identity remain immutable until a matching
batch outcome resolves them. Known absence MUST not discard already accepted
history; unknown MUST not become an invented noncommit or replacement edit.

Write the new regression consumers before changing types/scaffold helpers, then
record compilation and meaningful behavioral RED against the refusing step. Keep
all existing domain obligations; extend the eight trace rows rather than replacing
their actual released-engine consumer. Structural, source, policy, process,
format/clippy and affected conformance gates remain. No classification/dependency/
selector/budget relaxation, new crypto/storage claim, canonical remote schema,
Gyld engine change or live activation is part of this correction.

Use one merged patch. Both originating reviewers MUST verify their own original
counterexamples on the corrected tuple. Because callback and storage-outcome
interfaces change, also dispatch fresh full peer-blind Consistency/Safety axes;
the old broad proofs do not cover the new shape. File all testimony verbatim.
No finding is self-closed by the drafter or lane owner. Only after this gate may
the successful IC-2 kernel implementation begin.
