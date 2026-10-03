# Glade Raft Q2 contract remediation 1 — CONSISTENCY-AXIS REVIEW

**Review object:** Committed remediation diff `0d2649369b91e21ddb8ed557b28032c89334fa9a..db2db3bba1bdbd931468949fffbd81d444044a3a`; proposed persistence contract, recovery specifications and private process-crash oracle. Contract gate only; implementation remains intentionally unqualified.

**Baseline:** Revised root `db2db3bba1bdbd931468949fffbd81d444044a3a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. All four HEADs matched at start and end. Sources were read through pinned `git show` and the exact committed diff.

**Date:** 2026-10-03

**Axis:** Focused consistency re-verdict on the prior lint/diagnostic finding and changed receipt-oracle specifications. Independent, adversarial, read-only. The merged prior-round remediation plan is legitimate input; no current-round peer report was read or used. Filed verbatim by the lane owner.

**Verdict: GO** — Consistency P3-1 is closed. No new P0/P1/P2/P3 findings. This accepts the revised contract/specification checkpoint, not persistence implementation or actual crash survival. Closure of Safety’s originating P2 remains that reviewer’s responsibility.

---

## 0. Evidence base

Read `GladeRaftQ2Contract-RemPlan-1.md`, the complete committed remediation diff, revised PersistenceContract and QualificationEvidence, and the unchanged public `RequestId`, `Receipt`, `Outcome`, `Resource` and `Rejection` declarations. Retained the original review’s controlling-graph analysis; the narrow patch does not change its architecture or production-authority premises.

Executed the permitted commands with the supplied PROTOC:

| Command/target | Result |
|---|---|
| `process-crash.py --self-test` | Four tests passed |
| Clippy, locked/offline, all targets, `-D warnings` | Passed |
| `proofs/raft-adoption/check.sh` | Architecture, process-global, conditional-source and formatting checks passed; seven sources, zero allowlist entries |
| Proof recovery target | One negative passed; six compiling behavioral failures |
| Disk conformance target | Zero passed; eight compiling behavioral failures |
| Process-crash worker `--no-run` | Compiled |
| Process runner with the reported executable | Exited before the first kill cut because genesis returned `NotQualified`; no crash/recovery PASS |

The focused recovery failure now explicitly reports `reviewed port accepts valid genesis: NotQualified`.

At the final inspection, an uncommitted evidence paragraph had been appended to PersistenceContract after the pinned amendment. It was excluded from this committed-source review. The four HEADs did not move. This reviewer made no edits or Git mutations.

## 1. Prior-finding closure

| Prior finding | Recheck and disposition |
|---|---|
| Consistency P3-1: recovery specification fails warnings-denied lint and loses the recovery error | **Closed.** `recovery.rs:136–137` calls `Result::expect` directly. Related recovery call sites and the process worker likewise retain their `Result` errors. The original Clippy command passes, while the intended behavioral failure now includes `NotQualified`. |
| Safety P2-1: crash oracle compares insufficient outcome evidence | **Consistency correction verified; originating Safety closure reserved.** The parent retains the complete pre-kill ACK receipt and compares both fresh-process lookup and retry against it. The before-apply case uses an independent complete expected receipt. The named adversarial regressions pass. |
| Safety P3-1: same diagnostic/lint issue | Same correction and passing command verified; no separate Consistency defect remains. |

No new findings.

## 2. Changed-range and invariant analysis

**The patch preserves the reviewed boundary.** It changes recovery-test diagnostics, private worker stdout encoding, the Python oracle, and supporting contract/evidence text. Storage traits, boundary values, manifests, classifications, dependency edges, journal publication rules, host scaffolds and production authority are unchanged. It does not introduce adapter or recovery implementation before contract acceptance.

**The receipt encoding covers the actual public value.** The worker emits all five request namespace fields, the original index and an explicit outcome tag. Accepted outcomes include all seven Resource fields; rejected outcomes encode every current Rejection variant through an exhaustive Rust match. No receipt field is silently omitted.

**The comparison has an independent reference.** At the acknowledged cut, `expected` is obtained from the worker’s complete ACK record before SIGKILL and remains in the parent. Both recovered records must equal it. The runner additionally checks the original ACK against the independently specified fixture. At the before-apply cut, it uses that fixture because no acknowledged receipt exists.

The former counterexample therefore fails: lookup and retry cannot agree on an altered home, generation or Rejected outcome and pass merely because their request/index and materialized payload remain unchanged. The regression tests exercise both lifecycle-field mutations and accepted-to-rejected substitution. Parser tests verify complete accepted/rejected records and reject missing newline, duplicate records and incomplete receipts.

**Evidence claims remain bounded.** Four passing Python tests qualify the oracle predicate/parser. They do not qualify storage. The disk and host specifications still compile and fail behaviorally, and the actual runner still refuses before termination. Updated documentation preserves that distinction and makes no power-loss or production-adoption claim.

## 3. Risks and next action

Actual Ready/LightReady ordering, disk recovery, nonvacuous quarantine validation and process-kill survival remain implementation obligations. The two selected kill cuts must not become an exhaustive boundary-coverage claim.

The next action is the lane-owner verdict merge with the originating Safety re-verdict, followed—if accepted—by the specified TDD implementation and separate Code/State gate.
