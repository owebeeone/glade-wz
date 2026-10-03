# Q4-A internal whole legacy Store retirement interlock Contract — SAFETY-AXIS REVIEW

**Review object:** DRAFT contract/specification at workspace root `6a35216a6d97aa22e9d53b54256f7395c306da3c`: `dev-docs/GladeRaftProductionIntegrationPlan.md`, `GladeRaftLegacyStoreSealContract.md`, and `GladeRaftQ4ASealEvidence.md`; refusing scaffold and compiling consumers at Glade `19a269dd12b5f109d03e2361e3af4108d4b1fcbc`. Contract-stage review only; no implementation acceptance.

**Baseline:** Root `55b6ee30bb18fa58c80805be2d29a44b404077f6`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`. Discovery remained `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld remained `ca04499a360d910fbf8ee2540ed446facd051b35`. Scoped sources were read using `git show <exact SHA>:<path>` and baseline-to-reviewed-SHA diffs.

**Date:** 2026-10-03

**Axis:** Safety: attack mutation exclusion, irreversible publication, degraded recovery, mixed versions, scope isolation, and evidence honesty. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2, or P3 findings against this contract-stage object. This verdict accepts the proposed internal contract and compiling behavioral specification, not a working seal or production migration.

---

## 0. Evidence base

The required four-repository tuple matched at the beginning and end. No source/report files or Git state were changed. Only permitted inspection and test/gate commands were used.

Read process authority: root `AGENTS.md`, `AGENTS_GWZ.md`, `glade/AGENTS.md`, and `/Users/owebeeone/.claude/skills/review-loop/SKILL.md`. Read pinned controlling documents:

- `GladeRaftLegacyStoreSealContract.md:1–93`, including LS-001–007, publication mechanics, platform limits, and deferred implementation witnesses.
- `GladeRaftProductionIntegrationPlan.md:1–100` and `GladeRaftQ4ASealEvidence.md:1–30`.
- `GladeBuildEntry.md:1–88`, `LibraryBoundaryAndTestingPolicy.md:1–108`, and `GladePackageArchitecture.md:1–160`.
- `GladeRaftAdoptionContract.md:1–97`, particularly RA-003–012 and proposed canonical reconciliation.
- `GladeRaftQualificationPlan.md:1–183`, including the local process binding in §6.
- The carrier and boundary source audits, treated as factual inputs with their stated limitations, not acceptance reviews.

Inspected pinned Glade `node/src/store.rs`, especially open/replay/rename/rewrite at lines 185–268; scaffold at 271–278; append classification, proof publication, and checkpoint rewrite at 285–403; cached read methods at 406–481; journal/proof I/O and torn-tail repair at 660–750; and existing Store tests. Read `node/tests/legacy_store_seal.rs:1–202`, including the disabled non-Unix module. Checked `Server::open` at `server.rs:90–111` and both composition roots’ calls to it. Read the existing node manifest and architecture policy. The scoped member diff adds only the refusing operation and its consumer target.

Read pinned external Gyld allocations: Records at lines 339–362, StorageAdapter at 473–484, and NodeAssembly at 558–608, with the related profile/host-port descriptions.

Executed:

- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --test legacy_store_seal`: compiled and ran seven tests; **one passed, six failed**, zero ignored or filtered. Exit 101 was behavioral RED. The successful-seal cases failed on the deliberate Unsupported result; marker cases observed `Ok(Appended)`; sealed torn-journal open proceeded.
- `cargo run --quiet --locked --offline --manifest-path glade-discover/tools/architecture-check/Cargo.toml -- glade/node`: **PASS** for the gate’s declared coverage.
- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib store::`: initially 15 passed and seven failed with duplicate/missing fixture contents. The unchanged fixture helper uses fixed temporary directory names (`store.rs:838–841`); overlapping execution was a plausible explanation, not established from peer testimony. A subsequent identical run passed **all 22 tests**, with 337 filtered and execution reported as 0.04 seconds. The initial result is retained here rather than silently omitted.

No non-Unix execution, actual seal publication, lock race, injected sync fault, or process-kill witness was performed. Those cannot be inferred from this scaffold.

## 2. Invariant analysis

**One physical root is retired, without widening authority.** The operation owns the whole existing Store root, not a share, zone, generation, or protected identity. This is explicit at contract lines 15–26 and 66–77. Whole-store scope also stops HOME gossip; the text requires draining the selected migration target and prohibits automatic installation on the running desk. The independent-root consumer gives the intended isolation observable behavior. No scoped edit introduces genesis, authenticated evidence, declaration mutation, protected receipts, or an activation path.

**The mutation fence covers the dangerous branches.** Existing append can write a new journal record, persist an equivocation/rewrite proof, or rewrite a HOME journal following checkpoint placement. Existing open can trim a torn journal, rename unverified HOME journals, repair proof tails through shared decoding, and rewrite covered history. LS-002, LS-003, and LS-005 jointly require fence inspection before these effects and one root-specific lock held through the entire operation. A lock confined to the ordinary append write, or a marker check followed by independent mutation, would violate the proposed contract.

The duplicate branch is deliberately fenced too: its absence of a journal write does not make it a permitted successful legacy admission after retirement. The existing-handle consumer requests a duplicate and fork after seal and checks retained bytes and absent proof creation.

**Independent handles cannot rely on a cached fence decision.** Each operation must reopen and exclusively acquire the same stable lock file before inspecting the marker. Thus a handle opened before another handle seals cannot justify a later append using its original open result. The explicit LS-003 race witness must demonstrate the entire critical section; merely showing two sequential handles is insufficient. Cached content remains inspectable, but the contract explicitly withholds freshness and readiness claims. It does not promise general cache coherence between independently opened unsealed Stores.

**Marker ambiguity closes admission.** `symlink_metadata` establishes absence only through NotFound. Corrupt bytes, a directory, and a dangling symlink are therefore fences, rather than parsing failures that restore legacy behavior. Existing regular markers may be resynced without truncation; unexpected kinds remain fenced while seal returns an error. The marker fixtures express these attacks, and the torn-tail consumer exercises a real repair-capable open path.

**Publication has a monotonic failure direction.** Marker creation precedes marker sync and containing-directory sync, with the operation lock retained throughout. Failures after creation may leave the root irreversibly sealed; the text does not promise rollback or known noncommit. Retry completes sync of the retained marker and directory without deleting or replacing application data. This prevents a cleanup-on-error interpretation from reopening admission. A missing typed commit-status result is not unsafe here because callers receive no contractual noncommit assurance from an error.

**Unsupported platforms cannot manufacture successful durability.** Non-Unix publication must refuse before mutating the fence until separately qualified, while existing-fence recognition remains cross-platform. The disabled platform branch contains an explicit Unsupported/no-publication consumer. This review establishes its source shape, not execution or cross-platform implementation correctness. Conditional sections are enclosed by modules, including disabled branches; the scoped scaffold introduces no process-global state or dependency.

**The broader production contract remains blocking.** Advisory locking cannot stop an old unaware binary, separate Registry snapshots, external effects, or raw replacement of the lock/marker. These limitations are stated directly. Stable-root assumptions bound this local interlock; they do not amend RA-012’s complete writer exclusion and rollback obligation. The production plan retains baseline reconciliation, old-binary exclusion, canonical authority, real receipts, and independent failure domains as separate gates. Its carrier/profile recommendations remain proposals.

**The RED evidence is honest and satisfiable.** The signature compiles and always refuses. The observed six failures reproduce the claimed missing behavior without treating compiler errors as RED. LS-003/006 witnesses must precede their implementations. Existing Store lifecycle ownership is meaningful within the already classified integration package; no new abstraction or policy relaxation is smuggled into the contract.

## 3. Risks and next action

The contract still needs actual OS-lock behavior, controlled interleavings, create/sync/directory-sync faults, process interruption, byte preservation across every recovery branch, and relevant consumer evidence. The non-Unix recognition requirement also needs verification beyond source inspection. Architecture PASS supplies structural evidence only.

The single next action is to obtain the other independent contract verdict, then—only if both are GO—add the promised LS-003/006 behavioral RED witnesses before implementation and carry the completed adapter through its separate Code/State acceptance gate. No desk seal, startup installation, migration cut, or production activation follows from this GO.