# Glade Raft Q3 learner bootstrap clarification — SAFETY-AXIS REVIEW

**Review object:** Root diff `c6beda7b2c94fef4060f7bc9a069b876fd2aa7d9..d4589feb02ad86b92f58686da35c8a216c370c44`: bounded amendment to `dev-docs/GladeRaftQ3ImplementationAllocation.md`, `GladeRaftQ3Allocation-RemPlan-1.md`, carrier-only `proof/tests/q3_seeded_snapshot.rs`, and the qualification ledger. DRAFT clarification pending renewed review; no algorithm acceptance.

**Baseline:** Root `c6beda7b2c94fef4060f7bc9a069b876fd2aa7d9`; unchanged Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Root controls were read using `git show d4589feb02ad86b92f58686da35c8a216c370c44:…`; Gyld allocation was read at its pinned commit.

**Date:** 2026-10-03

**Axis:** Safety: attack bootstrap authority, premature participation, destructive recovery, snapshot satisfiability and false qualification. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This approves the bounded clarification on this axis, subject to the unchanged implementation gates.

---

## 0. Evidence base

All four HEADs matched the exact requested tuple at both review boundaries:

- Root: `d4589feb02ad86b92f58686da35c8a216c370c44`.
- Glade: `90dc1a60981185fa26ae5bfafbbb5377c12a413b`.
- Glade-discover: `52ea2d118f45d9e7c3d9a789310dd5d669958851`.
- External Gyld: `ca04499a360d910fbf8ee2540ed446facd051b35`.

Read root AGENTS instructions, `AGENTS_GWZ.md`, the review-loop skill and QualificationPlan §6. Read the exact scoped diff and these pinned controls:

- Allocation supplement, especially lines 24–72 and its complete exit obligations.
- RemPlan-1, lines 3–16.
- Carrier regression, lines 1–55.
- ConfigurationSnapshotContract §§1–7, especially lifecycle/recovery lines 17–27, learner/joint authority lines 35–43, and snapshot ordering lines 47–65.
- AdoptionContract RA-001–012; PersistenceContract lifecycle, publication, recovery, rollback and crash-oracle requirements.
- QualificationPlan, QualificationEvidence and ledger allocation/bootstrap entries, including ledger lines 305–363.
- BuildEntry, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, architecture revision 3, and pinned Gyld Admission/Records/StorageAdapter/NodeAssembly allocations.
- Pinned q3-api values and lifecycle declarations; refusing V2/Q3 provider scaffolds; development composition; lifecycle specifications; manifests and architecture inventory.

The prior allocation Safety report was a legitimate prior-round input. No current peer prompt or report was accessed.

Inspected cached raft-rs 0.7.0 directly: `raft.rs:1350–1508,2520–2665`, `storage.rs:230–282,410–475`, and `raw_node.rs:115–175,463–535`. These establish vote handling, snapshot recipient checks, matching-term fast-forward, MemStorage seeding and how an unstable snapshot becomes Ready.

No test, build, formatter, architecture gate or process-kill command was run. RemPlan-1 records the original behavioral RED and the corrected carrier regression’s one PASS, including an owner rerun. I inspected that evidence and its pinned source; these are recorded execution results, not my independent execution.

All uncommitted Q3 algorithms were excluded. At completion, the scoped working-tree diff showed an uncommitted initializer rewrite in `q3_seeded_snapshot.rs`; only its committed bytes were reviewed. No verdict depends on current implementation state.

## 2. Invariant analysis

**The original counterexample has a bounded root cause.** The former allocation precondition required a learner image already containing the applied join cut while the combined witness required actual incoming snapshot installation at that same cut. In the regression, MemStorage seeding installs index 5/term 1 and commit 5. Incoming snapshot 5/1 reaches the carrier’s matching-term fast-forward path, which returns false without installing an unstable snapshot. Consequently, Ready has no received snapshot at 5. The original-genesis receiver lacks that cut and takes actual restoration instead.

I classify this as one bounded allocation/consumer satisfiability defect, addressed by changing the permitted bootstrap state. The changed mutation boundary warrants this fresh contract gate. This review finds no additional independent architectural root cause and does not reopen the accepted semantic design.

**An empty image does not become group authority.** I attacked creation from an absent path, a discovery observation, an unauthorized intent and an uncommitted AddLearner. Allocation lines 39–41 require retained, committed, accepted existing-group admission before invoking the factory. The zero-cut image is evidence of local incompleteness, not evidence granting genesis, membership or replacement authority. Its exact original authorized ConfState also avoids attempting to reconstruct the group by applying only AddLearner to empty configuration.

**Private construction does not grant participation.** The amendment explicitly blocks campaign, voting, quorum/home readiness, serving and outcomes until validated restoration. This protection must be enforced by the host: the carrier’s vote-request path can record votes, so constructing this RawNode alone does not establish nonvoting behavior. That is an implementation obligation, not an omitted document invariant. The carrier regression clearly disclaims authorization and application qualification.

Restoring the join configuration also does not independently authorize promotion. The controlling contract still requires actual complete durable/applied catch-up, a retained private readiness envelope and deterministic verification against the true predecessor cut. A queued application command can invalidate that readiness. Neither the new initial image nor externally observed join counters can substitute for it.

**Restart cannot erase incomplete or retained history.** I considered interruption after exclusive creation, during catch-up and after an ambiguous publication error. The amendment requires recovering existing-group admission from retained history and reopening the owned path. Failure to establish admission blocks participation. Allocation line 45 preserves physical reopen, independent floors and refusal of missing, empty, corrupt, incompatible or wrong-bound required storage. The allowance for an explicitly new learner path does not authorize resetting an existing learner file.

**Full-log and snapshot restoration remain distinct legal paths.** Original-log replay can reconstruct the actual genesis, noop, configuration and application history. Snapshot catch-up requires a validated checkpoint containing the recipient at its real authorized cut. A cached snapshot excluding learner 4 cannot acquire a rewritten ConfState. Seeded initialization remains permitted but cannot count as same-cut actual transfer. The combined witness must receive the genuine learner-add snapshot and apply a later original suffix.

**Publication and recovery guarantees remain intact.** The clarification changes initial bootstrap state, not coherent V2 publication. Complete checkpoint, HardState, suffix, configuration and reconstructible application cut still publish together before candidate exposure, messages or receipts. Post-write errors poison; partial tails quarantine; committed history and same-term votes remain immutable; legitimate uncommitted suffix replacement remains legal. Full semantic replay, current disclosure checks and cross-store comparison are unchanged.

**Scope remains contained.** Both joint majorities, explicit leave, current-home checks, retained refusals and removal-independent resource identity remain required. No trait, dependency edge, role classification, schema, authority universe, process-global exception or production profile changes. The std-only injected lifecycle retains separate create/open operations and held-handle ownership. Carrier-only opaque bytes are not presented as complete application state, disk durability or SIGKILL evidence.

## 3. Risks and next action

The residual risk is implementation fidelity, especially enforcing the private follower gate before vote/pre-vote processing and restoring that gate after interruption. The current regression demonstrates carrier satisfiability only. Host admission/no-create-on-denial, restart without reset, complete application restoration, readiness, actual snapshot-plus-suffix, fault and process-kill witnesses remain mandatory before Q3 acceptance.

The next action is to merge the independent verdicts for this pinned clarification. After GO/GO, the changed learner rule may proceed through the authorized TDD implementation work; complete settled-tuple Code/State qualification remains required.
