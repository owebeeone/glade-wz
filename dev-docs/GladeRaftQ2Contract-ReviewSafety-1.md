# Glade Raft Q2 persistence contract — SAFETY-AXIS REVIEW, REMEDIATION 1

**Review object:** Committed remediation diff `0d2649369b91e21ddb8ed557b28032c89334fa9a..db2db3bba1bdbd931468949fffbd81d444044a3a`; persistence contract and `proofs/raft-adoption` specifications. Proposed experimental boundary; Contract/RED gate only.

**Baseline:** Revised root `db2db3bba1bdbd931468949fffbd81d444044a3a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources were read through pinned `git show`, the exact committed diff and matching source reads. All four HEADs matched at start and end.

**Date:** 2026-10-03  
**Axis:** Safety: independently verify the original receipt-survival counterexample, recovery diagnostics and changed-range safety. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — both prior Safety findings are closed; zero new P0–P3 findings. This accepts the corrected contract specifications, not a disk implementation, successful SIGKILL recovery or production adoption.

---

## 0. Evidence base

Read the committed remediation plan, contract’s crash-oracle amendment, updated qualification evidence and exact source diff. Retained the controlling-document and persistence-invariant analysis from the original review.

Inspected `process-crash.py:13–146`, particularly full-record parsing, adversarial tests, parent-held acknowledged receipt and fresh-process comparisons; `proof/tests/process_crash.rs:35–165`, including complete receipt encoding and both worker cuts; and all Result-diagnostic edits in `proof/tests/recovery.rs`.

Executed the permitted commands:

- `python3 proofs/raft-adoption/process-crash.py --self-test`: **4 tests passed**, including separate generation/home subcases and accepted-to-rejected substitution.
- All-target Clippy with `-D warnings`: **PASS**.
- `proofs/raft-adoption/check.sh`: **PASS**; architecture, seven production-source files with zero process-global exceptions, explicit conditional boundaries and formatting.
- Durability API package: **1 passed**.
- Disk `conformance`: **0 passed, 8 failed**, as expected against the refusing scaffold.
- Proof `recovery`: **1 passed, 6 failed**, with `NotQualified` preserved in the injected-genesis diagnostic.
- `process_crash --no-run`: compiled `proofs/raft-adoption/target/debug/deps/process_crash-f51f7c71ab7ad8b0`.
- Python runner with that executable: exited **1**, because genesis returned `NotQualified` before the first kill cut.

Cargo commands used `--locked --offline --manifest-path proofs/raft-adoption/Cargo.toml` and the previously authorized protobuf-build 0.14.1 `protoc-osx-x86_64` path.

At end, a working-tree-only appendix had appeared in `GladeRaftPersistenceContract.md`, repeating evidence. It was excluded; this verdict concerns the committed revision above. No review edits or Git mutations occurred.

## 1. Prior-finding closure

| Finding | Disposition | Independently verified closure |
|---|---|---|
| Safety P2-1 — process-kill witness discarded original receipt | **Closed** | Actual ACK receipt is encoded completely and retained in the parent before SIGKILL. Fresh-process lookup and retry are each compared with that original. The original home/generation and rejection counterexamples now fail the oracle. |
| Safety P3-1 — discarded recovery error and failing lint command | **Closed** | Recovery sites use Result `expect`/`unwrap` directly. Clippy passes; behavioral RED reports `NotQualified` explicitly. |

No new findings.

## 2. Changed-range and invariant analysis

The original P2 sequence no longer passes. The worker encodes all five request namespace fields, index, explicit outcome tag and every Resource field, or an exhaustive rejection code. The parent parses exactly one complete ACK record, validates it against the independent fixture and retains it outside the terminated worker. After recovery, agreement between two newly reconstructed receipts is insufficient: **both** must equal the external original.

The executed generation/home mutants change one accepted-resource field while keeping request, index and payload unchanged. Both are rejected. The accepted-to-rejected mutant likewise preserves request/index and assumes materialized payload 23; it is rejected even when lookup and retry agree with each other.

For the before-apply cut, the parent uses the independently specified complete mutation receipt, appropriately avoiding a nonexistent pre-cut receipt. The Rust encoder’s field order matches this specification. Partial, duplicate and incomplete oracle records are rejected by the exercised parser tests.

The amendment strengthens the qualification assertion without changing `DurableStore`, binding/genesis rules, journal publication, poisoning, rollback-floor semantics, dependency roles or authority boundaries. Disk and host implementation remain intentionally refusing. Structural GREEN and oracle GREEN do not replace their behavioral RED or establish actual persistence.

The earlier contract-level protections remain intact: immutable committed prefix, legitimate uncommitted suffix replacement, monotonic hard state, persistence before messages/application/receipts, complete pre-service replay validation, explicit single-writer lifecycle and honest ambiguous write outcomes.

## 3. Risks and next action

Actual disk synchronization, fault recovery, LightReady ordering and complete host restoration remain unproven. No successful process kill/recovery occurred in this review; the runner’s pre-cut refusal is expected at this gate.

The next action is to merge this focused verdict with the independent Consistency re-verdict at the same tuple. If both are GO, proceed with the separately gated TDD implementation and later Code/State acceptance, including execution of both real SIGKILL modes using the corrected complete-receipt oracle.
