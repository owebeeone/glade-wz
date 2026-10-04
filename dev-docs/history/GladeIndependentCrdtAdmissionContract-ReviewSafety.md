# IC-1 internal admission contract — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, DRAFT at root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`; typed refusing scaffold, compiling behavioral RED consumers and frozen Gyld allocation.

**Baseline:** Reviewed root `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`; Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`; Glial `5fd46ba5180051eb20d7b5547f59f52c0f3ebe06`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `64666e8b1caadde8922b9d42163afbab90655c65`. Diff baselines: root `4d735893c8db0a9ac9b4de9cde01600873b20ce3`, Glade `c65a6e87f0c257c15de8db080c29d365a883af85`, Glial `4c6e856`, Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were inspected directly and against Git baseline contents.

**Date:** 2026-10-04

**Axis:** What the contract permits to go wrong under degraded operation, irreversible transitions and recovery. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks acceptance. I pre-commit to GO on a revision that resolves P2-1 as specified, provided the correction introduces no additional blocking defect.

---

## 0. Evidence base

Start and end `git rev-parse HEAD` checks matched all five reviewed revisions. The complete generated Safety prompt was read; its SHA256 matched `627a18df3cfc52ab1227fea8de52db80e48ad5235d8e6d1289fa93065e9c3061`.

Inspection included:

- Complete contract §§1–9 and committed evidence record, including intentional RED results and baseline Mypy limitations.
- Complete `types.rs:1–373`, refusing `lib.rs`, `records_host_contract.rs:1–904`, test support, representation tests, Rust syntax guard, Rust text example and both new JS consumers.
- Accepted design’s identity, authorization, historical qualification, custody, bounds, recovery, migration, supersession and IC-1/IC-2 requirements; resource consistency requirements, delivery plan and GDL-054–057.
- Library policy, BuildEntry, package architecture and architecture ownership notes; existing BindingResolver/ReplicaSync contracts; relevant Substrate W rules, Authz current-access/private-key rules, CRDT adapter, coexistence/seal provisions and legacy admission/store/signing/text code.
- Package manifests, architecture policy, selection and ARCH-002 fixtures.
- Complete new Gyld declaration/capture host and overlay tests. Read-only hash/comparison checks confirmed the supplemental design text matches the root design, its digest matches, the base ledger/declaration/checker/selector are unchanged, and the original architecture-test prefix is retained.

No builds, tests, network actions, writes or Git mutations were performed. Recorded GREEN timings and RED executions are the drafter’s committed evidence, audited against source rather than independently rerun.

## 1. Findings

### [P2-1] Unknown-outcome recovery requires an admission receipt for commits that contain no admission

**Location:** `glade/contracts/crdt-admission-core/src/types.rs:180–185,201–205,229–238,257–277`; contract §6, especially lines 242–257; recovery consumers at `records_host_contract.rs:312–358,647–780`.

**Violated invariant:** Every emitted atomic commit must have a recoverable outcome without inventing application admission or abandoning retained reservations. Accepted design §7 explicitly separates candidate/evidence retention commits from accepted application batches.

`CommitKind` declares `Candidate`, `SecurityEvidence` and `QualifiedFork` alongside `AcceptedBatch`. A candidate or security-evidence plan can legitimately contain retained `records` and no `StagedAcceptance`: for example, a signed replica successor awaiting its predecessor, or a bare signed rival with `evidence.admission == None`.

However, the only committed lookup result is:

```rust
LookupResult::Committed { receipt: Receipt, revision: u64 }
```

`Receipt` obligatorily includes an admission digest. Contract §6 requires that committed lookup to return the **exact stored receipt**. No separate batch-retention receipt, optional admission, committed result without an application receipt, or specified non-admission receipt semantics exist.

**Reproduction sequence:**

1. Emit a candidate-retention or security-evidence commit with no accepted admission.
2. Storage atomically retains it, but its acknowledgement is lost.
3. Receive `OutcomeUnknown`; retain the plan and reservation.
4. Resume exact lookup. Storage knows the batch committed, but there is no application admission receipt to return.

The typed recovery grammar now offers no faithful committed answer. Returning `Unknown` indefinitely preserves a stuck reservation and may pause all commits in that instance. Constructing an arbitrary `Receipt` requires inventing semantics for its admission field and “exact stored receipt,” crossing the custody/admission boundary this contract intends to preserve.

The RED tests do not expose this: every staged recovery fixture is an `AcceptedBatch` with one acceptance and an existing receipt. They contain no unknown-outcome journey for the other declared commit kinds.

**Impact:** The internal interface is incomplete for its declared persistence lifecycle. This is a P2 recovery/interface defect, not a claim of active corruption by the safe-refusing scaffold. It must be corrected before successful behavior depends on the interface.

**Required correction:** Define committed recovery at the atomic batch boundary for every commit kind. Bind revision, exact batch identity and achieved storage class, and distinguish retention-only outcomes from admitted-operation receipts. State how each kind installs retained state and releases reservations. Candidate/security retention must not require a synthetic admission or produce `AcceptedLocal`.

**Closure/regression tests:** Add compiling RED consumers for unknown → committed, known absent and still unknown across all four commit kinds. Include a candidate with no admission and a bare rival with no admission. Recovery must retain the correct candidate/security/fork state, count charges once, release only resolved reservations, preserve pending/common-prefix semantics, leave independent instances operational and emit no false local-admission receipt. Retain the existing original-receipt recovery assertions for accepted batches.

This is an architectural interface root cause with a bounded typed-contract remedy.

## 2. Invariant analysis

The other attacks did not establish blocking defects within this gate’s scope.

`Bytes` and `Operation` privately own their values and derive digests/canonical bytes. Descriptor and certificate snapshots remain mutable internal values, but the normative verifier obligations bind every parsed field to exact authenticated bytes. Public `Facts` and preloaded state are explicitly trusted inputs; the contract does not authorize peer ingress to create them.

Verification queries carry the operation, descriptor and evidence. Seal continuations retain verified facts and original validation mode. Foreign or changed callbacks must preserve valid continuations. Accepted staging retains enough information to reconstruct custody without reminting operation identity.

Policy changes before physical commit start invalidate queued local intent; changes after valid start preserve the historical admission and original receipt. That distinction is coherent under the explicitly trusted host ordering assumption. Actual clock, revocation and physical-start serialization remain adapter qualifications.

Historical qualification is independent of derived quarantine. Bare unauthorized rivals cannot convict; qualified forks retain both branches and dependent custody while reducing eligibility to the common prefix. Fresh epochs use new canonical origins. These rules avoid arrival-selected winners and circular fork qualification.

The released-text consumers obtain eligibility from kernel reads and contain no successful fallback. Eight exact rows are required; ABC uses isolated local receipts and opposite-direction operation offers. This is meaningful behavioral RED, while live automatic mesh synchronization remains unproven.

The package’s Pure classification and minimal dependencies are consistent with its boundary. ARCH-002 retains an untouched positive control and exact refusal checks across members. Disabled inline Rust branches are visited; macro-opacity limits are disclosed. Gyld adds allocation accounting over frozen sources without expanding its engine or marking requirements satisfied.

## 3. Risks and next action

Crypto, physical interruption/restart, custody and antirollback, automatic duplex transfer, current-serving enforcement, bounded production retention and mixed-version/seal activation remain mandatory later gates. Volatile replies, synthetic evidence and released merge controls establish none of those properties. The six recorded baseline Mypy errors remain limitations, not new admission defects.

The next action is one scoped remediation of P2-1: complete retention-only batch recovery semantics and add the corresponding RED consumers, then obtain the required review of the revised interface before implementing successful kernel behavior.