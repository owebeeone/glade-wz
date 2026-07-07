# Workstreams

Plan: `GLP-0003`

| Workstream | Owner | Write Scope | Must Not Touch | Output | Merge Risk |
| --- | --- | --- | --- | --- | --- |
| Coordination | maintainer / coordinating agent | `plan-docs/plans/GLP-0003-rust-orbitdb/`, registry rows | Implementation crates unless also assigned | Accepted plan, checkpoint updates, decisions, risk updates | Low; root docs only |
| Submodule bootstrap | TBD | root `.gitmodules`, `rust-orbitdb/` initial commit | Existing submodules, `third-party/` | New first-party Rust workspace pinned as submodule | Medium; root submodule metadata can conflict |
| Core conformance | TBD | `rust-orbitdb/crates/rust-orbitdb-core/`, fixtures, core tests | libp2p adapter, Python binding, Node runner | Entry/clock/order/head/log behavior matching JS fixtures | High; canonical bytes and signatures are brittle |
| Storage and sequencing | TBD | `rust-orbitdb/crates/rust-orbitdb-store/`, core operation sequencer tests | transport adapter crates | Atomicity/replay contract and crash-injecting tests | High; failures can look like network bugs |
| Document CRDT | TBD | `rust-orbitdb/crates/rust-orbitdb-doc-crdt/`, model tests | transport adapters except conformance hooks | Multi-writer document semantics, conflict handling, indexes, convergence tests | High; semantics affect API and storage |
| Sync state machine | TBD | `rust-orbitdb/crates/rust-orbitdb-sync/`, `rust-orbitdb-substrate/` | `rust-orbitdb-libp2p/` except trait implementation needs | Transport-independent head exchange, one-shot head transfer, and content-addressed fetch | Medium; API shape affects both simulators and libp2p |
| Direct simulator and stress | TBD | `rust-orbitdb/crates/rust-orbitdb-sim/`, `rust-orbitdb-testkit/`, stress tests | libp2p internals | Deterministic in-memory network and Monte Carlo suite | High; simulator can become too abstract to falsify claims |
| JS OrbitDB runner | TBD | `rust-orbitdb/node/rust-orbitdb-sim-node/`, trace fixtures | vendored JS OrbitDB source | Node plugin that drives JS OrbitDB through simulator scenarios | High; validates the test harness itself |
| libp2p simulator | TBD | `rust-orbitdb/crates/rust-orbitdb-libp2p-sim/` | core public APIs except reviewed trait changes | In-memory libp2p semantics simulator with simulated links/failures and D0014 block-fetch transport tests | High; may expose libp2p scaling limits |
| libp2p adapter | TBD | `rust-orbitdb/crates/rust-orbitdb-libp2p/` | core/sync/document public APIs except reviewed trait changes | Adapter conformance over `rust-libp2p`, including D0014 block-fetch transport | Medium; dependency churn, block-fetch protocol, and browser transport complexity |
| Runtime handoff | TBD | `Support/NetworkingRuntimeStudy.md` | semantic crates and adapter implementation | Tokio/mio C10k guidance for downstream host/server work | Medium; runtime concerns can drift back into core |
| Security/audit | TBD | `rust-orbitdb/tests/security/`, audit docs, fuzz/model-check config, dependency-boundary audit scripts | product integration code | Threat model, fuzzing, dependency audits, SBOM, proof-oriented invariant checks, semantic-crate libp2p ban | High; release-blocking by design |
| Python binding | TBD | `rust-orbitdb/crates/rust-orbitdb-py/`, Python tests | core semantics except through reviewed API requests | PyO3/maturin package and smoke tests | Medium; async/GIL boundary mistakes |
| Generic application scenarios | TBD | `rust-orbitdb/tests/integration/`, scenario fixtures | downstream product names/contracts | Product-agnostic interactive-session scenarios | Medium; avoid leaking downstream naming |
