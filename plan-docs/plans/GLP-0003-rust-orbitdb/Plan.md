# GLP-0003: rust-orbitdb

Status: deferred (long-term; near-term moves on an off-the-shelf Go/JS path — see State.md)
Owner: maintainer coordinating; implementation owners TBD by workstream
Affected modules: root `plan-docs`, new first-party submodule
`rust-orbitdb`, host bindings, optional `rust-libp2p` adapter

## Goal

Create `rust-orbitdb`: a Rust-first OrbitDB-compatible oplog, sync, simulator,
document CRDT, and binding workspace. The port MUST preserve the small OrbitDB
semantic core, MUST avoid coupling core semantics to libp2p, and MUST ship with
test simulators that can drive correctness, fault, churn, throughput, latency,
and massive Monte Carlo tests.

The semantic rust-orbitdb crates MUST NOT do OS/network I/O directly. They expose
pure data structures, state machines, traits, fixtures, and deterministic
simulators. Real I/O belongs only in explicit adapter or host layers outside the
semantic core, such as a feature-gated libp2p adapter, Node.js runner, Python
binding boundary, or downstream server/host plan.

This is not a massive source port, but it is not only a 1,498 LOC rewrite. The
critical vendored JS spine is about 1,498 raw LOC, with about 1,181 raw LOC
before sync. That count excludes the compatibility surface around dag-cbor, CID,
multiformats, identity providers, keystore behavior, access controllers,
content-addressed block storage, encryption paths, and libp2p adapter behavior.
The work is small in source size and high in correctness pressure: canonical
encoding, signature compatibility, deterministic ordering, storage atomicity,
append/join serialization, transport nondeterminism, and document semantics.

## Core Claim

The implementation SHOULD be structured as:

```text
rust-orbitdb submodule
  -> OrbitDB-compatible Rust core
  -> storage traits and deterministic operation sequencing
  -> transport-independent sync state machine
  -> multi-writer document CRDT semantics
  -> direct in-memory simulator implementing the same substrate trait
  -> in-memory libp2p semantics simulator for adapter/integration tests
  -> optional real libp2p adapter crate
  -> optional Python binding crate
  -> Node.js runner for validating JS OrbitDB through the simulator
```

The simulators are not auxiliary test code. They are first-class modules and the
primary acceptance gates. The real libp2p adapter MUST be swappable with the
direct simulator and the in-memory libp2p semantics simulator without changing
core, sync, storage, document, or public API code.

## Non-Goals

- Do not blindly port OrbitDB database wrapper internals, manifests,
  access-controller plumbing, or IPFS storage as runtime dependencies.
- Do not make `rust-libp2p` a dependency of the core, storage, sync, direct
  simulator, fixtures, or document CRDT crates.
- Do not implement a TCP server or direct OS socket path in the semantic
  rust-orbitdb crates.
- Do not make wasm or browser Rust networking a first delivery gate.
- Do not port `py-libp2p` to browser APIs.
- Do not treat million-scale simulation as a separate feature from small-scale
  testing. The same simulator infrastructure MUST run at `N=2`, `N=3`,
  scenario scale, and million-peer scale by parameter.

## Inputs

- `third-party/orbitdb/src/oplog/log.js`
- `third-party/orbitdb/src/oplog/entry.js`
- `third-party/orbitdb/src/oplog/heads.js`
- `third-party/orbitdb/src/oplog/conflict-resolution.js`
- `third-party/orbitdb/src/oplog/clock.js`
- `third-party/orbitdb/src/oplog/oplog-store.js`
- `third-party/orbitdb/src/sync.js`
- `third-party/orbitdb/src/orbitdb.js`
- `third-party/orbitdb/src/database.js`
- `third-party/orbitdb/src/manifest-store.js`
- `third-party/orbitdb/src/databases/documents.js`
- `third-party/orbitdb/src/identities/identities.js`
- `third-party/orbitdb/src/identities/identity.js`
- `third-party/orbitdb/src/identities/providers/*`
- `third-party/orbitdb/src/key-store.js`
- `third-party/orbitdb/src/access-controllers/*`
- `third-party/orbitdb/src/storage/*`
- `third-party/orbitdb/test/databases/documents.test.js`
- `third-party/orbitdb/test/databases/replication/documents.test.js`
- `third-party/rust-libp2p` as read-only reference material for adapter and
  in-memory libp2p simulator design
- Official Rust/Tokio/mio/libp2p ecosystem documentation referenced in
  `Support/NetworkingRuntimeStudy.md`

## Requirements

| ID | Requirement | Acceptance test |
| --- | --- | --- |
| `ROR-001` | The OrbitDB semantic core MUST be implemented in Rust in a new first-party git submodule named `rust-orbitdb`. | Root `.gitmodules` contains `rust-orbitdb`; submodule CI runs `cargo test --workspace`. |
| `ROR-002` | Core, storage, sync, direct simulator, fixtures, and document CRDT crates MUST NOT depend on libp2p. | Per-crate dependency audit proves `cargo tree -p <semantic-crate>` contains no `libp2p`; `cargo deny` or an equivalent `xtask` gate enforces the ban for semantic crates. |
| `ROR-003` | Entry encoding, entry version compatibility, CID, and signature verification MUST match OrbitDB JS fixtures. | JS fixture conformance tests pass for encode, decode, entry `v:2`, CID, sign, verify, and tamper rejection; unsupported historical versions are explicit deltas. |
| `ROR-004` | Append, join, heads, traversal, ordering, and refs MUST match the critical OrbitDB oplog behavior unless a difference is explicitly recorded. | Rust tests ported from OrbitDB append/join/heads/iterator/conflict tests pass, with deltas in `Decisions.md`. |
| `ROR-005` | The Rust core MUST define append-vs-join serialization and storage atomicity. | Deterministic schedule tests interleave append/join at each await boundary; crash-injection tests recover or fail by contract. |
| `ROR-006` | Sync MUST be a transport-independent state machine. | The same head announcement, one-shot head transfer, content-addressed block fetch, retry, and idempotence tests pass with fake transport, direct simulator, in-memory libp2p simulator, and real libp2p adapter conformance harnesses; disagreement between layers becomes a regression before proceeding. |
| `ROR-007` | The direct in-memory simulator MUST model bounded streams, point-to-point frames, announcements, drops, duplicates, reordering, partitions, reconnects, churn, and random starts/stops. | `rust-orbitdb-sim` fault suites pass with deterministic seeds recorded on failure. |
| `ROR-008` | The simulator infrastructure MUST support `N=2` through high-scale Monte Carlo runs by parameter, using compressed models at high scale, to characterize the scaling boundary and enforce complexity discipline (see D0015). | Exact `N=2`/`N=3` runs and compressed runs preserve the same invariant set; named hot paths are shown sub-quadratic and live state stays O(active); a scaling sweep records where memory/latency/throughput degrade and the limiting bottleneck. No fixed peer-count or absolute memory/time budget is required to pass. |
| `ROR-009` | The real libp2p adapter MUST be loose and replaceable. | No libp2p type appears in public core/sync/document APIs; adapter implements only the substrate trait boundary. |
| `ROR-010` | Python binding MUST wrap stable Rust API boundaries without owning core semantics. | PyO3/maturin package smoke tests open a log, append, join fixture data, and run simulator-backed sync. |
| `ROR-011` | Browser path SHOULD use existing OrbitDB JS/js-libp2p first for cheap interop; Rust wasm MAY be a later portability proof. | Browser interop test proves fixture compatibility against OrbitDB JS before any Rust wasm networking work. |
| `ROR-012` | Document semantics MUST cover both OrbitDB-compatible LWW Documents behavior and the rust-orbitdb multi-writer CRDT extension. | OrbitDB Documents compatibility fixtures pass for oplog-reduced LWW behavior; the accepted rust-orbitdb CRDT model has operation, conflict, index, delete, and order-independent convergence tests across Rust and simulator fixtures under modeled delivery and data-availability assumptions, with JS comparison only where the JS baseline actually exists. |
| `ROR-013` | The simulator MUST be able to drive JS OrbitDB through a Node.js plugin to test the test harness itself. | JS OrbitDB runs an early smoke scenario and selected conformance scenarios through the same scenario definitions, emits comparable traces, and fails at least one deliberate harness mutation before high-scale testing. |
| `ROR-014` | Application adaptation scenarios MUST be tested without naming any downstream product. | Generic interactive-session scenarios cover `open`, input stream, output log, replay, resize/control, stop, restart, and reattach; tests scan for banned downstream product strings. |
| `ROR-015` | A second simulator variant SHOULD exercise libp2p semantics in memory. | In-memory libp2p swarms or libp2p-compatible instances communicate through simulated links/failures without real TCP/UDP. |
| `ROR-016` | The semantic rust-orbitdb crates MUST do no direct OS/network I/O; C10k/Tokio work is downstream host/runtime guidance. | API/dependency audit shows no `std::net`, Tokio network, or OS socket use in core/store/sync/doc/direct-sim crates; runtime study is accepted as handoff material only. |
| `ROR-017` | Security, auditability, and proof-oriented correctness MUST be first-class plan outputs. | Threat model, security test suite, fuzzing, dependency audit, SBOM, and formal/model-checking artifacts exist before release. |
| `ROR-018` | Vendored `third-party/` sources MUST be read-only unless an explicit upstream-PR decision is made. | Fixture generation reads vendored code without modifying it; any upstream PR work uses a separate fork/clone and a recorded decision. |
| `ROR-019` | OrbitDB-compatible replication MUST use head announcements plus content-addressed block fetch for missing transitive entries. | Sync tests prove heads are advisory signals and missing entries are resolved by get-by-CID/storage fetch with typed missing, malformed, unavailable, unauthorized, timeout, and cancellation cases; any custom head/entry request-response catch-up is recorded as a protocol delta and excluded from JS equivalence claims. |
| `ROR-020` | Identity, access-controller, storage, and encryption compatibility surfaces MUST be inventoried before implementation depends on them. | Source inventory and fixture plan identify required identity double-signature, key format, access-control, block-store, and encryption-path behaviors, with explicit in-scope/out-of-scope decisions. |
| `ROR-021` | Real-adapter block fetch MUST have an assigned wire transport and a scoped compatibility claim. | `D0014` selects the initial over-the-wire block-fetch transport; adapter tests prove CID/block compatibility, and no bitswap/IPFS wire-compatibility claim is made unless an explicit dependency decision adds that stack. |

## Document Baseline

rust-orbitdb has two document baselines:

1. OrbitDB compatibility baseline: `third-party/orbitdb/src/databases/documents.js`
   and its document/replication tests define the existing JS Documents behavior:
   `PUT`, `DEL`, `get`, `query`, `iterator`, `all`, custom `indexBy`, and
   replication convergence. This baseline is an oplog-reduced, keyed
   last-write-wins document store, not a field-merge CRDT. Compatibility
   fixtures MUST be generated before the Rust document implementation.
2. rust-orbitdb CRDT baseline: `D0009` MUST define the accepted multi-writer
   document CRDT operation model before implementation. The initial proposed
   default is OrbitDB-faithful total-order whole-document registers and
   tombstones keyed by document id, ordered by the accepted oplog conflict
   rules. Observed-remove or field-level merge semantics MAY be added only after
   the whole-document model is specified and tested.

Any difference between JS Documents compatibility behavior and the rust-orbitdb
CRDT extension MUST be explicit in `Decisions.md` and covered by fixtures. The
CRDT convergence gate MUST state that replicas converge to the same document
state regardless of delivery order, provided the modeled delivery and
data-availability assumptions are eventually satisfied and every entry/block
referenced by a delivered head remains retrievable from at least one reachable
peer or storage tier within the modeled fetch window. If that data-availability
assumption is violated, the system MUST report typed unavailability rather than
silently diverge.

## Proposed Submodule

The new module SHOULD be first-party, not `third-party/`, because it will carry
the Rust implementation, simulators, Node.js runner, Python binding, security
policy, and release policy.

Proposed path and repository:

```text
path: rust-orbitdb
repo: git@github.com:owebeeone/rust-orbitdb.git
branch prefix: codex/
```

Initial root-repo execution sequence:

```bash
git switch -c codex/glp-0003-rust-orbitdb
git submodule add git@github.com:owebeeone/rust-orbitdb.git rust-orbitdb
git submodule status rust-orbitdb
git status --short .gitmodules rust-orbitdb
```

If the remote repository does not yet exist, create it first with an empty Rust
workspace commit, push it, then add it as a submodule from the root repo. The
root repo MUST pin a specific submodule commit. The root repo MUST NOT depend on
the developer's local path outside the submodule.

## Third-Party Policy

`third-party/` MUST be treated as read-only reference material by default.
Fixture generation MAY read vendored JS OrbitDB and rust-libp2p sources. If the
team decides to make upstream PRs, the work SHOULD happen in an explicit fork or
separate clone outside `third-party/`, and the decision MUST be recorded before
editing vendored code.

## Rust Workspace Layout

The submodule SHOULD start as a Cargo workspace:

```text
rust-orbitdb/
  Cargo.toml
  rust-toolchain.toml
  rustfmt.toml
  clippy.toml
  README.md
  crates/
    rust-orbitdb-core/
    rust-orbitdb-store/
    rust-orbitdb-sync/
    rust-orbitdb-substrate/
    rust-orbitdb-doc-crdt/
    rust-orbitdb-sim/
    rust-orbitdb-libp2p/
    rust-orbitdb-libp2p-sim/
    rust-orbitdb-py/
    rust-orbitdb-fixtures/
    rust-orbitdb-testkit/
  node/
    rust-orbitdb-sim-node/
  xtask/
  tests/
    conformance/
    integration/
    stress/
    security/
```

Crate boundaries:

| Crate / package | Purpose | libp2p dependency |
| --- | --- | --- |
| `rust-orbitdb-core` | Entry, clock, conflict ordering, heads, log append/join/traverse, refs, segment anchors | MUST NOT |
| `rust-orbitdb-store` | Local storage traits, memory store, crash-injecting store, persistence adapter contracts; no concrete OS file store or networked block exchange in core | MUST NOT |
| `rust-orbitdb-sync` | Transport-independent head exchange, one-shot head transfer, and content-addressed block fetch orchestration | MUST NOT |
| `rust-orbitdb-substrate` | Narrow transport traits and message types | MUST NOT |
| `rust-orbitdb-doc-crdt` | Multi-writer document CRDT semantics and indexes | MUST NOT |
| `rust-orbitdb-sim` | Direct deterministic in-memory network and Monte Carlo simulator | MUST NOT |
| `rust-orbitdb-libp2p` | Adapter from `rust-libp2p` streams/request-response/pubsub/rendezvous to neutral substrate traits; owns the real over-the-wire content-addressed block-fetch transport selected by `D0014` | MUST |
| `rust-orbitdb-libp2p-sim` | In-memory libp2p semantics simulator with simulated links, block-fetch transport tests, and failure modes | MAY |
| `rust-orbitdb-py` | PyO3/maturin binding around stable Rust APIs | SHOULD NOT, except through feature-gated adapter use |
| `rust-orbitdb-fixtures` | JS/Rust fixture generation and corpus loading | MUST NOT |
| `rust-orbitdb-testkit` | Shared proptest strategies, model check helpers, simulation scenario DSL | MUST NOT |
| `node/rust-orbitdb-sim-node` | Node.js plugin that drives JS OrbitDB through simulator scenarios | MAY, only in Node package dependencies |

`rust-orbitdb-fixtures` is marked `MUST NOT` for libp2p because fixtures must be
deterministic, offline, and usable before any networking layer is implemented.
It MAY call Node.js fixture scripts or load corpora produced by JS OrbitDB, but
the Rust fixture crate itself MUST NOT link libp2p or require a running network.

The workspace MUST use Cargo feature flags so libp2p, Python, wasm, and heavy stress
jobs are opt-in. The default workspace test path MUST
exercise core, store, sync, document CRDT, direct simulator, fixtures, and
Python-free Rust tests without libp2p.

## I/O Ownership

rust-orbitdb semantic crates MUST be I/O-free:

- `rust-orbitdb-core`
- `rust-orbitdb-store`
- `rust-orbitdb-sync`
- `rust-orbitdb-substrate`
- `rust-orbitdb-doc-crdt`
- `rust-orbitdb-sim`
- `rust-orbitdb-fixtures`
- `rust-orbitdb-testkit`

Those crates MAY define traits, in-memory implementations, deterministic clocks,
encoded byte frames, and scenario data. They MUST NOT open TCP sockets, read or
write production files directly, depend on Tokio network types, or call libp2p.

Allowed I/O boundaries:

- `rust-orbitdb-libp2p` MAY perform real network I/O behind an explicit feature.
  It owns real adapter block-fetch transport; semantic crates only see neutral
  content-addressed fetch outcomes.
- `rust-orbitdb-libp2p-sim` MAY instantiate libp2p-facing in-memory transports
  for integration-depth testing.
- `rust-orbitdb-py` MAY cross the Python/native boundary but SHOULD keep core
  semantics in Rust.
- `node/rust-orbitdb-sim-node` MAY use Node.js I/O needed to run JS OrbitDB
  scenarios.
- Downstream host/server projects MAY use the runtime guidance in
  `Support/NetworkingRuntimeStudy.md`.

## Transport Boundary

The transport boundary MUST represent OrbitDB/rust-orbitdb needs, not libp2p
concepts. No `PeerId`, `Swarm`, `Multiaddr`, libp2p stream type, `Behaviour`, or
libp2p error type MAY appear in core, sync, or document CRDT public APIs.

The substrate trait SHOULD expose neutral rust-orbitdb concepts:

- local node identity as an opaque substrate node id
- session/log id
- participant membership changes
- state/head announcement events
- point-to-point frame exchange
- bounded byte streams for one-shot head transfer
- content-addressed block fetch by CID through the storage/substrate boundary,
  modeled as a sans-io request/response: the sync state machine emits a
  "need CID" request and later accepts the fetched block or a typed failure as
  input, rather than awaiting a fetch future (see D0007, D0011)
- backpressure and cancellation
- deterministic clock/timer handles supplied by the runtime or simulator
- explicit error classes for unavailable peer, dropped stream, malformed frame,
  missing block, malformed block, unauthorized identity, and timeout

Head announcements MUST be modeled as advisory delivery signals, not as a
reliable ordering primitive. The substrate and simulators MUST allow head
announcements to be dropped, duplicated, delayed, and reordered. Correctness
must come from idempotent head joins, deterministic conflict rules, and
content-addressed block fetch, not from assuming reliable announcement timing.

Content-addressed block fetch is a semantic requirement, not a JS wire-protocol
compatibility claim. The real libp2p adapter MUST own the selected block-fetch
wire transport. The initial proposed path is a bounded get-by-CID protocol over
libp2p request-response, as recorded in `D0014`; a bitswap/IPFS-equivalent stack
MAY replace it only behind a later dependency and audit decision. This transport
is distinct from the banned application-level head/entry catch-up protocol.

The real libp2p adapter MAY map these neutral concepts to libp2p-specific
request-response, streams, gossipsub, rendezvous, relay, or DCUtR. Those words
MUST NOT appear in core/sync/document public APIs. The direct simulator MUST map
the same neutral concepts to in-memory events. The libp2p semantics simulator
SHOULD run libp2p-facing behaviours over in-memory links so adapter logic can be
tested without real network sockets.

## Simulator Strategy

The simulator architecture is expanded in `Support/SimulatorArchitecture.md`.
The plan has two simulator variants:

1. Direct semantic simulator: implements the rust-orbitdb substrate trait
   directly, with exact deterministic scheduling and compressed Monte Carlo.
2. In-memory libp2p semantics simulator: exercises libp2p-facing adapter logic
   with simulated links, failures, and in-memory transport where feasible.

Both variants SHOULD consume the same scenario definitions. The Node.js plugin
SHOULD allow JS OrbitDB to run selected scenarios through the same harness, so
the simulator can be tested against the implementation it is trying to match.

The versioned scenario DSL and trace schema MUST exist before simulator or
Node.js runner implementation. Trace comparison MUST define the comparable
fields, ignored fields, timing buckets, and failure classification before any
cross-implementation result is accepted.

High-scale simulation is a scale parameter, not a separate mode. The same
scenario MUST be runnable at small `N` for debugging and at high `N` for
throughput, latency, event storm, and lifecycle pressure. The goal of the high-N
sweep is to characterize the scaling boundary and enforce complexity discipline
(see D0015), not to certify a fixed peer count: sweep `N` upward, find where
memory/latency/throughput degrade, and record the limiting bottleneck. Named hot
paths MUST stay sub-quadratic and live state MUST stay O(active or sampled), not
O(total inactive sessions). Compressed high-scale runs MUST preserve the same
invariant set proven by exact `N=2` and `N=3` runs, plus explicit aggregate
invariants for memory, latency, fan-out, and event queues. Any high-scale
failure SHOULD be shrunk or translated into a small deterministic regression
test.

## Runtime And Networking Strategy

rust-orbitdb itself is not a TCP server and its semantic crates MUST NOT perform
direct OS/network I/O. The runtime study in
`Support/NetworkingRuntimeStudy.md` is host/adapter handoff material. It
evaluates how Tokio on top of mio maps to epoll, kqueue, and IOCP; how Tokio's
scheduler handles 10,000+ sockets without per-connection OS threads; and where
Glommio or Smol may be superior for narrow downstream workloads.

If a downstream host adopts the C10k target, it is not just "many sockets open."
It MUST include bounded memory per connection, backpressure, cancellation,
read/write fairness, overload behavior, runtime metrics, and cross-platform
execution on Linux, macOS, and Windows.

## Security And Auditability Strategy

The security plan is expanded in `Support/SecurityAuditPlan.md`. The plan MUST
produce auditable artifacts: threat model, invariant list, fuzz targets,
dependency audit, SBOM, supply-chain policy, key/signature review, model tests,
and where feasible formal or bounded verification for critical invariants.

"Provably correct" MUST be scoped precisely. The project SHOULD aim to prove
specific invariants, such as canonical encoding stability, signature rejection
for tampered entries, append-only history constraints, deterministic conflict
resolution, and convergence under modeled delivery and data-availability
assumptions. It SHOULD NOT claim whole-system proof across arbitrary runtime,
operating system, network, and dependency behavior.

## Implementation Phases

All implementation phases are TDD-first. Each phase MUST begin with failing
tests or fixtures that define the behavior to be implemented.

| Phase | Goal | Acceptance gate |
| --- | --- | --- |
| `P00` | Plan, repository, submodule, CI, and audit bootstrap | `GLP-0003` accepted; `rust-orbitdb` submodule added and pinned; workspace skeleton builds; threat model and CI skeleton exist. |
| `P01` | Scenario DSL, fixture, JS smoke, and conformance harness | Versioned scenario DSL exists; JS fixture corpus generated from vendored OrbitDB; failing Rust conformance tests committed before implementation; early JS OrbitDB smoke runs; third-party remains read-only. |
| `P02` | Pure Rust entry and clock core | Entry, dag-cbor, CID, signature, Lamport clock, and conflict ordering pass fixture and property tests. |
| `P03` | Heads, storage traits, append/join log core | Append/join/traverse/refs/heads pass ported OrbitDB tests; per-log operation sequencer exists; crash-injecting store tests define atomic/replay behavior. |
| `P04` | Multi-writer document CRDT semantics | JS Documents compatibility fixtures pass; document model, conflict resolution, indexes, delete semantics, and exact local replica convergence tests pass under named availability assumptions. |
| `P05` | Transport-independent sync state machine | Head exchange, one-shot head transfer, content-addressed block fetch, duplicate delivery, out-of-order delivery, retry, and idempotence pass using fake in-process transport. |
| `P06` | Direct simulator | `rust-orbitdb-sim` drives the same neutral substrate API; exact small scenarios, document convergence scenarios, and fault suites pass. |
| `P07` | JS OrbitDB simulator runner | Node.js plugin runs JS OrbitDB through selected simulator scenarios and reports comparable traces. |
| `P08` | Scaling-boundary Monte Carlo simulator | Same scenarios run from `N=2` through compressed high-scale; the scaling boundary and limiting bottleneck are recorded; named hot paths are shown sub-quadratic and live state stays O(active); failures become small deterministic regressions. |
| `P09` | In-memory libp2p semantics simulator | libp2p-facing adapter logic runs over simulated links/failures without real TCP/UDP where feasible. |
| `P10` | Real libp2p adapter | `rust-orbitdb-libp2p` implements the substrate trait; conformance suite that passed in sim is rerun against real `rust-libp2p`. |
| `P11` | I/O boundary audit and host runtime handoff | Semantic crates are audited as I/O-free; Tokio/mio C10k study is handed off for downstream host/server implementation. |
| `P12` | Python binding | PyO3/maturin module exposes log/session/document APIs, simulator-backed local tests, and optional adapter selection. |
| `P13` | Generic application adapter scenarios | Terminal-shaped but product-agnostic scenario proves open session, live input, output log, replay, resize, stop, restart, and reattach. |
| `P14` | Browser/interop follow-up | OrbitDB JS/js-libp2p browser spike proves fixture and protocol interop; Rust wasm is evaluated only after native core, simulators, Python, and JS browser interop are stable. |

Detailed roll-build checkpoints live in `Checkpoints.md`. Each implementation
checkpoint SHOULD be around 500 LOC or less. If a checkpoint wants to exceed
that size, split it into `a`, `b`, `c` sub-checkpoints before implementation.

## Verification

Required commands inside `rust-orbitdb`:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --no-default-features
cargo test --workspace
cargo test -p rust-orbitdb-core
cargo test -p rust-orbitdb-doc-crdt
cargo test -p rust-orbitdb-sim --features stress-small
cargo test -p rust-orbitdb-libp2p --features libp2p-conformance
cargo test -p rust-orbitdb-libp2p-sim --features libp2p-sim
cargo nextest run --workspace
cargo llvm-cov --workspace
cargo xtask dependency-boundary-audit
```

Optional but recommended gates:

```bash
cargo deny check
cargo audit
cargo mutants --package rust-orbitdb-core
cargo fuzz run entry_decode
maturin develop -m crates/rust-orbitdb-py/Cargo.toml
pytest crates/rust-orbitdb-py/tests
npm test --prefix node/rust-orbitdb-sim-node
```

Root-repo verification after submodule addition:

```bash
git submodule status rust-orbitdb
git status --short .gitmodules rust-orbitdb plan-docs/plans/GLP-0003-rust-orbitdb
```

## Completion Criteria

- `rust-orbitdb` exists as a pinned root git submodule.
- Core, storage, sync, document CRDT, simulators, libp2p adapter, fixtures,
  testkit, Node.js runner, and Python binding have clear ownership and
  CI gates.
- Core, sync, and document APIs contain no libp2p types, and semantic crates
  pass the per-crate libp2p dependency-boundary audit.
- Semantic crates perform no direct OS/network I/O.
- Direct simulator and in-memory libp2p simulator can run without real network
  sockets and are primary acceptance gates before real libp2p.
- JS OrbitDB runs selected simulator scenarios through the Node.js runner.
- JS conformance fixtures pass for canonical bytes, CIDs, signatures, ordering,
  heads, append, join, traversal, refs, and document semantics where applicable.
- Fault tests cover append/join interleaving, store atomicity, iterator
  consistency, duplicate delivery, partition/heal, crash/restart, data
  unavailability, and publish-vs-stop races.
- Monte Carlo runs are reproducible by seed and can model `N=2` through
  million-scale scenarios using the same scenario infrastructure.
- Real libp2p adapter passes the same conformance scenarios as the simulators.
- Tokio/mio C10k study exists as downstream host/runtime handoff material.
- Python module smoke tests pass and do not own core semantics.
- Security audit artifacts exist and release-blocking findings are resolved or
  explicitly accepted.
