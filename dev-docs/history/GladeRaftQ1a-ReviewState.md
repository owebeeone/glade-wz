# Glade Raft Q1a — STATE-AXIS REVIEW

**Review object:** Root diff `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee..5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`, `proofs/raft-adoption`, GDL-052 and controlling draft `dev-docs/GladeRaftQualificationPlan.md` / `GladeRaftAdoptionContract.md`. Q1a implementation acceptance pending; memory-only profile; no production activation.

**Baseline:** Root `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling sources were read using `git show <pin>:<path>`, not dirty member copies. All four HEADs matched at the start and end. The proof-source working diff was empty.

**Date:** 2026-10-03.

**Axis:** State transitions, memory persistence/application ordering, retry recovery, readiness cuts and honest durability boundaries. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0, P1, P2 or P3 findings. This accepts only the bounded Q1a memory implementation and its stated partial witnesses.

---

## 0. Evidence base

Read the canonical State prompt first, root `AGENTS.md`, `AGENTS_GWZ.md`, the supplied standing control-flow/conditional-compilation instruction and the [review-loop skill](/Users/owebeeone/.claude/skills/review-loop/SKILL.md).

Implementation inspection covered:

- `proof/src/application.rs:1–303`: retained state, contiguous application, index replay, exact retry, permissions, movement, creation and retirement.
- `proof/src/cluster.rs:1–271`: actual `RawNode<MemStorage>` construction, Ready/LightReady handling, message scheduling, partitions, campaigns, proposal/reply distinction and private readiness capture.
- `proof/src/codec.rs`: complete encoding/decoding and both codec tests.
- `api/src/lib.rs`, `api/tests/public_contract.rs`, `proof/tests/qualification.rs:1–596`, manifests, architecture policy, README and local gate sources.
- Downloaded pinned `raft-0.7.0/src/raw_node.rs:88–300,440–709` and `storage.rs:180–235,295–381`, checking adapter obligations against actual dependency behavior.

Controlling-document inspection covered QualificationPlan §§1–6; AdoptionContract §§1–4 and RA-001–012; QualificationEvidence and the Q0 review ledger; ImplementationEvaluation; GDL-052; BuildEntry; LibraryBoundaryAndTestingPolicy; PackageArchitecture; BuyBuildMatrix’s discovery, source-authority and receipt boundaries; DiscoveryModel §§0,3–7; WorkspaceDirectory §§1–4 and WD-8; AuthzModel §§1,3a,3b,4,4a,7a; pinned SubstrateV1 §2 and §6 receipt/write rules; CrossNodeWritesPlan §§2–3; architecture candidate revision 3 and corresponding pinned Gyld Records/Supplier/Runtime allocation; ownership evaluation and its ReviewCycle.

Executed only the authorized commands:

- Compatible `PROTOC` plus `cargo test --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml`: **21 tests passed**—2 API type/substitution witnesses, 2 codec tests and 17 behavioral qualification tests.
- `proofs/raft-adoption/check.sh`: architecture, owned-source process-global scan, conditional-source token scan and formatting **passed**; 5 owned production Rust sources, zero allowlisted items.
- Compatible `PROTOC` plus the specified all-targets Clippy command with `-D warnings`: **passed**.

No custom tests, source writes, Git mutations, production builds or current-round peer reports were used. Historical RED evidence was inspected in the committed ledger/evidence; it was not independently recreated. Recorded cold-build measurements were not rerun.

## 2. Invariant analysis

**Application acceptance follows actual committed history.** `Cluster::propose` serializes the complete command and private readiness envelope into the Raft entry. There is no successful preflight followed by an independent resource append. `Voter::apply` receives committed entries, decodes them and invokes the common application implementation. The authoritative numeric payload resides in that deterministic application; its complete command also resides in replicated log entries. The isolated-old-leader and two-voter either-loss tests exercise missing-quorum refusal without manufacturing acceptance.

**Ready ordering preserves the named memory guarantee.** At `cluster.rs:54–65`, entries are appended and hard state updated before either outgoing-message class is released. Application runs before `advance`; subsequent LightReady entries are applied before the explicit application-progress advance. The LightReady commit update changes only `HardState.commit`, preserving term/vote. Inspection confirms that upstream `MemStorage::commit_to` would also rewrite term, so avoiding that helper is material. No state-changing `step`, `campaign` or `propose` interleaves while a Ready is outstanding. Messages accumulated inside `Voter::ready` become externally deliverable only after it returns.

**Replay cannot silently invent an application prefix.** `application.rs:97–109` rejects changed content at an applied index and any gap before execution. Noops occupy indexes. Terminal outcomes and per-index replay evidence are retained before `apply_committed` returns; applied progress advances last. The direct trait conformance test exercises these obligations through `dyn CommittedMachine`. The recorder’s two API tests are correctly described as type/substitution witnesses, not behavioral application conformance.

**Retry identity survives current-state changes.** At `application.rs:110–135`, retained request identity is resolved before resource generation, retirement, policy or readiness checks. Equality covers the complete fixture command. Changed bytes produce a transient `RetryConflict` without replacing the retained original outcome. Separate principal/scope/incarnation fields prevent namespace aliasing. The failover retry, intentional Create-ID reuse and changed-payload tests exercise this distinction. `reply` tracks the attempted command; `outcome` retrieves the original retained result.

**Successor readiness is checked at the actual cut.** The public `successor_applied` field alone supplies no evidence. Only the harness captures a successor’s complete application frontier and associates its private witness with the exact command (`cluster.rs:235–268`). Application checks successor membership, identity, field/witness agreement and equality to `index - 1` before changing home/generation (`application.rs:202–211`). Thus an intervening committed write or noop invalidates an older observation rather than inventing completeness. Tests cover forged future/current frontiers, successor lag, catch-up and queued intervening mutation. Within this fixed, monotonic memory history, the applied frontier includes policy and retry outcomes. It is not a production readiness certificate.

**Lifecycle and disclosure fences hold within their scope.** Retirement leaves resource identity, names and outcomes retained, rejecting new Create/Mutate/Move while recovering permitted historical retries. Ordered fixture governance changes update the application policy frontier. Both serving accessors check current local disclosure permission; trusted lookup and resource inspection are explicitly different boundaries. Unseen remote revocation is expressly unqualified. Unsupported external effects are terminally rejected without executing a sink.

**Unknown remains unknown.** Proposal admission errors are discarded into `Proposal::Unknown`; absent local outcomes never establish noncommit. Leader observation is documented as a local observation, not a linearizable read or home grant. Leadership changes do not mutate resource home/generation.

The executable preflight mutant demonstrates its claimed unsafe append and contrasts it with ordered rejection. It is a narrow counterexample, not exhaustive protocol exploration.

## 3. Risks and next action

This harness is synchronous, FIFO and memory-retaining. It does not independently qualify arbitrary duplication/reordering schedules, crash interruption, restart, fsync, torn writes, power loss, snapshots or compaction. Memory operations between resource mutation, outcome insertion and history advancement are not a durable transaction. The documents accurately retain those obligations.

Private movement evidence extends the driver/application path; public trait application cannot supply it and refuses new Move commands. That limitation is explicit, and both paths share the ordering/retry implementation. It does not qualify a reusable production movement port.

Numeric authority, fixed genesis/configuration, complete-data voters, unbounded retention, atomic Move and ambient upstream election RNG remain restricted or explicitly unqualified. No full RA requirement or canonical production amendment is falsely closed.

The next action is to file this report and combine the independent acceptance verdicts for this exact tuple, preserving all Q2–Q4 gates.
