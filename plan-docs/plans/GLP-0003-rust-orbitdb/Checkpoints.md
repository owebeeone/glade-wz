# Checkpoints

Plan: `GLP-0003`

## Roll-Build Criteria

This file is the roll-build plan for `rust-orbitdb`.

Each implementation checkpoint SHOULD be small enough that an agent or human can
hold the whole change in context. The approximate target is **less than 500 LOC
of implementation change** per checkpoint. This is not a hard metric; it is a
reasoning guardrail. If a checkpoint is too broad, split it into `a`, `b`, `c`
sub-checkpoints before implementation.

Every checkpoint MUST follow TDD:

1. Add failing tests, fixtures, or scenario definitions first.
2. Implement the smallest change that makes those tests pass.
3. Run focused verification.
4. Commit and tag only when the checkpoint goal is actually met.
5. Record the completed tag, verification, and rollback notes here.

## Tag Shape

Use meaningful tags:

Tags are namespaced under the plan id with a slash: `glp-0003/<phase>-<checkpoint>`.

```text
glp-0003/p00a-repo-contract
glp-0003/p03c-log-append
glp-0003/p08b-scale-parameter
```

Commit trailers SHOULD include:

```text
Plan: GLP-0003
Phase: P03c
Checkpoint: log-append
```

## Roll-Build Sequence

| Checkpoint | Phase | Approx size | Required evidence | Stop / split condition |
| --- | --- | ---: | --- | --- |
| `repo-contract` | `P00a` | docs only | Plan accepted; `rust-orbitdb` name/repo/path confirmed | Stop if first-party path or repo URL is disputed |
| `submodule-bootstrap` | `P00b` | <500 LOC | `rust-orbitdb` submodule added and pinned; empty workspace commit exists | Stop if remote repo is unavailable |
| `workspace-ci-skeleton` | `P00c` | <500 LOC | Cargo workspace, pinned toolchain, fmt/clippy/test/nextest CI wired; gates run green from the first crate (P01a) — no empty placeholder crates manufactured | Split if CI, crate layout, and docs exceed one checkpoint |
| `security-threat-model` | `P00d` | docs + <200 LOC | Threat model skeleton exists with trust boundaries, key assumptions, and release-blocking finding policy | Stop if trust boundaries or the release-blocking finding policy cannot be stated concretely |
| `security-invariant-list` | `P00e` | docs + <200 LOC | Initial invariant list maps encoding, signature, log, storage, document, and simulator invariants to planned tests | Stop if invariants are too vague to test |
| `security-audit-placeholders` | `P00f` | <200 LOC | `cargo deny`, `cargo audit`, fuzz, and SBOM placeholders are wired but may be empty | Stop if audit tools cannot be made reproducible |
| `formal-verification-scope` | `P00g` | docs + <200 LOC | First model-checking or bounded-verification target, tool candidate, owner, and artifact path are recorded | Stop if proof-oriented claims have no producing checkpoint |
| `complexity-discipline-contract` | `P00h` | docs only | Named hot paths (append/join/traverse/heads/conflict-order/head-exchange/doc-index/sim-loop/compressed-state) are listed with their required sub-quadratic bound; the O(active)-not-O(total) state rule and seed-reporting format are recorded (see D0015) | Stop if a hot path has no statable complexity bound |
| `third-party-readonly-gate` | `P00i` | <200 LOC | Fixture scripts prove read-only use of `third-party/`; no vendored writes | Stop if upstream PR work is needed without a decision |
| `dependency-boundary-audit` | `P00j` | docs + <300 LOC | `cargo deny` bans and/or `xtask dependency-boundary-audit` check libp2p absence per semantic crate | Stop if per-crate dependency graphs cannot be checked separately from all-feature adapter builds |
| `scenario-dsl-schema` | `P01a` | <500 LOC | Versioned scenario DSL schema, minimal parser tests, and trace schema tests fail before implementation, then pass | Stop if simulator and JS runner would need different scenario models |
| `fixture-format` | `P01b` | <500 LOC | Fixture schema and corpus loader tests fail before implementation, then pass | Split if schema and generator both grow large |
| `entry-fixtures` | `P01c` | <500 LOC | JS OrbitDB entry encode/decode/CID/signature fixtures generated; the operation envelope `{op,key,value}` and op vocabulary (PUT/DEL/ADD) are locked as the typed payload contract (D0016) | Stop if fixture generation is not reproducible or the envelope is not pinned |
| `append-fixtures` | `P01d` | <500 LOC | Append/clock/head fixtures generated | Stop if append fixture ordering is ambiguous |
| `join-fixtures` | `P01e` | <500 LOC | Join/duplicate/missing-dependency fixtures generated | Stop if join fixture graph is not reproducible |
| `iterator-conflict-fixtures` | `P01f` | <500 LOC | Iterator/refs/conflict-order fixtures generated | Stop if iterator semantics are undefined |
| `document-fixtures` | `P01g` | <500 LOC | JS Documents put/del/get/query/iterator/custom-index/replication fixtures generated | Stop if document baseline source is disputed |
| `js-node-runner-contract` | `P01h` | <500 LOC | Node.js plugin API contract and failing harness tests exist | Stop if JS OrbitDB cannot be loaded without patching vendored code |
| `js-node-runner-smoke` | `P01i` | <500 LOC | JS OrbitDB runs one minimal scenario through the shared DSL and emits a trace | Stop if the harness cannot test JS early |
| `entry-types` | `P02a` | <500 LOC | Rust entry field model and malformed input tests pass; payload is the typed D0016 operation envelope, not an opaque blob | Stop if type model diverges from JS fixtures or the envelope contract |
| `dag-cbor-encode` | `P02b` | <500 LOC | dag-cbor encode/decode/canonicalization fixture tests pass | Stop if IPLD canonical form is ambiguous |
| `cid-multihash-multibase` | `P02c` | <500 LOC | CID, multihash, and multibase fixture tests pass | Stop if JS fixture CIDs cannot be reproduced |
| `identity-model` | `P02d` | docs + <300 LOC | Identity, key format, provider, access-controller, and double-signature assumptions are documented with failing tests | Stop if identity source of truth is ambiguous |
| `signature-verify` | `P02e` | <500 LOC | Sign/verify/tamper tests pass against fixtures | Stop if key format is ambiguous |
| `clock-conflict-order` | `P02f` | <500 LOC | Lamport clock and conflict ordering property tests pass | Stop if zero comparator behavior differs |
| `operation-contracts` | `P03a` | docs + <200 LOC | Append/join/iterator/storage atomicity contracts are written before implementation | Stop if iterator snapshot vs weak semantics are unresolved |
| `store-traits` | `P03b` | <500 LOC | Storage trait and memory store tests pass | Split memory store from trait if needed |
| `store-failure-model` | `P03c` | <500 LOC | Failure-mode tests for storage errors and partial writes pass | Stop if failure classes are undefined |
| `heads-maintenance` | `P03d` | <500 LOC | Heads add/remove/find tests pass against fixtures and model | Stop if read-modify-write semantics are undefined |
| `log-append` | `P03e` | <500 LOC | Append tests pass with per-log operation sequencer | Stop if append and store atomicity interact unexpectedly |
| `log-join` | `P03f` | <500 LOC | Join, duplicate, malformed dependency, and access failure tests pass | Split traversal dependencies if needed |
| `iterator-refs` | `P03g` | <500 LOC | Iterator/traverse/refs tests pass with the predeclared consistency contract | Stop if implementation violates the contract |
| `crash-replay` | `P03h` | <500 LOC | Crash-injecting store tests recover or fail by contract | Stop if replay-repair contract is undefined |
| `security-signature-review` | `P03i` | docs + <200 LOC | Signature/key/canonicalization review notes exist before document CRDT work | Stop on unreviewed key-handling ambiguity |
| `doc-compat-model` | `P04a` | docs + <300 LOC | OrbitDB Documents compatibility model and fixtures are documented with failing tests; the store get/set/query/iterator/all API and LWW reduction are pinned per D0016 | Stop if JS Documents baseline or store API contract is disputed |
| `doc-crdt-model` | `P04b` | docs + <300 LOC | Multi-writer document CRDT operation model and invariants documented with failing tests | Stop if CRDT operation set is disputed |
| `doc-crdt-put-delete` | `P04c` | <500 LOC | Concurrent whole-document put/delete/tombstone tests pass | Split put and delete if needed |
| `doc-crdt-update-conflicts` | `P04d` | <500 LOC | Concurrent update conflict tests pass under accepted ordering rules | Stop if conflict rule differs from D0009 |
| `doc-crdt-indexes` | `P04e` | <500 LOC | Document query/index convergence tests pass; reads are index-backed via the apply/update reduction, not scan-per-get (D0016/D0015) | Stop if index semantics hide CRDT conflicts or reads regress to full-log scans |
| `doc-crdt-exact-replica` | `P04f` | <500 LOC | Multi-replica convergence tests pass using exact local model, without direct simulator dependency | Stop if exact model cannot express convergence |
| `substrate-types-errors` | `P05a` | <500 LOC | Neutral transport-independent types and typed error tests pass; the trait is sans-io (synchronous step/poll, no `async fn`) per D0007 | Stop if libp2p concepts/types leak into public APIs or the trait requires an executor |
| `fake-transport-delivery-contract` | `P05b` | <500 LOC | Fake transport tests prove advisory heads may drop, duplicate, delay, and reorder | Stop if delivery guarantees are undefined |
| `sync-head-exchange` | `P05c` | <500 LOC | Head announcement, receive, and one-shot head transfer tests pass | Stop if head delivery semantics are ambiguous |
| `content-addressed-fetch` | `P05d` | <500 LOC | Missing transitive entries are resolved by get-by-CID/storage fetch with typed missing/malformed/unavailable failures under the modeled availability assumption | Stop if storage/substrate ownership is unclear |
| `sync-duplicates-reorder` | `P05e` | <500 LOC | Duplicate and reorder delivery tests pass | Stop if idempotence state is ambiguous |
| `sync-timeout-retry` | `P05f` | <500 LOC | Timeout and retry tests pass | Stop if retry budget is undefined |
| `sync-lifecycle` | `P05g` | <500 LOC | Restart and stop/start lifecycle tests pass | Stop if lifecycle races mutate state incorrectly |
| `sim-event-loop-seed` | `P06a` | <500 LOC | Deterministic event loop and seed replay tests pass | Stop if failures are not reproducible |
| `sim-node-lifecycle` | `P06b` | <500 LOC | Node start/stop/restart lifecycle tests pass in the simulator | Stop if lifecycle cannot be isolated from delivery faults |
| `direct-sim-delivery-faults` | `P06c` | <500 LOC | Drop/duplicate/reorder/delay tests pass | Split delay/backpressure if needed |
| `direct-sim-partitions` | `P06d` | <500 LOC | Partition/heal/asymmetric reachability tests pass | Stop if membership model is unclear |
| `direct-sim-storage-faults` | `P06e` | <500 LOC | Storage crash boundary events run through scenarios | Stop if crash faults cannot be scheduled deterministically |
| `direct-sim-data-availability` | `P06f` | <500 LOC | Last-holder-departs and unreachable-block faults report typed unavailability instead of silent divergence | Stop if convergence assumptions are not expressible in scenarios |
| `direct-sim-doc-convergence` | `P06g` | <500 LOC | Document CRDT convergence scenarios pass under direct simulator with modeled data availability | Stop if P04 exact model and simulator disagree |
| `direct-sim-shrinking` | `P06h` | <500 LOC | Failing high-scale scenario can produce smaller replay fixture | Stop if shrink output is not actionable |
| `node-plugin-load` | `P07a` | <500 LOC | Node package loads JS OrbitDB and simulator plugin without patching vendored sources | Stop if supply-chain or module loading is not reproducible |
| `scenario-drive-trace` | `P07b` | <500 LOC | Node plugin drives one selected scenario and emits a trace under the shared DSL | Stop if JS OrbitDB requires real network APIs for selected tests |
| `js-trace-schema` | `P07c` | docs + <300 LOC | Comparable fields, ignored fields, timing buckets, and failure classes are normative | Stop if traces cannot be compared deterministically |
| `js-trace-compare` | `P07d` | <500 LOC | Rust and JS runs emit comparable traces for selected scenarios | Split scenario families if needed |
| `js-harness-regression` | `P07e` | <500 LOC | A known bad simulator mutation fails against JS OrbitDB in CI | Stop if the test cannot test the test |
| `scale-parameter` | `P08a` | <500 LOC | Same scenario runs at `N=2`, `N=3`, and configured higher N | Stop if high-scale path forks from small-scale path |
| `compressed-exact-bridge` | `P08b` | <500 LOC | Compressed runs preserve exact-run invariants for `N=2`/`N=3` and sampled exact sessions | Stop if compressed state hides semantic failures |
| `compressed-state` | `P08c` | <500 LOC | Compressed counters/sampled exact sessions pass aggregate invariants | Split counters from sampled exact sessions if needed |
| `scale-boundary-probe` | `P08d` | <500 LOC | High-N sweep records the scaling boundary and the limiting bottleneck, with seed, memory, latency, throughput per N; named hot paths shown sub-quadratic | Stop if resource use grows with total inactive sessions or a hot path is superlinear |
| `high-scale-to-unit` | `P08e` | <500 LOC | At least one seeded high-scale failure path is captured as small regression | Stop if failures cannot be minimized |
| `libp2p-memory-harness` | `P09a` | <500 LOC | In-memory libp2p transport/swarms or compatible instances run without real TCP/UDP | Stop if adapter requires real sockets |
| `libp2p-link-failures` | `P09b` | <500 LOC | Simulated link drop/reorder/partition/failure modes affect libp2p-facing adapter | Split link model from swarm harness if needed |
| `libp2p-scale-probe` | `P09c` | <500 LOC | In-memory libp2p sim probes scaling limits and records bottlenecks | Stop if libp2p instance cost prevents useful scale |
| `libp2p-block-fetch-harness` | `P09d` | <500 LOC | D0014 block-fetch transport runs over in-memory libp2p links with missing/malformed/unavailable cases | Stop if selected block-fetch wire protocol cannot be tested without real sockets |
| `real-libp2p-adapter` | `P10a` | <500 LOC | Adapter implements substrate trait with feature-gated libp2p dependency | Stop if public core APIs need libp2p types |
| `real-libp2p-heads` | `P10b` | <500 LOC | Head announcement scenarios pass against real rust-libp2p | Stop if adapter disagrees with neutral substrate |
| `real-libp2p-content-fetch` | `P10c` | <500 LOC | D0014 content-addressed missing-entry fetch scenarios pass against real rust-libp2p adapter; compatibility is asserted at CID/block layer | Split fetch/backpressure if needed |
| `real-libp2p-lifecycle` | `P10d` | <500 LOC | Restart/disconnect/reconnect scenarios pass against real rust-libp2p | Stop if lifecycle behavior diverges from simulator |
| `io-boundary-audit` | `P11a` | docs + <200 LOC | Core/store/sync/doc/direct-sim crates audited as free of direct OS/network I/O, free of any async-runtime dependency, and free of `async fn` in public APIs (sans-io per D0007) | Stop if semantic crates need sockets, concrete file I/O, or an executor |
| `runtime-study-handoff` | `P11b` | docs only | Tokio/mio C10k study marked as downstream host/server guidance, not core implementation | Stop if the plan still implies rust-orbitdb is a TCP server |
| `python-api-contract` | `P12a` | <500 LOC | PyO3 API shape and failing smoke tests exist | Stop if binding wants to own core semantics |
| `python-smoke` | `P12b` | <500 LOC | maturin + pytest append/join/document/sim smoke tests pass | Split runtime integration if needed |
| `generic-session-negative-names` | `P13a` | <200 LOC | Tests scan public scenario/API strings for banned downstream product names | Stop if product naming enters API |
| `generic-session-open-io` | `P13b` | <500 LOC | Product-agnostic open/input/output-log contract tests fail then pass | Split input and output if needed |
| `generic-session-replay` | `P13c` | <500 LOC | Replay and cursor scenario passes in direct sim | Stop if replay semantics are ambiguous |
| `generic-session-reattach` | `P13d` | <500 LOC | stop/restart/reattach/resize-control scenario passes in direct sim | Split replay and live-tail cutover if needed |
| `browser-js-interop` | `P14a` | <500 LOC | OrbitDB JS/js-libp2p browser fixture interop passes | Stop if Rust wasm is being used to mask JS interop failure |
| `wasm-evaluation` | `P14b` | docs + <300 LOC | Rust wasm feasibility report records bundle/storage/runtime/transport risks | Defer if native/sim/JS paths are not green |

## Completion Log

Record completed roll-build checkpoints here:

| Checkpoint | Tag | Commit | Verification | Notes |
| --- | --- | --- | --- | --- |
| `repo-contract` (P00a) | — (root plan-docs, untagged) | `a250c5d` (glial-dev) | Plan + all decisions (D0001-D0016) accepted | Docs-only checkpoint; lives in root, no rust-orbitdb tag |
| `submodule-bootstrap` (P00b) | `glp-0003/p00b-submodule-bootstrap` (rust-orbitdb) | `9541d92` (rust-orbitdb), pinned by `2b88755` (glial-dev) | Empty Cargo workspace pushed to origin/main; submodule added and pinned | Root carries only `.gitmodules` entry + submodule pin |
