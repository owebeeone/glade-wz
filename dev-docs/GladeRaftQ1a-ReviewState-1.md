# Glade Raft Q1a remediation 1 — STATE-AXIS REVIEW

**Review object:** Root `31bbea0cf1da3c6ae437cf482cb744d561693c08`, focused remediation diff from `5e81483f80b23a64c7914986fc4d31e1c3d9fd4f`. Controlled by `GladeRaftQ1a-RemPlan-1.md`, the draft AdoptionContract and QualificationPlan. Memory-only Q1a acceptance; no production activation.

**Baseline:** Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources read through pinned `git show` and the scoped committed diff. All four HEADs matched at review start and end; the proof-source working diff was empty.

**Date:** 2026-10-03.

**Axis:** Focused State re-verification of replay correction, retained evidence and fail-closed movement boundaries. Independent, adversarial, read-only. Prior-round reports are legitimate inputs; no current-round peer report was read. Filed verbatim by the lane owner.

**Verdict: GO** — zero open State findings and no new P0–P3 findings. Independent inspection and execution verify the specified correction for prior Code P2-1 without expanding movement authority or durability claims.

---

## 0. Evidence base

Read the remediation plan, prior Code and State reports, revised qualification evidence/review ledger and complete scoped correction. The initial review’s controlling-source inspection remains applicable: public signatures, codec, dependencies, authority rules and Ready processing are unchanged.

Focused inspection covered:

- `proof/src/application.rs:91–146,291–312`: private envelope replay, retained history and revised public adapter.
- `proof/src/cluster.rs:17–92`: actual-driver/public-interface regression.
- Revised README and qualification evidence, including the distinction between first application and replay.
- Committed diff from the previous reviewed root across the proof and qualification documents.

Authorized reruns passed:

- Compatible `PROTOC` plus locked/offline proof-workspace tests: **22 passed**—2 API witnesses, 2 codec tests, 1 new driver-to-public replay regression and 17 qualification tests.
- `proofs/raft-adoption/check.sh`: architecture, owned-source process-global scan, explicit conditional-source boundaries and formatting passed; 5 owned production Rust files, zero allowlisted items.
- Specified all-targets Clippy command with `-D warnings`: passed.

No writes, Git mutations, custom tests or production activity occurred. The committed evidence records the new regression’s prior RED result; this review independently ran GREEN, not the historical RED checkpoint.

## 1. Prior-finding disposition

| Prior finding | State re-verification |
| --- | --- |
| State initial review: zero findings | No State closure was pending. Its memory-ordering and readiness conclusions remain applicable to unchanged implementation paths. |
| Code P2-1: exact public replay conflicts with retained private Move evidence | Correction verified. The regression applies an accepted Move through actual RawNode processing, then replays its exact index/Command through `dyn CommittedMachine` and receives the original receipt. Changed command content conflicts; a fresh unwitnessed Move still refuses. Formal closure remains with the originating reviewer and lane merge. |

The Code counterexample was valid. The initial State review did not exercise replay through the public interface after privately attested movement; its direct trait test covered Create/noop history. The new regression closes that coverage gap.

## 2. Invariant analysis

**Replay recovers existing evidence without creating authority.** The new branch at `application.rs:300–304` first requires a retained entry at the supplied index and exact equality of its canonical public Command. It returns the stored receipt without executing the command, changing application progress, modifying resource state or replacing private evidence. The absent public envelope therefore cannot invalidate accepted history, and cannot manufacture a previously unapplied Move.

**Changed content remains fail-closed.** Any changed command or command/noop substitution at an existing index returns `ConflictingReplay`. Equality includes request namespace, generation, home, policy frontier and action fields. The correction is not a lookup by request ID alone. The unchanged private `apply_committed` path continues comparing both command and readiness envelope for private full-entry replay.

**New application retains the original gate.** If no history exists at the supplied index, public application still calls `apply_committed(index, command, None)`. Contiguous-index checks remain in force. A new Move therefore requires private readiness and returns `IncompleteSuccessor` without it, including when the public numeric frontier names an apparently complete successor. The new regression verifies this refusal immediately after successful movement.

**Retention and recovery semantics remain coherent.** Exact existing-index replay returns the original per-index result, including its original receipt index. Retained request outcomes are untouched. Existing changed-retry, failover, retirement and disclosure tests still pass. Public `apply` remains a trusted host boundary; client disclosure continues through the separately guarded serving accessors.

**Protocol ordering is unchanged.** The correction does not alter RawNode proposal encoding, memory storage ordering, Ready/LightReady application, successor frontier capture or cut revalidation. No new race, recovery state or independent mutation path was introduced. Modified Rust control flow is braced, and no conditional declaration attributes were added.

## 3. Risks and next action

Acceptance remains restricted to fixed complete-data logical voters, retained memory history and the exercised schedules. Disk/restart/power loss, authenticated bootstrap, production readiness certificates, two-stage movement, capacity, snapshots/membership, automatic-election RNG compliance, real effects and legacy activation remain unqualified.

The next action is to file this focused report and merge the independent revised-tuple verdicts. No further State remediation is required.
