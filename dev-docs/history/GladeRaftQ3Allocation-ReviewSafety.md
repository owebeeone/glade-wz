# Glade Raft Q3 implementation allocation — SAFETY-AXIS REVIEW

**Review object:** Root diff `e1c260f6cf992f5d890c2ca454e2323b0d8e78b1..ccab267c6b23bfec7471923098944048a7959563`, principally `dev-docs/GladeRaftQ3ImplementationAllocation.md`, DRAFT dated 2026-10-03. Scope is the proposed lifecycle boundary, provider allocation, dependency edges, refusing scaffolds and compiling behavioral specifications before algorithms.

**Baseline:** Workspace root `e1c260f6cf992f5d890c2ca454e2323b0d8e78b1`; unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Root sources were read with `git show ccab267c6b23bfec7471923098944048a7959563:…`; controlling member and Gyld sources were read at their pinned commits.

**Date:** 2026-10-03

**Axis:** Safety: attack destructive lifecycle behavior, incomplete recovery, authority/quorum mistakes, ambiguous publication and misleading qualification claims. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This approves the bounded allocation supplement and compiling specifications on this axis. It does not qualify any Q3 algorithm, storage durability or production activation.

---

## 0. Evidence base

The four HEADs were verified at both review boundaries and matched the exact requested tuple:

- Root: `ccab267c6b23bfec7471923098944048a7959563`.
- Glade: `90dc1a60981185fa26ae5bfafbbb5377c12a413b`.
- Glade-discover: `52ea2d118f45d9e7c3d9a789310dd5d669958851`.
- External Gyld: `ca04499a360d910fbf8ee2540ed446facd051b35`.

Scoped working-tree diffs were empty before execution and at completion. Unrelated dirty member state was not edited or used as controlling specification. No current peer prompt or report was accessed.

Read process authority: root `AGENTS.md`, `AGENTS_GWZ.md`, the review-loop skill and QualificationPlan §6. Read allocation supplement §§1–6; accepted ConfigurationSnapshotContract §§1–7; AdoptionContract requirements RA-001–012 and allocation; PersistenceContract’s lifecycle, publication, rollback and crash-oracle obligations; QualificationPlan; QualificationEvidence; and relevant accepted-through/preparation ledger entries.

Read BuildEntry, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture and architecture revision 3. Checked the pinned Gyld declaration’s Admission, Records, StorageAdapter, NodeAssembly and GladeArchitecture allocations. Pinned Glade SubstrateV1 and Glade-discover LibraryBoundaryChecks supplied canonical context without treating this private experiment as production ratification.

Inspected every changed source/manifests/policy/lockfile entry, including:

- `q3-api/src/lib.rs:196–215`: required dyn-compatible lifecycle.
- `disk/src/v2.rs:1–72`: refusing store and injected-path factory.
- `proof/src/q3.rs:1–70`: refusing session and recovery constructor.
- `q3-spec/tests/common/mod.rs:1–153`: development-only concrete composition and fixture ownership.
- `disk/tests/v2_lifecycle.rs:1–194`: eight lifecycle/fault specifications.
- `q3-spec/tests/configuration_snapshot.rs:1–98`: all nineteen retained consumer cases.
- Accepted shared store, membership, snapshot, joint-exit and typed-replay conformance functions.

Checked cached raft-rs 0.7.0 source directly: `raw_node.rs:371–398,646–712`; `raft.rs:2009–2100,2564–2665,2750–2788`; `storage.rs:242–278`. These substantiate the supplement’s refused-configuration handling, proposal rewriting, snapshot fast-forward/recipient constraints and Ready advancement obligations.

Executed permitted locked/offline commands with the specified cached PROTOC:

- Q3 API: **3 PASS**.
- V2 lifecycle: **8 RED**, all against the deliberately refusing constructors/open operations.
- Q3 configuration/snapshot consumer: **19 RED**, all during concrete provider construction with `NotQualified`; zero ignored or filtered cases.
- Existing application API, durability API, proof and Q2 disk conformance selections: **51 PASS**. The two explicit ignored Q2 disk unit cases and external process worker were not executed.
- `proofs/raft-adoption/check.sh`: **PASS**, including declared architecture boundaries, format checks, token source-boundary scan and process-global inventory of **23 files, zero exceptions**.
- All-target Clippy with warnings denied: **PASS**.

No source writes, Git mutations or process-kill runs were performed.

## 2. Invariant analysis

**Creation cannot silently become recovery or authority.** I attacked missing learner storage as a route to replacement genesis, unauthorized joins reaching filesystem creation, and recovery resetting incompatible files. Supplement §2 explicitly requires host validation before `create`, binds learner creation to a committed existing-group intent, and forbids construction from creating directories or deriving authority from empty paths. `open` is a distinct operation with no create/repair behavior. Invalid instance, exclusive creation, missing/empty/old-format files and wrong binding have concrete RED specifications. A successfully created structural image still requires host semantic validation before participation.

**The lifecycle has a usable ownership boundary.** `StoreLifecycle` returns `Box<dyn CheckpointStore>` without leaking paths, carrier or concrete provider types. The injected factory owns its root; a store owns its held file handle. Restart must drop old handles, physically reopen stores, then validate/replay every required image before constructing serving RawNodes. Loading an old live handle cannot substitute for reopening. Drop supplies the lock-release half; destructive removal/reset is not implicitly required or authorized.

**Rollback detection remains honest.** I considered whether restart could manufacture its trusted minimum revision from the replacement journal. The lifecycle explicitly accepts an independent floor, and the contract forbids deriving it from the same rollbackable files. The valid-old-journal specification requires rejection with the saved external floor and demonstrates admission without it. Neither the supplement nor its evidence claims complete rollback exclusion in the latter case.

**Ambiguous writes cannot authorize continued serving.** The accepted whole-image boundary retains checkpoint, suffix, HardState, configuration and reconstructible application cut together. Pre-write validation/capacity refusal preserves usable prior state; errors after publication begins poison the instance until reopen. The new fault specification distinguishes before-write usability from later poison and tests complete recovery versus partial-record quarantine. Supplement §5 separately requires actual Ready/LightReady failure evidence and actual process termination; synthetic faults are not promoted into SIGKILL or power-loss proof.

**Configuration authority cannot be reduced to availability or metadata.** The supplement preserves complete-data learners, both joint majorities, retained private readiness envelopes and deterministic predecessor-cut validation. It retains the prohibition on applying a refused ConfChangeV2 and explicitly warns that an empty change means leave-joint. Nested/wrong leave must be rejected before the carrier destroys the intent context by rewriting its proposal. Current-home protection remains required at actual joint exit, including queued placement, retained refusal, restart and a separately authorized successful resolution.

**Snapshots cannot replace complete history with plausible state.** Required witnesses preserve full commands, original entries/readiness bytes, typed accepted/refused configuration outcomes and the actual initial noop. Recovery must compare complete replay against materialized resources, reservations, tombstones, policy and configuration. The learner snapshot must contain the learner at its authorized cut; an old snapshot cannot acquire a rewritten ConfState. Actual install, matching-term fast-forward and rejection remain separate required carrier cases.

**Allocation does not widen production authority.** Contract crates retain no concrete dependency. Disk gains only the reviewed normal q3-api edge; the proof consumes the injected API normally while concrete disk remains developmental composition. q3-spec’s concrete providers are dev dependencies used by integration tests. Existing roles and required suites were not weakened, and no third-party dependency or process-global exception was added. Structural PASS is explicitly distinguished from architectural approval and algorithm qualification.

## 3. Risks and next action

The principal residual risk is implementation fidelity. All Q3 behavioral cases currently stop at refusing construction; subsequent assertions have compiled but have not established safety. The old-format byte fixture is explicitly noncanonical, and a real Q2 journal compatibility witness remains mandatory. Sync ordering, semantic adversaries, actual snapshot transfer, physical reopen and externally retained SIGKILL oracles still require implementation evidence.

The next action is for the lane owner to merge the independent allocation verdicts. After GO/GO, proceed with Q3a through actual failing host/adapter regressions before algorithms, then satisfy the complete Q3a/Q3b exit matrix before settled-tuple Code/State acceptance.
