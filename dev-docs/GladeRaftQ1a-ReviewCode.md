# Glade Raft Q1a — CODE-AXIS REVIEW

**Review object:** Root diff `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee..5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`; isolated memory-only implementation under `proofs/raft-adoption`, controlled by the DRAFT adoption contract and qualification plan. Q1a acceptance pending; no production activation.

**Baseline:** Root `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through `git show` at these pins. All four HEADs matched at review start and end. Scoped diffs showed no working changes to the proof or checker sources used.

**Date:** 2026-10-03  
**Axis:** Architecture, interfaces, actual call paths and compatibility. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks. I pre-commit to GO on a revision that resolves P2-1 as specified, preserves the readiness protection, and passes the focused regression and existing gates.

---

## 0. Evidence base

Read the canonical Q1a Code prompt, root AGENTS/AGENTS_GWZ rules, review-loop skill and QualificationPlan §6 process binding. Inspected:

- `proof/src/application.rs:1–303`, `cluster.rs:1–271`, `codec.rs:1–268`, public exports, manifests, architecture policy, source checker and gate script.
- `api/src/lib.rs`, including the complete command/outcome types and `CommittedMachine` contract; `api/tests/public_contract.rs:1–73`.
- `proof/tests/qualification.rs:1–596`, including ordering/replay conformance, minority/failover, readiness forgery, intervening mutation, authorization, retirement and mutant schedules.
- QualificationPlan §§1–6, AdoptionContract §§1–4, qualification evidence and review ledger, implementation evaluation and GDL-052.
- BuildEntry; LibraryBoundaryAndTestingPolicy, especially LBT-002/006/009/011; PackageArchitecture; relevant BuyBuildMatrix, DiscoveryModel, WorkspaceDirectory and Authz clauses.
- Pinned Glade SubstrateV1 and CrossNodeWritesPlan; architecture revision 3 and the pinned external Gyld Records/Admission/NodeAssembly allocation.
- Installed `raft-0.7.0` RawNode advance/LightReady implementation and MemStorage commit behavior.

Permitted commands completed successfully:

- Explicit compatible `PROTOC` with `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml`: **21 passing tests**—2 API witnesses, 2 codec tests and 17 behavioral tests.
- `proofs/raft-adoption/check.sh`: architecture PASS, process-global scan of 5 owned Rust sources with zero allowlisted items, conditional-boundary scan PASS and formatting PASS.
- The same `PROTOC` with clippy `--all-targets -- -D warnings`: PASS.

No custom tests, source writes, Git mutations or current-round peer reports were used. P2-1 is a source-derived counterexample; the existing suite does not execute it.

## 1. Findings

### [P2-1] Public exact replay fails for privately applied movement entries

**Location:** `proof/src/application.rs:91–101,137–142,291–297`; actual driver call at `proof/src/cluster.rs:30–39`; public contract at `api/src/lib.rs:125–136`.

**Violated invariant:** `CommittedMachine` requires an exact replay of an applied index to recover its original result. Its public input is `(index, Option<Command>)`. The implementation additionally includes private readiness in replay identity, although public `apply` always supplies `None`.

**Concrete sequence:**

1. A voter applies a valid Create through the driver.
2. The driver commits a valid Move at index `k`. Its decoded private readiness is `Some(Readiness { home, frontier: k - 1 })`; application accepts the Move and retains that envelope alongside its command and receipt.
3. Replay the same `k` and exact same public Command on that Application through `CommittedMachine::apply`.
4. The adapter supplies readiness `None`. Line 98 compares this with the retained `Some(...)` and returns `ConflictingReplay`, rather than the original receipt.

This is a reachable state: successful movement tests exercise precisely the private application path. The counterexample concerns replay of accepted history, not an attempt to forge readiness for a new Move.

**Impact:** The concrete Application violates its declared ordering/replay contract on history produced by its own real driver. The green conformance result does not cover this: `committed_machine_ordering` tests public Create/noop replay, while successful movement calls `apply_committed` directly. The API recorder establishes substitution of types only. A host using the declared replay interface cannot recover every result that this implementation retains.

The evidence candidly describes private readiness and public refusal of unwitnessed movement. That appropriately protects new application, but does not amend the public exact-replay guarantee.

**Required correction:** Preserve the private evidence requirement for first application while making exact public replay return the retained result for the same index and canonical Command. Alternatively, revise the effective committed-input boundary to represent the envelope explicitly, with the applicable contract review and consumer tests. Do not repair this by allowing public numeric frontier fields to mint readiness or by silently weakening replay guarantees.

**Closure regression:** Apply a valid movement entry through the actual private driver/application path, then replay its index and identical Command through `&mut dyn CommittedMachine`; require the original receipt. Also require changed public command content to produce `ConflictingReplay`, and retain the existing tests proving that a new unwitnessed Move cannot activate either a lagging or caught-up successor.

## 2. Invariant analysis

**Actual authoritative data path:** Commands contain the complete bounded numeric payload, not merely ownership metadata or a digest. The private codec retains command identity, action payload and readiness. Committed application updates the authoritative resource state and stores original outcomes before returning receipts. I found no independent production append or external-effect execution behind a quorum preflight.

**Ready ordering:** `Voter::ready` appends entries and retains HardState before released messages enter the delivery queue. Ready committed entries are applied before `advance`; LightReady commit updates preserve term/vote, further committed entries are applied, and application progress is advanced afterward. This agrees with the inspected synchronous RawNode obligations for the named memory profile. No physical durability follows.

**Leader versus home:** Manual leadership changes preserve resource home, generation and payload. Movement is an application transition rather than an inference from Raft term. Minority proposals expose no accepted outcome; proposal admission always reports `Unknown`.

**Retry and disclosure:** Retained outcomes are keyed by the complete fixture request namespace and exact Command equality. Changed retries return a transient conflict without overwriting the original outcome. Exact retries precede current lifecycle/precondition rejection. Client-facing reply/outcome accessors apply current local disclosure permission; trusted lookup is separately documented. Unseen remote revocation remains expressly unqualified.

**Readiness:** Public frontier guesses cannot create the private envelope. Captured readiness binds the exact command and must equal the actual preceding application cut. Intervening committed work invalidates it; lagging successor movement refuses. Under the fixed complete-data configuration, the successor frontier includes policy and retry application. P2-1 is a contract replay defect, not evidence that these new-movement checks fail.

**Scope honesty:** Evidence consistently limits qualification to partial RA witnesses. Crypto, bootstrap, metadata-only voters, two-stage movement, capacity, disk/restart, snapshots, automatic-election RNG compliance, real sinks and legacy activation remain open. The mutant demonstrates the unsafe preflight/local-append counterexample and compares it with the rejecting ordered path; it does not claim production migration proof.

## 3. Risks and next action

The private FIFO harness and manual campaigns establish bounded application/protocol results. They do not establish arbitrary transport schedules, automatic election behavior or production persistence. Existing canonical contracts remain controlling outside the proposed activated profile.

The next action is one scoped regression-first correction for P2-1, followed by the existing proof tests, local gates and focused re-verification. Q1a should remain acceptance-pending until that public replay counterexample closes.
