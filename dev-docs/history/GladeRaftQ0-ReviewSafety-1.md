# Glade Raft Q0 remediation 1 — SAFETY-AXIS REVIEW

**Review object:** Q0 draft at root `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee`, with focused remediation diff from `581ef60a65bfebda8b39645aeb122f12c23828ee`. This review closes the prior fixture finding; it does not accept an application or Raft implementation.

**Baseline:** Original reviewed root `581ef60a65bfebda8b39645aeb122f12c23828ee`; revised root `2d21e8579006f698bc9de6a6afd4ff32fd2c63ee`. Unchanged members: `glade` `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; `glade-discover` `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `/Volumes/projects/limbo/gyld-wz/gyld` `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through pinned `git show` and committed diffs.

**Date:** 2026-10-03

**Axis:** Safety—verify the original counterexample, its regression witness, and whether the changed ranges introduce another safety root cause. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on its current-round report. Filed verbatim by the lane owner.

**Verdict: GO** — Safety P2-1 is independently closed. Zero open P0, P1, P2, or P3 findings. This accepts Q0’s draft specifications and allocation within their stated boundary only.

---

## 0. Evidence base

All four HEADs were verified at review start and end. Both checks matched the revised tuple above without movement.

Read:

- `dev-docs/GladeRaftQ0-RemPlan-1.md`, lines 1–21.
- The complete committed remediation diff for `proofs/raft-adoption/proof/tests/qualification.rs` and `dev-docs/GladeRaftQualificationEvidence.md`.
- Revised request/command helpers, qualification test lines 9–41.
- Corrected namespace witness and new collision regression, lines 134–176.
- Remediation evidence, lines 46–55.

A scoped committed diff confirmed that the qualification source and evidence document are the only changed substantive files within the reviewed proof/contracts/decision scope. The API, providers, manifests, architecture policy, adoption contract, qualification plan, implementation evaluation, and GDL-052 are unchanged. A scoped working-tree diff returned empty.

The original review’s controlling-source analysis remains applicable to those unchanged bytes. No current-round peer report was read.

No builds, tests, writes, or Git mutations were performed. The revised evidence records compiling specifications, two API passes, twelve refusing-provider assertion failures, and passing formatting. These are recorded results, not independently rerun results.

## 1. Prior-finding closure

| Finding | Disposition | Independent verification |
| --- | --- | --- |
| Safety P2-1: principal-namespace success reused Create’s request identity | **Closed** | Corrected positive identities are distinct from Create; the original collision is separately required to return `RetryConflict`, preserve payload, and preserve Create’s receipt. |

The corrected positive schedule now uses these complete identities:

| Command | Scope | Resource | Incarnation | Principal | Sequence |
| --- | --- | --- | --- | --- | --- |
| Setup Create | 7 | 100 | 1 | 1 | 1 |
| User mutation | 7 | 100 | 1 | 10 | 2 |
| Changed user retry | 7 | 100 | 1 | 10 | 2 |
| Administrator mutation | 7 | 100 | 1 | 1 | 2 |

The user and administrator mutations differ only by principal. Neither aliases the setup Create. Explicit assertions establish these properties before the refusing provider is exercised. The changed user command deliberately retains the user request identity and requires `RetryConflict`, while preserving its original outcome.

The separate test at lines 164–175 reproduces my original counterexample exactly: Create and the administrator mutation share `(7, 100, 1, 1, 1)` but have different command bytes. It now requires refusal, payload 11, and the original retained Create receipt.

Changing both fresh mutation sequences instead of changing Create’s sequence is equivalent to the proposed remedy. No action discriminator was added to retry identity, and RA-004 was not weakened.

## 2. Invariant analysis

The changed ranges remove the contradiction between the positive namespace witness and exact-retry semantics. A conforming implementation can now satisfy both the distinct-principal success case and the same-identity changed-command refusal case.

The regression also detects two unsafe attempts to satisfy the former test: allowing Create-to-Mutate identity reuse, or overwriting retained Create history with the conflicting attempt. It checks both application state and retained outcome.

No new architectural root cause was found. The patch changes test values, adds identity assertions, adds the negative witness, and appends bounded evidence. It introduces no implementation, dependency, permission, persistence, or readiness mechanism.

The evidence continues to distinguish specification repair from implementation qualification. Twelve behavioral failures remain expected against Q0’s refusing provider. They are not reported as application conformance success.

## 3. Risks and next action

All original deferred gates remain open: production durability and failure domains, cryptography, authenticated genesis/membership, metadata witnesses, physical disk recovery, BeginMove/Activate, capacity/compaction, production migration, external sinks, and automatic-election randomness.

The next action is for the lane owner to merge the independent Q0 verdicts. If both are GO, proceed to the authorized Q1a implementation and its separate Code/State acceptance gate.
