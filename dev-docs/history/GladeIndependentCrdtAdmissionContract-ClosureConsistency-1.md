# IC-1 independent CRDT admission contract — CONSISTENCY ORIGINATING-ROLE VERIFICATION

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, internal contract and compiling behavioral RED remediation1.

**Corrected tuple:**

| Repository | Exact HEAD |
| --- | --- |
| Workspace root | `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb` |
| Glade | `6a0cc5a78da38a023615f6adc5fc354bba4af0b4` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `64666e8b1caadde8922b9d42163afbab90655c65` |

**Date:** 2026-10-04.

**Axis:** Independent, adversarial, read-only verification of the original Consistency counterexamples. No current peer reports were inspected.

**Restored context disclosure:** This is a new reviewer process restoring the originating Consistency role from its original verbatim prompt and report. It is not the original live reviewer and cannot claim uninterrupted agent continuity. The preserved artifacts contain sufficient concrete counterexamples, required corrections and closure witnesses to independently retrace all three findings. This report provides restored-role verification, with that procedural limitation explicitly recorded.

**Verdict: GO** — Consistency P2-1, P2-2 and P3-1 are closed for the **internal contract/compiled RED gate only**. No new concrete finding was identified. This does not establish successful kernel behavior or independently accept the whole object.

## Prior-finding closure table

| ID | Disposition claimed | Independently verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Distinct lookup invocation identity; retirement, retry, restoration and exhaustion semantics | `types.rs:274–291,389–391` separates `lookup_id` from retained `plan_id`. Contract `90,253–267` requires fresh checked IDs, high-water restoration, invocation retirement and reservation preservation. `recovery_contract.rs:98–174,499–526` traces L1→Unknown→L2, all three stale L1 answers, original-receipt recovery without Seal/Commit, exhaustion and restored high-water handling. | **Closed at contract/RED tier** |
| P2-2 | Actual combined post-fork E1 recovery witness | `recovery_contract.rs:393–495` establishes E0 seq1 fork quarantine before submitting fresh E1 seq0, in both rival orders, then checks exact eligibility and retained receipts/retries. `text_admission_trace.rs:108–238` drives that same state through recovery. Glial consumer `independent_admission_contract.mjs:71–82` requires exact identities, released AB, preceding A, persistent quarantine, qualified rivals, exact retries and fresh receipt. | **Closed at contract/RED tier** |
| P3-1 | Refuse retired/mismatched callbacks rather than elapsed delay | Contract `129–132` explicitly permits delayed matching replies subject to policy/start ordering; `230–240,275–277` retains prior-cut historical commits. `records_host_contract.rs:845–927` preserves delayed direct commit and unknown-recovery receipt assertions after policy revocation. | **Closed** |

## Changed-range analysis

Compared the original reviewed root/Glade/Glial revisions (`7c5ac428…`, `885249a2…`, `5fd46ba5…`) with the corrected tuple.

The authored changes are the contract/evidence, lookup and atomic-batch types, new recovery consumer, existing fixture adaptations, Rust text trace and Glial consumer. Filed historical review/prompt/remediation records and workspace pin bookkeeping explain the additional root changes.

The shared recovery interface changed: `LookupResult::Committed` now carries `BatchCommitted`, operation targeting is optional, and retention outcomes and explicit qualified-fork pairs are represented. These implement the merged remediation rather than unrelated scope. Fresh full review remains necessary because the original broad interface proofs do not cover the new shape.

**Architectural classification:** Original Consistency P2-1 is an internal continuation-interface root; its correction does not redesign the accepted admission semantics. P2-2 is a bounded regression-coverage root. P3-1 is contradictory normative wording. The merged plan separately identifies Safety’s retention-recovery architectural interface root; this report does not close that reviewer’s finding. No **new architectural root cause** was identified, and no third-root escalation is triggered by this verification.

## 0. Evidence base

Read root `AGENTS.md`, `AGENTS_GWZ.md`, review-loop skill and canonical template; the complete original Consistency prompt/report; merged `RemPlan-1`; new `Remediation1-Evidence`; controlling contract; and accepted semantic design.

Read the changed types and relevant complete recovery, trace, fixture and JS consumer paths, plus existing delayed-commit/lookup consumers. Inspected committed revision diffs and tracked working-tree cleanliness.

Read-only hashes confirmed:

- Original prompt: `d8e686ecae732fd13f6bbb21962a9ae73e106f07e3ea8913cb922b3aebc2d2a1`.
- Original report: `9cea2abf9d2698bb0ea7dcdd44f7099623839c4b290bb5bda9b70503e9b0e890`.
- Refusing `lib.rs`: `5e59d4d2f5bc574c81c7412e3f1e3546bc9dbcc6aba4cad48572f447bfd45539`.

Scoped comparisons confirmed the semantic design, refusing implementation, architecture policy and Cargo lock are unchanged from the original reviewed revisions.

All five HEADs matched at start and end. Tracked diffs were empty. The two untracked current prompts were excluded and unread.

No writes, tests, builds, network calls or Git mutations occurred. Execution results below are committed drafter evidence audited against source, not independently rerun results.

## 1. Findings

No new concrete findings.

## 2. Invariant analysis

**Repeated Unknown is now representable without callback ambiguity.** L1 and L2 retain identical immutable commit identity but have distinct invocation identities. An old Unknown, KnownAbsent or Committed response cannot legally consume L2. The regression compares the entire state after each stale response, covering outstanding continuation and reservation preservation. Restoration explicitly assumes trusted high-water recovery; it does not claim disk qualification.

**The original escaping post-fork mutant is now caught.** A future implementation that refuses every fresh-origin edit whenever quarantine is nonempty would fail the combined Rust assertion and actual-text fresh-receipt/AB assertions. The trace keeps the pre-recovery forked state, submits E1 against it, and selects operations solely from kernel-returned eligibility. No expected-operation fallback supplies AB. Separate AB-buffer rows retain dependency-order coverage.

These witnesses agree with accepted design `402–412` and ICD-009 at `649`: qualified E0 rivals exclude the forked suffix while preserving E0 seq0; separately certified E1 references that eligible prefix under a new canonical origin.

**Delay and identity retirement now have coherent outcomes.** A delayed matching prior-cut committed reply installs historical custody; a retired or mismatched continuation yields CallbackMismatch. Current revocation cannot retroactively erase the committed outcome, while stale pre-start validation remains separately constrained.

**Evidence strength remains bounded correctly.** `Remediation1-Evidence:81–116` records successful compilation, 21 existing plus six new behavioral Rust failures, and ten text failures against the unchanged safe-refusing scaffold. It expressly discloses that the looped recovery matrix stops at its first assertion. These are compiled future obligations, not successful recovery demonstrations. Structural/architecture/source checks are recorded GREEN without relabeling domain RED as implementation acceptance.

## 3. Risks and next action

The IC-2 quarantine mutant must still be rejected after successful behavior exists. Genuine cryptographic evidence, custody/time trust, physical atomic commits and restart, automatic duplex mesh, client lifecycle, capacity/loss and activation remain later qualifications.

The lane owner should file this report verbatim, preserve the restored-process disclosure, and combine it with independent Safety originating verification and fresh full Consistency/Safety verdicts on this exact tuple. Only the resulting accepted contract gate can authorize successful IC-2 implementation.