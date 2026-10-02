# Glade Raft Q0 remediation 1 — CONSISTENCY-AXIS REVIEW

**Review object:** Q0 draft at workspace root `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee`, with focused remediation diff from `581ef60a65bfebda8b39645aeb122f12c23828ee`. Contracts and intentionally refusing harness remain draft specifications; this is not implementation acceptance.

**Baseline:** Previous reviewed root `581ef60a65bfebda8b39645aeb122f12c23828ee`; revised root `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee`. Unchanged members: Glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; Glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` at `ca04499a360d910fbf8ee2540ed446facd051b35`. Revised sources were read through `git show` and the exact commit-range diff.

**Date:** 2026-10-03

**Axis:** Focused Consistency re-verdict: independently verify the original counterexample’s correction and attack changed ranges for new contradictions. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current-round report. Filed verbatim by the lane owner.

**Verdict: GO** — Consistency P2-1 is closed. Zero open P0, P1, P2 or P3 findings. This accepts Q0 draft/consumer fitness only.

---

## 0. Evidence base

All four HEADs matched the revised tuple at both the start and end of this review.

Inspected:

- `dev-docs/GladeRaftQ0-RemPlan-1.md`, lines 1–21.
- Exact diff `581ef60a65bfebda8b39645aeb122f12c23828ee..2d21e8579006f698bc9de6a6afd4ff32fd2c63ee` for `proofs/raft-adoption` and `GladeRaftQualificationEvidence.md`.
- Revised qualification source, lines 1–188, including unchanged constructors and response helpers, the corrected namespace schedule at lines 135–160, and the new intentional collision schedule at lines 164–175.
- Revised qualification evidence, lines 1–55.
- A name-only diff confirming no changes to the adoption contract, qualification plan, implementation evaluation, API, refusing provider source, manifests, lockfile, architecture policy, gate scripts or process-global allowlist.

The original Consistency review and its controlling-source analysis remain the basis for unchanged ranges. The merged remediation plan is legitimate prior-round evidence; no current-round peer report was read.

No files were written, builds or tests run, Git state mutated, or subagents launched. The recorded compilation, API test, formatting and behavioral RED results were inspected, not independently rerun.

## 1. Prior-finding closure

| Finding | Original counterexample | Revised evidence | Disposition |
| --- | --- | --- | --- |
| Consistency P2-1 | The administrator mutation reused the earlier Create request identity but required acceptance, contradicting RA-004’s changed-byte refusal. | Fresh namespace mutations now use sequence 2; Create retains sequence 1. Identity assertions precede harness setup. A separate test intentionally repeats the Create identity and requires conflict without data/history overwrite. | **Closed** |

No new findings.

## 2. Invariant analysis

**The positive namespace schedule is now satisfiable.** Unchanged constructors yield these exact identities:

| Command | Scope | Resource | Incarnation | Principal | Sequence |
| --- | --- | --- | --- | --- | --- |
| Setup Create | 7 | 100 | 1 | 1 | 1 |
| Original user mutation | 7 | 100 | 1 | 10 | 2 |
| Changed user retry | 7 | 100 | 1 | 10 | 2 |
| Fresh administrator mutation | 7 | 100 | 1 | 1 | 2 |

The two user commands intentionally share identity and differ in payload. The administrator mutation differs from the user identity only in principal and does not alias Create. Lines 141–151 assert these relationships before the refusing provider can fail setup.

The behavioral assertions still require the original user mutation to succeed, changed bytes to produce `RetryConflict`, the original outcome to remain retained, and the fresh administrator mutation to succeed. Thus the repair preserves the intended principal-namespace witness and RA-004.

**The original counterexample is retained as a negative witness.** Lines 165–168 deliberately reconstruct the former collision: Create and administrator Mutate share `(7,100,1,1,1)`. Lines 172–175 require `RetryConflict`, unchanged payload 11 and the original Create receipt. A provider that partitions retry identity by action or overwrites the original outcome would fail this specification. No action discriminator or weaker retry rule was introduced.

**Changed-range analysis found no new root cause.** The correction is confined to test identities, assertions and an additional negative schedule. API semantics, trusted lookup versus transient response, current disclosure checks, command ordering and dependency boundaries are unchanged. The new negative assertions agree with those boundaries.

**Evidence remains honest.** The remediation section records compiling specifications, two passing API tests and twelve failing behavioral qualification tests. The earlier eleven-failure record remains identifiable as pre-remediation history. The document explicitly calls this a test repair rather than implementation or reviewer closure.

The refusing provider therefore remains correctly RED. A passing application implementation is neither present nor required to close this Q0 specification defect.

## 3. Risks and next action

This GO establishes that the corrected Q0 consumer schedule no longer demands a violation of its own retry contract. It does not establish that any implementation enforces the invariant.

Production deployment/library selection, cryptography, genesis/membership, metadata witnesses, physical durability, staged movement, retention, migration, external sinks and automatic elections remain explicitly unqualified.

The next action is to combine this independent verdict with the other axis’s verdict at the same tuple and record Q0 acceptance only if both are GO. Subsequent implementation must satisfy the corrected positive and negative witnesses under the declared Q1 acceptance gate.
