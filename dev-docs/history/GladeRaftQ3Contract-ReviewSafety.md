# Glade Raft Q3 Contract — SAFETY-AXIS REVIEW

**Review object:** Root diff `14c347612678e76affd561abd31728adac6a9043..fa1ff8b9fd0be53932300730ff925d0e41c76b1a`: DRAFT `dev-docs/GladeRaftConfigurationSnapshotContract.md` and compiling RED specifications under `proofs/raft-adoption`. Contract gate only; no production activation or implementation qualification.

**Baseline:** Reviewed root `fa1ff8b9fd0be53932300730ff925d0e41c76b1a`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Pinned controlling documents were read with `git show`; scoped working-tree comparisons found no differences. All four HEADs matched at start and end.

**Date:** 2026-10-03

**Axis:** Safety: attack permitted degraded paths, irreversible transitions, durable recovery and consumer satisfiability. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — three P2 findings block contract acceptance. I pre-commit to GO on a revision that resolves P2-1, P2-2 and P2-3 as specified.

---

## 0. Evidence base

Read root instructions, `AGENTS_GWZ.md`, the canonical Safety prompt and the review-loop skill. Reviewed:

- Configuration/snapshot contract, lines 1–114; Q3 evidence and complete new API/conformance/refusing-provider sources.
- BuildEntry; LibraryBoundaryAndTestingPolicy §§1–6; PackageArchitecture §§1–8; AdoptionContract §§1–4; QualificationPlan §§1–6; PersistenceContract, including its crash-oracle and implementation amendments.
- QualificationEvidence and Qualification-ReviewCycle, including their accepted Q2 limits and Q3 preparation.
- Architecture revision 3 and the pinned external Gyld allocation for Admission, Policy, Records, StorageAdapter and NodeAssembly; pinned AuthzModel’s serving-hop and creation-rooted authority clauses.
- Existing application semantics, especially `proof/src/application.rs:240–282`, and accepted create fixtures.
- Local raft-rs 0.7.0 sources: RawNode configuration operations; Raft configuration application, pending-configuration tracking and snapshot restoration; ConfChange joint-entry/leave grammar.

Independent execution, using the permitted pinned PROTOC where applicable:

- Targeted q3-api/q3-spec `cargo test --locked --offline ... --no-run`: PASS.
- q3-api tests: one compiler consumer PASS.
- q3-spec `configuration_snapshot`: 15 failures, all at `NotQualified`, exit 101; no ignored tests.
- `proofs/raft-adoption/check.sh`: PASS; architecture, formatting and explicit source boundaries; 19 owned files, zero process-global exceptions.

No Q3 implementation exists, so the counterexamples below are source/contract deductions, not claims of executed carrier failures. No broad Q2 or process-kill rerun was necessary for this source-only object. Nothing was edited.

## 1. Findings

### [P2-1] Shared create fixture violates the retained application preconditions

**Location:** `q3-api/src/conformance.rs:11–35`; consumers include `conformance/snapshot.rs:5–44` and `membership.rs:175–233`. Existing semantics are `proof/src/application.rs:262–266`.

**Invariant:** Q3 preserves the existing application contract and supplies meaningful success/failure/edge specifications under LBT-006/007/009. A success setup must actually create its resource.

**Reproduction:** `create()` delegates to `command()`, which sets `generation=1` and `home=1`. The retained create semantics require command preconditions `generation=0`, `home=0`; `Action::Create.home=1` separately selects the new resource’s home. A faithful application therefore returns a terminal `Rejected(StaleGeneration)` receipt, with no resource. The snapshot-original test accepts any receipt through `expect("receipt")`, submits a mutation against the nonexistent resource, and eventually fails when requiring payload 23. Removal and movement specifications likewise cannot establish their intended live resource.

**Impact:** Several compiling specifications are unsatisfiable against the retained semantics. Their initial `NotQualified` failure conceals this defect; implementing until these cases turn GREEN could encourage an unauthorized create-semantics change.

**Required correction:** Give the create helper explicit zero generation/home preconditions. Assert the complete expected Accepted resource in success setup, rather than merely requiring a receipt.

**Closure test:** Add a focused fixture regression asserting the complete create command and its compatibility with the existing committed application. Preserve negative tests for nonzero create preconditions. Keep the corrected Q3 scaffold behaviorally RED without weakening existing semantics.

### [P2-2] Home eligibility is checked at joint entry but not protected through joint exit

**Location:** Configuration/snapshot contract lines 39–41; QM-007 and implementation matrix lines 79, 105–112.

**Invariant:** The bounded profile does not introduce nonvoting resource homes; membership must preserve independently qualified home placement.

**State sequence:** Start with no live home on node 3. Admit/catch up learner 4, then accept EnterJoint with incoming `[1,2,4]` and outgoing `[1,2,3]`. The specified EnterJoint home check passes. While joint, node 3 remains an outgoing voter with complete data. Order a new valid Create choosing home 3, or a qualified Move to 3. The text does not restrict these operations to incoming voters. Then separately authorize and apply LeaveJoint: its specified preconditions do not recheck current homes. Node 3 disappears from the resulting configuration while remaining the live resource home.

**Impact:** A valid ordered sequence can strand a live resource on a removed member. Quorum safety alone does not establish availability or continued home eligibility. Checking only the earlier EnterJoint predecessor state cannot cover later application entries.

**Required correction:** Define home eligibility during joint configuration explicitly. Either forbid new placement on outgoing-only voters, or require deterministic current-home validation at the actual LeaveJoint index, retaining `Refused(HomeInUse)` when necessary. Any admission check must also survive intervening committed placement changes.

**Closure test:** Add a compiling RED schedule covering EnterJoint, intervening Create/Move toward outgoing-only node 3, and LeaveJoint. Require placement refusal or retained leave refusal; demonstrate that qualified movement/retirement permits exit. Repeat with restart while joint and with placement queued after leave admission.

### [P2-3] Exact-index replay cannot return a configuration entry’s original result

**Location:** Configuration/snapshot contract line 45; `q3-api/src/lib.rs:81–87,199–212`; snapshot conformance currently exercises application replay only.

**Invariant:** Every original applied index, including configuration entries, retains its original result; exact original-index replay returns that result. LBT-006 requires a satisfiable boundary.

**Reproduction:** Apply an accepted configuration intent and retain its complete `ConfigReceipt`; repeat with an ordered refused intent. Obtain each original `StoredEntry`, checkpoint, install and restart. The promised exact-index replay must recover the original configuration result. However, `QualificationSession::replay` returns `Result<Option<Receipt>, Error>`, where `Receipt` contains an application RequestId and application Outcome. It cannot represent ConfigIntent, ConfigOutcome or Configuration. Returning `None` loses the original result at that boundary. Keyed `configuration_outcome` is useful but does not fulfill the distinct index-replay promise.

**Impact:** The proposed interface cannot satisfy its complete original-history contract. A consumer can verify replay identity for application entries but cannot receive the required accepted/refused configuration result through the same operation.

**Required correction:** Provide a typed replay result covering application receipts, configuration receipts and noops, or an explicit configuration-index replay operation with equivalent guarantees. Preserve complete history and changed-envelope rejection.

**Closure test:** Compile consumers that obtain complete accepted and refused configuration results by original index. Add RED checkpoint/restart cases comparing those results with externally retained originals, alongside noop and changed-envelope cases.

## 2. Invariant analysis

Several attacks failed against explicit requirements. Joint commitment requires separate incoming/outgoing data-bearing majorities; learners cannot supply old-set votes. Promotion evidence is retained privately and checked against the actual predecessor state, so queued application work produces a deterministic ordered refusal rather than replica-local decisions.

Incoming snapshots are privately replayed before publication, with a coherent checkpoint/applied cut. Ready and LightReady publication precede message/receipt release. Post-write errors poison participation; restart admits complete history or quarantines partial history. Same-term historical votes survive removal, while uncommitted suffix replacement remains legal.

Complete replay history, tombstones, policy, name reservations and original bytes remain mandatory through compaction. Capacity exhaustion cannot evict evidence. Independent rollback floors and process-versus-power-loss limits are stated honestly. The refusing provider is consistently identified as RED scaffolding, never safety qualification.

## 3. Risks and next action

Physical V2 lifecycle, valid-format semantic corruption, real carrier refusal/installation paths and externally checked SIGKILL recovery remain mandatory later witnesses. They are deferred implementation evidence, not findings against this contract checkpoint.

The next action is one bounded contract/specification remediation addressing all three findings, followed by applicable independent re-review before implementation. The end tuple remained unchanged.
