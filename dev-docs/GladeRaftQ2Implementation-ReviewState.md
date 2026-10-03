# Glade Raft Q2 implementation — STATE-AXIS REVIEW

**Review object:** Root diff `96bb1b54a420fbf7b8471642fe5c15288940ac11..ac69bbcc325c0946bbf215309bcce5edd3210db6`, covering the Q2 persistence contract and `proofs/raft-adoption` private experiment. Implementation acceptance gate; no production activation.

**Baseline:** Reviewed tuple: workspace root `ac69bbcc325c0946bbf215309bcce5edd3210db6`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. All four HEADs matched at review start and end. Scoped working-tree diffs were empty. Member canonical sources were read with `git show` at their pinned revisions.

**Date:** 2026-10-03

**Axis:** Durable-state semantics, publication ordering, crash adversity and restart legality. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks acceptance. I pre-commit to GO on a revision that resolves P2-1 as specified, with its regression passing and the existing acceptance tiers remaining green.

---

## 0. Evidence base

Read root `AGENTS.md`, `AGENTS_GWZ.md` and the bound review-loop skill. Read BuildEntry, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, AdoptionContract §§1–4, QualificationPlan §§1–6, PersistenceContract lines 1–130, QualificationEvidence’s Q2 RED/GREEN and implementation sections, and the qualification ledger.

Checked architectural allocation against `arch1/GladeArchitecture.md`, the pinned Gyld declaration’s Records, StorageAdapter and NodeAssembly boundaries, and the cited canonical substrate/cross-node write documents. Inspected the buy/build and discovery distinctions relevant to the experiment’s scope. These sources do not authorize production activation.

Inspected:

- `durability-api/src/lib.rs` and its model/conformance consumer.
- `disk/src/lib.rs` lines 28–184; all of `codec.rs`, `journal.rs` and `validation.rs`; all twelve disk conformance cases.
- `proof/src/voter.rs` lines 17–147, `recovery.rs` lines 9–73 and `cluster.rs` lines 118–409.
- Private command codec and application replay behavior, all twelve recovery cases, and the explicit real LightReady test.
- `process-crash.py` and its Rust worker, including complete receipt encoding, externally retained originals and both kill cuts.
- Manifests, architecture policy, source checker and empty process-global allowlist.
- Local pinned raft-rs 0.7.0 source: `raft.rs` candidate/campaign transitions, particularly lines 1145–1155 and 1261–1269; `config.rs`’s default `pre_vote: false`.

Executed the permitted locked/offline Cargo workspace test command with the specified PROTOC: **48 tests passed**, with LightReady and process-worker cases correctly ignored. Separately executed `q2_real_light_ready -- --ignored`: **one passed**. Python oracle self-tests: **four passed**. External worker runner: **both actual SIGKILL/fresh-process cycles passed**, at acknowledged receipt and durable commit before application.

`proofs/raft-adoption/check.sh` passed architecture, source-boundary, format and process-global checks: thirteen owned source files, zero allowlisted entries. All-target Clippy with warnings denied passed.

No custom regression source was written under this read-only mandate. P2-1 is a source-derived counterexample; the existing tests do not execute its predecessor-state transition.

## 1. Findings

### [P2-1] A legal campaign creates durable state that the recovery constructor refuses

**Location:** `proof/src/cluster.rs:234–241`, `proof/src/recovery.rs:14–16`, `proof/src/voter.rs:66–105`; supporting disk acceptance in `disk/src/validation.rs:10–45`.

**Root cause:** Live transition admission and restart admission disagree at the maximum term. Campaign refuses only when the **current** term already equals `u64::MAX`. Recovery refuses every image whose term equals that value. Consequently, a permitted campaign from `u64::MAX - 1` enters and persists a state outside the recovery grammar.

**Violated invariant:** RP-009/010 require coherent reconstruction of retained committed history; RP-011 requires acknowledged outcomes to survive the named process-crash/restart profile. A successful live transition must not manufacture a subsequently unrecoverable history merely because its term crossed an admission boundary.

**Reproduction sequence:**

1. Create three explicit genesis stores with their valid fixed bindings.
2. Using the public store boundary, persist otherwise valid empty images with term `u64::MAX - 1`, vote zero and commit zero. Disk validation accepts this monotonic transition; no checksum forgery or malformed history is required.
3. Recover the cluster. `recovery::entries` accepts that term.
4. Campaign voter 1. The current-term guard permits it. raft-rs `become_candidate` computes `self.term + 1`, which is representable and becomes `u64::MAX`.
5. Drain the normal controlled schedule. The candidate and followers persist this term before vote messages. Nothing in `persist_ready`, disk image validation or ordinary message stepping rejects it. The election can complete and the leader can commit a valid Create or mutation and return its applied receipt.
6. Drop or kill the process and reopen the three files. Disk recovery accepts the valid journals, but `Cluster::recover` immediately returns `CapacityExhausted` from `recovery::entries`.

Even without another mutation, the campaign can make previously retained outcomes inaccessible through the recovered cluster. This is a new stuck state created by the host’s own accepted transition, rather than an unavoidable overflow operation.

**Impact:** A legal live execution can acknowledge an outcome whose intact, synchronized supporting history cannot restart through the promised host boundary. The edge requires an extreme term, so I classify it P2 rather than a likely destructive release failure. The explicit capacity work brings this boundary into scope; deferring comprehensive production capacity qualification does not excuse this inconsistent restart grammar.

**Required correction:** Make live publication and startup admission agree. Either prevent transitions into the reserved unrecoverable term before publication/message release, or safely recover retained outcomes at that term while refusing operations requiring another increment. Preserve prior committed history and avoid resetting or rewriting terms.

**Closure test:** Add an observed-RED actual-disk regression starting at `u64::MAX - 1`. Exercise campaign and the normal driver, then reopen all stores. Assert either explicit refusal before an unrecoverable publication, with prior receipts still recoverable after restart, or successful recovery of the complete newly acknowledged receipt. Cover both reviewed fixed configurations and retain the existing exact-MAX refusal tests. The current tests cover the destination state directly and therefore miss this transition.

## 2. Invariant analysis

Publication ordering otherwise survived the attacks. The disk adapter validates before writing, poisons before entering publication, and clears poison only after `sync_all` succeeds. Errors after complete writes remain unknown. Reopen validates every record and resynchronizes the file and parent before admission. Exclusive OS locking covers recovery and the store lifetime; explicit creation cannot reset an existing empty or corrupt file.

Recovery consumes the entire journal. Partial tails, malformed lengths/counts, wrong versions, revision gaps, changed bindings and conflicting committed bytes quarantine without repair. The parser bounds each allocation. The externally trusted revision floor rejects a valid older journal, while the documented no-floor limitation remains honest.

Committed entries remain immutable; uncommitted suffix replacement and shortening remain legal. The actual restart/failover test demonstrates real Raft reconciliation without applying the isolated leader’s abandoned mutation.

Ready persistence precedes the memory mirror and message release. LightReady commit-only persistence preserves term/vote and precedes application. The separately executed real LightReady failure case returns no messages or new outcome. Leader/follower storage failures stop participation and receipt serving.

Startup validates serialized entry framing and private command grammar across committed and uncommitted entries, compares common committed prefixes, reconstructs policy, tombstones, movement fences and exact outcomes, and restores `Config.applied`. The process oracle compares complete recovered lookup and retry receipts against an original retained outside the killed worker.

## 3. Risks and next action

Actual termination evidence covers two declared SIGKILL cuts. Synthetic write faults supplement it; neither establishes power-loss certification or exhaustive syscall-interleaving coverage. Unbounded journal retention, snapshots, membership, independent failure domains, genuine authentication, transport, automatic-election RNG and production legacy/effect exclusion remain explicitly open.

The next action is one scoped TDD correction for P2-1, followed by focused State closure verification and rerunning the existing Q2 acceptance tiers on a newly settled tuple.
