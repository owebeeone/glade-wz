# IC-2 Pure admission kernel — STATE-AXIS REVIEW

**Review object:** IC-2 Pure admission/reconciliation transition and complete immutable batch encoder, `glade/contracts/crdt-admission-core`, at Glade `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14`; controlling implementation DRAFT and evidence at workspace root `332e7d95f5887300bf762ac76737e4cc38bcd151`.

**Baseline:**

| Repository | Reviewed HEAD |
|---|---|
| Workspace root | `332e7d95f5887300bf762ac76737e4cc38bcd151` |
| Glade | `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were read directly and compared with committed bytes using read-only inspection and `git show`. All five HEADs matched at the beginning and end.

**Date:** 2026-10-04  
**Axis:** Closed recovery grammar, lifecycle finality, bounded state, custody, projection and completeness. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — five P2 findings block. All five roots are nonarchitectural: existing accepted types can express the required corrections. I pre-commit to GO on a revision that resolves P2-1 through P2-5 as specified, with closure evidence and unchanged contract boundaries.

---

## 0. Evidence base

Read the generated State prompt completely; its SHA256 matched `7e83038e6ce4c4471532594640f2ebca929ab647407a692898bc1900ec40b696`.

Inspected root/member instructions, review-loop skill/template and ReviewCycle; controlling implementation DRAFT/evidence; admission design/contract requirements concerning qualification, bounded retention and read cuts; storage-attempt design/contract lifecycle, recovery, integrity-stop and supersession clauses; plan, resource profiles, library policy, package architecture and build entry.

Read all eight core source files, all retained behavioral consumers and assembly support, new edge/encoder tests, shared storage API and memory provider, actual text trace, released Taut consumer/source guard and canonical corpus. Read-only byte comparison confirmed the original 43 obligations, assembly support, shared API/provider, architecture policy and test selector remain unchanged from the nominated baseline.

Audited recorded RED/GREEN logs, final core/API results, ten actual-Taut rows, canonical-corpus results, and rejecting origin/encoder mutants with restored controls. Recorded final results include 86 core and 34 API tests. These are audited execution evidence; this reviewer ran no builds or tests. Recorded timing is cached/concurrent evidence, not an independently measured cold-build guarantee.

Inspected unchanged library roles, dependency edges and Gyld allocations directly. No architecture-check result was treated as independent approval of those boundaries. No files, repository state, network or live services were modified.

## 1. Findings

### [P2-1] Delayed Started callback downgrades a terminal attempt

**Location:** [callbacks.rs:86](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/callbacks.rs:86), especially lines 125–129; [lifecycle.rs:151](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/lifecycle.rs:151).

Terminal comparison protects only terminal replies. A fully matched live `Started` reply can overwrite an already terminal phase.

**Sequence:** Prepare and Begin through the actual memory port with publication held; save its Started reply without delivering it. Publish the attempt. Resume recovery and deliver an Inspect reply containing Committed. This installs the terminal result once while the original Begin remains outstanding. Deliver the saved Begin/Started reply. Its request and cut authenticate, but the handler changes `AttemptPhase::Terminal` back to Started and reports OutcomeUnknown.

The same downgrade is possible after a terminal fence result. It violates immutable terminal-state grammar. The retained reconstruction consumer rejects the phase/result disagreement. Subsequent ResumeRecovery issues further inspections; duplicate terminal replies do not repair the phase, so invocation history can be consumed repeatedly.

**Correction:** Preserve terminal phase for every late nonterminal reply, while retaining authenticated contradictory-terminal detection and once-only installation. Prevent completed attempts from returning to recovery issuance.

**Closure:** Actual-port tests must deliver terminal Inspect before delayed Started, for committed and fenced outcomes across all four kinds. Assert coherent phase/result, successful validated restoration, unchanged receipt/charges, and no renewed lookup.

**Root classification:** Nonarchitectural; a missing transition guard in the existing lifecycle grammar.

### [P2-2] In-flight verification can create new work after integrity stop

**Location:** [admission.rs:207](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:207) and [staging.rs:72](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:72). The stop check exists only on initial offer at admission lines 66–69.

**Sequence:** Submit local A and B from distinct origins before resolving either verification. Complete A through Committed. Deliver an authenticated contrary terminal assertion for A’s issued invocation; this sets `storage_integrity_failed`. Now deliver B’s previously issued Verified reply and then its Sealed reply. Neither continuation checks the stop. They emit a new Seal and Prepare; the actual port can commit B.

Storage-attempt requirements make the affected instance unavailable for new work after an integrity contradiction. A pre-existing verification is not authorization to create a new storage attempt after that stop. Independent Y should remain available.

**Correction:** Check the stop at continuation boundaries that create new admission/storage work. Preserve already issued uncertain work, original custody, terminal settlement and conflict evidence; do not discard genuine late commits.

**Closure:** Park both verification and sealing continuations before contradiction, then deliver them afterward. Assert no new Prepare/Begin/admission, retained original evidence/reservations, and Y progress.

**Root classification:** Nonarchitectural; incomplete enforcement of an existing stop flag.

### [P2-3] Derived missing slots exceed the configured pending-item limit

**Location:** [projection.rs:106](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/projection.rs:106), [admission.rs:303](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:303), and staging lines 103–110.

Capacity checks count verification queries and candidate records. Projection separately unions missing dependency slots without checking `pending_items`.

**Sequence:** Use a valid descriptor with `pending_items=1`, at least two permitted refs and sufficient byte budgets. Offer a valid historical sequence-zero candidate with sorted references to missing B:0 and C:0. Verification emits both missing slots. Candidate staging passes because there are zero retained candidates. After its atomic commit, refresh stores two pending slots.

This exceeds the declared count bound despite successful acceptance. Existing frontier slots and additional candidates can enlarge the discrepancy. The contract requires finite candidate/pending counts and refusal before acceptance, rather than clipping retained work afterward.

**Correction:** Reserve/check the deduplicated missing-slot union before staging/publication and before emitting bounded recovery requests. Capacity refusal must record sticky incompleteness without evicting custody or inventing charges.

**Closure:** Test one versus two missing slots at the boundary, plus combinations with existing frontier slots and multiple candidates. Assert retained state and emitted work respect the cap.

**Root classification:** Nonarchitectural; an omitted bound on an existing derived collection.

### [P2-4] Frontier observations retain arbitrarily large origin names

**Location:** [lib.rs:69](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/lib.rs:69), particularly lines 85–95 and 118–124.

Frontier validation checks head count and sequence range, but neither origin length nor aggregate retained bytes.

**Sequence:** With the fixture’s `max_name_bytes=64` and `pending_bytes=8192`, observe one unknown head whose origin is a million-byte string and whose sequence is zero. Count and sequence checks pass. The kernel retains that string in a pending Slot and copies it into RequestMissing. Later transitions clone the enlarged State. No used-byte accounting changes.

A peer inventory hint therefore admits arbitrary retained memory despite fixed configured limits. This is distinct from P2-3: even one slot exceeds the intended byte boundary.

**Correction:** Validate names and the aggregate retained pending footprint before storing or emitting frontier work. Oversized observations must fail boundedly and preserve honest incompleteness; they must not evict acknowledged data or fabricate quota usage.

**Closure:** Test empty, limit-sized, oversized and very large names, plus aggregate byte-boundary cases involving previously retained slots. Assert bounded state/effects and unchanged custody/accounting.

**Root classification:** Nonarchitectural; missing validation for variable-sized input under existing limits.

### [P2-5] Effect-counter exhaustion forgets recovery refusal

**Location:** [admission.rs:150](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:150), [staging.rs:123](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:123), and [projection.rs:153](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/projection.rs:153).

These allocation-failure branches report Capacity without setting `recovery_incomplete`.

**Sequence:** Start from a valid known-empty instance with `next_effect_id=u64::MAX`. Offer a structurally valid historical candidate. Allocation fails before retaining a query or candidate. A following Read reports `complete_local=true`: no unresolved collection or sticky flag records the observed refusal.

A second path starts at `u64::MAX-1`: verification consumes the final allocatable identifier; delivering valid historical facts removes the query, then plan allocation fails. The read again becomes falsely complete.

The retained contract expressly requires false completeness after bounded recovery refusal. Counter exhaustion is finite capacity exhaustion, not proof that the observed history was classified.

**Correction:** Apply existing sticky recovery-loss bookkeeping consistently to historical allocation failures. Preserve custody and actual charges; local intent refusal alone need not taint known history.

**Closure:** Test both exhaustion points, persistence through ResumeRecovery/restoration, unchanged prior custody, and a local-refusal control.

**Root classification:** Nonarchitectural; omitted failure bookkeeping using the existing sticky flag.

## 2. Invariant analysis

Several adversarial attacks did not expose additional defects:

- Full issued-request matching precedes ordinary callback consumption. Changed or unissued requests cannot independently trigger terminal contradiction handling.
- Pending and Refused remain uncertain; they do not become terminal NonCommit.
- Normal terminal installation preserves exact storage binding, receipts, revision and charges once.
- Qualification remains distinct from projection quarantine. Both fork orders, dependent custody, common prefix and fresh certified-origin continuation retain their original consumer path.
- Immutable encoding uses explicit framing and semantic fields rather than Debug output. Field mutation tests and the rejecting omission mutant support coverage.
- Actual-port current prestart observation and retained Started cuts preserve the distinction between current local authorization and historical settlement.

Unknown frontier digests conservatively set permanent incompleteness because the accepted Slot grammar cannot retain their expected hashes. This avoids falsely complete reads. It is an acknowledged progress limitation, not proof of completed live reconciliation.

These conclusions do not override the five reachable failures above.

## 3. Risks and next action

The next action is one scoped remediation with failing regressions for P2-1 through P2-5, followed by the affected package/consumer checks and adopting architecture gate, then closure review at a newly pinned tuple.

No architectural root is identified here. Historical review stops and caps remain intact. This report qualifies neither physical persistence/antirollback nor production crypto, clocks, automatic duplex, activation or user-facing Surface. Those deferrals do not excuse the Pure state failures reported here.