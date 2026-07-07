# Phase 1 Libp2p Test

Status: proposed spike specification

Purpose: define the first libp2p investigation for Glade. The goal is to test
transport surfaces, runtime choices, browser viability, and Python integration
before binding libp2p to the full Glade declaration model.

Related active plan:
`/Users/owebeeone/limbo/glade-wz/plan-docs/plans/GLP-0001-griplab-terminal-transport-spike/Plan.md`

## Core Claim

Phase 1 should answer one question:

```text
Can libp2p carry the first Glade surfaces well enough for GripLab?
```

It should not answer every production topology question.

The spike should test:
- peer identity
- explicit peer dialing
- local peer discovery
- request-like exchange traffic
- append-log-like streaming
- live-channel-like bidirectional streaming
- browser participation
- native deployable node participation
- Python provider integration options
- basic reconnect and failure diagnostics

## Phase 1 Anchor

The first concrete Phase 1 slice is:

```text
browser js-libp2p peer
  -> Rust libp2p provider peer
  -> local PTY process
  -> append-log-shaped output buffer
```

This is intentionally disposable. The purpose is to learn whether libp2p can
carry the GripLab terminal straight-line case with acceptable latency and
diagnostics.

Primary Phase 1 gates:

| Gate | Question | Output |
| --- | --- | --- |
| `Gate A: browser local loop` | Can browser `js-libp2p` run the spike exchange/log/live surfaces locally? | browser viability classification |
| `Gate B: Rust native loop` | Can Rust host the spike protocols and terminal provider harness cleanly? | native spine classification |
| `Gate C: browser-to-Rust terminal` | Can browser JS connect to Rust and drive a PTY live channel? | terminal transport classification |
| `Gate D: Python stdio bridge` | Can Python provider ergonomics work through a local stdio bridge? | provider bridge classification |

Other tracks are secondary. They should not block the terminal answer.

## Outcome Bands

Every gate MUST report one of:

| Band | Meaning |
| --- | --- |
| `green` | Works well enough to proceed with the planned Glade/GripLab path. |
| `yellow` | Works with constraints, setup friction, or a fallback such as a gateway or split streams. |
| `red` | Blocks the planned path; choose a fallback before continuing. |

The final Phase 1 result MUST be a short green/yellow/red decision, not a long
runtime survey.

## Provisional Runtime Decision

Use a multi-track evaluation:

| Track | Runtime | Purpose | Decision |
| --- | --- | --- | --- |
| Browser/application track | `js-libp2p` | Prove browser and GripLab UI participation. | Required. |
| Native/deployment track | `rust-libp2p` | Prove a deployable Glade node, relay, and provider-side transport spine. | Required. |
| Python provider track | stdio bridge first, `py-libp2p` smoke test second | Keep Python provider ergonomics without betting the transport spine on Python maturity. | stdio bridge preferred for Phase 1. |
| Python native-extension track | Rust module exposed to Python | Analyze whether Python providers can use a Rust libp2p core through a native module. | Analyze only, do not implement in Phase 1. |

Do not choose `py-libp2p` as the primary Phase 1 transport spine yet.
For the V0 Python provider bridge, choose stdio unless a concrete blocker is
found.

Reasoning:
- GripLab needs browser participation, which makes `js-libp2p` unavoidable for
  at least one side of the spike.
- Rust is the strongest candidate for a deployable Glade node because it is
  native, performance-oriented, and has a mature libp2p implementation surface.
- Python is important for providers, but provider ergonomics can be preserved
  with a local bridge to a JS or Rust Glade node.
- A Rust-backed Python module may be a strong later option if `rust-libp2p`
  has an acceptable dependency footprint and can be packaged cleanly with
  GitHub Actions.
- `py-libp2p` is active and promising, but the project still describes itself
  as progressing toward full production maturity. It should be evaluated, not
  assumed.

## Source Check Summary

Current source check as of 2026-06-02:
- `py-libp2p` PyPI has version `0.6.0`, uploaded 2026-02-16.
- `py-libp2p` README says the project has moved beyond experimental roots and
  is progressing toward production readiness.
- `py-libp2p` lists implemented TCP, QUIC, WebSocket, circuit relay v2,
  AutoNAT, hole punching, Noise, TLS, bootstrap, random walk, mDNS,
  rendezvous, Kademlia DHT, FloodSub, GossipSub, Yamux, Mplex, records, ping,
  peer, and identify.
- Browser connectivity docs say libp2p browser nodes use WebSocket,
  WebTransport, and WebRTC.
- Browser connectivity docs distinguish streams from HTTP-style request and
  response, which matches Glade live-channel requirements.
- Browser connectivity docs list WebRTC Direct browser-to-node support for
  Rust, Go, Chrome, Firefox, and Safari, while browser-to-browser WebRTC is
  listed as supported in `js-libp2p`.
- libp2p hole-punching docs describe Identify, AutoRelay, Circuit Relay, and
  DCUtR as the relevant NAT traversal pieces.

Local tool versions:
- `rustc 1.95.0`
- `cargo 1.95.0`
- `node v22.19.0`
- `Python 3.10.15`

## What Libp2p Provides To Glade

For Phase 1, libp2p should be treated as transport infrastructure.

It provides:
- transport peer identity
- multi-addressing
- secure connections
- stream multiplexing or native stream use
- protocol negotiation
- ping/liveness signals
- identify/address exchange
- peer discovery mechanisms
- pubsub mechanisms
- relay and NAT traversal mechanisms
- browser-to-node and browser-to-browser transport options

It does not provide:
- Glade declarations
- authorization semantics
- provider claims
- leases
- route ownership
- cache policy
- retention
- exchange lifecycle
- content/log semantics
- agent delegation

The design boundary remains:

```text
libp2p answers: how can bytes move between peers?
Glade answers: who should talk, what may they do, and what does it mean?
```

Transport `PeerId` is not a Glade principal. Phase 1 frames MUST label libp2p
identity as transport-only and MUST keep placeholder fields for future
authorization identity.

## Phase 1 Non-Goals

Do not implement:
- full Glade record envelope
- real package signing
- distributed control plane
- production scheduling
- CRDT
- global hosted namespace admission
- marketplace or registry semantics
- full terminal integration with GripLab
- full file cache semantics

The spike may use simplified signed/unsigned JSON frames as long as the future
Glade boundary is visible.

## Test Protocols

Define temporary protocol ids for the spike:

```text
/glade-spike/hello/0.1.0
/glade-spike/exchange/0.1.0
/glade-spike/log/0.1.0
/glade-spike/live/0.1.0
/glade-spike/diag/0.1.0
```

These are not final Glade protocols. They are disposable Phase 1 protocol ids.

## Test Frame Format

Use length-prefixed JSON or newline-delimited JSON for the spike.

The choice should optimize implementation speed and debugging, not final
wire efficiency.

Minimum frame envelope:

```json
{
  "frame_id": "frame:01J...",
  "kind": "live_output",
  "transport_peer_id": "peer:...",
  "principal_id": "principal:dev-placeholder",
  "session_id": "session:dev-a",
  "capability_ref": "cap:dev-placeholder",
  "surface_id": "live:terminal-pty",
  "seq": 42,
  "ts_ms": 1730000000000,
  "payload": {}
}
```

`transport_peer_id` is used only for transport diagnostics. It MUST NOT be used
as authorization identity in the spike.

Frame kinds to test:
- `hello`
- `provider_claim`
- `exchange_request`
- `exchange_progress`
- `exchange_result`
- `exchange_diag`
- `log_open`
- `log_append`
- `log_close`
- `live_open`
- `live_input`
- `live_output`
- `live_resize`
- `live_close`
- `ack`
- `diag`

## Test Topologies

## Required Artifacts

Phase 1 MUST produce:

- `observations.md`
- `decision.md`
- disposable frame schema
- JSONL diagnostics sample
- browser-to-Rust setup note
- Python stdio bridge setup note
- transport capability matrix
- "not proven by Phase 1" security checklist

If the experiment stays under `scratch/`, these artifacts MAY start there. If
the result becomes useful, promote them into a committed experiment or plan
support folder.

## Transport Capability Matrix

Record each transport path with green/yellow/red outcome:

| Path | Gate | Required? | Notes To Capture |
| --- | --- | --- | --- |
| Browser JS local loop | `Gate A` | yes | secure-context needs, refresh behavior, stream setup |
| Rust native loop | `Gate B` | yes | selected crate features, protocol negotiation, diagnostics |
| Browser JS -> Rust WebSocket/WSS | `Gate C` | yes if simplest | certificate/setup friction, LAN viability |
| Browser JS -> Rust WebTransport | `Gate C` | optional | browser support and certificate friction |
| Browser JS -> Rust WebRTC Direct | `Gate C` | optional but important | cert-hash workflow and LAN viability |
| Browser JS -> gateway fallback | fallback | yes as fallback | whether same Glade-shaped frames survive |
| Python stdio bridge | `Gate D` | yes | cancellation, provider crash, framing under load |
| Py-libp2p direct | secondary | no | classify only |
| Rust-backed Python module | secondary analysis | no | packaging and async/GIL risk |

### T0: JS Local Loop

Purpose: prove browser/application ergonomics quickly.

Shape:
- one Node.js libp2p peer
- one browser libp2p peer
- explicit peer address or local relay

Tests:
- browser dials Node.js peer
- browser opens `/glade-spike/exchange/0.1.0`
- browser opens `/glade-spike/live/0.1.0`
- Node.js streams output back

Success:
- browser can participate without bespoke WebSocket API semantics
- live frames arrive at interactive latency on localhost

### T1: Rust Native Loop

Purpose: prove native deployable node viability.

Shape:
- two Rust libp2p peers
- TCP or QUIC transport
- Noise or TLS security
- Yamux or native stream multiplexing

Tests:
- peer identity and ping
- protocol negotiation
- request-like exchange stream
- append-log stream
- bidirectional live stream

Success:
- Rust node can host the Phase 1 protocols cleanly
- logs show peer id, protocol id, stream open/close, and diagnostics

### T2: Browser To Native Node

Purpose: prove the most important future GripLab path.

Shape:
- browser `js-libp2p` peer
- Rust libp2p native node
- WebRTC Direct, WebTransport, or WebSocket fallback

Test in this order:
1. WebSocket or WSS if simplest.
2. WebTransport if practical.
3. WebRTC Direct if practical.
4. Relay-mediated WebRTC only after direct browser-to-node is understood.

Required checks:
- browser refresh and reattach
- LAN setup
- secure-context constraints
- WebRTC Direct certificate-hash workflow if WebRTC Direct is attempted
- live terminal input/output latency
- burst output while input is still being sent

Success:
- browser can open at least one stream to the Rust node
- stream can carry live-channel frames
- limitations are documented concretely

### T3: Browser To Browser Via Relay

Purpose: understand whether browser-to-browser is useful for team sessions.

Shape:
- two browser peers
- one public or local relay/rendezvous peer
- `js-libp2p` WebRTC

Tests:
- peer discovery through relay/rendezvous
- browser A opens live stream to browser B
- basic reconnect or failure diagnostic

Success:
- browser-to-browser works well enough to classify as useful, optional, or
  not worth Phase 1 dependency.

### T4: Python Provider Bridge

Purpose: keep Python provider ergonomics without forcing Python to own libp2p.

Shape:
- Rust or Node.js Glade spike node
- Python provider process connected locally by stdio
- libp2p node handles p2p transport
- Python provider handles application logic

V0 decision:
Use stdio first. Unix socket, HTTP, and WebSocket bridges are fallback or later
tracks, not co-equal Phase 1 choices.

Tests:
- Python receives `exchange_request`
- Python sends progress and result
- Python sends append-log chunks
- Python handles live-channel input/output using local bridge
- provider process crash is visible
- cancellation is visible
- burst output does not corrupt stdout/stderr framing

Success:
- provider ergonomics are good
- Python code does not need to understand libp2p to serve Glade surfaces
- bridge latency is acceptable for provider-side work

### T5: Py-Libp2p Direct Smoke Test

Purpose: evaluate whether `py-libp2p` is ready for direct Glade development.

Shape:
- two Python peers
- optional Python peer to Rust or JS peer if interop is practical

Tests:
- peer identity
- ping
- explicit dial
- protocol stream
- pubsub if easy

Success:
- direct Python libp2p is classified as one of:
  - usable now for development
  - useful for experiments only
  - defer and use bridge

This test should not block the main Phase 1 path.

### T6: Rust-Backed Python Module Analysis

Purpose: evaluate whether Python providers could use a Rust libp2p core
through a native Python extension instead of direct `py-libp2p` or a separate
local bridge process.

This is an analysis track only in Phase 1. Do not implement it unless the
bridge and direct Python options both look weak.

Possible shape:
- Rust crate owns libp2p node, transports, protocol negotiation, streams, and
  diagnostics.
- Python module exposes a provider-friendly async API.
- Python provider code registers handlers for Glade surfaces.
- Rust module handles p2p transport and calls into Python handlers.

Candidate tooling:
- `PyO3`
- `maturin`
- GitHub Actions wheels for macOS, Linux, and Windows

Analysis questions:
- How many dependencies does `rust-libp2p` pull in for the required transports?
- Can the required feature set be compiled with a modest feature selection?
- Can async Rust streams map cleanly into Python `asyncio`?
- Can Python handler callbacks be invoked without unacceptable GIL contention?
- Can wheels be built reliably for common platforms?
- How hard is local developer installation?
- How are Rust diagnostics surfaced into Python?
- Can the module expose a stable API while the Rust internals evolve?
- Does this simplify deployment compared with a separate local bridge process?
- Does this make browser interoperability easier, harder, or unchanged?

Success:
- classify the approach as:
  - promising for provider ergonomics
  - useful only for later packaging
  - too complex compared with a bridge

The likely Phase 1 answer should still be bridge-first unless analysis shows
the native module is clearly simpler.

## Discovery Tests

Test discovery in increasing complexity:

1. explicit multiaddr copy/paste
2. local mDNS
3. bootstrap peer list
4. rendezvous
5. pubsub provider announcements
6. DHT or delegated routing

Phase 1 only requires 1 and one of 2 or 3.

Do not make DHT behavior a blocker for the first GripLab slice.

## Relay And NAT Tests

Relay and NAT traversal are important, but Phase 1 should treat them as a
separate risk area.

Minimum:
- understand circuit relay v2 setup
- document what is required for browser-to-browser WebRTC
- document what is required for browser-to-native WebRTC Direct or
  WebTransport

Optional:
- AutoNAT
- DCUtR hole punching
- relay reservation

Do not block the local terminal slice on hole punching.

## Pubsub Tests

Use pubsub only for low-criticality announcements in Phase 1.

Possible topics:

```text
glade-spike.providers
glade-spike.diag
glade-spike.discovery
```

Do not use pubsub as the authoritative exchange or live-channel transport in
Phase 1. Direct streams are easier to reason about for terminal and exchange
behavior.

## Surface Tests

### Exchange Surface

Test:
- requester opens stream
- sends `exchange_request`
- provider sends `exchange_progress`
- provider sends `exchange_result`
- provider may send `exchange_diag`

Success:
- lifecycle is visible
- retry can be simulated with a new attempt id

### Append Log Surface

Test:
- producer opens log
- appends numbered chunks
- consumer tails chunks
- consumer disconnects and resumes from a cursor

Success:
- chunk order is visible
- cursor semantics are not confused with transport delivery

### Live Channel Surface

Test:
- one side sends input frames
- other side sends output frames
- resize/control frames are interleaved
- backpressure or slow receiver behavior is observed
- burst output is generated while input continues
- input and output are tested as one stream first, then split streams if
  latency is poor

Success:
- suitable latency on localhost
- stream close/error is visible as diagnostics

### Terminal Harness Surface

Test:
- browser sends `terminal_input`
- Rust provider writes to PTY
- Rust provider emits `terminal_output`
- output is appended with monotonically increasing sequence
- browser refresh attaches from cursor
- resize sends cols/rows to the PTY
- provider crash emits diagnostic or closes visibly

Success:
- straight-line interactive terminal behavior is usable locally
- replay is honest even if incomplete
- burst output does not permanently starve input

## Diagnostics Requirements

Every spike node should print structured diagnostics for:
- peer started
- listening address
- discovered peer
- dial attempt
- dial success/failure
- protocol negotiation success/failure
- stream opened
- stream closed
- frame parse error
- peer disconnect
- retry or reconnect attempt

Diagnostics should use structured JSON lines where practical.

JSONL diagnostic fields SHOULD include:

- `ts_ms`
- `node_kind`
- `transport_peer_id`
- `principal_id`
- `session_id`
- `capability_ref`
- `event`
- `surface_id`
- `terminal_id` when applicable
- `seq` when applicable
- `status`
- `error_code` when applicable

## Measurement Requirements

Record:
- connection establishment time
- stream open time
- round-trip time for exchange request
- append log throughput for small chunks
- live channel median and p95 frame latency on localhost
- terminal input p50 and p95 echo latency
- terminal input p95 latency during burst output
- browser refresh and reattach behavior
- slow-reader behavior
- behavior when provider process exits
- behavior when browser refreshes

No production benchmark is required. The goal is to detect obvious blockers.

## Implementation Location

Start in ignored scratch if needed:

```text
/Users/owebeeone/limbo/glial-dev/scratch/phase1-libp2p/
```

If the spike becomes useful, promote it to a committed experiment folder such
as:

```text
/Users/owebeeone/limbo/glial-dev/experiments/phase1-libp2p/
```

Do not let scratch-only code become a hidden dependency.

## Candidate Layout If Promoted

```text
experiments/phase1-libp2p/
  README.md
  protocols/
    frames.schema.json
  js/
    browser-peer/
    node-peer/
  rust/
    glade-spike-node/
  python/
    provider-bridge/
    py-libp2p-smoke/
  notes/
    observations.md
    decision.md
```

## Decision Gates

### Gate A: Browser Local-Loop Viability

Question:
Can the browser participate in exchange, log, and live-channel streams without
a bespoke WebSocket API?

If no:
- libp2p remains native/backend-only
- browser uses a Glade gateway protocol

If yes:
- continue browser-native Glade participation path

Required result:
`green`, `yellow`, or `red`.

### Gate B: Rust Native Viability

Question:
Can a Rust node serve the spike protocols cleanly enough to become the Glade
native spine?

If no:
- consider JS/Node as first Glade dev spine
- revisit Rust later

If yes:
- make Rust the preferred deployment node candidate

Required result:
`green`, `yellow`, or `red`.

### Gate C: Browser-To-Rust Terminal Interop

Question:
Can browser JS libp2p connect to the Rust node over an acceptable transport and
drive the terminal harness?

If no:
- use Node.js bridge or browser gateway for early GripLab

If yes:
- use Rust as the Phase 2 native node path

Required result:
`green`, `yellow`, or `red`.

### Gate D: Python Stdio Bridge Viability

Question:
Can Python provider logic work through stdio without owning libp2p transport?

If no:
- evaluate Unix socket, HTTP, WebSocket, direct `py-libp2p`, or Rust-backed
  extension later

If yes:
- keep stdio bridge as the V0 Python provider path

Required result:
`green`, `yellow`, or `red`.

### Secondary Gate: Python Direct Viability

Question:
Is `py-libp2p` reliable enough for direct provider development?

This is classification only and MUST NOT block Gate C.

### Secondary Gate: Rust-Backed Python Viability

Question:
Would a Rust-backed Python extension provide better provider ergonomics and
deployment simplicity than a separate bridge process?

This is analysis only and MUST NOT block Gate C.

## Security Not Proven By Phase 1

Phase 1 does not prove:

- real Glade authorization
- package signing
- capability revocation
- metadata privacy
- tenant isolation
- relay trust
- production TLS/certificate lifecycle
- durable storage integrity
- canonical record signing

Any successful Phase 1 transport result MUST still be treated as transport
evidence only.

## Recommended Phase 1 Answer Before Testing

Expected answer:

```text
JS/libp2p for browser participation.
Rust/libp2p for deployable native Glade node and relay experiments.
Python providers integrate through a stdio bridge first.
py-libp2p remains a tracked option, not the critical path.
Rust-backed Python extension remains an analysis track and possible later
provider-packaging option.
```

The spike exists to validate or falsify this expectation.

## References

- py-libp2p GitHub README:
  `https://github.com/libp2p/py-libp2p`
- py-libp2p PyPI package:
  `https://pypi.org/project/libp2p/`
- py-libp2p PubSub docs:
  `https://py-libp2p.readthedocs.io/en/stable/libp2p.pubsub.html`
- rust-libp2p GitHub README:
  `https://github.com/libp2p/rust-libp2p`
- rust-libp2p docs:
  `https://libp2p.github.io/rust-libp2p/`
- libp2p browser connectivity:
  `https://libp2p.io/docs/browser-connectivity/`
- libp2p WebRTC browser connectivity:
  `https://libp2p.io/docs/webrtc-browser-connectivity/`
- libp2p WebRTC:
  `https://libp2p.io/docs/webrtc/`
- libp2p WebTransport:
  `https://libp2p.io/docs/webtransport/`
- libp2p hole punching:
  `https://libp2p.io/docs/hole-punching/`
- libp2p protocols:
  `https://libp2p.io/docs/protocols/`
- libp2p stream multiplexing:
  `https://docs.libp2p.io/concepts/multiplex/overview/`
