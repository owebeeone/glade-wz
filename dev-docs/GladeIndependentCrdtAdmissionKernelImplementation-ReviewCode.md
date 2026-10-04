# IC-2 Pure admission/reconciliation kernel — CODE-AXIS REVIEW

**Review object:** IC-2 deterministic transition and complete immutable batch encoder in `glade/contracts/crdt-admission-core`, Glade `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14`; controlling DRAFT `dev-docs/GladeIndependentCrdtAdmissionKernelImplementation.md` and its evidence at workspace root `332e7d95f5887300bf762ac76737e4cc38bcd151`.

**Baseline:**

| Repository | Exact reviewed HEAD |
| --- | --- |
| Workspace root | `332e7d95f5887300bf762ac76737e4cc38bcd151` |
| Glade | `4124cbcb04f3a7eb2db6a9072ae92b2bbea1cf14` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `95a426595bba8e248a5f484272e483a070c73918` |

Sources were read from the checked-out files and checked against pinned `git show` bytes. All five HEADs matched at both start and end.

**Date:** 2026-10-04  
**Axis:** Architecture, interfaces, call graphs, and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — four P2 findings block. All four roots are nonarchitectural: the existing contracts, state fields, and ownership boundaries suffice for their corrections. I pre-commit to GO on a revision that resolves P2-1 through P2-4 as specified.

---

## 0. Evidence base

Read the generated Code prompt completely; its SHA256 matched `6b15adadf9394011975c9de622317a8d3a35b927ef93b0ba42a0fd1e08c3847d`. Followed root/member instructions, `AGENTS_GWZ.md`, review-loop skill/template, and the local review-cycle history.

Inspected the complete implementation DRAFT/evidence; accepted admission design and retained contract; storage-attempt design and typed contract, including the supersession table; library/package policies, build entry, admission plan, resource profiles, and source-qualified Gyld allocation. The Pure/Contract roles and Records lifecycle ownership were inspected directly.

Read all production kernel modules:

| Source | Inspected lines |
| --- | --- |
| `lib.rs` | 1–129 |
| `types.rs` | 1–324 |
| `admission.rs` | 1–432 |
| `staging.rs` | 1–201 |
| `projection.rs` | 1–161 |
| `lifecycle.rs` | 1–293 |
| `callbacks.rs` | 1–243 |
| `encoder.rs` | 1–243 |

Read the retained Records/recovery/storage contract suites and complete assembly support; new kernel-edge and encoder mutation suites; fixture composition, representation, and source checks; unchanged shared storage API and development provider; public conformance consumers; complete text trace and released Glial consumer, canonical corpus, and source guard.

Read scoped root and Glade diffs. Read-only byte comparisons confirmed 42 inspected implementation/control files against HEAD; critical retained consumers, shared API/provider, policy, and selector against the Glade baseline; controlling documents and Glial consumers against their respective pins.

No builds or tests were executed. Audited existing RED/GREEN and mutant logs against source. The final adopted log reports 86 core and 34 API tests, formatting/lint/architecture success; the text log reports the actual released consumer and corpus success. The process-global log reports 88 files, three permanent entries, and no new debt. The two isolated production-source mutants genuinely fail their intended regression and their controls pass. Chronology discloses initially green encoder controls and the authorized obsolete scaffold-assertion replacement; these were not treated as fabricated RED evidence.

## 1. Findings

### [P2-1] A delayed Started reply rewinds an already terminal attempt

**Location:** [callbacks.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/callbacks.rs:124), lines 124–135; terminal comparison at 45–61 applies only to terminal payloads. [lifecycle.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/lifecycle.rs:151), lines 151–166, subsequently selects the rewound attempt for recovery.

**Invariant:** StorageAttemptDesign §3 gives Committed and NonCommit no successor. Typed contract §§4–6 requires immutable terminal identity and restoration consistent with terminal results.

**Sequence:** Prepare a valid local plan. The real memory provider begins it with publication held, producing an authenticated Started reply; delay delivery. `ResumeRecovery` issues Inspect while Begin remains outstanding. Publish at the provider and deliver Inspect’s Committed terminal first. The kernel installs custody once and sets Terminal. Deliver the delayed, exactly issued Started reply. Its cut matches, its invocation remains live, and the removed plan is optional on this path. Lines 127–128 overwrite Terminal with Started.

**Impact:** The kernel reports OutcomeUnknown after knowing the terminal truth. Validated restoration rejects the inconsistent phase/terminal map. Subsequent recovery issues unnecessary Inspect requests; same-terminal replies are consumed and rejected without repairing the phase, spending finite invocation history.

**Required correction:** Reconcile nonterminal observations against retained terminal identity before changing phase or reporting uncertainty. An authenticated delayed Started/Pending/Refused observation must not undo terminal settlement. Preserve contrary-terminal detection and evidence.

**Closure:** Exercise terminal-before-delayed-nonterminal and the reverse order, for both terminal outcomes and all applicable batch kinds. Assert terminal phase, unchanged installed custody/charges, valid restoration, and no renewed recovery lookup after settlement.

**Root classification:** Nonarchitectural; missing monotonic reconciliation within the existing callback representation.

### [P2-2] The integrity stop is bypassed by pending verification and qualified-fork paths

**Location:** [admission.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:207), lines 207–399 and 401–423; [staging.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:63), lines 63–201. The stop check exists only at fresh `offer`, lines 66–68.

**Invariant:** StorageAttemptDesign §5 and typed contract §§5/7 require authenticated contradictory terminals to stop new work for the affected instance while preserving existing reservations/evidence.

**Sequence:** Issue independent verification queries A and B for X before either stages. Complete and commit A. Deliver a fully authenticated contrary terminal for A, setting X’s `storage_integrity_failed`. Deliver B’s still-outstanding exact Verified reply, followed by its valid seal. Neither continuation nor staging checks the stop. A fresh plan is prepared and begun after the integrity failure. Separately, `OfferFork` for two already qualified retained records directly stages QualifiedFork after the stop.

**Impact:** X remains capable of new atomic mutations after its lifecycle authority has contradicted itself. The ordinary offer guard does not cover the actual call graph.

**Required correction:** Enforce the existing stop before further new verification/sealing/staging work, including delayed continuations and direct fork retention. Continue resolving previously owned uncertain storage work and preserving receipts/custody; do not implement the stop by deleting those reservations.

**Closure:** Hold B’s verification and seal across A’s authenticated contradiction; require no new Prepare/Begin or charges afterward. Test the already-qualified `OfferFork` route. Preserve exact retries and recovery of existing attempts, and prove independent Y still progresses.

**Root classification:** Nonarchitectural; incomplete enforcement of an existing instance gate.

### [P2-3] Frontier origin strings bypass finite name and byte bounds

**Location:** [lib.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/lib.rs:69), lines 69–95 and 108–124.

**Invariant:** Retained admission contract §§3/6 requires bounded peer inventory and bounded allocation/event work. The existing storage limits provide `max_name_bytes`; ordinary admission enforces it.

**Sequence:** With a valid fixture descriptor, `max_name_bytes = 64`, and `pending_bytes = 8192`, send one unknown frontier head whose origin is a one-million-byte string, sequence zero, and a valid fixed-size digest. The count and sequence checks pass. The function clones and retains the entire origin in `pending` and returns it in RequestMissing. Increasing that string increases retained state without encountering any named byte limit.

**Impact:** A single peer hint can create arbitrarily large persistent kernel state; subsequent transition/read cloning repeats that allocation. Setting sticky incompleteness does not bound the retained value.

**Required correction:** Validate frontier origins against the existing name limit before retaining or duplicating them, and ensure the aggregate retained hint representation satisfies applicable finite byte bounds. Refuse oversized observations honestly; preserve prior pending work and sticky incompleteness where an observed cut cannot be retained.

**Closure:** Cover the exact name limit, limit plus one, a very large origin, and aggregate retained-hint boundaries. Assert that rejected input creates no oversized state/effect, false committed charge, or falsely complete read.

**Root classification:** Nonarchitectural; missing ingress validation using existing limits.

### [P2-4] Candidate-derived missing slots bypass the pending-item cap

**Location:** [admission.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/admission.rs:303), lines 303–317; [staging.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/staging.rs:109), line 109; [projection.rs](/Volumes/projects/limbo/glade-wz/glade/contracts/crdt-admission-core/src/projection.rs:106), lines 106–115.

**Invariant:** The retained contract requires pending-capacity refusal rather than overflowing recovery representation. The implementation DRAFT/evidence explicitly establishes that aggregate missing slots cannot exceed `pending_items`; frontier processing enforces that union.

**Sequence:** Use a valid descriptor with `pending_items = 1`, at least two allowed refs, and sufficient byte budgets. Offer a historically admissible sequence-zero candidate with sorted references to absent `a:0` and `b:0`. Verification succeeds. The missing-closure branch emits both slots and stages Candidate; staging checks candidate count, not the prospective missing-slot union. Actual committed installation retains the candidate and `refresh` installs two pending slots.

**Impact:** An accepted event path exceeds the same pending-slot cap that frontier events enforce. Sticky bounded-refusal handling is bypassed. Further distinct closures amplify the discrepancy.

**Required correction:** Check the prospective distinct missing-slot union before issuing/staging this candidate, consistently with frontier and retained candidate sources. On exhaustion, preserve existing custody and report bounded recovery refusal with sticky incompleteness.

**Closure:** Test one candidate with two missing refs against a one-slot limit, exact-limit success, overlapping refs with deduplication, and candidate/frontier combinations. Assert the cap at every transition, no improper head advancement, and preserved custody/charges on refusal.

**Root classification:** Nonarchitectural; missing quota calculation within existing state and bounds.

## 2. Invariant analysis

Several attacks did not establish defects. Descriptor/profile/operation validation rejects malformed numeric values, structural predecessor/ref shapes, unsorted refs, foreign instances, and oversized candidate/proof inputs. Verification binds exact issued queries and certificate/origin/epoch facts; trusted Facts remain explicit model inputs rather than peer authority.

The encoder has explicit framing/version and semantic encodings for descriptor, candidates, evidence, facts, verification/seal continuations, fork pairs, receipt previews, and charges. It derives the batch digest from exact bytes without recursively encoding that derived digest or substituting Debug output. Mutation coverage and inspected call sites support complete staged-field encoding.

Projection separates custody from eligibility. Security-only rivals cannot convict; qualified same-slot forks retain common-prefix custody and original receipts, including transitive dependents and newly earlier forks. Fresh certified origins remain distinct from old display identities. The released text-consumer path remains real and retained.

Full callback request matching precedes ordinary consumption; foreign ownership, altered binding, never-issued identity, and retired same-terminal callbacks fail authentication or duplicate handling. Pending/Refused do not become NonCommit. Checked counters, finite history, exact encoded-byte charging, and per-instance uncertain-work isolation generally hold, subject to the findings above.

Unknown frontier digests cannot be retained in the accepted Slot grammar. The implementation’s monotonic `recovery_incomplete` is a conservative, contract-permitted limitation: it avoids false completion and has no claimed clearing operation. This is not an additional blocker or a claim of eventual full-cut reconstruction.

Package roles, shared interfaces, dependency edges, and Gyld allocations remain intact. No budget, classification, dependency allowlist, or process-global exemption was loosened. Source guards inspect disabled Rust branches; macro opacity is disclosed rather than presented as complete expansion coverage.

## 3. Risks and next action

This review qualifies only the deterministic component. Production cryptographic domains, physical persistence/antirollback, cancellation/drain, live transport, migration, Surface, and activation remain explicitly deferred and were not converted into objections.

The next action is one scoped, test-first correction set for P2-1–P2-4, followed by required verification and fresh review of a settled five-repository tuple. No architectural root was identified in this round; historical review caps and stopped checkpoints remain unchanged.