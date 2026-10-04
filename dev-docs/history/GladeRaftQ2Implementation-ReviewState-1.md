# Glade Raft Q2 implementation remediation 1 — STATE-AXIS REVIEW

**Review object:** Q2 implementation at root `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`, encompassing root diff `96bb1b54a420fbf7b8471642fe5c15288940ac11..c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`. Focused remediation-1 acceptance review of the private persistence/process-crash experiment. No production activation.

**Baseline:** Initial State review at root `ac69bbcc325c0946bbf215309bcce5edd3210db6`. Revised root `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`; unchanged glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`, glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`, external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. All four HEADs matched at start and end. Scoped working-tree diffs were empty.

**Date:** 2026-10-03

**Axis:** Durable-state semantics, publication ordering and restart legality, with focused verification of the originating State counterexample. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current-round report. Filed verbatim by the lane owner.

**Verdict: GO** — State P2-1 is closed. No new P0/P1/P2/P3 findings. This accepts the bounded private Q2 implementation on this exact tuple.

---

## 0. Evidence base

Read the generated remediation-1 State prompt, merged remediation plan, my complete initial State report, revised persistence-contract allocation and the new qualification-evidence section. Inspected the committed remediation diff from `ac69bbcc325c0946bbf215309bcce5edd3210db6` to `c3fe2e0f4afb08df4dfd39ec5021f0415cac1df5`.

The initial review’s controlling-source analysis remains applicable: the contract, package roles, fixed configuration, journal format and production dependency boundary have not changed. The remediation introduces a campaign admission guard, a reserved-term publication check, concrete regressions, and a corrected supplemental LightReady schedule.

Focused source inspection covered:

- `proof/src/cluster.rs:234–277`: campaign admission and ordinary driver failure handling.
- `proof/src/cluster.rs:412–481`: actual incoming terminal-term heartbeat test and reopen.
- `proof/src/voter.rs:38–151`: reserved-term rejection, Ready publication, LightReady persistence and application ordering.
- `proof/tests/recovery.rs:578–609`: actual-disk predecessor-term campaign/reopen regression for both fixed configurations.
- All of `proof/src/cluster/light_ready_tests.rs`, including lifecycle guard, actual persistence notification, failure assertions and complete receipt recovery.
- Cached pinned raft-rs 0.7.0 `raw_node.rs:471–475,617–698` and `raft.rs:1033–1058`, establishing the supported Ready lifecycle and the persistence notification that produces the real commit-only LightReady.

Executed the permitted Cargo commands using the specified compatible PROTOC, `--locked --offline`, and the proof workspace manifest:

- Default workspace tests: **49 passed**; two explicit disk unit cases and the external process worker remained correctly ignored.
- Explicit `-p glade-raft-adoption-proof --lib q2_real -- --ignored`: **both disk unit cases passed**.
- Python receipt-oracle self-test: **four passed**.
- External process runner using the executable reported by Cargo: **both actual SIGKILL/fresh-process recovery cycles passed**.
- Local architecture/source/format/process-global gate: **passed**, thirteen owned source files and zero allowlisted items.
- All-target Clippy with warnings denied: **passed**.

These results give **51 executed Rust tests**, four Python oracle cases and two process-kill/recovery cycles. The predecessor-term regression passed within the thirteen-test host recovery target. No source, report file or Git state was changed during this review.

## 1. Prior-finding closure

| Finding | Original counterexample | Revised correction and independently checked evidence | Disposition |
| --- | --- | --- | --- |
| State P2-1: a legal campaign creates durable state that startup refuses | Recovery accepts `MAX-1`; campaign increments to `MAX`; the host persists that term and can retain outcomes; subsequent startup rejects the intact history. | Campaign now refuses current term `>= MAX-1` before invoking the carrier. The persistence helper independently rejects a `MAX` image before calling the store, covering peer-learned terms. Actual-disk campaign/reopen and incoming-heartbeat tests pass for `[1,2]` and `[1,2,3]`; exact prior receipts recover and persisted predecessor terms remain unchanged. | **Closed** |

There are no new findings.

The committed evidence records the owner’s observed RED results: the predecessor campaign regression failed at reopen with `CapacityExhausted`, and the incoming terminal-term case returned a heartbeat response rather than refusal. I did not recreate that RED by modifying source under the read-only mandate. I independently inspected the regression sequences and executed their GREEN forms on the revised settled object.

## 2. Invariant analysis

The original counterexample is now blocked before its dangerous transition. `Cluster::campaign` checks the predecessor term before calling raft-rs, so the carrier never increments that voter into the term excluded by startup admission. The driver records `CapacityExhausted` and stops participating. This sacrifices progress at the reserved boundary while preserving the durable history.

The regression supplies stronger closure evidence than a destination-state check. For each reviewed configuration it first commits a real Create and retains its full original receipt. It then reopens the actual disk stores, legally raises HardState to `MAX-1`, campaigns through the ordinary host, drops the cluster, and reopens every configured store. Every voter’s recovered lookup equals the original receipt. Subsequent store loads confirm that the durable term remains `MAX-1`. The old defect therefore cannot hide behind a successful refusal assertion while still damaging restart.

The publication guard closes the alternate route through a received higher term. The incoming-heartbeat test lets a real RawNode observe `MAX`, then invokes its live Ready helper. The result is `Err(CapacityExhausted)`, rather than an outgoing message vector; the complete cached StoredState remains unchanged and every disk file subsequently reopens through `Cluster::recover`.

Source ordering confirms the refusal precedes store publication, memory-mirror advancement, Ready message extraction and application. The ordinary `drain` path marks a voter failed on that error, preventing continued campaigning, proposal admission, message handling and receipt serving. Restart returns to the preserved durable observation. No term rewriting, reset, truncation or repair is introduced.

I also checked that the supplemental LightReady evidence now rests on the carrier’s documented lifecycle. The first Ready is persisted and returned through `advance_append_async` before a follower response is stepped into the leader. A subsequent Ready reaches the unchanged live `finish_ready` helper; ordered persistence notification then produces the actual LightReady commit change. This replaces the unsupported initial schedule disclosed in the merged plan.

The explicit disk case observes exactly one commit-only image with preserved term/vote. Success applies the independently specified complete receipt and recovers it after reopening all stores. Failure returns an error, leaves application and retained outcome unchanged, and preserves the old stored commit. My State verdict relies on that executed corrected witness, not the withdrawn historical schedule.

The broader rerun preserves the original durable invariants: committed-byte immutability, legal uncommitted suffix replacement, malformed-history quarantine, single-writer locking, poisoned ambiguous writes, complete payload/policy/tombstone replay and exact retry recovery. Both externally driven SIGKILL cuts still recover the complete expected receipts in fresh processes.

## 3. Risks and next action

This correction reserves one terminal term consistently; it does not qualify every capacity limit or automatic-election path. The process witnesses cover the declared acknowledgment and durable-before-apply cuts. Synthetic I/O faults remain distinct from actual termination, and neither establishes power-loss or controller-cache certification.

Unbounded journal retention, independently trustworthy rollback floors, snapshots, membership, real authentication, transport, physical failure domains, automatic-election RNG and production legacy/effect exclusion remain open as documented. No production consumer or dependency was changed.

The next action is to file this report verbatim and merge the independent verdicts on this exact tuple. The State lane requires no further remediation.
