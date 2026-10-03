# IC-1 internal admission contract — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, DRAFT remediation1 at root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`; typed refusing scaffold, compiling behavioral RED consumers and frozen Gyld allocation.

**Baseline:** Reviewed Glade `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; external Gyld `64666e8b1caadde8922b9d42163afbab90655c65`. Correction baselines: root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`, Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`, Glial `5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`. Sources were read directly, checked against exact-pin `git show` contents and inspected through scoped baseline diffs.

**Date:** 2026-10-04

**Axis:** What the contract permits to go wrong under degraded operation, irreversible transitions and recovery. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks acceptance. I pre-commit to GO on a revision resolving P2-1 as specified, provided the correction introduces no additional blocking defect.

---

## 0. Evidence base

All five HEADs matched the stated tuple at the beginning and end of inspection. The complete canonical generated Safety-2 prompt, complete contract, original evidence, merged RemPlan-1, remediation1 evidence and prior-round Safety report were read. No current peer report or restored closure was read.

Inspection covered:

- Contract §§1–9; complete `types.rs:1–397`, refusing `lib.rs`, `records_host_contract.rs:1–927`, `recovery_contract.rs:1–526`, support, representation and source-boundary tests.
- Complete Rust text trace and both JS consumers, including the ten-row inventory, released-engine reference control and actual kernel eligibility consumption.
- Accepted design’s identity, authorization, historical qualification, custody, bounds, recovery, migration, canonical supersession and IC-1/IC-2 requirements; resource consistency requirements, delivery plan and GDL-054–057.
- Library policy, BuildEntry, package architecture and architecture ownership; BindingResolver, ReplicaSync and SnapshotStore contracts; relevant Substrate W1–W8, Authz private/current-access rules, discovery/genesis, CRDT adapter, shape catalogue and Raft/seal coexistence provisions.
- Relevant legacy admission, Store, envelope, signing, mesh and Glial text sources; package manifests, architecture policy, selectors, ARCH-002 fixtures and tooling regressions.
- Complete Gyld overlay declaration/capture host and added capture tests.

Read-only comparisons confirmed inspected contract/source files match their exact pins. Frozen Gyld design text matches the canonical design and its digest; the base ledger digest matches. Original architecture declaration, base ledger, checker, selector and original architecture-test prefix remain unchanged.

No builds, tests, network actions, writes or Git mutations were performed. Recorded compilation, GREEN checks, timings and behavioral RED executions are committed drafter evidence audited against source, not independently rerun.

## 1. Findings

### [P2-1] Negative lookup recovery does not require finality against the original in-flight commit

**Location:** Contract §6, lines 242–267 and 295–302; `types.rs:267–291`; `recovery_contract.rs:176–247`.

**Violated invariant:** An uncertain atomic commit’s reservation and outcome identity may be released only after definitive noncommit, not merely after observing that its batch is presently absent. Cancellation must not imply rollback.

The contract explicitly recognizes that cancellation cannot be assumed to roll back a plan. Nevertheless, its negative lookup result is `KnownAbsent`, described as releasing the reserved plan. Neither its type documentation nor normative semantics requires the provider to establish that the original attempt has terminated or been fenced and cannot subsequently commit.

“Release only a proven absent batch” establishes absence, but does not establish terminal noncommit. The host’s serialization of commit barriers does not itself order a lookup behind completion or fencing of an outstanding storage mutation. The distinction matters precisely on the unknown/cancellation path.

**Credible interleaving:**

1. Plan P begins under a valid policy cut. Its asynchronous storage work has not yet installed the atomic batch.
2. The host loses the result or cancels its waiting task and reports `OutcomeUnknown`; P remains reserved.
3. Lookup L reads the storage outcome index before the original work finishes. The exact batch is absent. The provider returns the fully matched `KnownAbsent` answer.
4. The kernel follows §6: retires L and releases P.
5. The original work completes, atomically installing P at its expected revision. Its delayed committed callback now lacks an outstanding reservation and is rejected as stale.
6. In-memory custody, charges, receipt recovery and physical storage diverge. A newly scheduled plan may also encounter an unexplained revision change.

The provider need not forge bytes, mismatch identities or violate atomic installation to produce this sequence. The missing obligation is terminal-negative recovery. Current tests inject `KnownAbsent` directly and verify release; they do not distinguish temporary absence from a resolved original attempt.

**Impact:** This is a P2 recovery/interface defect before successful implementation, not active corruption by the refusing scaffold. It can strand physically committed admissions or retention batches after the kernel has discarded their recovery identity.

**Required correction:** Define `KnownAbsent` as a definitive terminal noncommit assertion for the exact original plan, or rename it accordingly. The provider MUST establish under its commit/outcome ordering that the original attempt cannot subsequently install the batch, including outstanding work and restart recovery. A missing index entry, timeout or cancellation alone MUST return `Unknown`. Apply the same terminal-negative requirement to direct `KnownNotCommitted`. Mechanism selection remains an adapter gate; the semantic precondition must be fixed now.

**Closure/regression:** Add a compiling deterministic host consumer with a pending original write and an absent outcome observation. It must return `Unknown` and preserve P until that attempt resolves or is demonstrably fenced. Exercise both later commit recovery and terminal noncommit, preserving once-only charges/original receipts and independent-instance progress. Retain the four-kind recovery matrix. Physical cancellation/reopen qualification remains separately required at IC-3.

This is an **architectural storage-outcome lifecycle root cause**, with a bounded normative/interface and consumer-test remedy.

## 2. Invariant analysis

The other attacks did not establish a blocking defect.

Immutable operation/byte ownership prevents independently asserted digest substitution. Descriptor/certificate snapshots and trusted `Facts` have explicit exact-byte authentication obligations; public internal structs are not authorized ingress values.

Verification and seal continuations carry complete queries, original validation modes and facts. Lookup invocation identity is now independent of plan identity; retirement, repeated Unknown, stale L1 replies, high-water restoration and counter exhaustion have concrete compiled obligations.

Batch recovery now covers all four commit kinds without synthetic admission receipts. Candidate/security retention has empty admissions; qualified forks retain explicit independently qualifying pairs; committed lookup binds complete original staged receipts, revision, batch and storage class. The prior retention-only grammar defect is addressed by this shape.

Historical qualification remains independent of derived quarantine. Bare invalid rivals cannot convict; qualified forks retain custody while excluding branches/dependents. Common-prefix heads and fresh E1 recovery preserve old identities. Combined AB assertions cover the actual post-fork journey.

The text consumer contains no successful fallback. Ten exact rows require kernel-selected operations and released Taut projection; ABC checks isolated receipts, opposite-direction offers and pinned corpus bytes.

Pure classification, dependencies, disabled-branch source checks and all-member ARCH-002 controls remain scoped. Gyld preserves inherited ownership/source relationships and adds the explicit ICD allocation without engine expansion or satisfaction claims.

## 3. Risks and next action

Synthetic evidence, volatile barriers and released merge controls do not qualify crypto, physical storage, antirollback, current serving, automatic mesh transfer, client lifecycle, quotas or mixed-version activation. Surface review remains mandatory before user-facing freeze. Recorded baseline Mypy limitations remain disclosed.

The next action is a scoped correction of P2-1 and its compiling RED consumer, followed by required verification/review before successful kernel implementation.