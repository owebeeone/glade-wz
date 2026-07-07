# Requirement Trace

Plan: `GLP-0003`

| Requirement | Primary Tests | Supporting Tests |
| --- | --- | --- |
| `ROR-001` new submodule | Root submodule status check; submodule CI | Root registry/checkpoint review |
| `ROR-002` no libp2p in core/default crates | `dependency-boundary-audit`; per-crate `cargo tree`/`cargo deny` ban for core/store/sync/substrate/sim/doc/fixtures/testkit | Code review on public APIs; all-feature workspace builds only for adapter checks |
| `ROR-003` JS entry compatibility | JS fixture encode/decode/entry-version/CID/signature tests | Tamper tests, malformed fixture tests |
| `ROR-004` oplog behavior | Ported OrbitDB append/join/heads/iterator/conflict tests | Proptest over random DAGs and clocks |
| `ROR-005` serialization/atomicity | Schedule tests for append/join; crash-injecting store tests | Replay/recovery regression fixtures |
| `ROR-006` transport-independent sync | Fake transport head announcement, one-shot head transfer, content-addressed fetch, and idempotence conformance | Direct simulator, libp2p simulator, real libp2p scenario replay, and disagreement classification |
| `ROR-007` direct simulator fault model | Deterministic sim scenarios for drop/duplicate/reorder/partition/reconnect | Seed replay and minimized regression cases |
| `ROR-008` scale parameter + boundary/complexity (D0015) | Same scenario at exact `N=2`, exact `N=3`, sampled exact high-N, and compressed high-N | Named invariant bridge, sub-quadratic hot-path assertions, O(active) state assertion, scaling-boundary/bottleneck report, and shrink-to-unit regressions |
| `ROR-009` swappable libp2p | Public API scan for libp2p types; adapter conformance | Feature matrix tests with and without libp2p |
| `ROR-010` Python binding | maturin smoke tests; pytest append/join/doc/sim tests | Runtime/GIL boundary tests |
| `ROR-011` browser interop first | OrbitDB JS/js-libp2p fixture interop test | Optional wasm evaluation report |
| `ROR-012` document semantics | JS Documents LWW compatibility fixtures; CRDT model tests; concurrent update/delete/index order-independent convergence tests under named availability assumptions | Direct simulator convergence under partition/heal; explicit JS-vs-extension delta tests; data-unavailability negative faults |
| `ROR-013` JS OrbitDB through simulator | Early JS smoke; Node runner executes JS OrbitDB with shared scenario definitions | Trace comparison and harness mutation tests before high-scale work |
| `ROR-014` generic application adaptation | Product-agnostic interactive-session scenario tests | Negative downstream-name string tests; replay/live-tail cutover and reattach tests |
| `ROR-015` in-memory libp2p simulator | libp2p-facing adapter runs over in-memory links/failures | Scaling probes and conformance replay |
| `ROR-016` no direct I/O in semantic crates | API/dependency audit for `std::net`, Tokio network types, libp2p, and concrete OS file I/O | Downstream Tokio/mio C10k handoff study |
| `ROR-017` security/auditability | Threat model, fuzz tests, dependency audit, SBOM, invariant model tests | Formal or bounded verification where feasible |
| `ROR-018` third-party read-only | Fixture generation no-write check; clean `third-party/` status | Upstream PR decision log if needed |
| `ROR-019` content-addressed replication | get-by-CID/storage fetch conformance with missing, malformed, unavailable, unauthorized, timeout, and cancellation cases | JS-equivalence exclusion test for head/entry catch-up extensions; D0014 block-fetch transport tests |
| `ROR-020` compatibility surface inventory | Identity/access/storage/encryption source inventory and fixture plan | Decision log for in-scope/out-of-scope surfaces before implementation |
| `ROR-021` real-adapter block fetch ownership | D0014-selected block-fetch transport tests in libp2p memory harness and real adapter | CID/block compatibility fixtures; no bitswap wire claim without explicit dependency decision |
