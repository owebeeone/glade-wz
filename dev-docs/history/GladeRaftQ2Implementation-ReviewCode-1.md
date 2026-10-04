# Glade Raft Q2 Implementation Remediation 1 — CODE-AXIS REVIEW

**Review object:** Q2 private persistence/recovery experiment at root `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`, including the corrected LightReady witness and controlling DRAFT persistence contract. Focused re-verdict of Code P2-1 from the initial implementation review. No production activation or contract ratification.

**Baseline:** Initial review root `ac69bbcc325c0946bbf215309bcce5edd3210db6`; revised root `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`. Unchanged pins: Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. All four HEADs matched at start and end. Pinned documents and the committed remediation diff were inspected; scoped local source matched the revision with no working-tree diff.

**Date:** 2026-10-03

**Axis:** Carrier interface lifecycle, persistence/application call paths, experimental boundary compatibility and qualification fidelity. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current-round report. Filed verbatim by the lane owner.

**Verdict: GO** — Code P2-1 is closed. No new P0, P1, P2 or P3 findings arose from the focused review. This verdict accepts only the reviewed private APFS/process-crash experiment and its bounded witnesses.

---

## 0. Evidence base

Read the generated remediation prompt, my complete initial Code report, the merged `GladeRaftQ2Implementation-RemPlan-1.md`, revised PersistenceContract implementation allocation, and QualificationEvidence’s withdrawal of the initial LightReady claim and remediation RED/GREEN record.

The controlling architecture, library-boundary, adoption and qualification documents remain those inspected during the initial review. Checked the remediation diff for changes affecting their earlier analysis. Contract crates, disk implementation, journal format, dependency inventory, process-global allowlist, command codec and recovery parser are unchanged. Production members and external Gyld retain their exact initial pins.

Focused code inspection covered:

- `proof/src/cluster/light_ready_tests.rs:12–50`: lifecycle guard and refusal/permission checks.
- The same file, lines 144–211: first Ready persistence, asynchronous advance, follower acknowledgement, later Ready and live `finish_ready`.
- Lines 212–279: genuine commit-only publication assertions, complete expected receipt, failure suppression and actual disk reopen.
- `proof/src/voter.rs:38–142`: terminal-term refusal and unchanged Ready/LightReady persistence-before-application helpers.
- `proof/src/cluster.rs:234–272`: bounded campaign refusal and ordinary Ready/message orchestration.
- Added disk/recovery regressions and the README’s explicit disk-tier selection.
- Pinned cached `raft 0.7.0` `raw_node.rs:470–478,606–704` and `raft.rs:1033–1062`.

QualificationEvidence records that inserting the lifecycle guard into the original schedule produced `state mutation while Ready outstanding` at the leader step. I inspected that recorded RED and verified that the guard would reject the original outstanding-Ready sequence. I did not restore or modify old source to repeat RED during this read-only review.

Executed permitted commands with the specified compatible PROTOC:

| Verification | Independently observed result |
|---|---|
| Locked/offline proof workspace tests | 49 passed; two disk unit cases and external worker intentionally ignored |
| Explicit `--lib q2_real -- --ignored` tier | Both real-disk unit cases passed |
| Focused host recovery target | All 13 tests passed |
| `proofs/raft-adoption/check.sh` | Architecture, source boundaries, formatting and process-global checks passed |
| All-target Clippy with warnings denied | Passed |
| Python crash-oracle self-tests | Four passed |
| External compiled process worker runner | Both SIGKILL/fresh-process recovery cycles passed |

The worker executable reported by Cargo was `proofs/raft-adoption/target/debug/deps/process_crash-f51f7c71ab7ad8b0`. The runner’s acknowledged and durable-before-apply cuts both recovered complete original receipts and exact retry outcomes.

No source writes, report filing or Git mutations were performed. Authorized test writes remained in ignored/disposable locations.

## 1. Prior-finding closure

| Original finding | Disposition | Independent closure evidence |
|---|---|---|
| Code P2-1: LightReady witness steps RawNode before advancing its outstanding Ready | **Closed** | First Ready now returns through `advance_append_async` before leader acknowledgement processing. A guarded later Ready reaches the unchanged live `finish_ready` helper and generates a genuine commit-only LightReady. Success/failure assertions and successful disk reopen passed. The unsupported historical schedule is explicitly withdrawn. |

No new findings.

The correction satisfies the original requested remedy rather than weakening the requirement. It retains actual RawNodes and real disk stores, preserves the LightReady assertion, exercises the live persistence/application helpers, and adds the complete receipt/reopen check requested by the initial report.

## 2. Invariant analysis

**The carrier lifecycle now holds.** After the mutation proposal, the test checks `has_ready`, records the outstanding Ready and obtains it. It verifies that this first batch has entries but no committed application batch, then calls `persist_ready` to publish the complete image and update the storage mirror. Line 173 returns the Ready through `advance_append_async`; line 174 clears the guard. Only afterward does the test deliver the follower’s acknowledgement through the leader’s `step`.

This is the documented asynchronous lifecycle: storage must make the updates readable before asynchronous advance, and persistence notification may occur later. Here physical persistence already succeeded; delaying its carrier notification does not fabricate durability.

**The LightReady update is genuine.** The test delivers exactly one follower’s persisted data acknowledgement. Before the later Ready, it asserts that the leader’s committed frontier still equals the previous cut. A permitted ping produces subsequent work. That second Ready has no entries and has no new commit in its HardState. Returning it through `finish_ready` invokes `advance_append`, whose ordered notification drains both Ready records. The carrier updates the leader’s local persisted match and can then establish quorum commitment.

Consequently the observed next commit-only store call is attributable to LightReady. It cannot be an ordinary Ready containing the new commit, because the test checks that Ready’s entries and HardState immediately before calling the helper. The observed image retains the prior term/vote and advances commit by exactly one.

**Publication remains before application and message release.** The live helper retains outgoing messages in a local vector. It persists the LightReady commit before applying either committed batch and before returning that vector. On the armed store failure, the result is `Err(Io)`, application stays at the previous cut, the mutation has no reply, and the retained stored commit stays unchanged. The helper returns no failed-batch message vector.

The earlier append sent to follower 2 belongs to the already persisted, asynchronously advanced first Ready. It is not a message released by the later failed commit batch. This distinction is consistent with the corrected schedule and the unchanged ordinary driver.

**Recovery proves the complete successful result.** The successful branch compares the live reply with an independently constructed Receipt containing every request field, original index, Accepted tag and Resource field. It drops the cluster, reopens all three actual disk stores and reconstructs the host. Recovered outcome and materialized Resource equal that complete expectation. This closes the original request for a durable reopen witness, rather than comparing two newly reconstructed values only to each other.

**Boundary and evidence integrity remain intact.** The remediation adds no public persistence interface, journal format or dependency change. Term refusal is bounded and preserves prior stored history. Existing recovery, disk conformance and Q1a regressions pass. The evidence distinguishes historical execution from accepted qualification and attributes earlier measurements to the initial checkpoint instead of claiming they were repeated.

## 3. Risks and next action

The test-only lifecycle guard protects this controlled witness; it is not a production carrier wrapper or exhaustive interleaving proof. Full-image journal retention and replay remain experimental limitations. Production failure domains, power-loss durability, authenticated bootstrap/governance, snapshots, membership, automatic-election randomness, real transport and legacy/effect exclusion remain open as explicitly deferred.

The next action is to file this report verbatim and merge the independently returned verdicts for the same settled tuple. Code P2-1 needs no further remediation. Acceptance must remain scoped to the named private process-crash experiment.
