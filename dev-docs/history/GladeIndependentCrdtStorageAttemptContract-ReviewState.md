# Redesigned CRDT storage-attempt typed contract — STATE-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptContract.md` and its typed-contract/allocation/compiling RED checkpoint at root **`a696f0eef38614fe0cfe2a6b053c352470e800b3`**. DRAFT, dated 2026-10-04.

**Baseline:** Root `a696f0eef38614fe0cfe2a6b053c352470e800b3`; Glade `3cf1fa79cd752012acd0d2ff66d595e293b3433c`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources were inspected directly and compared with exact-pin `git show` bytes.

**Date:** 2026-10-04

**Axis:** Durable-state semantics, recovery legality, finality, bounded capacity and actual consumer/effect evidence. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block acceptance. I pre-commit to GO on a revision resolving P2-1 and P2-2 as specified, provided the correction introduces no additional blocking defect.

---

## 0. Evidence base

All five HEADs matched the stated tuple at the start and end. No writes, builds, tests, network actions or Git mutations were performed. No current peer report was accessed.

Inspection covered:

- Complete canonical State prompt, review-loop skill/template, workspace instructions and local review-cycle authority.
- StorageAttemptContract §§1–9, Evidence chronology/results, accepted lifecycle design and completed design reviews; relevant accepted admission semantics, resource profiles, delivery plan, library/package policy, BuildEntry and GDL-054–057.
- Failed IC-1 contract/evidence, remediation evidence, Safety-2 counterexample, root classification, escalation and preserved review-cycle history.
- API `src/lib.rs:1–265`; memory provider `tests/support/mod.rs:1–676`; all public journeys, six-mutant probe and source guards.
- Core lifecycle/state types, complete STA consumers, shared driver, original Records/recovery consumer setups and assertions, unchanged refusing kernel, actual text trace and released-Taut consumer.
- Manifests, policy, selectors, tooling tests and all-member ARCH002 fixture; relevant SnapshotStore, RecordsFile, system-directory and legacy-seal contracts.
- Frozen Gyld declarations/ledgers, lifecycle capture host and provenance/ownership negatives.

Read-only comparisons established that all 19 changed Glade files and four changed Gyld files matched their pins. The refusing kernel and original text trace remained byte-identical to the Glade baseline. Gyld’s historical semantic declaration/ledger remained unchanged. Its frozen lifecycle text matched the accepted root `01602d89a7130df9cc09c6f4ba889d2b7ab4a4bc`; differences from the reviewed design root were acceptance-status text.

Inventory contains 12 classified libraries, 21 original Records tests, six recovery tests, 13 STA tests and 17 public-port journeys. Recorded GREEN/RED executions and timings were audited against source, not rerun.

## 1. Findings

### [P2-1] Recovery conflates invocation identity high-water with retained-history consumption

**Location:** API `src/lib.rs:220–229`; provider `tests/support/mod.rs:31–81,158–171,448–462`.

**Violated invariant:** Restoration must preserve the finite capacity actually consumed by issued requests independently of scalar identity floors. Recovery must not create an exhausted, unfenceable state from a valid live state.

The live provider tracks separate `invocation_floor` and `invocation_count`. It accepts any fresh scalar above the floor and increments the count once. `Recovery` exports only the high-water scalar. Restoration then sets `invocation_count` to that scalar.

**Reproduction:** With default `invocation_history=128`, prepare one Candidate using invocation number 128. This is accepted: the live count becomes one, its floor becomes 128, and the attempt remains Reserved. Recover that authentic state, restore it, and open generation two. A correctly owned inspect or fence using number 129 is rejected with Capacity because restoration manufactured a count of 128. Before restoration, the equivalent fresh command would consume only the second history entry.

Close remains Pending while the Reserved attempt exists, but every command capable of resolving or fencing it is now refused. Neither corrupt input nor rollback is required.

**Impact:** The development provider has an unintended permanent recovery stop. The typed recovery boundary loses information required to reproduce valid capacity semantics, undermining STA-008/009 and later conformance.

**Required correction:** Retain and validate actual invocation-history consumption or sufficient authenticated retained history separately from the scalar floor. Restore both consistently; do not infer cardinality from the largest ID or silently require dense numbering.

**Closure test:** Execute sparse-ID preparation, authentic recovery/restoration, stale-ID rejection and fresh inspect/fence through the public port. Assert equal remaining capacity before and after restoration, then exhaust the actual declared history count without deleting evidence.

**Architectural-root classification:** **Architectural** — the recovery boundary omits independently necessary retained-capacity state. This is one new architectural root on this redesigned typed object, separate from the failed IC-1 history.

### [P2-2] The actual kernel driver’s local policy fixture cannot satisfy the host’s start check

**Location:** Core `tests/support/mod.rs:36–71,175–178,260–267,320–330`; API provider `tests/support/mod.rs:366–384,566–573`; Records test `records_host_contract.rs:8–22`.

**Violated invariant:** Behavioral RED consumers must be satisfiable through their actual injected producer while preserving local authorization and physical-start semantics.

Core’s initial policy is `Bytes::new(b"granted-test-cut")`; its digest is `4cf19c86fd0a823443c8a8a3a5dbc78bf25fbe25001e8af62c780dbf7fe5231f`. Local staged plans use that policy digest. The shared driver opens a fresh memory session and never supplies its trusted observation. The host therefore uses its fallback `cut()`, whose policy is `[8;32]`.

**Reproduction:** Trace the first valid isolated-local-edit consumer through a conforming future kernel: Verify and Seal succeed, Prepare retains the local plan, and Begin carries that plan’s `granted-test-cut` expectation. The actual host checks both the immutable precondition and its own observation. The latter has a different policy, so it installs NonCommit. The test nevertheless requires AcceptedLocal.

A kernel can make this fixture succeed only by weakening local-start binding, omitting its required precondition, or bypassing the actual producer. The refusing kernel’s present RED result conceals this second failure source.

**Impact:** The checkpoint’s actual-port path cannot demonstrate the intended local success obligation. This affects the original isolated-admission witness and local portions of the released-text journeys.

**Required correction:** Configure the producer with the explicitly trusted policy/time observation used by the core fixture before driving local work. Preserve the independent host check; copying each Begin caller’s claim into trusted observations would defeat the contract.

**Closure test:** Add a port-level journey using the core fixture’s exact policy digest and start interval, proving original receipt publication through the configured producer. Change the independently injected observation and prove stale Begin produces NonCommit. Keep kernel/domain consumers behaviorally RED against the refusing scaffold and preserve all original assertions and ten text rows.

**Architectural-root classification:** **Non-architectural** — bounded test-provider wiring defect; the existing observation injection operation can support the correction.

## 2. Invariant analysis

The original absence-before-publication attack fails against the normative lifecycle: missing outcomes, unknown registration and lost replies remain Pending; only retained fencing can establish NonCommit. Prepare deduplication retains the original PlanKey/attempt mapping. Started cuts survive later revocation. Publication and fencing share one terminal predicate, with no rebase past a tombstone.

Committed outcomes retain whole original bindings, receipts and achieved class. Terminal contradictions require authenticated issued/consumed requests, retained contrary evidence and an instance integrity stop. Sticky recovery incompleteness has no clearing event, preventing quota restoration from inventing completeness.

The narrow Contract/Pure split is meaningful. Shared values do not import admission algorithms or concrete storage. Selectors include both affected packages; framework refusal covers every classified member. Gyld adds explicit lifecycle ownership while preserving frozen semantic allocation and source-qualified obligations.

The initial TDD deviation remains disclosed; compiling behavioral RED is distinguished from compiler failures and successful model conformance. The deliberately refusing kernel and deferred physical implementation are not missing delivered features.

## 3. Risks and next action

Physical publication ordering, old submitted I/O, process termination/reopen, antirollback, quota reservation, cryptography and live duplex remain unqualified. Existing RecordsFile locks/fsync do not establish composite attempt finality. Recorded Gyld timing has only 0.042 seconds of budget margin.

The next action is one scoped remediation addressing both findings, followed by the required settled-tuple review. Successful kernel work must remain stopped.