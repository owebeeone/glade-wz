# Independent CRDT admission design — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design dated 2026-10-04, at root `bba04ad27311db50e3e6aedddb4d91780e4e4483`; restricted new-lane diff from `9baa7b8cd1c876ff9e608ecdffdd7490b58ebe9b`.

**Baseline:** Root `bba04ad27311db50e3e6aedddb4d91780e4e4483`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld member `ca04499a360d910fbf8ee2540ed446facd051b35`. Committed sources were inspected with `git show <pin>:<path>`.

**Date:** 2026-10-04

**Axis:** Internal coherence, consistency with controlling contracts, exact amendment boundaries, and satisfiability of required witnesses. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks. I pre-commit to GO on a revision that resolves P2-1 as specified.

---

## 0. Evidence base

The complete generated prompt was read. Its SHA256 matched `1a3e1d52c366b194e3bdbbad2495e65a917306019c7f197a4bc85fa9218c35e1`. All four specified HEADs matched at the start and end; the external Gyld SHA names `/Volumes/projects/limbo/gyld-wz/gyld`, rather than its enclosing workspace root.

Inspection covered:

- Complete design, lines 1–681; complete delivery plan, requirement capture, and new review ledger; DecisionLog GDL-054–057; scoped diff.
- AGENTS.md, AGENTS_GWZ.md, review-loop skill and canonical template; RaftQualificationPlan §6.
- LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, GladeBuildEntry; arch1/GladeArchitecture lines 1–178; external Gyld declaration’s ports, components, state ownership and assembly allocation.
- Authz §§1–7b, DiscoveryModel §§0–6, WorkspaceDirectory §§1–7b; Substrate §6 R1–R8/W1–W7 and client refusal behavior; CrossNodeWrites §§1–3.
- Complete CrdtAdapter, ShapeDispatch, Zones and TautShapeCatalogAdoption; NodeSigning D1–D9; ApplicationContractDraft and BindingResolver/ReplicaSync contracts.
- RaftAdoptionContract, ProductionIntegrationPlan and LegacyStoreSealContract.
- Pinned Glade acceptance, Store append/shape validation, home envelope, signing, mesh route/serve and legacy-seal source.
- Root-lock-pinned Glial `text_crdt.ts`, particularly lines 176–237; Taut-shape text corpus’s `concurrent_siblings` scenario; Taut-shape-ts generated `CrdtOp` and engine identity/equivocation logic at its root-lock pin.

Only read-only inspection commands ran. No builds, tests, network calls, mutations or current peer reports were used. Counterexamples below are source-derived traces, not executed qualification evidence.

## 1. Findings

### [P2-1] Fresh origin epochs lack a collision-free mapping into canonical CRDT identity

**Location:** Design lines 92–100, 131–137, 297–316 and 374–383; ICD-007/009 at lines 544–546. Controlling adapter: `GladeCrdtAdapter.md` lines 14–19. Canonical engine: `taut-shape-ts/src/crdt/engine.ts` lines 31, 105–114; generated `CrdtOp` lines 19–24.

**Violated invariant:** Distinct valid origin incarnations must remain distinct through admission, causal references and the released merge engine. The design distinguishes signed slots by `(instance, origin, epoch, seq)`, but preserves canonical app operations and the existing mapping from `Op.origin` directly to `CrdtOp.origin`. Taut identifies operations solely by `(origin, seq)` and has no epoch field.

**Counterexample:** An instance retains an eligible prefix containing origin `a`, epoch E0, Glade seq 0, inserting `A`. A later fork at seq 1 quarantines both successors while preserving that prefix. Under lines 315–316, authority certifies a fresh origin epoch E1 for recovery. The draft does not require a different canonical `Op.origin` when the epoch changes. A recovery implementation can therefore admit E1’s initial operation as origin `a`, Glade seq 0, inserting `B`, with a valid new certificate and permit.

The eligibility layer sees different epochs, so this is not its same-slot fork. Both operations map to Taut `(a,1)`. The released engine treats them as equivocation and retains one variant, rather than the two eligible operations. Alternatively, an implementation indexing chains only by `Op.origin` rejects the newly certified epoch as reused identity. The draft does not settle which behavior is required.

The generic “reused origin/rollback fail” witness does not close this gap: it does not define whether an authorized new epoch must receive a new canonical origin identifier.

**Impact:** The expressly provided post-fork/backup recovery path can acknowledge an operation that cannot coexist with retained eligible history under the required engine, or implementations can disagree about admissibility. This is an identity-contract defect before kernel implementation, not a deferred choice of key custody or wire-container fields.

**Required correction:** Define the effective canonical origin identity. The simplest compatible rule is that every fresh certified epoch MUST receive a fresh, never-reused `Op.origin` within the instance; certification MUST bind that exact identifier. All refs, frontiers, retry identities, quarantine keys and editor identities must use the same effective origin. Specify that old-epoch history remains retained and that an epoch change cannot reset seq under an existing canonical origin. If another mapping is chosen, pin it explicitly and demonstrate compatibility with unchanged inner Op bytes and the released engine.

**Closure/regression witness:** Extend ICD-T07/T09 with a retained E0 prefix, a later fork, and authorized E1 recovery. Assert two distinct effective origin identities, retention of both eligible operations, correct causal references and identical released-text projection under opposite delivery orders. Include rejection of a certificate proposing E1 with E0’s canonical origin. For this design gate, the revised normative mapping and exact expected witness are sufficient; execution belongs to the contract/code tranche.

## 2. Invariant analysis

The principal attacks otherwise failed:

- **Independent identity:** Descriptor binding separates roots, incarnations, declarations and profiles. Empty discovery never authorizes genesis. First observed shape cannot select admission. Foreign permits, cursors and refs are expressly excluded.
- **Partition availability:** Qualified replicas can admit without a holder or quorum. Synchronization requires actual bidirectional application operations and evidence, enrollment survives browser unsubscribe, and directory convergence cannot masquerade as application convergence.
- **Authorization:** The proposal preserves requester attribution across forwarding, identity-derived private scope and placement checks. It explicitly proposes finite permits, trusted time uncertainty and historical admission eligibility after concurrent revocation. Current serving rights remain separately checked. The trusted-admitter assumption is disclosed as a review decision, not smuggled in as cryptographic proof of nonrevocation.
- **Custody and recovery:** Accepted receipts remain immutable evidence even when projection becomes quarantined. Lost replies require exact retry; coupled commit includes retry state and recoverable handoff. Candidate retention is distinguished from accepted-batch ingestion, preserving ReplicaSync’s known-error atomicity.
- **Bounds:** Full-history retention is paired with finite capacity refusal, reserved recovery capacity and explicit incomplete sync. Sole-copy loss receives no fabricated recovery promise.
- **Compatibility:** Legacy/strong bindings retain their contracts. New proof negotiation precedes routing, old Op encoding cannot be casually extended, and whole-store sealing remains broader than instance activation.
- **Evidence and architecture:** The `concurrent_siblings` corpus does specify `ABC`. The pure kernel tranche does not claim live crypto, physical durability or network qualification. Gyld allocation, compiling RED consumers, package classification and affected-consumer checks remain prerequisites; source/global rules are preserved.

The supersession table identifies the major holder, receipt, refusal, shape and historical-authorization changes while retaining unactivated contracts. No independent blocking contradiction was established there.

## 3. Risks and next action

Real certification, time evidence, physical commit, protocol exclusion and automatic application anti-entropy remain substantial qualification work. They are explicit later gates; this review supplies no implementation acceptance.

The next action is a scoped semantic revision resolving P2-1, followed by originating-reviewer verification on a new settled tuple. No kernel implementation or live activation follows from this NO-GO.