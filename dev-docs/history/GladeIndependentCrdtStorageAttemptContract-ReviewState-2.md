# Redesigned CRDT storage-attempt typed contract — STATE-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptContract.md` and the internal typed-contract/allocation/compiling RED checkpoint at root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`. DRAFT remediation 1, dated 2026-10-04.

**Baseline:** Workspace root `e5b7f3e14c2da8f9e0c3f64f17bf3161e6c7a539`; Glade `346d963f09089a0636a01fac8a257f067908147d`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Direct source reads were checked against exact-pin `git show` contents.

**Date:** 2026-10-04

**Axis:** Durable-state semantics, restoration grammar, finality, ownership, bounded capacity and executable consumer obligations. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block; zero P0, P1 or P3 findings. I pre-commit to GO on a revision resolving P2-1–P2-3 as specified, provided the correction introduces no additional blocking defect.

---

## 0. Evidence base

All five HEADs matched the prescribed tuple at start and end. No writes, builds, tests, network actions or Git mutations occurred. No current-round peer or closure report was accessed.

Inspection covered:

- Complete canonical State prompt, review-loop skill/template, workspace/member instructions and local process authority.
- StorageAttemptContract §§1–9, original and remediation evidence, merged remediation plan, prior-round Code/State reports, complete accepted lifecycle design and completed design reviews.
- Relevant accepted admission semantics, original contract/evidence/remediation evidence, Safety-2, root classification, escalation, full ReviewCycle history, admission plan/resource requirements, library/package policy, BuildEntry and GDL-054–058.
- Complete API declarations; development provider `tests/support/mod.rs:1–787`; twenty public journeys, six-mutant probe and source guards.
- Core types/refusing kernel; shared assembly/support; six fixture closures; all twenty-one Records, six recovery and fifteen STA consumers; actual text trace and released-Taut consumer.
- Manifests, README, architecture policy, selectors, all-member ARCH002 fixture and synthetic-member tooling regression.
- SnapshotStore, RecordsFile CAS/replacement/bridge/decoding, sysdir ownership and legacy Store/seal boundaries.
- Frozen Gyld declarations/ledgers, capture implementation and provenance/ownership-negative consumers.

Read-only byte comparisons found no mismatches among seven controlling root files, eighteen API/core files and six Gyld files. Both frozen Gyld source texts equal their source-qualified canonical revisions and hashes. Actual inventory is twelve libraries and exactly matches policy. Original Records/recovery assertion counts remain 52/55; STA grows from 40 to 46. The refusing kernel retains its recorded digest.

Recorded GREEN/RED results and timings were inspected, not rerun. The evidence reports twenty public journeys, six assembly closures, forty-two compiling domain failures and ten released-text failures.

## 1. Findings

### [P2-1] Historical recovery still demands authority from an unissued lookup

**Location:** Core `tests/records_host_contract.rs:972–1015`; `tests/support/mod.rs:436–471,752–790`; Contract §§3/5/8.

**Invariant:** A lookup reply can install custody only through its complete retained issued request. The secondary lookup index cannot create callback authority.

**Sequence:** The delayed-prior-cut consumer moves plan 7 into `unknown_commits`, calls `register_attempt`, then inserts Inspect invocation 40 directly into `lookups`. `register_attempt` retains Prepare 7 and Begin 8, not Inspect 40. The consumer nevertheless injects the lookup’s Committed reply and requires ExactRetry.

A conforming kernel must reject invocation 40 because `storage_invocations` lacks its issued request. Satisfying this mandatory assertion would require weakening the new authentication rule. The unchanged refusing scaffold masks that independent fixture failure.

**Impact:** One original historical-authorization/recovery obligation remains unsatisfiable under the corrected continuation contract. The claim that every manually issued lookup uses `register_lookup` is false.

**Correction:** Register this lookup through the complete helper before advancing the counter, preserving its existing historical receipt assertion.

**Closure test:** Require this exact prior-cut recovery to install the original receipt from issued Inspect 40, and require the otherwise identical unissued reply to preserve state and report CallbackMismatch. Keep the consumer compiling RED until kernel implementation is authorized.

**Architectural-root classification:** **Non-architectural**; residual instance of prior Code P2-3, not a new root. Existing representation/helper supports the correction.

### [P2-2] Restoration accepts duplicate attempt identities and overlapping unresolved owners

**Location:** API provider `tests/support/mod.rs:36–129,372–447`; Contract §§3/6; accepted lifecycle design §4.

**Invariant:** An attempt identity permanently denotes one immutable binding, and an instance has at most one unresolved preparation/attempt capable of mutation. Validated restoration must preserve this closed grammar.

**Source-traced reproduction:** Recover a Reserved Candidate for X, plan 1, attempt number 1, `next_attempt=2`. Append another Reserved entry with plan number 2 but the same complete AttemptId; set invocation count/high-water to 2. Both bindings fit default limits.

Restore accepts: it checks unique PlanKeys, but neither unique AttemptIds nor per-instance unresolved cardinality. Open generation 2, hold publication, and Begin the two returned bindings using fresh invocations 3 and 4. Each independently matches its stored record and expected revision 0; both become Started.

This is an internally inconsistent recovery image admitted by the validator, not a physical rollback claim.

**Impact:** The trusted development producer can reopen two mutators for X. Their shared AttemptId also collides in the kernel’s terminal-evidence namespace, defeating immutable attempt ownership.

**Correction:** Validate complete ledger identity uniqueness and per-instance unresolved exclusivity, including queued preparations, before constructing a usable host. Reject inconsistent images unchanged.

**Closure test:** Add restored duplicate-ID, distinct-ID overlapping-active, and queued-plus-active negatives. Preserve a positive image containing multiple historical terminals and one current unresolved attempt, with independent Y progress.

**Architectural-root classification:** **Non-architectural**; missing validation in the development provider. Recovery already carries the identities/phases needed to enforce the accepted invariant.

### [P2-3] Revision mismatch creates an authentic terminal that cannot be restored

**Location:** API provider `tests/support/mod.rs:287–365,416–438,86–89`; Contract §§4/6.

**Invariant:** NonCommit changes no application revision, and authentic exported lifecycle states must remain legal restoration inputs.

**Source-traced reproduction:** On an empty host, prepare a fresh Candidate binding with `expected_revision=1`. Binding validation accepts it; preparation installs Reserved and initializes X’s actual revision to 0. Begin detects the mismatch and installs terminal NonCommit with revision 1, copied from the expectation. Recovery exports X revision 0 and that terminal revision 1. `MemoryHost::restore` rejects the authentic image because terminal revision exceeds current revision.

No corruption, identity replay or physical failure is needed. Explicit Fence has the same expectation-copying behavior.

**Impact:** Ordinary revision refusal generates an unrecoverable model state and a terminal claiming an application revision the store never held. This undermines restoration/failure conformance.

**Correction:** Reject incompatible expected revisions before exposing Prepared, or otherwise implement a coherent mismatch-finalization rule that preserves the actual application revision and agrees with terminal validation. Do not repair this by weakening restoration to accept invented revisions.

**Closure test:** Exercise fresh bindings with ahead/stale expectations across all four kinds. Assert unchanged actual custody/revision and authentic recovery/restoration parity after refusal or terminal fencing; retain valid-current-revision publication controls.

**Architectural-root classification:** **Non-architectural**; producer revision validation/finalization defect. No new boundary information is required.

## 2. Invariant analysis

The original absence-before-publication attack fails against the normative lifecycle and covered model journeys: temporary absence, lost preparation acknowledgement and Started work remain Pending. Publication/fencing repeat one retained terminal result; late workers cannot rebase past NonCommit.

Sparse invocation restoration now preserves count separately from high-water. The matching core policy observation is independently injected, and mandatory text replicas retain their sessions. These corrections address the earlier counterexamples at the inspected contract/model tier; originating closure remains separate.

Bindings retain exact bytes, charges, original receipts and storage class. Issued/consumed request history distinguishes duplicates from authenticated terminal contradictions. Sticky recovery incompleteness has no clearing event.

Contract/Pure roles and dependency direction remain meaningful and narrow. Selectors cover both affected packages; framework refusal covers every classified library. Gyld preserves the frozen 31/124 semantic allocation and adds explicit STA ownership without satisfaction claims or engine expansion.

The initial TDD deviation remains disclosed. Compiler failures, compiling domain RED and successful fixture conformance are distinguished. No successful admission or physical qualification is claimed.

## 3. Risks and next action

Physical barriers, submitted I/O after owner termination, antirollback, process reopen, quota reserves, genuine policy/clock provenance and live duplex remain later qualifications. Existing locks/fsync do not supply composite attempt finality. Gyld’s recorded timing margin remains narrow.

One scoped correction must resolve these findings and undergo required verification/review. The kernel must remain refusing. No new architectural root is classified here; the previously recorded architectural root and failed IC-1 history remain intact.