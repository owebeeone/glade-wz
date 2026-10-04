# Glade Raft Q2 contract — CONSISTENCY-AXIS REVIEW

**Review object:** Root diff `96bb1b54a420fbf7b8471642fe5c15288940ac11..0d2649369b91e21ddb8ed557b28032c89334fa9a`; proposed `dev-docs/GladeRaftPersistenceContract.md` and `proofs/raft-adoption` contract/RED checkpoint. No production activation.

**Baseline:** Root `0d2649369b91e21ddb8ed557b28032c89334fa9a`; glade `90dc1a60981185fa26ae5bfafbbb5377c12a413b`; glade-discover `52ea2d118f45d9e7c3d9a789310dd5d669958851`; external Gyld `ca04499a360d910fbf8ee2540ed446facd051b35`. Controlling sources were read using `git show <pinnedSHA>:<path>`. All four HEADs matched at both start and end. Scoped working-tree diffs were empty.

**Date:** 2026-10-03

**Axis:** Consistency against the controlling document graph, boundary contracts and executable specifications. Independent, adversarial, read-only. The other axis runs in parallel; nothing here relies on it. Filed verbatim by the lane owner.

**Verdict: GO** — zero P0/P1/P2 findings; one nonblocking P3 finding. This accepts the proposed contract/RED checkpoint only. It does not qualify disk persistence, restart, process-kill survival or production adoption.

---

## 0. Evidence base

Read:

- Root `AGENTS.md`, `AGENTS_GWZ.md`, and the review-loop skill.
- PersistenceContract lines 1–82, QualificationPlan §§1–6, AdoptionContract §§1–4, QualificationEvidence and Qualification-ReviewCycle.
- BuildEntry; LibraryBoundaryAndTestingPolicy, especially LBT-001–012; PackageArchitecture; architecture candidate revision 3.
- Relevant pinned reconciliation sources: BuyBuildMatrix D-06/R7/R9/R16, WorkspaceDirectory §4, DiscoveryModel §§0/3/7, glade SubstrateV1 §§2/6 and CrossNodeWritesPlan, AuthzModel’s authority/policy sections.
- External Gyld allocation, including Records, StorageAdapter, Admission, Policy and NodeAssembly.
- All four proof manifests, architecture policy, process-global inventory, source checker and local gate.
- Durability API and shared conformance; model provider; disk scaffold and eight conformance tests; host recovery scaffold and seven recovery tests; process worker and Python runner; existing RawNode driver and README.

Executed the permitted locked/offline commands with the supplied PROTOC:

| Target/check | Observed result |
|---|---|
| Durability API | One model semantic test passed |
| Disk `--test conformance` | Eight compiling behavioral failures on the refusing scaffold |
| Proof `--test recovery` | Six failures; one initially vacuous negative passed |
| Proof `--test process_crash --no-run` | Worker compiled |
| Python process-crash runner | Worker exited on `NotQualified` before the first kill cut; no kill/recovery PASS |
| Existing API, proof library and qualification targets | All 22 existing memory tests passed |
| `proofs/raft-adoption/check.sh` | Architecture, process-global, conditional-source and formatting checks passed; seven owned sources, zero allowlist entries |
| Clippy `--all-targets -- -D warnings` | Failed with `clippy::ok_expect` at recovery.rs:136–138 |

No files or Git state were modified by this review. Test/build activity was confined to the expressly permitted artifacts and disposable fixtures.

## 1. Findings

### [P3-1] The new recovery specification fails the required warnings-denied lint check

**Location:** `proofs/raft-adoption/proof/tests/recovery.rs:136–138`.

**Violated rule:** Root Definition Of Done requires affected-package lint checks to pass; PersistenceContract §7 also names Clippy among verification gates. Expected behavioral RED does not require a lint failure.

**Reproduction:** Run the permitted PROTOC-prefixed command:

```sh
cargo clippy --locked --offline --manifest-path proofs/raft-adoption/Cargo.toml --all-targets -- -D warnings
```

Clippy rejects `Cluster::recover(...).ok().expect(...)` with `clippy::ok_expect`. Converting the `Result` to `Option` additionally discards the recovery error from this specification’s failure diagnostic.

**Impact:** The new test target prevents the declared lint command from passing independently of the deliberately unqualified implementation. This is bounded test-source debt, not a persistence defect.

**Correction:** Call `expect("reviewed port accepts valid genesis")` directly on the `Result`; retain warnings-denied enforcement.

**Closure test:** Rerun the same Clippy command and the focused recovery target. Clippy must pass; recovery must retain the intended behavioral RED while the constructor remains a scaffold.

## 2. Invariant analysis

The following attacks did not establish a blocking inconsistency:

**Boundary allocation and dependencies.** The new contract has meaningful required `load`/`persist` operations and owned values without Raft, protobuf or runtime types. The adapter normally depends only on that contract; the harness selects disk through a development dependency. Explicit proposed classifications and edges satisfy the contract-review stage without silently authorizing production composition. Records’ application-history extension remains a scoped proposal; Supplier/Application retain external-effect responsibility.

**Complete recoverable history.** Full original Entry bytes, hard state, fixed binding and commit are carried together. Application state and terminal receipts are reconstructed from the committed prefix rather than a separately acknowledged checkpoint. RP-009/010 require semantic validation and complete replay before startup, preserve uncommitted entries for Raft reconciliation, and prohibit treating them as applied state. The specification therefore does not substitute recognizable labels or digests for protected payload.

**Monotonicity versus legal overwrite.** RP-003/004 distinguish nonregressing term/commit and same-term vote stability from replaceable uncommitted suffixes. Shared conformance exercises suffix replacement/shortening, commit-only updates, stale revisions, committed-byte mutation, committed shortening, invalid votes and gaps. The model passed this sequence. Disk conformance remains intentionally RED.

**Publication and failure semantics.** RP-005/008 explicitly require persistence before Ready messages/application/receipts and before LightReady commit-only effects. Post-write errors poison the store and stop serving; the fault matrix preserves uncertainty rather than asserting noncommit. Recovery consumes and validates the entire journal and quarantines partial or corrupt history without truncation. Exclusive creation, lifetime locking and separate open/genesis operations form a complete cooperating-writer lifecycle.

**Rollback claims.** RP-007 explicitly requires an independently trusted floor for detecting a valid older whole journal. The negative limitation and the no-floor admission test are disclosed; no comprehensive rollback-detection closure is claimed. This must remain partial RA-011 evidence, not satisfaction of its entire recovery requirement.

**Evidence maturity.** The new evidence correctly separates model success, compiling behavioral RED, a vacuous negative, disk reopen and actual process termination. The process runner’s current refusal is accurately recorded. Its explicit environment and owned worker processes do not create a production Rust process-global exception. APFS/process-crash evidence is distinguished from power-loss certification and independent physical failure domains.

**Canonical reconciliation.** The inherited amendment list remains unchanged and proposed. The cited sources support its distinctions between discovery routing, local checkout exclusion, forwarded process-crash retention and protected ordered application acceptance. The Q2 object neither supersedes those production clauses nor expands authorization through majority voting.

## 3. Risks and next action

No implementation safety conclusion follows from this GO. Malformed-length/overflow parser cases, nonvacuous recovery validation, Ready/LightReady fault ordering and the actual process-kill tier remain implementation obligations. The controlling plan’s boundary-by-boundary crash coverage must remain visible: the two named process cuts alone must not be reported as exhaustive Q2 closure.

The next action is to resolve P3-1 and, subject to the independently merged contract verdict, implement the adapter and injected host through the specified TDD sequence, followed by the separate Code/State implementation gate.
