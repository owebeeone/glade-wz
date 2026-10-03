# Glade independent CRDT admission design — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design at root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`, dated 2026-10-04. Restricted new-lane diff from `bba04ad27311db50e3e6aedddb4d91780e4e4483`.

**Baseline:** Workspace root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Member implementation sources were inspected with `git show <pin>:<path>`. The inspected design’s SHA256 matched its committed bytes.

**Date:** 2026-10-04

**Axis:** Safety — authorization, degraded projection, custody, recovery, disclosure and irreversible transitions. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This verdict accepts the proposed semantic design under its explicitly declared trusted-admitter/time model; it supplies no implementation or activation qualification.

---

## 0. Evidence base

The complete generated Safety prompt was read and its SHA256 matched `d9daf1c6aebe8563eeae26278dec770e27646f780c65888da18139a425500c66`.

All four HEADs matched the required tuple at start and end. An initial external-Gyld lookup used `/Volumes/projects/gyld`, which did not exist; the correct `/Volumes/projects/limbo/gyld-wz/gyld` path verified the required pin before substantive review.

Inspection covered:

- Complete revised design, lines 1–777, and its scoped diff.
- Resource requirements RC-001–007, complete IC-1–4 plan, DecisionLog GDL-054–057, committed initial Consistency/Safety reports and RemPlan-1.
- AGENTS.md, AGENTS_GWZ.md, review-loop skill and canonical prompt template; RaftQualificationPlan §6.
- BuildEntry, library-policy classifications and behavioral requirements, package architecture, arch1 architecture and component allocation.
- External Gyld declaration’s ports, accepted-history/handoff ownership, Admission/Records/Delivery, NodeAssembly and responsibility allocations.
- Authz §§1–7b and §11; DiscoveryModel §§0–4; WorkspaceDirectory §§1–7.
- Substrate’s core model, session/read/write rules; CrossNodeWrites options and W1–W8; complete CRDT adapter, ShapeDispatch, Zones and catalogue adoption; NodeSigning D4–D8.
- BindingResolver and ReplicaSync source, especially descriptive bindings, current-access checks, canonical-byte retention and known-error atomicity.
- Raft adoption contract, production integration plan and complete legacy Store seal contract.
- Pinned Glade acceptance, Store, envelope, signing, mesh route/serve and legacy-seal source.
- Root-lock-pinned Glial text mapping and Taut TypeScript CRDT identity/text projection; text corpus including `concurrent_siblings` and its exact `ABC` result.

Commands were read-only `git rev-parse`, `git show`, scoped `git diff`, `cat`, `sed`, `nl`, `rg` and hashing. No builds, tests, writes, network calls, Git mutations or current reports/closure testimony were used. Traces below are source-derived semantic checks.

### Changed-range analysis

The semantic changes are confined to design §§2/3/4/5/7 and ICD-006/007/009. They establish fresh canonical origins for certified epochs, historical qualification before fork conviction, qualification independent of derived quarantine, explicit security-evidence treatment and exact `AD`/`AB` witnesses.

These changes implement RemPlan-1’s two root causes. They introduce neither a new merge engine nor stronger receipt, authority or storage promises. The supporting requirement/plan/DecisionLog files have no changes in this restricted range.

**Architectural classification:** No new architectural root cause was identified. This remains the first architectural remediation round; the two-round cap is unchanged.

### Prior counterexamples independently retraced

| Prior finding | Revised rule and trace | Result |
| --- | --- | --- |
| Safety P2-1: unauthorized signed rival quarantines valid history | Lines 225–231 and 324–369 require both rivals to have qualifying historical admission. A revoked/expired writer’s bare signature cannot convict. Lines 384–394 require retained legitimate identities and exact `AD` in both orders. | Original counterexample blocked at the design level. |
| Safety P2-2 / Consistency P2-1: recovery epoch aliases old engine identity | Lines 92–104 require a new canonical `Op.origin`; lines 463–479 preserve old identities and refuse reuse. Lines 395–405 retain E0’s prefix and E1’s distinct identity with exact `AB`. | Original counterexample blocked at the design level. |

Execution of these witnesses remains mandatory in the contract/code tranches.

## 2. Invariant analysis

**Historical eligibility and forks.** I attacked the revised predicate with expired permits, revoked writers, invalid payloads, unknown keys, missing ancestors and opposite arrival orders. Missing evidence produces pending qualification; proven invalid rivals cannot remove legitimate projection. Genuine earlier admissions remain historically qualifying after expiry/revocation. Qualification depends on finite acyclic evidence rather than projection eligibility, so excluding both fork branches cannot invalidate their conviction proof. Own predecessor hashes bind exact bytes; ambiguous cross-origin slots do not select an arrival-order winner. Earliest-fork quarantine and transitive dependency filtering are deterministic over equal complete evidence sets.

**Identity and recovery.** Roots, incarnations, declarations, policies and exact capabilities bind the namespace before admission. Equal display names and empty discovery cannot combine histories or issue replacement genesis. Fresh epochs become distinct canonical origins before signing/hashing and remain distinct through refs, retries, text actors and Taut’s unchanged `(origin,seq)` identity. Strict seq0/full-history mapping blocks replica-dependent renormalization.

**Authorization and disclosure.** Authenticated forwarding nodes cannot substitute for original requester proof. Private `self`, membership, owner access and operator placement remain separate checks. Historical import cannot grant current serving rights. Finite permits, whole-interval time validation, durable rollback floors and current local revocation constrain honest disconnected admission. The residual ability of a malicious authorized admitter to lie about its historical time/view is explicitly part of the proposed trust model, rather than presented as cryptographic nonrevocation proof. Resources rejecting that assumption must refuse partitioned writes through fresh-policy admission.

**Custody and degraded operation.** Exact retry preserves canonical bytes and the original outcome; unknown commit boundaries cannot mint equivalent new operations. Receipt custody survives projection quarantine. Coupled physical commit includes retry evidence and recoverable handoff; a later best-effort outbox is forbidden. Candidate/evidence commits cannot masquerade as ReplicaSync’s no-mutation errors. Sole-copy loss, bounded pending overflow and exhausted recovery capacity produce explicit loss/incomplete/degraded states.

**Synchronization and compatibility.** Reconciliation must exchange application history and proof closure in both directions, with enrollment surviving browser unsubscribe. Directory-only convergence cannot satisfy the feature. Finite full-history retention refuses growth instead of evicting accepted evidence. Legacy/strong bindings retain their routes and receipts. Negotiation precedes provisional payload routing; new roots, old-binary exclusion and admission fencing are mandatory. Whole-store sealing retains its broader scope and cannot implement per-instance conversion.

**Implementation evidence.** The minimum tranche uses released text operations and two independent instances, with exact identities and projections. Fixtures cannot qualify cryptography, physical persistence or live synchronization. Gyld allocation, typed contracts, behavioral RED consumers, affected-consumer checks and source/global enforcement remain prerequisites.

## 3. Risks and next action

The design deliberately permits finite-window stale-policy admission and process-restart receipts that can lose acknowledged edits with the sole machine. Production acceptance still requires explicit owner choices and genuine adapter evidence. Full-history quotas also impose eventual capacity refusal; checkpointing remains a separate profile.

The next action is to record the accepted semantic dispositions and complete the stated Gyld allocation and typed behavioral RED contract prerequisites before kernel implementation. This GO does not authorize runtime enrollment, migration, sealing or production activation.