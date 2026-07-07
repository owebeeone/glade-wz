# Libp2p + OrbitDB + Glade Shape

Status: plan-local working shape
Plan: `GLP-0001`

Purpose: pin the Phase 1 terminal-test shape now that `rust-orbitdb` work has
started, without accidentally turning the disposable transport spike into the
full Glade substrate implementation.

## Current Position

Phase 1 is active at `P03`, but the immediate implementation track has changed
to p2p communications only:

```text
Node.js libp2p + Helia + OrbitDB sidecar in grip-lab
current GripLab backend remains in place
```

The terminal contract exists in `Support/TerminalSliceContract.md`.
`rust-orbitdb` is only at workspace bootstrap, so it is not ready to be the
runtime dependency for this first Phase 1 check.

Therefore the current Phase 1 test SHOULD use JavaScript OrbitDB directly in a
Node.js sidecar to expose libp2p, Helia, OrbitDB, write-access, and replication
friction quickly. Rust provider and browser PTY integration move to a later
checkpoint after this p2p substrate risk is better understood.

## Core Integration Claim

For the GripLab terminal test:

```text
libp2p carries hot-path terminal bytes.
OrbitDB/oplog shape carries replayable terminal output.
Glade gives the bytes and log entries their authority, lifecycle, and meaning.
```

The layers MUST stay separate:

| Layer | Phase 1 role | Not responsible for |
| --- | --- | --- |
| libp2p | open streams, negotiate protocols, carry terminal frames, expose diagnostics | authorization, terminal lifecycle semantics, replay correctness |
| OrbitDB/oplog shape | append-only output history, cursor/replay model, future sync pressure | keystroke transport, PTY execution, Glade claims/leases |
| Glade | `OpenTerminal`, `TerminalPty`, `TerminalOutput`, diagnostics, owner/lease placeholders | raw network transport, terminal emulation |
| Grip Share / GripLab | terminal tap shape and UI compatibility | distributed substrate semantics |

Important storage split:

- live terminal bytes MUST stay on libp2p/WebSocket streams
- OrbitDB MUST be replay/history, not keystroke transport
- Phase 1 MUST NOT add a Helia/IPFS file/archive tier for terminal logs
- Helia exists in this JavaScript PoC only because OrbitDB uses it underneath

## Source Facts

OrbitDB's current official README describes OrbitDB as using IPFS for storage
and libp2p pubsub for database sync, with databases implemented on an immutable,
cryptographically verifiable OpLog/Merkle-CRDT. It also says the JavaScript
implementation works in browsers and Node.js.

OrbitDB's sync API documentation says sync exchanges log heads when peers join
and publishes updated heads over pubsub. It explicitly does not guarantee
message order, delivery, or timing.

Those facts support the Glade stance already taken in `GLP-0003`:

- use OrbitDB as an oplog/sync semantic pressure point
- do not couple the Glade semantic core to libp2p
- treat pubsub/head announcements as advisory, not authoritative
- keep content-addressed fetch and log convergence above the transport

References:
- `https://github.com/orbitdb/orbitdb`
- `https://api.orbitdb.org/sync.js.html`
- `https://api.orbitdb.org/module-Log.html`

## Phase 1 Concrete Shape

### 1. Open Terminal

Surface:

```text
OpenTerminal exchange
```

Phase 1 protocol:

```text
/glade-spike/exchange/0.1.0
```

Frame flow:

```text
client -> provider: open_terminal
provider -> client: open_terminal_result { terminal_id, output_log_id, cursor }
provider -> diagnostics: provider_claim placeholder
```

Glade meaning:

- request intent exists
- provider accepted the request
- terminal id was allocated
- output log id and initial cursor were published

Phase 1 shortcut:

- no real signed Glade record
- no real capability enforcement
- real OrbitDB database addresses are allowed inside the Node sidecar only

### 2. Live Terminal Channel

Surface:

```text
TerminalPty live channel
```

Phase 1 protocol:

```text
/glade-spike/live/0.1.0
```

Frame flow:

```text
client -> provider: terminal_input
client -> provider: terminal_resize
provider -> client: terminal_output
either -> either: terminal_close | terminal_diag
```

libp2p role:

- carry bidirectional terminal frames over one stream first
- expose backpressure and stream-close behavior
- let the spike decide whether input/output need separate streams

Glade meaning:

- this is live, directed, ephemeral transport
- it is not the replicated source of truth
- reconnect is application policy, not transport magic

### 3. Terminal Output Log

Surface:

```text
TerminalOutput append log
```

Phase 1 protocol:

```text
Node.js OrbitDB events database managed by the Phase 1 sidecar
```

Minimum entry:

```js
{
  glade_contract: "glade.phase1.terminal.output.v0",
  session_id: "phase1-terminal",
  stream_name: "terminal.output",
  writer_id: "12D3Koo...",
  captured_at_ms: 1730000000000,
  previous_entry_cids: ["zdpu..."],
  local_sequence: 42,
  payload: new Uint8Array([...])
}
```

`payload` MUST be stored as native dag-cbor/IPLD bytes, not as base64 text and
not as UTF-8 text. Terminal output is a byte stream; control sequences and split
multi-byte sequences MUST survive unchanged.

`local_sequence` MAY exist only as a single-writer diagnostic convenience. It
MUST NOT be the authoritative replay cursor.

Cursor:

```text
cursor:<terminal-id>:<entry-cid>
```

Rules:

- default OrbitDB access permits only the database creator to write
- `--allow-any-writer` MAY be used only for Phase 1 replication testing
- output MUST be coalesced before append; a log entry SHOULD represent a
  bounded time/size window, not every tiny PTY write
- initial target window SHOULD be 16-64 KB or roughly 20 ms, whichever comes
  first, then measured
- attach from cursor replays entries after the last-seen entry CID
- unavailable history produces `terminal_diag`
- cursor/replay semantics MUST NOT depend on libp2p stream reconnection
- per-session database and shared database topologies MUST remain a Phase 1
  knob; the PoC may pick one default, but it MUST NOT decide the production
  scaling strategy

Database topology knobs:

| Topology | Benefit | Risk |
| --- | --- | --- |
| one events DB per session | simple PoC replay and retention boundary | pubsub/topics/heads can grow with total sessions |
| one shared events DB | avoids per-session topic explosion | per-session replay may require filtering or indexing |

Both topologies SHOULD feed the `rust-orbitdb` simulator. Phase 1 should report
append throughput, replay behavior, and subscription pressure rather than
ossifying one topology.

Why this is real OrbitDB but still Phase 1 only:

- append-only entries
- explicit write-access pressure
- CID-cursor/replay pressure
- real libp2p + Helia dependency pressure
- real OrbitDB database addresses, entry hashes, and events-store behavior
- no claim that JavaScript OrbitDB becomes the production Glade substrate

## Future OrbitDB Mapping

When the production substrate is ready, `TerminalOutput` should map toward:

| Phase 1 field | Future OrbitDB / Glade concept |
| --- | --- |
| OrbitDB database address | log id / share id |
| `session_id` | terminal instance id |
| entry CID | native replay cursor |
| `local_sequence` | optional single-writer diagnostic or segment-local index |
| `previous_entry_cids` | previous entry CID / head ancestry |
| `payload` | dag-cbor/IPLD bytes |
| `writer_id` | entry identity / Glade provider principal |
| `cursor` | Glade replay cursor, initially last-seen entry CID |

The future mapping SHOULD preserve entry/log compatibility pressure where it is
useful, but Phase 1 MUST NOT claim full OrbitDB wire compatibility.

Segment rotation remains useful later, but Phase 1 SHOULD defer an archive tier.
For Rust, the future archive/content-addressed tier is the `EntryStore` seam, not
Helia/UnixFS.

## What Lives In libp2p Later

The likely later adapter split is:

| Need | libp2p lever | Notes |
| --- | --- | --- |
| live terminal input/output | stream protocol | hot path; maybe split input/output if backpressure hurts |
| exchange/open/attach | request-response or stream protocol | bounded control path |
| log head announcement | gossipsub or direct notification | advisory only |
| log block/entry fetch | request-response get-by-CID | matches `GLP-0003` D0014 |
| known peer introduction | mDNS/rendezvous/hub | not authoritative |
| browser reachability | WebSocket/WSS, WebRTC Direct, WebTransport, gateway fallback | Phase 1 classification |

## What Phase 1 Must Decide

Phase 1 should produce a short decision on these points:

1. Can browser-to-Rust libp2p carry interactive terminal frames?
2. Can the Node.js libp2p + OrbitDB sidecar converge terminal output records
   between two peers with acceptable latency and clear diagnostics?
3. Does OrbitDB write access force an early Glade authority model, or can Phase
   1 continue with an explicit test ACL?
4. Does a single multiplexed stream starve input during burst output?
5. Is replay-from-cursor understandable after browser refresh?
6. Is the native CID cursor better than a single-writer local sequence?
7. Does append coalescing keep signing/encoding/storage cost acceptable under
   burst output?
8. Which database topology should feed the simulator: per-session DB, shared DB,
   or both?
9. Does the fallback gateway preserve the same frame/log semantics if
   browser-to-Rust libp2p is yellow or red?

It should not decide:

- final OrbitDB Rust API
- final Glade canonical record envelope
- durable storage implementation
- multi-participant input arbitration
- gossipsub provider discovery
- production browser relay topology

## Recommended Immediate Track

Implement or continue `GLP-0001` as:

```text
GripLab current backend remains in place
  + Node.js libp2p/Helia/OrbitDB sidecar
  + Glade-shaped terminal output frames
  + two-peer append/read/visibility measurement
```

Do not wait for `rust-orbitdb` or a Rust provider before measuring the p2p
communications friction.

Use the output of this spike to feed:

- `GLP-0002` reattach and multi-participant terminal semantics
- `GLP-0003` generic interactive-session scenario tests
- future Glade substrate contract promotion

## Stop / Redirect Rules

Stop expanding Phase 1 if:

- the work starts implementing production OrbitDB sync policy
- the work starts implementing full Glade records
- Node/OrbitDB setup dominates terminal latency/replay learning
- gossipsub or DHT setup becomes required before a single terminal works

Redirect to a gateway fallback if:

- browser-to-Rust libp2p is red or slow to prove
- the same terminal/replay answer can be obtained faster with Glade-shaped
  frames over a simpler local bridge
