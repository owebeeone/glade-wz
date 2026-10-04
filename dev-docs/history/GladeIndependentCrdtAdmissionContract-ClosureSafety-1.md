# IC-1 internal admission contract remediation 1 — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, DRAFT internal semantics and compiling behavioral RED gate at root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`.

**Baseline:** Corrected tuple: root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`; Glade `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` at `64666e8b1caadde8922b9d42163afbab90655c65`. Sources inspected directly, with committed baseline contents and scoped diffs.

**Date:** 2026-10-04

**Axis:** Safety, focused verification of the original retention-only unknown-outcome counterexample and its correction. Independent, adversarial, read-only. No current peer report was read. Filed verbatim by the lane owner.

**Restored context:** I am a new reviewer process. The original live Safety reviewer was unavailable after interruption. I restored its role from its complete verbatim report and original prompt, as explicitly required by the review ledger. I independently retraced its counterexample; I do not claim original-agent continuity. The preserved prompt, report, baseline types, correction and executable consumer provide sufficient evidence for this focused verification.

**Verdict: GO** — Safety P2-1 is closed for the internal contract/RED gate. Zero new P0–P3 findings within this focused review. This verdict does not replace the required fresh full reviews or accept successful kernel behavior.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Safety P2-1 | Recover every atomic commit kind through a batch-level committed result; distinguish retention custody from admission receipts. | Independently retraced candidate and bare-rival retention → lost acknowledgement → unknown → exact committed lookup. Empty receipt lists now represent both outcomes faithfully. Installation, reservation release and once-only charges are specified and encoded as compiling RED obligations across all four kinds. | CLOSED for IC-1 semantics/RED; implementation remains deliberately absent. |

## Changed-range analysis

The correction introduces `BatchCommitted`, an optional operation retry target, `BatchRetained`, explicit retained fork pairs and independent lookup invocation identities. It extends recovery consumers while preserving original receipt recovery assertions. These changes implement the merged remediation dispositions.

The original defect remains classified as an **architectural interface root cause**: the former committed grammar required application admission where only batch retention existed. Its remedy is bounded and now represented coherently. No new architectural root cause was established in this focused verification.

## 0. Evidence base

Read:

- `AGENTS.md`, `AGENTS_GWZ.md`, the review-loop skill and canonical review template.
- Original `GladeIndependentCrdtAdmissionContract-ReviewSafety.md` and `PromptSafety.md`; their SHA256 values match the recorded `ffbe0aace9804eefff5ea0e06d05eb8b496bd37afb0089e9de20310642f33442` and `627a18df3cfc52ab1227fea8de52db80e48ad5235d8e6d1289fa93065e9c3061`.
- Merged `RemPlan-1.md`, complete `Remediation1-Evidence.md`, controlling contract and accepted semantic design, especially custody/qualification separation and design §7’s explicit evidence-retention outcomes.
- `types.rs:1–397`, `recovery_contract.rs:1–526`, `tests/support/mod.rs:1–333`, refusing `lib.rs`, relevant original recovery consumers and their remediation diff.
- Original Glade types at `885249a2a093e082aad6e1dc9936a7fd5c54052b`, confirming mandatory `Receipt` in the former committed lookup result.

Start and end HEAD checks matched all five corrected revisions. Read-only comparisons confirmed inspected corrected contract, evidence, types and recovery consumers match HEAD. The refusing implementation, architecture policy and Cargo lock are unchanged from the original reviewed Glade revision.

No writes, builds, tests, network actions or Git mutations were performed. Recorded compilation, GREEN checks, 27 Rust behavioral failures and ten released-text failures are drafter evidence, inspected against source rather than independently rerun.

## 1. Findings

No new findings.

## 2. Invariant analysis

The original counterexample was real at the old tuple: `CommitKind::Candidate` or `SecurityEvidence` could retain records without any staged admission, yet committed lookup required a `Receipt` with a mandatory admission digest. Storage could not faithfully acknowledge committed retention without inventing admission semantics.

The corrected grammar removes that requirement. `types.rs:258–291` defines committed lookup through `BatchCommitted`, binding batch digest, revision, achieved storage class and a receipt vector. `LookupRequest.operation` is optional. `types.rs:310–322` distinguishes batch custody from `AcceptedLocal` and identifies uncertainty by batch digest.

Contract §6, lines 269–302, supplies the necessary semantics:

- Candidate and security plans require empty acceptances and receipt lists.
- Matching committed lookup attests the entire original batch and uses the same installation path as a direct committed barrier.
- Candidate installation retains unresolved records and missing slots without accepted/head advancement.
- Bare security evidence installs no accepted record or conviction.
- Qualified forks require independently qualified records, preserving genuine historical receipts where new custody is staged.
- Resolution releases the reserved plan and adds charges once; duplicate answers cannot repeat installation.
- Known absence preserves existing custody and charges; unknown preserves the reservation and existing classification.
- Independent instances remain operational.

Consequently, the original retention-only sequence now has a faithful terminal answer: the exact committed batch with `receipts=[]`, followed by `BatchRetained`. It requires neither a synthetic admission nor indefinite uncertainty.

The regression consumer encodes this rather than merely constructing a new type. `recovery_contract.rs:8–64` creates an unresolved candidate and bare rival with `admission=None` and empty acceptances. Lines 176–347 exercise unknown direct barriers followed by committed, absent and still-unknown results for every commit kind. Assertions cover retained state, once-only charges, reservation release/preservation, original custody, pending/common-prefix behavior, nonconviction, duplicate refusal and independent-instance local progress. Lines 349–390 reject changed batch digest, revision, storage or invented receipts. Original accepted-batch exact receipt recovery remains at `records_host_contract.rs:719–793`.

The scaffold still reports `Unavailable` for every event. The looped matrix therefore fails before completing all twelve journeys; these are compiled future behavioral obligations, not demonstrated recovery successes. That limitation is stated accurately in the evidence and is consistent with this gate.

## 3. Risks and next action

This closure accepts a coherent internal recovery contract and meaningful RED obligations. It does not qualify physical atomicity, restart, antirollback, cryptography, trusted clocks, custody, automatic duplex synchronization or activation.

The lane owner should combine this restored-role closure with the other required closure and fresh full peer-blind verdicts on the exact tuple. Successful IC-2 behavior must remain deferred until that gate is accepted, then satisfy the existing recovery consumers without weakening their assertions.