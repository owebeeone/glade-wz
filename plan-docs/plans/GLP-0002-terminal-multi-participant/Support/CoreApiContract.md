# GLP-0002 — Core API Contract (P00 draft)

Status: working draft — plan-local. **Promotes to `glade/dev-docs/` at the
`contract-lock` checkpoint.**

Purpose: define the API surface the substrate core exposes, the
Substrate/Transport adapter boundary, what the core may assume about transport,
and the simulated-substrate requirements — so implementation can start the
moment `GladeRustOrbitStrategy.md` lands. Signatures are language-neutral
(Rust-trait flavored); the Python module mirrors them.

Normative language: `MUST`/`SHOULD`/`MAY`.

## 1. Layering

```text
consumers (grip-lab tap / channel handle ; provider PTY)
        │   Glade surfaces
        ▼
AppendLog · LiveChannel · Exchange · Presence
        │   core API (oplog/sync + segment/anchor)
        ▼
Substrate adapter  ──────────────  (the only boundary the core knows)
        ▼
 SimSubstrate   |   Libp2pSubstrate     (interchangeable)
```

- The oplog/sync core MUST NOT reference libp2p. It talks only to the
  `Substrate` trait.
- `SimSubstrate` and `Libp2pSubstrate` MUST implement the same `Substrate` trait,
  so every correctness/stress/scale test runs in-process; `P04` is a conformance
  re-run, not a re-implementation.

## 2. Substrate adapter contract

```rust
trait Substrate {
    // identity of this node
    fn local_peer(&self) -> PeerId;

    // best-effort datagrams (used by sync: head exchange, entry req/resp)
    fn send_datagram(&self, to: PeerId, topic: TopicId, bytes: Bytes) -> Result<()>;
    fn on_datagram(&self) -> Stream<(PeerId, TopicId, Bytes)>;

    // reliable, ordered, bidirectional streams (used by LiveChannel)
    fn open_stream(&self, to: PeerId, proto: ProtocolId) -> Future<Result<Stream>>;
    fn on_stream(&self) -> Stream<(PeerId, ProtocolId, Stream)>;

    // membership / reachability hints (NOT authoritative)
    fn on_lifecycle(&self) -> Stream<LifecycleEvent>; // Joined|Left|Unreachable

    // introduction: resolve a session to its current participants (hub-backed)
    fn resolve_session(&self, s: SessionId) -> Future<Result<Vec<PeerId>>>;
}
```

### 2.1 What the core MAY assume (the load-bearing table)

| Property | Datagrams (sync) | Streams (live-channel) |
| --- | --- | --- |
| **Identity** | every message authenticated to a `PeerId`; core trusts it | stream authenticated to a `PeerId` |
| **Ordering** | **none** — MAY arrive reordered | **FIFO within a stream**; none across streams |
| **Delivery** | best-effort; MAY drop / duplicate | reliable until the stream closes |
| **Backpressure** | substrate MAY drop on overload; `send` MAY fail | surfaced; `send` MAY block/err |
| **Retries** | **none** — the core's anti-entropy re-requests | substrate signals break; reconnect is core/app policy |
| **Lifecycle** | join/leave/unreachable are **hints**; a peer MAY reappear | stream break is authoritative for that stream only |

Consequences the core MUST honor:
- sync apply MUST be **idempotent** and **commutative** (CRDT convergence under
  drop/dup/reorder);
- the core MUST NOT assume a peer that "Left" is gone (idempotent rejoin);
- the core MUST drive its own re-request for missing entries (no transport
  retry guarantee);
- nothing in the core MAY assume infinite send buffering.

## 3. Simulated substrate (`SimSubstrate`) requirements

`SimSubstrate` MUST implement `Substrate` and additionally:
- be **deterministic** under a seed (injected RNG + virtual clock; no wall-clock,
  no ambient randomness);
- drive **N synthetic peers in one process**;
- inject faults on datagrams: **drop, delay, reorder, duplicate**;
- inject **partition / heal** and **peer churn** (join / leave / unreachable /
  reappear);
- inject **stream break** and **backpressure / overload**;
- expose a controllable **scheduler** so scenarios are reproducible.

This is the primary test vehicle for `P01`–`P03` (fan-out at `N=2..1000`,
reattach, lease races, churn, partition).

## 4. Surface A — AppendLog (read / observable / convergent → tap)

Single-writer per log. Read side is a tap.

```rust
// writer (provider only; holds the log's identity/lease)
trait AppendLogWriter {
    fn append(&self, payload: Bytes) -> Result<EntryId>;     // append-only
    fn checkpoint(&self) -> Result<Anchor>;                  // seal a segment
}

// reader (any participant)
trait AppendLogReader {
    fn head(&self) -> Cursor;
    fn read_window(&self, w: Window) -> Future<Result<Vec<Entry>>>; // lazy
    fn subscribe(&self, from: Cursor) -> Stream<Entry>;             // live tail
}

enum Window { LastN(u64), ByteBudget(u64), FromCursor(Cursor) }
```

- Lazy: `read_window` MUST fetch only the segments needed and trust the boundary
  `Anchor.state_hash` for older history (no full-history fetch).
- **Reattach**: a reader MUST be able to `read_window` to a `Cursor`, then
  `subscribe(from: cursor)` and receive the live tail with **no gap and no
  duplicate** at the cutover. (Cutover contract — see §8.)
- Each `Entry` carries the writer `PeerId`. The log has exactly one writer
  identity at a time (the lease holder).
- Consumer projection = a **tap**: `(materialized window, live updates)`. This is
  what `grip-lab/src/lab/terminalController.ts` binds.

## 5. Surface B — LiveChannel (directed / ephemeral → NOT replicated)

```rust
trait LiveChannel {
    fn open(&self, session: SessionId, peer: PeerId, proto: ProtocolId)
        -> Future<Result<Channel>>;
}
trait Channel {
    fn send(&self, bytes: Bytes) -> Result<()>;   // ordered, reliable
    fn recv(&self) -> Stream<Bytes>;
    fn close(&self);
}
```

- 1:1 bidirectional. Carries **keystrokes** (driver→provider) and the live output
  hot path; later the typing-presence heartbeat (§7).
- MUST NOT be persisted or replicated by the substrate. Ephemeral.
- Backpressure surfaced; on break the app re-opens (substrate gives no retry).

## 6. Surface C — Exchange (broker / bounded grant → shared-state)

Models intent → claim → publication per `GladeExchangeSemantics.md`.

```rust
trait Exchange {
    // requester
    fn submit_intent(&self, kind: Kind, target: Target, params: Params)
        -> Result<RequestId>;
    fn observe(&self, r: RequestId) -> Stream<ExchangeEvent>;

    // provider
    fn observe_intents(&self, kind: Kind) -> Stream<RequestIntent>;
    fn claim(&self, r: RequestId, owner_term: OwnerTerm) -> Result<ClaimRecord>;
    fn publish(&self, r: RequestId, result: Publication) -> Result<()>;
}
```

- `OpenTerminal` is an `Exchange`: the grant under which a `LiveChannel` +
  `AppendLog` are created (addressing by `SessionId`, authz, attribution).
- **The driving lease (P03 input arbitration) is a claim under `owner_term`**:
  "who holds the keyboard." Request / grant / handoff / expiry are exchange
  events. Exactly one active claim at a time (single-writer invariant).

## 7. Surface D — Presence (P05 — defined now, built later)

```rust
trait Presence {
    fn announce(&self, session: SessionId, state: ParticipantState); // heartbeat
    fn observe(&self, session: SessionId) -> Stream<PresenceView>;
}
enum ParticipantState { Idle, Typing, Driving }
```

- Ephemeral awareness signal: who's present, who's typing, who's driving.
- MUST be a live-stream-class surface (heartbeat + TTL, loss-tolerant). MUST NOT
  use the append-log. "Who is driving" derives from the §6 lease; "who is typing"
  is the debounced ephemeral signal.
- Defined in the contract now so identity + the surface exist before `P05`.

## 8. Read/write asymmetry — explicit prohibitions

- There MUST be **no** API to append directed input (keystrokes, presence) to a
  replicated log. Directed/ephemeral data flows only through `LiveChannel` /
  `Presence`.
- `AppendLog` writes MUST be single-writer and authenticated; there is no
  multi-writer append path in this contract (multi-writer is out of scope).

## 9. Identity model (threaded from P02)

- Every `Entry`, `Channel`, `ClaimRecord`, and presence announcement is
  attributed to an authenticated `PeerId`.
- The substrate authenticates identity; the core/surfaces trust the attribution.
- `SimSubstrate` provides synthetic but stable `PeerId`s with the same trust
  semantics, so identity-dependent logic (lease, avatar) is testable in sim.

## 10. Open contract questions (resolve at `contract-lock`)

- Exact **reattach cutover** dedup/ordering contract (§4) — the primary
  correctness seam.
- `TopicId` granularity for sync datagrams (per-log vs per-session).
- Lease handoff race + expiry/heartbeat policy (§6).
- Presence channel realization: hub-aggregated vs fan-out (§7).
- Whether `Anchor` signing is single-writer-only here (yes, per single-writer
  scope) — confirm against the strategy doc.

## References

- `glade/dev-docs/GladeTerminalSliceProposal.md` (design this implements)
- `glade/dev-docs/GladeRustOrbitStrategy.md` (core internals — pending)
- `glial-dev/dev-docs/glade/GladeExchangeSemantics.md` (exchange planes)
- `glial-dev/dev-docs/GLDevPlan.md` (terminal pipeline, replay cursor)
- `grip-lab/src/lab/terminalController.ts` (the tap/channel consumer seam)
