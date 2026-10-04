# Q4-A legacy Store retirement interlock contract — CONSISTENCY-AXIS REVIEW

**Review object:** DRAFT contract/specification checkpoint at workspace root `6a35216a6d97aa22e9d53b54256f7395c306da3c` and Glade `19a269dd12b5f109d03e2361e3af4108d4b1fcbc`: `dev-docs/GladeRaftProductionIntegrationPlan.md`, `GladeRaftLegacyStoreSealContract.md`, `GladeRaftQ4ASealEvidence.md`, `glade/node/src/store.rs`, and `node/tests/legacy_store_seal.rs`. Compiling behavioral RED; no implementation acceptance.

**Baseline:** Root `55b6ee30bb18fa58c80805be2d29a44b404077f6`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`. Sources were read through `git show` at the exact supplied SHAs; scoped diffs were inspected against these baselines. Discovery remained `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld remained `ca04499a360d910fbf8ee2540ed446facd051b35`.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling document graph, allocation, executable consumer specifications, and evidence claims. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, or P2 findings; one nonblocking P3 coverage finding. This verdict accepts the contract/specification checkpoint only.

---

## 0. Evidence base

All four tuple entries matched at both the beginning and end of this review.

Read authority: root `AGENTS.md`, `AGENTS_GWZ.md`, `glade/AGENTS.md`, the review-loop skill, and qualification plan §6.

Read the exact root sources:

- Seal contract lines 1–93, production integration plan lines 1–100, and seal evidence lines 1–30.
- Build entry lines 1–88; Library Boundary and Testing Policy, especially LBT-006–011 and §§4–6; Package Architecture §§1–2, 4–8.
- Raft adoption contract lines 1–97 and qualification plan lines 1–183.
- Both Q4 factual audits, retaining their stated distinction between source observations, proposals, and acceptance evidence.
- Supporting architecture notes, particularly Records, physical storage, assembly, and profile allocation.

Read external Gyld at its pinned SHA: declaration introduction and Admission/RecordHost/Profile ports, Records lines 339–362, StorageAdapter lines 473–484, and NodeAssembly construction lines 558–608.

Inspected the complete scoped member diff. The only Store implementation change is the deliberately refusing `seal_legacy` signature. Examined Store replay/rename, append classification, checkpoint rewrite, proof persistence, ordinary journal append, and torn-tail repair, including `store.rs:185–403,440–550,610–750`. Read all 202 lines of the new consumer target, including its disabled non-Unix module. Verified `Server::open` delegates to Store and checked its use by both composition roots. Checked material compatibility passages in SubstrateV1 and WorkspaceDirectory. Spot-checked the carrier audit’s ambient entropy/runtime/clock claims against the available pinned dependency sources.

Executed only permitted commands:

- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --test legacy_store_seal`: compiled and ran seven tests; one passed, six failed; no ignored or filtered tests. Cargo reported 0.33 seconds; execution reported 0.00 seconds.
- `cargo run --quiet --locked --offline --manifest-path glade-discover/tools/architecture-check/Cargo.toml -- glade/node`: PASS.
- `cargo test --locked --offline --manifest-path glade/node/Cargo.toml --lib store::`: initial run produced fifteen passes and seven failures involving duplicate retained records or disappearing fixture paths. Existing fixtures use shared fixed temporary paths (`store.rs:838–841`). A repeat of the same permitted command passed all 22 tests, with 337 unrelated tests filtered out; Cargo reported 0.16 seconds and execution 0.04 seconds. The initial result is recorded, not silently counted green; no reproducible scoped regression was established.

No sources or reports were written. No implementation-phase check suite or unlisted tests were run.

## 1. Findings

### [P3-1] Non-Unix consumers omit the cross-platform fence-recognition contract

**Location:** `glade/node/tests/legacy_store_seal.rs:2–3,107–185,188–202`; seal contract lines 33 and 51–54.

**Violated invariant:** Publication may be Unsupported on non-Unix, but recognition and refusal of an existing fence must remain cross-platform. LBT-007 requires behavioral coverage of failure and edge conditions.

**Reproduction:** Under `cfg(not(unix))`, every marker-recognition consumer is excluded. The sole remaining consumer opens an unsealed root, expects `seal_legacy` to return Unsupported, and checks that no marker was published. An implementation could leave non-Unix `open` and `append` completely unaware of existing markers while passing every consumer selected on that platform. The current refusing scaffold already satisfies that lone consumer.

**Impact:** The executable specification protects publication refusal but does not protect the separate compatibility obligation to recognize a retained fence. A later platform-specific implementation or conditional-compilation change could reopen admission without that target detecting it.

**Required correction:** Put platform-independent existing-marker consumers in a shared enclosing module. Exercise regular markers with empty and unknown contents, directory markers, fresh open, and append through a pre-opened handle. Keep successful publication tests Unix-specific and retain the non-Unix Unsupported/no-publication consumer.

**Closure test:** The shared recognition cases must compile and demonstrate behavioral RED against marker-unaware open/append before those paths are implemented. On a qualified non-Unix runner, they must subsequently pass while publication still returns Unsupported without creating a marker. Assert retained journal bytes and absence of new proof writes.

This is a bounded coverage defect, not a claim that the present RED scaffold should already implement sealing or that non-Unix publication is qualified.

## 2. Invariant analysis

**Whole-root scope is explicit and internally consistent.** The contract seals a physical legacy Store instance, including HOME journals. It does not pretend to select one application binding inside journals shared by `(share, origin)`. Draining and explicit target selection are required; automatic desk/startup installation is prohibited. Separate-root behavior is specified and has a compiling success consumer. Cached reads are explicitly stale observations, not baseline or readiness evidence.

**The mutation boundary covers the actual Store paths.** Open can trim torn records, rename unverified HOME journals, and rewrite checkpoint-covered history. Append can write ordinary journals, persist equivocation/rewrite proofs, or rewrite HOME journals. LS-002/003/005 require the shared root lock before fence inspection and through those effects. That excludes the dangerous check-then-independent-append sequence. Duplicate and fork consumers require admission refusal before otherwise idempotent acceptance or proof persistence. No public Store mutation path outside open/append/seal was found.

**Publication and uncertain outcomes are coherent.** Marker presence has meaning independent of its bytes. `symlink_metadata` avoids treating a dangling marker as absent. Existing regular markers are resynced without truncation; unexpected kinds remain fenced. Success requires marker sync followed by directory sync while retaining the lock. Failed publication cannot delete the marker or promise rollback. Retrying through an existing handle is compatible with fresh open refusing that same root.

**The profile does not silently satisfy RA-012.** Participating-build advisory locking cannot stop unaware old binaries. External replacement, rollback restoration, separate Registry snapshots, external effects, divergent histories, and baseline completeness remain outside this interlock’s guarantee and remain explicit Q4 activation blockers. The production plan preserves those obligations in Q4-E rather than weakening the adoption contract.

**Allocation remains meaningful without inventing a new library.** The seal is a journal-lifecycle operation inside the existing integration package. Gyld’s proposed Records/StorageAdapter/NodeAssembly allocation is used as architectural context; the change does not claim that those proposed extractions have been implemented. No dependency, classification, exception, declaration, or proof-harness import changes occur in the scoped diff.

**The RED evidence is honest.** The observed failures match the evidence: unsupported requested success, marker-unaware append returning `Ok(Appended)`, and sealed open proceeding into replay. These are the intended contract-phase counterexamples. LS-003/006 race/publication/process-interruption witnesses are expressly still required RED before their implementations; they are not represented as already passing or executed.

No superseded canonical clauses are claimed by Q4-A. Existing receipt meanings, genesis authority, and protected acceptance remain intact. The proposed production profile remains an owner-selection proposal. The broad implementation-authorized status is bounded by the specific dual-GO-before-implementation gate.

## 3. Risks and next action

The draft establishes a satisfiable local interlock contract, not operational migration safety. Its decisive remaining implementation evidence is real locking across independent handles, interruption and I/O failures at publication boundaries, refusal before all repair/rewrite paths, retained bytes, and affected-consumer verification.

The inherited Store unit fixtures can interfere across concurrent command invocations; the initial failure and clean retry should remain distinguishable in evidence. No broader fixture migration is accepted here.

The next action is to complete the remaining review gate, then add the required adversarial RED witnesses before implementation, including P3-1’s shared marker-recognition consumers. Implementation must later receive its own Code/State acceptance review.