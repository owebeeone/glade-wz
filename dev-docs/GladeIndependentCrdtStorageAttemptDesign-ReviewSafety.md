# Independent CRDT storage-attempt lifecycle — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptDesign.md`, DRAFT dated 2026-10-04, at root `d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`; owner-authorized lifecycle redesign after the stopped IC-1 object.

**Baseline:**

- Workspace root: `d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`
- Glade: `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`
- Glial: `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`
- Glade-discover: `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`
- External Gyld: `64666e8b1caadde8922b9d42163afbab90655c65`

Sources were read through exact-pin `git show` and direct reads checked against pinned contents.

**Date:** 2026-10-04

**Axis:** What the text permits to go wrong under degraded operation, irreversible transitions, storage races and recovery. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0–P3 findings. This verdict accepts the lifecycle design only; it does not accept revised types, successful implementation, physical storage qualification or activation.

---

## 0. Evidence base

All five HEADs matched the supplied tuple at the start and end of inspection.

Read the complete canonical Safety prompt and complete DRAFT, lines 1–411. The DRAFT’s SHA-256 is `239b386066ed4ee09ade7f35b9f0867be7673472418c33eab799a2267cf6b3db`.

Inspection included:

- `AGENTS_GWZ.md`, supplied workspace instructions, review-loop `SKILL.md`, its complete canonical prompt template, and `GladeRaftQualificationPlan.md` §6.
- Accepted `GladeIndependentCrdtAdmissionDesign.md`, particularly identity, historical authorization, fork qualification, receipts, bounds, storage, migration and IC-1–4 gates.
- Complete admission contract, original evidence, remediation1 evidence, escalation, prior Safety-2 report, root-classification addendum and full admission ReviewCycle history.
- Admission delivery plan, resource consistency requirements, BuildEntry, library policy, package architecture, DecisionLog GDL-054–058 and legacy Store seal contract.
- Pinned `persistence-api/src/lib.rs`; RecordsFile’s load/CAS/replace, lock, SnapshotStore bridge and snapshot decoding; `sysdir.rs` ownership and platform file-identity code; Store open/replay, append, rewrite and torn-tail handling.
- Frozen external Gyld supplemental declaration and ledger metadata; existing refusing kernel, Rust behavioral-test inventory and exact ten-row released-Taut consumer requirements.

The scoped root diff contains only the new design and changes to DecisionLog, delivery plan and ReviewCycle. Sixteen directly read controlling root files were compared with their exact-pin contents; no mismatches were found.

No files were written. No builds, tests, network actions, live actions or Git mutations were performed. No current peer report was read. Historical executed evidence was inspected, not independently rerun.

## 2. Invariant analysis

**Temporary absence and cancelled waiters.** I retraced the original failure: P starts, its waiter loses the answer, lookup sees no outcome, and P subsequently publishes. Lines 96–116 and 168–202 prohibit releasing P at that observation. Missing entries, direct errors, cancellation and incomplete fences remain Pending. A later commit recovers the original batch; terminal NonCommit requires exclusion of every later publication. This attack fails against the proposed semantics.

**Preparation and lost acknowledgements.** Lines 70–75 require durable plan-to-attempt deduplication and recovery by plan key when the allocated attempt number is unknown. Lines 120–133 prohibit overlapping unresolved preparations and require durable registration before application mutation. Lines 192–196 also fence late registration. Losing a prepare reply therefore cannot authorize another attempt or let a queued original prepare escape a negative result.

**Commit/fence races and old I/O.** The atomic predicate binds active status, attempt, execution generation and expected application revision. Commit and terminal fence share that predicate; neither losing worker nor restart recovery may rebase around the tombstone. Lines 185–190 explicitly distinguish owner-lock release from completion of submitted I/O. Lines 232–239 require qualified drain or fencing and settlement of interrupted replacement. Obtaining an OS lock alone cannot manufacture finality.

**Crash boundaries and coupled publication.** The permitted sequence is retained Reserved, protected Started, then one publication of complete custody, charges, receipts and Committed. Separate application and metadata revisions prevent preparation or fencing from advancing an admission head. Lost transition replies leave uncertainty; restored Reserved/Started entries remain Pending until resolved. The text prohibits a visible accepted tail followed by a best-effort outcome or outbox write. No crash interleaving examined permits partial custody to be reported as terminal success.

**Authorization ordering.** Queued work remains Reserved and must satisfy the current start precondition. Policy/time observation and the protected start boundary are serialized. Once a valid start cut exists, later revocation cannot retroactively erase the committed historical admission. Historical retention does not acquire a new local-admission check. These rules preserve the accepted distinction between current rights and historical qualification.

**Identity, duplicate replies and contradictory results.** Plan, attempt, invocation and execution identities have separate lifetimes. Retired callbacks cannot consume newer invocations or release reservations. Issuance/high-water exhaustion refuses rather than wraps. Direct and lookup installation share once-only accounting and original receipts. Lines 204–211 additionally require detection of contradictory terminal assertions after reservation release, followed by evidence preservation and an instance stop. Arrival order cannot choose a terminal truth.

**All four batch kinds.** Lines 275–290 require the same lifecycle and whole-batch recovery for AcceptedBatch, Candidate, SecurityEvidence and QualifiedFork. Candidate/security retention has empty admission receipts and cannot advance an accepted head. QualifiedFork retains independently qualified rivals and genuine historical admissions without inventing application success. Pending and NonCommit cannot erase earlier custody or charges.

**Capacity and completeness.** Critical metadata reserves include terminal outcomes, tombstones, identity floors, ownership fences and recovery. Failure of a physical barrier yields Pending/Unavailable with admission stopped. Retained outcomes and issuance history are not evicted; exhausting finite attempt capacity stops new attempts, including repeated negative attempts. This sacrifices progress explicitly instead of weakening finality. The proposed sticky incompleteness flag cannot replace actual reservations or clear merely because capacity returns.

**Reuse and trust boundary.** The existing SnapshotStore contract expressly lacks request deduplication. RecordsFile performs locked replacement but its legacy snapshot decoder can discard unknown metadata. The design correctly rejects both unchanged implementations as complete lifecycle hosts. It proposes qualification of reused primitives in a separate versioned root, rather than attributing composite finality to existing fsync or lock tests. Trusted terminal results must come from the actual host/provider assembly, not peer DTOs or caller booleans.

**Architectural-root classification:** No new architectural root cause was established on this redesign object. The stopped IC-1 object’s three architectural roots and remediation history remain recorded. This verdict does not retrospectively accept that object or close its defect through conceptual review.

## 3. Risks and next action

The proposed lifecycle is satisfiable, but its physical realization remains unproven. Real qualification must cover submitted I/O after owner termination, stable root/lock identity, interrupted replacement, quota/preallocation failure, retained issuance floors, mixed-version exclusion and exact promised restart durability. Deterministic simulation cannot supply that evidence.

The overflow flag’s authoritative reconstruction/clearing operation remains deliberately unavailable until its contract is reviewed. Finite retained metadata can permanently stop new attempts without a separately reviewed retirement or collection mechanism; the design states this limitation honestly.

The next action is the gated typed-contract and compiling lifecycle RED tranche: specify the actual producer/consumer assembly and exact superseded clauses, retain all 27 Rust and ten released-Taut obligations, and exercise STA-001–011 before successful implementation. Physical host, aggregate live-feature, Surface and activation gates remain separate.