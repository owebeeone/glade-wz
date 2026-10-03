# IC-1 independent CRDT admission contract — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/GladeIndependentCrdtAdmissionContract.md`, DRAFT IC-1 remediation1 internal contract, refusing scaffold, compiling behavioral RED consumers and frozen Gyld allocation, at root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`.

**Baseline:** Root `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`; Glade `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`; discovery `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`; external Gyld `64666e8b1caadde8922b9d42163afbab90655c65`; Glial `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`. Root comparison baseline: `7c5ac428a92c4087cf0ed1fff9e399d6f06c5850`. Sources were inspected directly at verified HEADs, with exact-pin `git show`, scoped root diff and read-only provenance comparisons.

**Date:** 2026-10-04

**Axis:** Internal consistency, agreement with controlling contracts, continuation completeness and satisfiability of test/evidence obligations. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — no P0, P1, P2 or P3 finding established. This accepts the internal semantic/interface and compiled RED allocation gate only; it does not establish successful admission, adapter qualification, a user-facing freeze or activation.

---

## 0. Evidence base

All five HEADs matched the exact tuple at review start and end.

Inspection included:

- Complete generated Consistency-2 prompt, controlling contract §§1–9, initial evidence, merged RemPlan-1, Remediation1-Evidence and legitimate initial-round reports.
- Complete `src/types.rs:1–397`, refusing `src/lib.rs`, original records consumer, support module, representation and source-boundary tests.
- Complete `recovery_contract.rs:1–526`, Rust text trace `1–331`, released-text JS consumer `1–90`, JS syntax guard and text runner.
- Accepted semantic design’s identity, authority, historical qualification, fork, custody, bounds, supersession, stable ICD requirements and staged-delivery sections; resource-profile requirements, delivery plan and GDL-054–057.
- Library policy, package architecture, BuildEntry, architecture ownership notes, relevant Substrate R/W rules, cross-node writes, Authz, discovery/directory, CRDT adapter, shape/catalogue/zone and Raft coexistence/seal provisions.
- Existing BindingResolver/ReplicaSync contracts and scoped acceptance, Store, mesh, signing/envelope and Glial text-source inspection.
- Package manifest, README, architecture policy, selector, ARCH-002 fixture and tooling regressions.
- Complete frozen Gyld overlay declaration, capture host and supplemental test class; supplemental ledger and read-only provenance comparisons.

Read-only hashing confirmed that the supplemental design text matches the accepted root design and its recorded digest; the base-ledger digest matches. The refusing kernel, architecture policy and Cargo lock remain byte-identical to the reviewed remediation baseline. Gyld’s base source ledger, architecture declaration and base capture host remain unchanged against the recorded baseline.

No builds, tests, writes, network calls or Git mutations were performed. No current peer report or restored closure was read. Recorded execution results are drafter evidence audited against consumers, not independently rerun results.

## 2. Invariant analysis

**Continuation identity and recovery shape hold.** Types distinguish immutable `plan_id` from freshly allocated `lookup_id`; outstanding lookups use the latter. Contract §6 requires exact matching, retirement after every valid answer, renewed identity after Unknown, reservation preservation and nonwrapping allocation. Recovery tests `98–174` explicitly exercise L1→Unknown→L2, all three stale L1 answers, unchanged L2/reservation, original-receipt recovery and exhaustion. The restoration consumer `498–526` preserves an injected trusted high-water mark and rejects an old callback. It correctly claims no physical restoration proof.

Verification/seal queries retain immutable candidate bytes, descriptor, evidence, facts and original validation mode. Commit and lookup replies bind retained plans. The revised delayed-callback wording now distinguishes elapsed delay from retired/mismatched identity, agreeing with the preserved prior-cut commit and unknown-recovery consumers.

**Atomic retention no longer requires invented admission.** `BatchCommitted` attests batch digest, achieved revision/storage and the complete original staged receipt list. Candidate/security plans require empty acceptances and receipts. QualifiedFork carries explicit independently qualifying pairs and permits genuine historical staged acceptances when needed. `BatchRetained` denotes custody rather than application admission.

The matrix at recovery lines `176–348` declares committed, absent and unknown recovery for all four kinds. It checks reservation release/preservation, once-only charges, existing custody, candidate pending/common-prefix state, security nonconviction, fork quarantine and independent-instance progress. The changed-batch consumer `350–391` rejects digest, revision, storage and receipt-list substitutions. Its loop stops at the scaffold’s first failed assertion; evidence explicitly presents later branches as compiled obligations, not twelve successful demonstrations.

**Historical qualification remains independent of projection quarantine.** Genuine qualified rivals can convict after both leave eligibility; dependent historical custody survives exclusion. Bare unadmitted, historically denied or expired rivals retain security evidence without degrading legitimate AD. Local new intent still requires eligible support. Common-prefix heads stop before qualified fork slots rather than selecting an arrival-dependent branch.

The missing combined journey is now present. Recovery lines `393–495` and text-trace lines `108–238` establish qualifying E0 seq1 rivals, quarantine floor1 and eligible E0 seq0, then attempt fresh E1 local recovery. Both rival orders require exact AB identities, old custody/receipts/retries and reused-E0 rejection. JS additionally requires persistent quarantine, prior projection A and a fresh local receipt. A future implementation refusing all fresh local edits merely because quarantine exists would fail these obligations.

**Real-text RED remains meaningful.** The trace exports only operations selected by an actual kernel `Read`; unavailable reads have no successful-operation fallback. JS uses released `CrdtNode`/`projectText` and requires exactly ten rows: post-fork AB, buffering AB-buffer, AD and A in both orders, ABC and isolation. ABC requires isolated B/C receipts, opposite-direction application-operation offers, equal identity sets and byte-exact pinned corpus payloads/refs. This tests delivered event schedules, not automatic live mesh recovery.

**Authorization, identity and coexistence remain coherent.** Exact immutable operation bytes determine identity. Descriptor/certificate parsed fields must authenticate against their exact evidence bytes. Equal labels cannot merge instances or confer authority. Local finite-window/current-policy checks differ explicitly from historical admission eligibility. Current serving authorization remains a separate host obligation, not a consequence of retained historical validity.

The accepted design’s scoped supersession does not activate generic option B or reinterpret unactivated legacy/strong bindings. Existing wire/node admission is unchanged. Canonical container/domain negotiation, private-principal enforcement, origin custody, mixed-version exclusion, separate roots and whole-store seal interaction remain named prerequisites.

**Architecture and source accounting retain their purpose.** Pure classification fits deterministic owned state and event/effect outputs. Dependencies remain minimal and exclude runtime/storage/merge implementations. Normal selection includes the new package; domain assertions are neither filtered nor inverted. ARCH-002 retains the untouched positive control and exact framework refusal across actual members, with the additional-member regression. Syntax guards inspect disabled inline declarations and associated/foreign/field/variant contexts; macro-opacity and broader migration limits are disclosed.

Gyld preserves inherited sources and checks exact owner/requirement relationships: 24 inherited plus seven new allocations, 107 inherited plus seventeen ICD obligations, and four new journeys. Frozen app-owned inputs avoid mutable sibling runtime fixtures. Allocation accounting does not mark requirements satisfied or expand Gyld’s engine/evaluator.

No new architectural or interface root-cause defect was established. The previous representational and combined-witness counterexamples are addressed on this tuple.

## 3. Risks and next action

The recorded GREEN checks establish representation, refusing-scaffold/source/tooling boundaries and allocation provenance. Behavioral consumers remain intentionally RED. Baseline Mypy errors and narrowly measured timing results remain disclosed limitations.

IC-2 must make the domain consumers GREEN through the actual pure kernel, including the specified rejecting mutant. Physical atomicity/restart, sole-copy loss, cryptographic provenance, policy/clock trust, live duplex transfer, client/editor lifecycle, capacity and compatibility/activation still require their later qualifications. Surface review remains mandatory before a user-facing freeze.

The next action is to complete the lane’s required acceptance records, then begin the gated IC-2 TDD implementation without treating this GO as adapter or activation authority.