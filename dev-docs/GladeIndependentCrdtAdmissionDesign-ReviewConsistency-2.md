# Independent CRDT admission design — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionDesign.md`, DRAFT semantic design dated 2026-10-04, at root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`. Restricted revision range: `bba04ad27311db50e3e6aedddb4d91780e4e4483..dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`.

**Baseline:** Glade workspace root `dcc8bd02e9eb3bf5a1ba982add9c405c2adc34e9`; Glade `c65a6e87f0c257c15de8db080c29d365a883af85`; Glade-discover `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld member `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling committed sources were inspected using `git show <pin>:<path>` and the scoped diff.

**Date:** 2026-10-04

**Axis:** Internal coherence, agreement with controlling contracts, exact amendment boundaries, and satisfiability of future witnesses. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This accepts the revised semantic design on the Consistency axis only; it supplies no implementation, adapter qualification, interface freeze or activation acceptance.

---

## Prior-finding closure table

| Prior finding | Correction | Independent retrace on this tuple | Result |
| --- | --- | --- | --- |
| Consistency P2-1 / Safety P2-2 | Fresh canonical `Op.origin` for each certified epoch | Lines 92–104 require distinct canonical origins before hashing/signing; lines 462–476 preserve old identities and forbid sequence reset under an old origin. The recovery witness at lines 393–405 retains E0’s prefix and admits E1 under `writer-e1`, producing distinct Taut identities and `AB`; reuse of `writer-e0` refuses before mutation. | Original collision counterexample is excluded at the semantic gate. |
| Safety P2-1 | Admission-qualified fork conviction | Lines 225–231 and 323–359 require qualifying historical admission for both rivals. The witness at lines 382–392 preserves legitimate `AD` when the revoked/expired writer supplies a bare signed rival. | Original unauthorized-rival counterexample is excluded at the semantic gate. |

These are fresh independent source retraces. They do not substitute for the separately required originating-reviewer verification.

## Changed-range analysis

The design changed by 116 added and 20 removed lines. Material changes are confined to canonical origin identity, certificate/ref/receipt binding, admission-qualified fork conviction, recovery semantics, and ICD-006/007/009 witnesses. The other changed files record prompts, initial reports, remediation and review process.

The historical qualification predicate is explicitly independent of projection quarantine. Qualifying predecessor/dependency evidence remains usable to establish a conflict after projection exclusion; therefore the correction does not introduce circular qualification or erase its own fork proof. Cross-origin disputed references require historical support at the canonical slot, rather than an arrival-selected winner.

No change outside the two remediation dispositions was established. **No new architectural root cause was found.** This remains architectural remediation round 1; this review does not consume or authorize another remediation round.

## 0. Evidence base

The complete generated prompt was read. SHA256 matched `5c4876ef83431b1853564c2451cd1bce2cf5b2e57008d763796b1d68a5aa2737`. All four HEADs matched at the beginning and end. The external Gyld pin names `/Volumes/projects/limbo/gyld-wz/gyld`.

Inspection covered:

- Complete revised design, lines 1–777; scoped diff; committed initial Consistency/Safety reports and RemPlan-1.
- AGENTS.md, AGENTS_GWZ.md, review-loop skill and canonical prompt template; RaftQualificationPlan §6.
- ResourceConsistencyProfiles, IndependentCrdtAdmissionPlan, DecisionLog GDL-054–057, BuildEntry, LibraryBoundaryAndTestingPolicy, PackageArchitecture, arch1 architecture/component allocation, and external Gyld declaration.
- Authz §§1–7b and §11; DiscoveryModel §§0–6; WorkspaceDirectory §§1–7b.
- Substrate §6 session/write rules; CrossNodeWrites §§1–3/7/8; CrdtAdapter, ShapeDispatch, Zones, NodeSigning and TautShapeCatalogAdoption.
- ApplicationContractDraft and BindingResolver/ReplicaSync source; MultiwriterSettingsEvaluation; RaftBootstrapGrowthDesign §§1–5; RaftAdoptionContract, ProductionIntegrationPlan and LegacyStoreSealContract.
- Pinned Glade acceptance, Store replay/append, envelope, signing, server Hello, mesh serve/route and legacy seal code.
- Root-lock-pinned Glial text adapter (`4c6e856…`), particularly sequence mapping and editor identities; Taut-shape text corpus (`5779c967…`); Taut-shape-ts CRDT engine/text projection (`1f2d6c4…`).

Only inspection commands ran. No builds, tests, writes, network calls, Git mutations or current-round report/closure testimony were used. Witness results below are source-derived expectations, not executed qualification.

## 2. Invariant analysis

**Canonical identity and recovery.** Equal names, profile names and display origins cannot combine roots or instances. The authenticated descriptor fixes declaration, shape and capabilities before admission. Empty discovery cannot authorize genesis. Fresh epoch identity now survives unchanged `Op.origin → CrdtOp.origin` mapping, causal refs, text actor identity and exact retry. Full history and fixed zero-base mapping exclude first-observation renormalization.

**Concurrent isolated admission and reconciliation.** The activated profile expressly permits independently authorized replicas to accept without holder/quorum consultation. It requires enrolled, instance-scoped, bidirectional application-history synchronization surviving browser unsubscribe/restart. The current mesh’s home-only synchronization cannot satisfy ICD-T12. The minimum `concurrent_siblings` witness agrees with the pinned corpus’s exact `{a:1,b:1,c:1}` operation set and `ABC` projection.

**Authorization and historical eligibility.** Writer certification, possession and immutable app proof preserve requester attribution across forwarding. Peer HELLO identity cannot replace it. Private scope, membership, account-owner access and placement remain separate checks. Finite permits, trusted time uncertainty and known-revocation refusal bound disconnected admission. Historical eligibility is distinguished from current serving rights, and the trusted-admitter/time assumption is expressly identified as an amendment to Authz’s placement-only trust claim.

**Forks, gaps and projection.** Bare signed unauthorized, invalid or unresolved rivals cannot convict admitted history. Genuine qualifying rivals still quarantine both branches and transitive dependents. Qualification uses finite acyclic historical evidence without depending on derived projection eligibility. Opposite arrival orders therefore preserve `AD` for a nonqualifying rival and preserve the exact old/new-origin identities yielding `AB` after qualified fork recovery. Missing closure remains pending; no falsely advanced accepted frontier is permitted.

**Receipts and storage.** Exact retry recovers the original identity/receipt separately from current custody/projection status. Lost replies and uncertain partial I/O cannot manufacture noncommit or a replacement operation. Coupled physical commit includes admission/retry state and recoverable handoff. Candidate/evidence retention is explicitly separated from ReplicaSync’s atomic accepted-batch/no-mutation-on-known-error contract.

**Capacity and loss.** Full-history retention is paired with finite quotas, reserved recovery capacity, capacity refusal and explicit incomplete synchronization. Accepted evidence cannot be evicted to admit more edits. Permanent loss of the only copy receives no fabricated recovery guarantee.

**Canonical reconciliation and coexistence.** The supersession table identifies holder routing/judgment, persistence, receipt, gap, shape and historical-authz changes while retaining unactivated legacy/strong behavior. New proof/schema negotiation must precede routing; frozen inner Op encoding cannot be casually extended. Whole-store sealing remains broader than per-instance activation, and old-binary exclusion requires protocol/enrollment fencing as well as separate directories.

**Architecture and evidence tiers.** Pure decision logic consumes supplied evidence and explicit commit outcomes; Records/Storage own physical execution. Gyld allocation, typed compiling RED consumers, package classification and affected-consumer checks remain prerequisites. ICD witnesses distinguish deterministic component proof from genuine crypto, storage and live-node qualification. Disabled-branch syntax enforcement and process-global rules remain explicit obligations.

## 3. Risks and next action

Certification/custody, time evidence, canonical proof encoding, physical commit, protocol exclusion and automatic application synchronization remain substantial unimplemented obligations. The stated deferrals preserve their mandatory gates and do not confer production defaults.

The next action is to merge independent verdicts and originating closure verification, then complete the specified canonical dispositions, Gyld allocation and typed compiling RED consumer/package gate before kernel implementation.