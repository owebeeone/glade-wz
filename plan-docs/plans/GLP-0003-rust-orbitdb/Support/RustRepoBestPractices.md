# Rust Repo Best Practices

Plan: `GLP-0003`

This is expected to be the first Rust-first repository in this tree. The repo
SHOULD start strict and simple, then loosen only where a concrete requirement
forces it.

## Workspace

- Use one Cargo workspace at the submodule root.
- Pin the toolchain in `rust-toolchain.toml`.
- Add `rustfmt.toml` and keep formatting automatic.
- Put shared lint policy in workspace lints where possible.
- Keep crate features explicit. The default feature set MUST NOT include
  libp2p, Python, wasm, downstream host benchmarks, or stress-only dependencies.
- Prefer small crates with crisp ownership over a single crate with feature
  tangles.
- Keep Node.js runner code under `node/`, not mixed into Rust crates.

## Dependencies

- Use `thiserror` for library errors.
- Use `tracing` for diagnostics; avoid `println!` in libraries.
- Use `serde` only at explicit encoding or config boundaries.
- Use `proptest` for property tests.
- Do not use Tokio or any async runtime in semantic crates. Use Tokio only in
  explicit adapter boundaries or downstream host/server work unless the runtime
  study reverses that recommendation.
- Semantic crates MUST be sans-io (see `D0007`): express behavior as synchronous
  step/poll functions and state machines driven by the host or simulator, which
  supply I/O results, block fetches, timer ticks, and wakeups. Inject clocks,
  timers, and schedulers through traits.
- Semantic crate public APIs MUST NOT contain `async fn`. An `.await` point or
  embedded executor inside a semantic crate re-introduces the runtime coupling
  `D0007` forbids.
- `async-trait`, async functions, and futures belong only in adapter/host/binding
  crates (`rust-orbitdb-libp2p`, `rust-orbitdb-py`, downstream hosts). There, use
  `async-trait` only if the ergonomics are worth the dispatch cost and API
  lock-in; prefer concrete async functions or associated future patterns on hot
  paths.
- Keep core data structures runtime-neutral when practical.
- Use `libp2p` only in `rust-orbitdb-libp2p` and
  `rust-orbitdb-libp2p-sim`.
- Use PyO3/maturin only in `rust-orbitdb-py`.

## Error And API Design

- Public APIs MUST return typed errors, not stringly errors.
- Core APIs SHOULD distinguish malformed data, unauthorized data, missing data,
  storage failure, transport failure, timeout, and internal invariant failure.
- Panics MUST be reserved for impossible internal invariants. External malformed
  input MUST return an error.
- Public types SHOULD be stable and small. Avoid leaking crate internals across
  crate boundaries.
- Core, sync, document, and direct simulator public APIs MUST NOT expose libp2p
  types.
- Semantic crates MUST NOT open sockets, depend on Tokio network types, or
  perform concrete OS file I/O directly.

## Testing

- Follow TDD: failing test first, minimal implementation, refactor only while
  green.
- Use fixtures for JS compatibility and property tests for state-space pressure.
- Add deterministic seeds to every randomized test failure.
- Keep slow stress tests behind explicit features or CI jobs, but keep a small
  representative stress suite in normal CI.
- Ensure the same simulator scenario can run at `N=2`, `N=3`, scenario scale,
  and high scale.
- Convert high-scale failures into small deterministic regression tests when
  feasible.
- Prefer `cargo nextest` for fast integration test runs.
- Add coverage with `cargo llvm-cov` once the core is nontrivial.
- Consider mutation testing (`cargo mutants`) for `rust-orbitdb-core` once the
  core behavior stabilizes.

## Complexity And Scaling

- Named hot paths MUST be sub-quadratic (see `D0015`): log append, join,
  traversal, heads maintenance, conflict ordering, head exchange/diff between
  peers, document index rebuild, the simulator event loop, and compressed-state
  representation. A superlinear algorithm in any of these is a defect, not a
  later optimization.
- Head exchange MUST be bounded by the number of heads, not the number of
  entries. Preserve OrbitDB's `refs`/segment-anchor skip structure so traversal
  stays sub-linear.
- Live/working state MUST be O(active or sampled), never O(total inactive
  sessions).
- The high-N simulator sweep exists to find the scaling boundary and the
  limiting bottleneck, not to certify a fixed peer count. Record the boundary
  and bottleneck; do not encode an arbitrary scale target as a pass/fail gate.

## CI

Minimum CI gates:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --no-default-features
cargo test --workspace
cargo nextest run --workspace
```

The `--no-default-features` gates are required to prove semantic crates do not
need adapter/runtime features. The `--all-features` gates are still useful, but
they MUST NOT be used as evidence that semantic crates are I/O-free.

Recommended CI gates:

```bash
cargo deny check
cargo audit
cargo llvm-cov --workspace
npm test --prefix node/rust-orbitdb-sim-node
```

Stress CI SHOULD split into:

- normal PR: deterministic small simulator and a small fixed seed set
- nightly: larger Monte Carlo seed sweep
- manual: million-scale compressed simulation
- manual: in-memory libp2p simulator scale probe

## Unsafe Code

The workspace SHOULD start with `#![forbid(unsafe_code)]` in core, store, sync,
substrate, direct sim, document CRDT, fixtures, and testkit crates. If unsafe
becomes necessary, it MUST be isolated, documented, and reviewed with tests that
exercise the safety boundary.

## Release And Versioning

- Use semver even before public release.
- Keep changelog entries for API-affecting changes.
- Record protocol/schema changes in plan decisions and later in the repository's
  own design docs.
- Do not publish crates until fixture compatibility, simulator gates, and
  security gates are stable.
