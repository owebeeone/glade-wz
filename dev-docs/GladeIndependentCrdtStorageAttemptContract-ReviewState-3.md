# Redesigned CRDT storage-attempt typed contract — STATE-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtStorageAttemptContract.md`, DRAFT remediation 2, and its internal typed-contract/compiling RED checkpoint at root `fb6d69ff5153865266782f0b5c11d7a00086552c`.

**Baseline:** Workspace root `fb6d69ff5153865266782f0b5c11d7a00086552c`; Glade `c6c4239beecb129aa0585fe74006cc287dff3b87`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `95a426595bba8e248a5f484272e483a070c73918`. Sources were inspected directly and through exact-pin Git reads and scoped diffs.

**Date:** 2026-10-04

**Axis:** Durable-state semantics, restoration grammar, finality, ownership, bounded capacity and executable consumer obligations. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one new P2 finding blocks; zero P0, P1 or P3 findings. All three originating State-2 findings close at this contract/RED tier. I pre-commit to GO on a revision resolving this report’s P2-1 as specified, provided no additional blocking defect is introduced and the existing review cap is respected.

---

## 0. Evidence base

All five HEADs matched the prescribed tuple at start and end. No writes, builds, tests, network actions or Git mutations occurred. No current peer report was accessed.

Read completely: canonical State-3 prompt, RemPlan-2, Remediation2-Evidence and corrected Contract. Retraced the complete originating State-2 report and inspected the complete corrected Glade range:

- API development provider `tests/support/mod.rs:1–867`.
- All three added public regressions, `tests/public_contract.rs:599–854`.
- Corrected historical consumer and new issued/unissued pair, core `tests/records_host_contract.rs:972–1058`.
- Added fixture source check, `tests/fixture_composition.rs:256–269`.

Retained review context includes the accepted lifecycle/admission designs, process authority and review history, library/package policies, complete API/core declarations and consumers, physical reuse boundaries, framework refusal fixtures, released text obligations and frozen Gyld provenance/allocation. The unchanged relevant sources were checked against the remediation input rather than treated as newly qualified.

The Glade diff contains exactly the four declared development provider/test files. Read-only byte comparisons confirm unchanged API and refusing kernel, matching the recorded SHA256 digests. Four other API files, ten other core files and six script files are unchanged. No package, dependency, role, selector, architecture-tooling, assembly seam or application mutation boundary changed. Root corrections strengthen existing validation obligations and preserve review history. These facts justify originating context retention under review-loop step 5; I found no contrary architectural/interface evidence.

Recorded verification was audited, not rerun: 29 API and 12 core fixture/representation/source checks GREEN; 43 compiling domain consumers and ten actual-Taut rows deliberately RED; compilation, lint, source and positive architecture checks GREEN. Unchanged framework negatives and Gyld checks rely on explicitly retained prior evidence. Compiler errors are distinguished from settled behavioral RED.

Originating closure table:

| State-2 finding | Result | Independent closure evidence |
| --- | --- | --- |
| P2-1: unissued historical Inspect40 | **CLOSED at contract/RED tier** | `register_lookup` now records the complete issued request before advancing the floor. Both indexes and next41 are asserted. The original historical ExactRetry remains, with an identical unissued reply requiring unchanged state and CallbackMismatch. Kernel behavior remains RED. |
| P2-2: duplicate identity/overlapping unresolved owners | **CLOSED** | Restore now checks complete AttemptId uniqueness and one unresolved owner across queued/Reserved/Started work. Eight negative images cover the requested cases and additional revision/capacity edges. The positive control retains historical terminals, current X and independent Y progress. |
| P2-3: revision refusal produces unrestorable terminal | **CLOSED** | New-plan revision validation precedes queued/Reserved exposure, after retained-plan deduplication and unresolved ownership checks. The 32 combinations cover all four kinds, stale/ahead expectations and Begin/Fence follow-ups; authentic recovery restores, and valid revision2 publication remains covered. |

## 1. Findings

### [P2-1] Restoration accepts two distinct commits at one application revision

**Location:** Glade API provider `contracts/crdt-storage-attempt-api/tests/support/mod.rs:88–102`; live publication predicate and increment at `529–562`; Contract §§4–6, especially `195–200` and `238–254`.

**Violated invariant:** Each distinct successful publication for an instance predicates on its current application revision and advances it by one. Two distinct attempts therefore cannot both commit at the same resulting application revision. Validated restoration must reject internally contradictory retained terminal history.

**Source-traced reproduction:**

1. Commit Candidate X/plan1 at revision1.
2. Prepare X/plan2 with expected revision1 and commit it at revision2.
3. Export the authentic recovery image. It contains distinct complete AttemptIds and PlanKeys, two terminal records, current X revision2, sufficient invocation consumption and valid bounds.
4. Change only plan2’s retained binding expectation from1 to0 and its terminal Committed revision from2 to1. Keep its distinct batch, identity and other payload fields.
5. Restore accepts the image. Both records satisfy `expected_revision + 1 == terminal_revision` and `terminal_revision <= current`; identity uniqueness, binding limits and unresolved-owner checks also pass. Opening generation2 repeats the same validation and succeeds.

The resulting “validated” image claims two different publications at revision1 and retains current revision2 without its original revision2 terminal. This is an internal contradictory-image attack, not a claim about detecting an external rollback floor.

**Impact:** The development producer certifies historical outcomes that its own publication state machine cannot produce. Inspection can repeat the altered terminal as authoritative, undermining the closed recovery grammar and custody/outcome revision coupling.

**Required correction:** Validate historical committed revision uniqueness per instance before constructing a usable host. Reject inconsistent images without rewriting identities, revisions or terminal history. Existing Recovery fields suffice.

**Closure/regression:** Start from the authentic two-commit image above and require rejection after the duplicate-revision mutation. Preserve positive controls for sequential revisions1/2, repeated NonCommit records at an unchanged revision, and different instances independently committing at revision1. The current positive “multiple historical terminals” control uses NonCommit terminals and does not discriminate this defect.

**Architectural-root classification:** **Non-architectural** — missing historical terminal consistency validation in the existing development provider. This is distinct from the now-closed identity/unresolved-cardinality finding and requires no new interface, durable field, assembly seam or mutation boundary.

## 2. Invariant analysis

The original absence-before-publication counterexample remains excluded by the normative lifecycle: unknown preparation, temporary absence and Started work stay unresolved. A retained fence excludes later publication; publication and fencing repeat one immutable terminal result. Session close cannot release unresolved ownership.

Revision refusal now preserves attempts, queues, actual revision and attempt allocation. Existing-plan recovery takes precedence, so historical deduplication remains legal after application progress. Reopening validates retained consumption and reservations before changing ownership or floors; incompatible reductions leave a compatible retry at the same proposed generation available.

Sparse invocation count remains separate from high-water. Full issued/consumed requests authenticate callbacks and contradictions; secondary lookup indexes cannot supply authority. Exact bytes, charges, original receipts, storage class and historical start authorization remain bound. Sticky recovery incompleteness has no clearing event.

Meaningful Contract/Pure roles, minimal dependency direction, original obligations, actual text consumers and frozen Gyld 31/124 allocation remain preserved. No successful admission implementation or physical qualification is claimed.

The redesigned typed object retains **one architectural root**. This review adds none. Failed IC1 history, the second-remediation-round status and the unchanged cap remain intact.

## 3. Risks and next action

Physical barriers, submitted I/O after owner termination, antirollback, process reopening, genuine policy/clock provenance, critical preallocation and live duplex remain later qualifications. Existing SnapshotStore locks/fsync do not establish composite attempt finality.

Keep the typed gate and successful implementation stopped. The lane owner must resolve the new validator blocker through the existing capped review process; this report does not authorize another automatic remediation round or reset the object’s history.