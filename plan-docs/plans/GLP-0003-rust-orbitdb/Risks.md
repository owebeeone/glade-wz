# Risks

Plan: `GLP-0003`

| Risk | Severity | Signal | Mitigation |
| --- | --- | --- | --- |
| Canonical bytes diverge from OrbitDB JS | High | CIDs/signatures differ for the same logical entry | Fixture-first tests; port `entry.js` behavior closely; keep deltas in `Decisions.md`. |
| Append/join race creates inconsistent heads | High | Concurrent local append and remote join yield nondeterministic heads | One per-log operation sequencer; deterministic schedule tests at await boundaries. |
| Storage crash leaves unrecoverable partial state | High | Entry bytes, verified index, and heads disagree after crash | Define atomic batch or replay-repair contract; fault-inject after every storage step. |
| Document semantics are conflated | High | JS Documents compatibility is treated as proof of a new CRDT extension | Keep LWW compatibility and rust-orbitdb CRDT extension separate; model operation set first; test conflicts, indexes, deletes, causal ordering, and order-independent convergence before optimizing. |
| Sync implementation invents a protocol and calls it compatible | High | Catch-up tests pass in Rust but cannot be compared to JS OrbitDB | Use head announcements plus content-addressed get-by-CID for the compatibility path; record any custom request/response as a separate protocol delta. |
| Block-fetch transport is unowned | High | Real adapter can announce heads but cannot fetch referenced blocks | Assign D0014 block-fetch transport to `rust-orbitdb-libp2p`; test it in the in-memory libp2p harness before real adapter acceptance. |
| Data availability is silently assumed | High | Replicas fail to converge after the last holder of a referenced block departs | State the availability assumption in convergence claims; add simulator faults that expect typed unavailability instead of silent divergence. |
| Simulator gives false confidence | High | Real libp2p or JS OrbitDB conformance fails cases that simulator passed | Keep substrate trait concrete; run JS OrbitDB through Node runner; rerun same scenarios on libp2p. |
| Node runner becomes a second bespoke harness | High | JS OrbitDB tests use different scenario definitions than Rust tests | One scenario DSL; separate language bindings only for execution. |
| Million-scale sim is too memory-heavy | Medium | Memory grows with total clients/sessions instead of active or sampled state | Use compressed models, sampled exact sessions, counters for inactive clients, and configurable payload modeling. |
| High-scale failures cannot be reduced | Medium | Million-scale failure has no small reproduction | Require seed replay and shrink/minimize hooks before accepting high-scale gate. |
| libp2p API churn leaks into core | Medium | Core API starts exposing libp2p types | Adapter-only crate; cargo-tree checks; code review gate for substrate trait changes. |
| In-memory libp2p simulator is too expensive for large scale | Medium | libp2p instance overhead prevents useful stress sizes | Treat it as integration-depth simulator; keep direct simulator as million-scale path. |
| Runtime/server concerns leak into semantic crates | High | Core/store/sync/doc crates start opening sockets/files or depending on Tokio | Add I/O boundary audits; keep C10k as downstream host handoff material. |
| Python binding owns async lifecycle incorrectly | Medium | GIL stalls, runtime nesting errors, deadlocks | Keep binding thin; document runtime ownership; test with pytest and Rust integration tests. |
| Security audit is added too late | High | Fuzzing, threat model, or dependency audit finds release-blocking issue near release | Start threat model and audit gates in `P00`; make findings checkpoint blockers. |
| Rust repo quality drifts because this is the first Rust module | Medium | Inconsistent lints, ad hoc crate layout, missing CI gates | Use workspace lints, rust-toolchain, fmt/clippy/test/nextest/coverage gates from day one. |
| wasm distracts from native/simulator delivery | Medium | Browser Rust work starts before core/sim/libp2p/Python are green | Keep wasm out of primary phases; use OrbitDB JS/js-libp2p interop first. |
| Submodule metadata conflicts with existing dirty root | Low | `.gitmodules` or submodule pointer conflicts | Perform submodule addition on a dedicated branch and keep root changes limited. |
