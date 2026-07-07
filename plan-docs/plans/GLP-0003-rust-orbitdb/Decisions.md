# Decisions

Plan: `GLP-0003`

## D0001: First-party submodule path

Status: accepted
Priority: high

Use a root-level first-party submodule named `rust-orbitdb` backed by
`git@github.com:owebeeone/rust-orbitdb.git`.

Reason: the port is an independent Rust OrbitDB-compatible implementation with
its own simulator, document CRDT semantics, Node.js runner, Python binding, and
release policy. It is not vendored third-party code.

## D0002: libp2p is adapter-only for core semantics

Status: accepted
Priority: high

The core, store, sync, substrate trait, direct simulator, fixtures, document
CRDT, and testkit crates MUST NOT depend on libp2p. `rust-orbitdb-libp2p` and
`rust-orbitdb-libp2p-sim` are the only Rust crates that SHOULD or MAY depend on
libp2p.

The substrate trait MUST also avoid libp2p-shaped vocabulary in semantic APIs.
Terms such as `gossipsub`, `rendezvous`, `relay`, `dcutr`, `swarm`,
`multiaddr`, and `PeerId` belong in adapter crates, not core/sync/document APIs.

Enforcement: a `cargo deny` `[bans]` rule MUST deny `libp2p` (and its
ecosystem crates) in the eight semantic crates, and CI MUST run a per-crate
`cargo tree -p <crate> -i libp2p` (or equivalent) absence check. This audit MUST
run per crate, NOT against the `--all-features` workspace graph, since
`--all-features` deliberately pulls libp2p in to lint the adapter crates.

Reason: the same core MUST run under direct simulation, in-memory libp2p
simulation, and real networking. Banning libp2p in prose is not a gate; the
dependency tree must be checked mechanically.

## D0003: simulator is a product module

Status: accepted
Priority: high

`rust-orbitdb-sim` MUST be a first-class crate, not a helper hidden under
`tests/`. It MUST support exact deterministic tests, scenario tests, and
compressed Monte Carlo using the same scenario definitions from small `N` to
million-scale `N`.

The versioned scenario DSL and `rust-orbitdb-testkit` MUST also be treated as
product modules. Simulator work MUST NOT start until the scenario DSL, trace
schema, seed replay format, and minimum invariant set are defined.

Reason: correctness, stress, replay, throughput, latency, and scale testing are
central requirements. The simulator must be reusable by core tests, adapter
tests, host-runtime handoff tests, binding tests, and generic application
adaptation tests.

## D0004: JS OrbitDB must test the simulator

Status: accepted
Priority: high

The simulator MUST provide a Node.js runner that can execute selected JS OrbitDB
scenarios through the same scenario definitions used for Rust. A minimal JS
OrbitDB smoke MUST run during `P01`, and at least one deliberate harness
mutation MUST fail in CI before high-scale `P08` work is accepted. The purpose
is to test the test harness, not just compare fixtures.

The JS comparison surface MUST be stated explicitly. It covers entry encoding,
CID generation, signatures, Lamport clocks, heads, append/join/traversal,
conflict ordering, and JS Documents compatibility behavior. It does not prove a
new rust-orbitdb CRDT extension unless a matching JS baseline exists, and it
does not prove any custom sync protocol that JS OrbitDB does not implement.

Reason: a simulator that only tests the new implementation can encode the same
wrong assumptions as the implementation. Running JS OrbitDB through the harness
creates an independent reference pressure point.

## D0005: browser path is interop-first

Status: accepted
Priority: high

Browser V0 SHOULD use OrbitDB JS/js-libp2p as the cheap interop and UI
validation path. Rust wasm MAY be evaluated later. Porting `py-libp2p` to
browser APIs SHOULD be rejected unless a future plan reverses this decision.

Reason: OrbitDB JS already runs in the browser; wasm/browser networking and
Python-in-browser transport work are higher-unknown paths.

## D0006: third-party is read-only by default

Status: accepted
Priority: medium

Vendored `third-party/` sources MUST be treated as read-only references for this
plan. If the team chooses to make upstream PRs, clone or fork explicitly outside
the vendored reference tree and record the decision before editing.

Reason: fixture generation must be reproducible and root work must not
accidentally mutate vendored baselines.

## D0007: Tokio/mio is downstream host/runtime guidance

Status: accepted
Priority: medium

rust-orbitdb semantic crates MUST NOT perform direct OS/network I/O and MUST NOT
depend on Tokio or any async runtime. Clocks, timers, and schedulers MUST be
injected through traits.

Semantic crates MUST be sans-io: behavior is expressed as synchronous step/poll
functions and state machines driven by the host or simulator, which supply I/O
results, block fetches, timer ticks, and wakeups. Semantic crates SHOULD NOT
expose `async fn` in their public APIs; an executor or `.await` point inside a
semantic crate re-introduces the runtime coupling this decision forbids. Async,
futures, and any runtime appear only at adapter/host/binding boundaries
(`rust-orbitdb-libp2p`, `rust-orbitdb-py`, downstream hosts). The sans-io shape
also makes the direct simulator and JS runner drive the same state machines
deterministically.

A downstream TCP/server host SHOULD evaluate Tokio on top of mio as the default
runtime. Alternatives such as Glommio and Smol remain comparison points, not
default choices, unless the runtime study shows a strict workload-specific
advantage.

Reason: the target requires Linux, macOS, and Windows uniformity while avoiding
legacy O(N) polling overhead, but that target belongs to host/adapter code, not
the OrbitDB semantic core. "No runtime dependency" is only enforceable if the
crates are sans-io; otherwise async leaks back in through injected executors.

## D0008: proof claims must be scoped

Status: accepted
Priority: high

The project SHOULD pursue proof-oriented correctness for specific invariants,
not claim whole-system proof. Accepted proof targets include canonical encoding,
signature rejection for tampered entries, deterministic conflict ordering,
append-only constraints, storage recovery invariants, and convergence under
modeled delivery and data-availability assumptions.

Reason: data security is paramount, but the runtime, OS, network, and dependency
stack make whole-system proof unrealistic. Scoped invariants are auditable.

## D0009: document CRDT authority

Status: accepted
Priority: high

The authoritative compatibility baseline for existing document behavior is JS
OrbitDB Documents: `third-party/orbitdb/src/databases/documents.js`,
`third-party/orbitdb/test/databases/documents.test.js`, and
`third-party/orbitdb/test/databases/replication/documents.test.js`. That
baseline is an oplog-reduced, keyed last-write-wins document store.

The authoritative rust-orbitdb multi-writer document model MUST be written
before implementation. The initial proposed default is total-order
last-write-wins whole-document registers and tombstones keyed by document id,
using the accepted oplog conflict rules. The CRDT acceptance gate MUST state
order-independent convergence across replicas under modeled delivery and
data-availability assumptions. Observed-remove semantics and field-level merge
MAY be added later only behind an explicit decision and tests.

Reason: the plan must distinguish compatibility with JS Documents from the new
multi-writer CRDT behavior, while still making both testable.

## D0010: neutral substrate API

Status: accepted
Priority: high

The substrate API MUST be neutral and rust-orbitdb-shaped, not libp2p-shaped.
It MAY describe participants, membership, announcements, point-to-point frames,
bounded one-shot head transfer, content-addressed block fetch, capabilities,
timers, and cancellation. It MUST NOT describe gossipsub, rendezvous, relay,
DCUtR, swarms, multiaddrs, or PeerIds in semantic crate APIs.

Reason: banning libp2p types is not enough if the abstract API still mirrors
libp2p concepts. The direct simulator and JS runner need a neutral model.

## D0011: OrbitDB-compatible sync model

Status: accepted
Priority: high

The OrbitDB-compatible replication model is head announcements plus
content-addressed block fetch. Head announcements tell a peer that a log has
new heads. One-shot head transfer ships current heads. Missing transitive
entries MUST be fetched by CID through the storage/substrate boundary, with
typed missing, malformed, unavailable, unauthorized, timeout, and cancellation
outcomes.

The block-fetch boundary MUST be sans-io (see D0007). The sync state machine
MUST surface a missing CID as a returned request/event that the host or adapter
fulfills, then accept the fetched block (or a typed failure) as a subsequent
input. The semantic sync crate MUST NOT `.await` a fetch future or own the fetch
transport. This shapes the `rust-orbitdb-sync` ↔ substrate API from P05a: design
it poll-based first, not async-first then retrofitted.

Application-level head/entry catch-up MAY be explored only as a recorded
rust-orbitdb extension. It MUST NOT be treated as JS OrbitDB-compatible behavior,
and D0004 JS trace comparison MUST NOT be claimed as validation for that
extension. A get-by-CID request/response protocol used only as the block-fetch
transport is allowed by `D0014`; it is not the banned head/entry catch-up
protocol.

Reason: JS OrbitDB sync does not define an application request/response
catch-up protocol. Compatibility pressure belongs on heads, entries, CIDs,
storage fetch, ordering, and idempotence.

## D0012: head announcements are advisory

Status: accepted
Priority: high

Head announcements MUST be modeled as best-effort advisory signals. They MAY be
dropped, duplicated, delayed, and reordered by the substrate, direct simulator,
in-memory libp2p simulator, or real adapter. No correctness invariant MAY depend
on announcement delivery order, exactly-once delivery, or bounded delivery time.

Reason: delivery behavior should be tested under the same nondeterminism that
real p2p transports expose. Correctness must come from idempotent joins,
content-addressed fetch, deterministic conflict ordering, and retry policy.

## D0013: compatibility surface exceeds the small JS spine

Status: accepted
Priority: medium

The small critical OrbitDB JS LOC count is useful for scoping the semantic
spine, but implementation MUST also account for dag-cbor, CID/multiformats,
entry version `v:2`, identity providers, keystore behavior, access controllers,
content-addressed storage, and encryption paths. Each surface MUST be inventoried
before implementation relies on it.

Reason: underestimating dependency and compatibility surface would create false
confidence in the roll-build size and fixture completeness.

## D0014: real-adapter block fetch transport

Status: accepted
Priority: high

The first real libp2p adapter SHOULD implement content-addressed block fetch as
a bounded get-by-CID protocol over libp2p request-response. The compatibility
claim is CID/block compatibility and OrbitDB sync semantics: canonical entry
bytes, CIDs, heads, storage fetch outcomes, ordering, and idempotence. The plan
does not claim bitswap/IPFS wire-protocol compatibility.

An external Rust IPFS/bitswap-equivalent stack MAY replace or augment this only
behind a later dependency, audit, SBOM, and conformance decision. If that path is
chosen, `rust-orbitdb-libp2p` still owns the adapter boundary and semantic crates
remain unaware of the concrete wire protocol.

Reason: rust-libp2p does not ship a bitswap/block-exchange protocol, and JS
OrbitDB delegates block exchange to Helia/IPFS outside OrbitDB itself. A custom
get-by-CID transport keeps the first roll-build bounded while preserving the
important compatibility layer: bytes, CIDs, and state-machine behavior.

## D0015: scaling is boundary characterization and complexity discipline

Status: accepted
Priority: high

The scale simulator's purpose is NOT to certify a fixed peer count. "Million"
is a direction to probe toward, not a pass/fail target. The accepted goal is to
sweep `N` upward, find where memory, latency, or throughput degrade, and record
the limiting bottleneck. A scaling-boundary report with the bottleneck named is
the acceptance artifact, not a million-peer certificate.

Complexity is a design and review gate, not a late discovery:

- Named hot paths MUST be sub-quadratic. These include log append, join,
  traversal, heads maintenance, conflict ordering, head exchange/diff between
  peers, document index rebuild, the simulator event loop, and compressed-state
  representation. A superlinear algorithm in any of these is a stop condition.
- Live/working state MUST be O(active or sampled), never O(total inactive
  sessions). This is the P08 no-growth invariant restated as a complexity rule.
- Head exchange MUST be bounded by the number of heads, not the number of
  entries. OrbitDB's `refs`/segment anchors exist as a skip structure to keep
  traversal sub-linear; preserving their semantics is a complexity requirement,
  not only a compatibility one.

No absolute memory/time budget number is fixed up front. Such a number is a
measurement output of `P08`, not a planning input. The protective gate is the
complexity invariants above plus the boundary report, not a pre-committed
threshold.

Reason: the boundary and the scaling shape are what set design constraints. An
accidental O(n^2) in join, head exchange, or state representation is the real
failure mode; an arbitrary million-scale budget would neither catch it nor
prove anything.

## D0016: operation record and store get/set API

Status: accepted
Priority: high

There are two contracts, pinned at two different points in the roll-build.

(a) Operation record (wire-significant; pinned before P02). Every store write
routes through `addOperation(op)`, and `op` becomes `entry.payload` verbatim via
`log.append`. The envelope is:

```text
{ op: <string>, key: <string|null>, value: <dag-cbor-encodable | null> }
```

with the op vocabulary `PUT`, `DEL` (keyed stores) and `ADD` (events; key=null).
Authoritative sources: `third-party/orbitdb/src/database.js`,
`databases/documents.js`, `databases/keyvalue.js`, `databases/events.js`. The
field names `op`/`key`/`value` and the op strings are inside the canonical
dag-cbor bytes that produce the CID and the signature; they are NOT free to
rename. This envelope MUST be modeled as a typed entry-payload contract and
locked by `entry-fixtures` (P01c) / `document-fixtures` (P01g) before any P02
encoding/CID/signature work, since every downstream hash depends on it.

(b) Store get/set API (consumer surface; pinned at P04a). Documents:
`put(doc) -> hash`, `del(key) -> hash`, `get(key) -> {hash,key,value}|undefined`,
`query(findFn over value) -> [value]`, `iterator({amount}) -> {hash,key,value}`,
`all()`. KeyValue mirrors it with `get(key) -> value` and two-arg
`put(key,value)`. Documents `put` derives `key` from `doc[indexBy]` (default
`_id`) and the value MUST carry that field. Read semantics are last-write-wins by
oplog conflict order: traverse newest-first, first op seen per key wins, `DEL`
masks the key.

The read path MUST be index-backed, not scan-per-get. JS OrbitDB `documents.get`
scans the whole log per call (O(n) per get, O(n^2) over n gets); OrbitDB ships
`keyvalue-indexed.js` precisely to avoid this. The Rust port MUST maintain the
key->latest-entry index via the apply/update reduction. This is a named D0015
hot path.

The API shape is stable across the D0009 LWW baseline and the later OR-map/
field-merge extension; only the reduction changes, not the get/set signatures.

Reason: the operation envelope is the most upstream contract in the plan — it
gates entry encoding, CIDs, and signatures from P02 on. Pinning it as typed data
first (not discovering it mid-encode) is what keeps the fixture corpus and every
downstream hash sound.
