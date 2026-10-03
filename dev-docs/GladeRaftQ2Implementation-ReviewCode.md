# Glade Raft Q2 Implementation — CODE-AXIS REVIEW

**Review object:** Root diff `96bb1b54a420fbf7b8471642fe5c15288940ac11..ac69bbcc325c0946bbf215309bcce5edd3210db6`, Q2 persistence contract and `proofs/raft-adoption`. Private experimental implementation gate; no production activation or contract ratification.

**Baseline:** Workspace root `ac69bbcc325c0946bbf215309bcce5edd3210db6`; Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. All four HEADs matched at start and end. Controlling root sources were read through pinned `git show` and matching local files; member canonical sources were read from their pinned commits. Scoped working-tree diff was empty before and after verification.

**Date:** 2026-10-03

**Axis:** Architecture, interfaces, call graphs, compatibility, error paths and implementation fidelity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks acceptance. I pre-commit to GO on a revision that resolves P2-1 as specified, preserves the reviewed boundaries, and passes the focused closure checks.

---

## 0. Evidence base

Read root instructions, `AGENTS_GWZ.md`, the review-loop skill, QualificationPlan §§1–6, PersistenceContract §§1–7 and implementation amendments, AdoptionContract §§1–4, QualificationEvidence’s Q2 RED/GREEN and measured-feedback sections, and the qualification review ledger. Checked BuildEntry, LibraryBoundaryAndTestingPolicy, PackageArchitecture and architecture candidate revision 3 against the pinned external Gyld allocation.

Inspected the canonical reconciliation clauses relevant to this experimental scope: BuyBuild D-06/R7/R9/R16, WorkspaceDirectory §4/WD-8, DiscoveryModel §§0–2, Authz ownership and signed-governance clauses, and pinned Glade SubstrateV1 R1/R2/R7/W1/W2/W7 plus CrossNodeWrites’ holder-routing model. The proof does not modify or replace these retained production paths.

Code inspection covered:

- `durability-api/src/lib.rs:1–158` and its model consumer.
- `disk/src/lib.rs:1–184`, `codec.rs:1–209`, `journal.rs:1–39`, `validation.rs:1–46`, and disk conformance tests.
- `proof/src/cluster.rs:118–409`, `voter.rs:1–148`, `recovery.rs:1–73`, private command codec and the application replay path.
- `proof/src/cluster/light_ready_tests.rs:1–161`, recovery integration tests, `proof/tests/process_crash.rs:1–167`, and `process-crash.py:1–146`.
- Manifests, lockfile changes, architecture inventory, source check, process-global allowlist and README tier commands.
- Locally cached pinned `raft 0.7.0` API/source, especially `raw_node.rs:470–478` and `649–700`, and its documented asynchronous Ready lifecycle.

Executed only permitted commands, using the specified PROTOC where applicable:

| Check | Result |
|---|---|
| Locked/offline proof workspace tests | 48 passed; LightReady and external worker intentionally ignored |
| Explicit ignored `q2_real_light_ready` tier | 1 passed |
| `proofs/raft-adoption/check.sh` | Architecture, process-global, conditional-boundary and formatting checks passed |
| All-target Clippy with `-D warnings` | Passed |
| `process-crash.py --self-test` | Four passed |
| `process-crash.py --worker …/process_crash-f51f7c71ab7ad8b0` | Both SIGKILL/fresh-process cycles passed |

No source edits or Git mutations were performed. Test writes were confined to authorized disposable/ignored artifacts.

## 1. Findings

### [P2-1] LightReady witness changes RawNode state before advancing its outstanding Ready

**Location:** `proofs/raft-adoption/proof/src/cluster/light_ready_tests.rs:102–135`; corresponding legal-schedule claim in `dev-docs/GladeRaftQualificationEvidence.md:227–231` and exercised-boundary description in `GladeRaftPersistenceContract.md:118–124`.

**Violated invariant:** RA-011 requires persisted state to obey the selected adapter’s ordering. RP-008 and the implementation gate require an actual conforming Ready/LightReady witness. The pinned carrier’s `RawNode::ready` documentation at `raft-0.7.0/src/raw_node.rs:473–475` explicitly requires returning Ready through an advance-family operation before calling state-changing functions such as `step`, `propose` or `campaign`.

**Reproduction:** Execute the permitted ignored LightReady test; it passes. Inspect its passing sequence: line 102 obtains the leader’s Ready. Line 107 persists its data and memory mirror, but does not advance that Ready. Lines 117–129 deliver appends to followers and call `leader.raft.step(response)` for their acknowledgements. Only line 135 returns the original Ready through `finish_ready`, which calls `advance_append`.

Thus the test produces its commit-only LightReady using a sequence explicitly prohibited by the public carrier contract. Persisting the storage mirror does not complete the outstanding Ready lifecycle.

**Impact:** This invalidates the claimed legal LightReady qualification. The assertions establish behavior for an unsupported carrier interleaving; they cannot establish RP-008 under a conforming host schedule. The defect is in the witness and its evidence claim. I found no corresponding illegal interleaving in ordinary `Cluster::drain`, which finishes each voter’s Ready before delivering another queued message.

**Required correction:** Replace the sequence with a documented lifecycle that returns every outstanding Ready before stepping that node. A carefully staged asynchronous Ready sequence is one possible approach; it must genuinely generate a commit-only LightReady and exercise the same live persistence/application helpers. Update the contract/evidence description to name the actual supported schedule.

**Closure test:** Add a regression guard that rejects state-changing operations while a Ready remains outstanding, demonstrating RED against this sequence. Run the corrected real-RawNode/real-disk success and failure cases: require a genuine LightReady commit update, preserved term/vote, durable commit before application, and no returned failed-batch messages or new outcome on persistence failure. Reopen successful state and confirm its committed result. Rerun the affected host tests and structural/Clippy gates. Removing the LightReady assertion or merely renaming the schedule does not close the finding.

## 2. Invariant analysis

**Boundary inversion held.** The persistence contract has meaningful dyn-compatible load/persist operations and owned data without Raft, protobuf, runtime or concrete storage types. Disk depends only on that contract; the host’s concrete disk dependency remains development-only. The inventory declares four roles and narrowly permitted edges. No production consumer or canonical wire format changed.

**Publication and lifecycle held under the inspected profile.** Invalid binding is rejected before creation. Exclusive genesis does not overwrite existing, empty or corrupt files; ordinary open does not initialize missing storage. The file lock spans recovery and the store lifetime. Successful publication performs complete append followed by `sync_all`; genesis synchronizes its parent, and reopen synchronizes validated evidence before admission. Post-write faults poison the instance; pre-publication rejection preserves usable state. Drop and the actual killed-worker reopen demonstrate lock release.

**History attacks failed.** Recovery streams bounded frames, checks version/length/checksum and consumes the entire journal. It requires exact empty revision-zero genesis, consecutive revisions, constant binding and legal transitions. Torn tails, checksum-valid conflicting histories, gaps and malformed counts quarantine without repair. Term/commit monotonicity, same-term vote retention and byte-identical committed prefixes are enforced. Legitimate uncommitted replacement/shortening remains possible. Trusted minimum revision rejects an otherwise valid older journal; absence of that external floor is honestly demonstrated.

**Ordinary host ordering held.** `persist_ready` publishes entries and associated HardState before changing MemStorage. `finish_ready` retains outgoing messages locally, advances append, persists any LightReady commit while preserving term/vote, then applies entries and advances the applied frontier. Errors propagate to `drain`, which marks the voter failed and blocks participation, proposals, replies and outcome serving.

**Restart and uncertainty attacks failed.** Startup validates every configured image and serialized entry, including uncommitted suffixes, before returning a usable cluster. It checks common committed prefixes, replays complete payloads/private readiness, reconstructs outcomes and restores configuration/HardState/applied position. Policy, movement, retirement, exact retry and real suffix reconciliation tests passed. The AfterSync test demonstrates that an error is unknown: persisted uncommitted work can subsequently commit without changing earlier receipts.

The process oracle retains complete original receipts outside the killed worker, compares recovered lookup and retry independently, and rejects altered home/generation/outcome records. Both executed SIGKILL cuts passed.

## 3. Risks and next action

Power-loss certification, physical quorum independence, authentic bootstrap/governance, snapshots, membership, real transport, automatic-election randomness and legacy/effect exclusion remain explicitly open. They are not findings against this bounded gate. Full-image journal retention and replay remain experimental costs.

The next action is one scoped remediation of P2-1, followed by the originating Code reviewer’s closure verification at a newly pinned tuple. The current passing suites should remain recorded, but the unsupported LightReady schedule must not be counted as accepted qualification evidence.
