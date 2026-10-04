# Independent CRDT storage-attempt lifecycle — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptDesign.md`, DRAFT dated 2026-10-04, at workspace root `d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`. Owner-authorized lifecycle redesign; no typed-contract, successful implementation, physical qualification or activation acceptance.

**Baseline:**

- Workspace root: `d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`
- Glade: `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`
- Glial: `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`
- Glade-discover: `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`
- External Gyld: `64666e8b1caadde8922b9d42163afbab90655c65`

Committed sources were inspected with exact-pin `git show`; the scoped root diff was `1fd0fb025606ee61edb33c1e385f558280aa6a62..d19235fb14af7f20dbdbf0b32c0c69fdba33c2c6`.

**Date:** 2026-10-04

**Axis:** Internal coherence and agreement with the controlling semantic, architectural, storage and process graph. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This verdict accepts the lifecycle design only.

---

## 0. Evidence base

All five repository HEADs matched the prescribed tuple at the beginning and end of inspection.

Read the complete generated Consistency prompt and complete lifecycle design, lines 1–411. Inspected:

- `AGENTS.md`, `AGENTS_GWZ.md`, review-loop `SKILL.md` and its canonical prompt template; `GladeRaftQualificationPlan.md` §6.
- Accepted `GladeIndependentCrdtAdmissionDesign.md`, particularly §§2–8 and the contract/component/live qualification gates; resource consistency requirements and admission delivery plan.
- IC-1 internal contract §§1–9, original evidence and remediation1 evidence; completed Safety-2 report, architectural root-classification addendum, escalation and full review-cycle ledger.
- `DecisionLog.md` GDL-054–058; BuildEntry, library-boundary policy and package architecture.
- Pinned Glade `contracts/persistence-api/src/lib.rs:26–61`; `node/src/records_file.rs:92–274`, including CAS, replace/barrier ordering, synchronous future bridge and unknown-field loss; `sysdir.rs:109–160` ownership/lock lifecycle; relevant Store open/append and journal-write paths.
- Legacy Store seal contract, including whole-root scope, stable-lock requirements, old-binary limitations and production transition blockers.
- Pinned admission `types.rs:205–397`, refusing `lib.rs`, relevant recovery-consumer journeys and Glial’s exact ten-row released-Taut consumer.
- Frozen external Gyld supplemental declaration and source ledger. Read-only Python comparison confirmed the frozen semantic text equals the canonical committed design, its SHA-256 agrees, and the inherited source-ledger digest agrees.

The scoped diff contains four documentation files: the lifecycle draft, decision entry, plan status and review ledger. No implementation/type/dependency change is part of this object.

No writes, builds, tests, network actions, live operations or Git mutations were performed. Prior execution counts are committed historical evidence, not executions of this design. No current peer report was read.

## 2. Invariant analysis

**The original negative-finality attack fails against the new text.** At lines 96–116 and 168–202, a correctly observed missing outcome, lost reply, timeout or cancelled waiter remains Pending. The host must account for queued preparation and owned mutation. Consequently, lookup L cannot release P before a late P commit. A later committed observation recovers P; a terminal-negative answer requires a retained fence/tombstone that prevents publication afterward. The same requirement covers direct negative replies and every retention kind.

**Attempt ownership and callback identity are separated coherently.** Lines 54–81 distinguish immutable plan binding, retained storage attempt, independently allocated invocation and execution generation. Lost prepare acknowledgement recovers by original plan key without allocating another attempt. Changed binding refuses. Invocation retirement does not retire owned storage work; restored high-water or independently proven namespace retirement excludes old callbacks. Lines 120–124 prohibit overlapping unresolved preparation as well as overlapping registered attempts for one instance, while retaining instance independence.

**The terminal grammar does not promise impossible progress.** Reserved and Started have explicit successors; Committed and NonCommit have none. Exactly one eventual outcome is conditional on completion or proven noncommit, with indefinite Pending allowed for unavailable/damaged providers. NonCommit includes future publication exclusion, rather than present absence. Commit/fence races share the publication predicate, and old workers may not rebase around a failed predicate or tombstone.

**Publication and recovery maintain one custody boundary.** Lines 126–164 require retained preparation before application mutation, a serialized physical-start cut, and one atomic installation of custody, evidence, charges, original receipts and terminal outcome. Internal metadata generations are explicitly distinct from application revision. Preparing, starting and negative fencing cannot falsely advance application custody. Crash before a completed registration remains uncertain; restored Reserved/Started entries require lifecycle resolution. Crash after publication recovers the original coupled result.

**Policy ordering preserves accepted historical semantics.** A stale pre-start plan cannot begin without the current check and qualified negative resolution. A valid Started cut survives subsequent revocation and reply loss without retrospective reauthorization. Historical retention does not acquire today’s local-admission check. These rules agree with the accepted semantic design’s historical-validity model and IC-1’s start-ordering obligation.

**Reordered terminal observations do not create arrival-order authority.** Lines 204–211 require once-only installation, duplicate invocation refusal and retained terminal identity sufficient to distinguish harmless duplication from contradictory terminal assertions. A contrary result stops new instance work and preserves evidence instead of selecting whichever callback arrived first.

**All four commit kinds remain representable.** Lines 275–290 retain complete AcceptedBatch admissions, provisional Candidate custody, nonconvicting SecurityEvidence and independently qualifying QualifiedFork pairs. Candidate/security results have empty admission receipts; fork recovery uses genuine historical admissions. No synthetic operation or placeholder receipt is required. Pending retains actual reservations; NonCommit changes no application custody or charges; committed installation charges once.

**The reuse recommendation matches the inspected primitives.** SnapshotStore expressly lacks request deduplication and exposes no attempt/start/fence operation. RecordsFile performs locked revision CAS and whole-file replacement, while its bridge completes in the first poll and its node decoding can drop unknown metadata. The draft correctly rejects using either unchanged as the complete lifecycle. It proposes a separate versioned envelope and qualified host rather than attributing composite finality to existing fsync calls or locks.

**Ownership and restoration do not rely on phantom fences.** Lines 225–254 require complete worker ownership, old-execution exclusion, interrupted-publication settlement and trusted incarnation/high-water restoration. They distinguish OS-lock release from completion of submitted I/O. Backup rollback and concurrent restored copies are not silently covered by LocalProcessRestart; missing ownership/issuance floors refuse writable reopen. Physical enforcement remains unqualified.

**Finite capacity and completeness remain conservative.** Critical preparation reserves include terminal/recovery footprint. Full or failed physical barriers cannot produce a volatile NonCommit. Terminal records and issuance history are retained without GC, and exhaustion stops new attempts. The proposed sticky recovery-incomplete flag records an otherwise unretainable observation without false charges or sentinel slots. It cannot substitute for attempt reservations or clear merely because capacity returns; a clearing action requires its own reviewed reconstruction witness.

**Supersession and evidence scope are bounded.** Lines 384–402 authorize a later typed tranche to identify exact replacements within unaccepted IC-1 lifecycle grammar and linked state/event/completeness descriptions. They do not presently amend the accepted CRDT design, frozen Gyld allocation, SnapshotStore or legacy seal contract. STA-001–011 provide stable future RED obligations and separate physical fault journeys, retaining the original 27 behavioral and ten released-Taut obligations. The stopped IC-1 object’s three architectural roots and remediation history remain intact.

**Architectural root classification:** No new architectural root cause was established on this redesign object. The three previously recorded IC-1 roots remain historical facts; this GO neither resets their accounting nor accepts the failed contract.

## 3. Risks and next action

The proposed CAS host still needs a reviewed typed boundary and executable conformance consumers. Physical fencing, process termination/reopen, trusted floors, quota/preallocation behavior and composite barrier durability remain unproven. The recovery-incomplete clearing witness is also a subsequent contract obligation.

The next action is the gated typed-contract/allocation tranche: identify exact superseded clauses, add compiling lifecycle RED consumers, preserve the original behavioral/text obligations, and obtain the required independent review before successful kernel or host implementation.