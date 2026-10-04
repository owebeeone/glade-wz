# Glade Raft Q2 persistence contract — SAFETY-AXIS REVIEW

**Review object:** Root diff `96bb1b54a420fbf7b8471642fe5c15288940ac11..0d2649369b91e21ddb8ed557b28032c89334fa9a`, principally `dev-docs/GladeRaftPersistenceContract.md` and `proofs/raft-adoption`. Proposed experimental boundary; Contract/RED gate only. No production activation.

**Baseline:** Root `0d2649369b91e21ddb8ed557b28032c89334fa9a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Sources read through pinned `git show` and scoped diffs. All four HEADs matched at both start and end; scoped working-tree diffs were empty.

**Date:** 2026-10-03  
**Axis:** Safety: adversarial attack on persistence, recovery, irreversible publication, and qualification claims. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks; one additional P3 finding. I pre-commit to GO on a revision that resolves P2-1 as specified. This verdict concerns the executable qualification specification, not the intentionally refusing implementation.

---

## 0. Evidence base

Read the persistence contract §§1–7, QualificationPlan §§1–6, AdoptionContract RA-001–012 and allocation, QualificationEvidence including Q2 RED evidence, and the qualification review ledger. Read BuildEntry, LibraryBoundaryAndTestingPolicy, GladePackageArchitecture, architecture revision 3, and the pinned external Gyld allocation, especially Records, StorageAdapter, ApplicationSupplier and NodeAssembly. Inspected pinned Glade Substrate V1 as an existing-contract source; this review does not ratify its proposed amendments.

Read the complete new durability API/model, disk scaffold/conformance tests, recovery specifications, process worker and Python runner. Inspected cluster persistence ordering and Q2 stubs, manifests, architecture-policy changes, source-check inventory and empty process-global allowlist.

Executed allowed commands using:

`PROTOC=/Users/owebeeone/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/protobuf-build-0.14.1/bin/protoc-osx-x86_64`

and the locked/offline Cargo manifest `proofs/raft-adoption/Cargo.toml`:

- Durability API package: **1 passed**.
- Disk `conformance`: **0 passed, 8 failed**, behaviorally against `NotQualified`.
- Proof `recovery`: **1 passed, 6 failed**. The passing invalid-input test remains vacuous against the refusing constructor.
- Existing adoption API, proof library and `qualification` targets: **22 passed**.
- `process_crash --no-run`: compiled executable `proofs/raft-adoption/target/debug/deps/process_crash-f51f7c71ab7ad8b0`.
- Python process runner: exited **1**, worker refused genesis before the first cut. No SIGKILL/recovery success was observed or claimed.
- `proofs/raft-adoption/check.sh`: **PASS**; architecture boundaries, seven production-source files with zero global exceptions, explicit conditional boundaries and formatting.
- All-target Clippy with `-D warnings`: exited **101**, `clippy::ok_expect` at `proof/tests/recovery.rs:136–138`.

No source edits, Git mutations, or peer-report reads occurred.

## 1. Findings

### [P2-1] Process-kill witness discards the original receipt and cannot establish its survival

**Location:** `proofs/raft-adoption/proof/tests/process_crash.rs:95–110`; `process-crash.py:15–47`; persistence contract RP-011 and §7’s promised original-receipt recovery.

**Violated invariant:** A receipt acknowledged before SIGKILL must recover as the same terminal receipt. Equality includes the complete request, index and outcome, not merely payload/index observations or consistency between two post-restart answers.

**Counterexample:** In `write-ack`, the worker checks only that replies exist, then discards them and emits the constant marker `Q2_ACK 3 23`. After restart it checks the materialized payload, obtains a receipt from recovered state, checks index 3, and compares the retry against that newly obtained receipt.

A recovery defect can preserve payload 23 and index 3 while reconstructing the retained receipt with a changed outcome—for example, an incorrect generation/home in its accepted Resource. Lookup and exact retry can consistently return that changed receipt. Every process-worker assertion still passes. Even an incorrect retained rejection can satisfy the receipt/index comparison while materialized payload remains 23.

The ordinary reopen test retains pre-restart receipts and compares them exactly, but that does not establish the same property across the separately required SIGKILL path.

**Impact:** The future runner can report successful process-crash qualification without proving the acknowledged terminal outcome survived. This is a missing safety assertion in the proposed executable contract, not evidence of current disk corruption.

**Required correction:** Preserve the complete pre-kill receipt outside the killed process and compare it with both recovered lookup and exact retry. An unambiguous stdout encoding consumed by the runner is sufficient. For the before-apply cut, compare recovery against an independently specified complete expected receipt; no pre-cut receipt exists there.

**Closure test:** The specification must reject a reconstructed receipt whose outcome differs while request/index and materialized payload remain unchanged. Demonstrate this for changed accepted-resource fields and accepted-to-rejected substitution. Retain behavioral RED against the stubs; the later implementation gate must execute both real kill modes with these assertions.

### [P3-1] Recovery assertion discards the error and fails the declared lint command

**Location:** `proofs/raft-adoption/proof/tests/recovery.rs:136–138`; the same error-discarding pattern appears at other recovery call sites.

**Violated requirement:** Required verification commands must pass, and recovery failures should retain their diagnostic cause.

**Reproduction:** The allowed all-target Clippy command fails with `clippy::ok_expect`. The injected-genesis test’s observed RED output reports only “reviewed port accepts valid genesis,” because `.ok()` discarded `StoreError::NotQualified`.

**Impact:** The new target prevents a clean lint run and obscures whether future failures arise from quarantine, binding, I/O or the intentional scaffold.

**Correction and closure:** Call `expect`/`unwrap` directly on the Result, preserving the error. Rerun Clippy with warnings denied and confirm behavioral RED still identifies the refusing constructor.

## 2. Invariant analysis

The committed-prefix attack failed at the contract level. RP-003/004 prohibit term and commit regression, same-term vote clearing/change, and byte changes through the previous commit. They explicitly allow legitimate uncommitted suffix replacement/shortening. Shared assertions exercise both categories and stale revision rejection without poisoning usable state.

The ambiguous-write attack also failed textually. After publication starts, errors poison the instance; the driver must stop serving, applying and emitting the failed Ready’s messages. Reopening may recover a coherent result or quarantine. No error certifies noncommit. Ready and LightReady persistence ordering are both explicit obligations, although the refusing host supplies no implementation evidence yet.

Restart completeness is explicit: all configured voter images and serialized entries must validate before service; committed replay reconstructs payload, outcomes, policy, retirement and movement fences; uncommitted entries remain available without application.

Genesis/open separation, lifetime OS locking, no automatic repair/reset, parent-directory synchronization and externally supplied rollback floors bound the local lifecycle. Whole-file valid rollback without an external floor is honestly outside detection. Power loss, physical quorum independence, malicious storage and production crypto remain expressly unqualified.

## 3. Risks and next action

The principal residual implementation risks are LightReady error ordering, full-history parser validation/allocation limits, and coherent Raft applied-frontier restoration. Current RED evidence proves specification execution, not these behaviors.

The next action is one scoped specification correction for P2-1, with the lint/diagnostic correction alongside it, followed by focused Safety re-verdict at a newly pinned tuple. Disk and host implementation should remain behind the Contract gate.
