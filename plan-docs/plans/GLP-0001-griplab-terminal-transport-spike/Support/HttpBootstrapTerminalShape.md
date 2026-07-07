# HTTP Bootstrap Terminal Shape

Status: throw-away Phase 1 proposal
Plan: `GLP-0001`

Purpose: describe the fastest plausible shape for testing browser + libp2p +
OrbitDB terminal communications with a Node.js sidecar running beside each
Python GripLab local client server.

This is not production Glade architecture.

## Claim

Phase 1 SHOULD keep one boring HTTP control edge.

The browser needs a trusted way to learn:

- which peer or sidecar to dial
- which browser-usable libp2p multiaddrs are valid
- which disposable protocols are enabled
- which short-lived authority token proves this browser may open or attach to a
  terminal
- which OrbitDB output log, if any, is bound to the terminal replay test

libp2p SHOULD carry the terminal test frames after bootstrap. It MUST NOT be
treated as the first source of credentials, authority, or user identity.

## Co-Resident Sidecar Topology

```text
Every GripLab local-client host
  -> Python local client server
  -> Node.js libp2p/Helia/OrbitDB sidecar
  -> local-only IPC between Python and Node when needed

Browser GripLab UI
  -> HTTP bootstrap endpoint on a Node sidecar
  -> browser js-libp2p peer
  -> Node sidecar libp2p mesh
```

The current GripLab local client server remains available for the existing
application. Phase 1 SHOULD still expose only the Node.js sidecar to the
browser for the p2p terminal test. The Python server and Node sidecar are
co-located, so any Python/Node bridge is local-only and does not require a
second browser-facing tunnel.

The Node.js sidecar is the Phase 1 mesh adapter:

- it owns the libp2p peer
- it owns the OrbitDB replay-log experiment
- it owns HTTP bootstrap for browser admission
- it maps browser libp2p frames to a local terminal provider
- it MAY use a disposable Node PTY provider for the first pressure test
- it MAY bridge to the Python local client over local IPC for a second pressure
  test
- it MUST NOT become the production terminal service or final Glade authority

This avoids requiring two browser-facing tunnels just to learn whether the
browser, libp2p, OrbitDB replay log, and terminal frame shape work together.

## Provider Options

### Option A: Node-Owned Disposable PTY

This is the recommended first checkpoint if the immediate goal is transport
pressure.

The sidecar opens a local PTY from Node.js, sends browser input to that PTY, and
streams PTY output back over libp2p while also appending output chunks to
OrbitDB.

Benefits:

- one browser-facing Phase 1 component
- no Python event-loop bridge during the first pressure test
- no dependency on the current local-client websocket protocol
- direct measurement of libp2p and OrbitDB friction under terminal output load

Costs:

- it does not test the current Python local-client terminal implementation
- it adds native Node PTY dependency/build friction
- it is POSIX-first unless Windows PTY support is explicitly added

### Option B: Node-To-Python Local Protocol Bridge

This is the recommended second checkpoint if the test needs to exercise the
current Python local-client terminal behavior.

```text
Browser -> Node sidecar -> local Python server/worker -> Python PTY/session code
```

The bridge MAY use one of:

- the current loopback websocket protocol
- a Unix domain socket JSONL protocol
- a Python child process with JSONL stdio

This is not FFI. It is a process boundary with a small typed protocol. It keeps
Node as the only browser-facing process and avoids embedding Python in Node.
Because Python and Node are co-resident on the same host, this bridge is local
IPC, not a network topology problem.

Costs:

- the bridge may hide libp2p latency behind Python/local-client behavior
- current websocket protocol shape may leak into the disposable frame contract
- terminal lifecycle and replay responsibility must be explicitly assigned

### Option C: PTY Descriptor/Handle Handoff

This is possible, but OS-specific. It may be useful if Python should create and
supervise the terminal while Node directly reads and writes the terminal byte
handles.

```text
Unix:
  Python opens PTY
  Python sends PTY master fd to Node over Unix domain socket
  Node pumps fd bytes over libp2p and OrbitDB
  Python keeps lifecycle/session responsibility

Windows:
  Python opens ConPTY or pipes
  Python passes duplicated/inherited pipe HANDLEs to Node, or exposes named pipes
  Node pumps pipe bytes over libp2p and OrbitDB
  Python keeps ConPTY/session responsibility
```

Mechanism:

```text
Unix/macOS/Linux:
  Unix domain socket + sendmsg/recvmsg + SCM_RIGHTS

Windows:
  CreatePipe/CreateNamedPipe + ConPTY
  handle inheritance at process creation, or DuplicateHandle to the target PID
  local control channel carries terminal id, handle metadata, resize, close
```

This is local-only, but that matches the intended co-resident sidecar model. It
does not send an fd or HANDLE across the p2p mesh. It gives the adjacent Node
process a local descriptor/handle for the same terminal byte stream.

Costs:

- generic Python-to-Node fd passing is not exposed cleanly by Node core
- generic Python-to-Node Windows HANDLE passing is also not exposed cleanly by
  Node core
- Node may need a native addon, small helper, or carefully chosen local IPC
  library to receive `SCM_RIGHTS` on Unix or duplicated HANDLEs on Windows
- resize, signals, child cleanup, EOF, and process groups still need a control
  protocol
- Windows ConPTY lifecycle is not equivalent to POSIX PTY lifecycle

This SHOULD NOT be the first checkpoint, but it is a plausible Phase 1B or
Phase 1C bridge if JSONL/websocket bridging is too indirect.

### Option D: Same-Process Embedded Python

This is a throw-away option if Phase 1 wants Python terminal code and raw
descriptor/handle sharing without Unix `SCM_RIGHTS` or Windows
`DuplicateHandle`.

```text
Node sidecar process
  -> native preload/addon/wrapper loads CPython
  -> Python interpreter runs on its own thread
  -> Python terminal code talks to Node through an in-process pipe/queue
  -> fd/HANDLE values are passed as raw process-local integers
```

In the same process:

- POSIX fd numbers are valid across all threads
- Windows HANDLE values are valid across all threads
- no descriptor copy is needed just to share the byte stream

The cleanest throw-away direction is Node as the host process. The spike MAY use
one of:

- a Node native addon that starts a Python thread
- a launcher/wrapper that loads Node and Python into one process
- Linux `LD_PRELOAD` with a constructor that starts the Python side thread
- macOS `DYLD_INSERT_LIBRARIES`, subject to local platform restrictions
- Windows DLL injection or a wrapper executable, if time-boxed and local-only

The Python thread SHOULD communicate with Node through an OS pipe, socketpair,
or native in-process queue. Node SHOULD NOT call into Python on the hot path.
That keeps GIL handling away from terminal byte transfer; Python still needs the
GIL for Python execution, but Node does not need to hold it just to read/write a
shared fd or HANDLE.

This option is allowed only as a time-boxed experiment. It answers whether
same-process descriptor sharing can remove IPC friction. It does not produce a
production architecture.

Costs:

- build and loader mechanics may dominate the time box
- macOS SIP/hardened runtime and Windows injection rules may make preload-style
  loading brittle
- one process crash loses both runtimes, which is acceptable only for this
  throw-away test
- terminal close/shutdown may leak resources; Phase 1 MAY tolerate that if the
  process exits cleanly enough for repeated manual runs

### Option E: Hot-Path Node/Python FFI

Calling Python APIs directly from Node on the terminal hot path SHOULD NOT be
used for Phase 1.

It is technically possible through native addons and embedded CPython, but it
adds the wrong risks for this spike:

- CPython GIL ownership on every cross-runtime call
- Node event loop and Python `asyncio` coordination
- callback lifetime and object ownership across runtimes
- native build and wheel/addon packaging without a transport payoff

Option D is different: it allows same-process embedding only if Python and Node
communicate over a pipe/queue and raw process-local descriptors or HANDLEs, with
no Python C API calls on the terminal byte path.

## Startup

1. The Node.js Phase 1 sidecar starts.
2. The Python local client may already be running, but it is not browser-facing
   for this Phase 1 path.
3. The sidecar starts libp2p, Helia, OrbitDB, and the selected local terminal
   provider.
4. The sidecar exposes a local HTTP bootstrap endpoint.
5. The browser loads GripLab normally, then requests Phase 1 mesh bootstrap
   metadata over HTTP.

Local dev MAY use loopback HTTP. LAN or remote browser tests SHOULD move to
HTTPS/WSS or another secure-origin-compatible setup as soon as browser transport
friction becomes the main question.

## Bootstrap Endpoint

Disposable endpoint:

```text
GET /phase1/bootstrap
```

Response sketch:

```json
{
  "bootstrap_id": "boot:dev-001",
  "actor_id": "actor:local-browser",
  "workspace_id": "workspace:local",
  "expires_at_ms": 1730000600000,
  "mesh_token": "opaque-dev-token",
  "sidecar": {
    "transport_peer_id": "12D3Koo...",
    "listen_multiaddrs": [
      "/ip4/127.0.0.1/tcp/41501/ws/p2p/12D3Koo..."
    ]
  },
  "protocols": {
    "exchange": "/glade-spike/exchange/0.1.0",
    "live": "/glade-spike/live/0.1.0"
  },
  "orbitdb": {
    "enabled": true,
    "output_log_address": null
  },
  "limits": {
    "max_token_age_ms": 600000,
    "max_frame_bytes": 65536
  }
}
```

For Phase 1, `mesh_token` MAY be an opaque random token stored in sidecar
memory. It SHOULD expire quickly. It MUST NOT be written to OrbitDB. It MUST NOT
be confused with a final Glade capability.

The first libp2p exchange frame SHOULD include `bootstrap_id` and `mesh_token`.
That is enough to test admission and failure diagnostics without designing final
identity.

## Open Terminal Flow

```text
browser -> HTTP: GET /phase1/bootstrap
browser -> libp2p sidecar: open_terminal + mesh_token
sidecar -> selected local provider: spawn shell/command
sidecar -> OrbitDB: create/bind output log
sidecar -> browser: open_terminal_result
```

The sidecar maps the disposable frame to the selected local provider:

| Phase 1 frame | Sidecar action |
| --- | --- |
| `open_terminal` | ask provider to spawn PTY with cwd, argv, env, cols, rows |
| `terminal_input` | write bytes to provider |
| `terminal_resize` | resize provider PTY |
| `terminal_close` | terminate provider PTY |
| `terminal_output` | emit provider output chunk and append log record |

The disposable `terminal_id` SHOULD be sidecar-local, for example
`terminal:phase1-001`. It MAY carry a `provider_kind` diagnostic with value
`node-pty`, `python-ws`, `python-stdio`, `python-uds`, or `python-fd` so the
test output is explicit about which terminal provider was used.

## Live Channel Flow

After `open_terminal_result`, the browser opens or reuses the Phase 1 live
libp2p stream:

```text
browser -> sidecar: terminal_input
browser -> sidecar: terminal_resize
sidecar -> browser: terminal_output
either -> either: terminal_close | terminal_diag
```

The sidecar sends input and resize to the PTY provider. It forwards PTY output
as `terminal_output`.

The sidecar SHOULD coalesce terminal output before appending to the OrbitDB
output log. A log entry SHOULD represent a bounded byte/time window, not every
tiny PTY write. Phase 1 MUST record whether append-before-send or
append-parallel creates unacceptable latency.

## Reattach Flow

Browser refresh is part of the test.

```text
browser refresh
browser -> HTTP: GET /phase1/bootstrap
browser -> libp2p sidecar: terminal_attach { terminal_id, cursor, mesh_token }
sidecar -> OrbitDB: replay from cursor
sidecar -> selected local provider: ensure terminal is still live
sidecar -> browser: replay output, then live output
```

If the requested cursor is unavailable, the sidecar MUST emit
`terminal_diag(code="replay_unavailable")` and then continue live if the PTY is
still attached.

## Why HTTP Is Acceptable Here

HTTP is the bootstrap and admission edge, not the terminal data plane.

This is pragmatic because:

- browsers need out-of-band dial information for libp2p
- browser libp2p transports have origin and address constraints
- authority must come from an already trusted process
- libp2p `PeerId` is transport identity, not a Glade principal
- Phase 1 must test communications friction, not invent final capability
  issuance

The later Glade version can replace the opaque token with a signed capability,
but it still probably needs an entrypoint that tells a browser where to start.

## Minimal Sidecar Responsibilities

The sidecar MUST:

- serve bootstrap metadata
- validate the disposable mesh token on first libp2p exchange/live frames
- keep a binding between `terminal_id`, provider session/process, and OrbitDB
  output log address
- translate terminal frames to the selected local provider
- append coalesced output-byte records to OrbitDB
- expose diagnostics for token failure, dial failure, PTY failure, replay
  failure, stream close, and slow append

The sidecar SHOULD NOT:

- replace the GripLab local client server
- treat libp2p `PeerId` as authorization
- own final provider placement
- hide failures behind automatic retries without diagnostics

## Disposable API Surface

Suggested HTTP endpoints:

| Endpoint | Purpose |
| --- | --- |
| `GET /phase1/bootstrap` | Return token, peer id, multiaddrs, protocols, and limits. |
| `GET /phase1/status` | Return sidecar health, libp2p peer id, connected peers, and OrbitDB status. |
| `GET /phase1/terminals` | Return sidecar-known terminal/session/log bindings for refresh tests. |

Suggested libp2p protocols:

| Protocol | Purpose |
| --- | --- |
| `/glade-spike/exchange/0.1.0` | `open_terminal`, `terminal_attach`, bounded replies. |
| `/glade-spike/live/0.1.0` | input, resize, output, close, diagnostics. |

No Phase 1 endpoint is stable.

## What This Proves

This shape can answer:

- can a browser join the local libp2p sidecar after HTTP bootstrap?
- can libp2p carry terminal input/output frames with acceptable latency?
- does a Node-owned disposable PTY produce enough real terminal pressure?
- if needed, can an adjacent Python local client be used without a second
  browser-facing tunnel?
- can OrbitDB act as a replay-log pressure test for terminal output?
- can refresh and cursor reattach be explained cleanly?
- does a single live stream starve input during burst output?

## What This Does Not Prove

This shape does not prove:

- production Glade credentials
- production provider placement
- NAT traversal
- relay behavior
- durable OrbitDB storage
- multi-writer terminal policy
- browser-to-Rust interop
- replacing the current GripLab backend
- final compatibility with the current Python local-client session store unless
  Option B or C is explicitly run

Those are later decisions.

## Biggest Risks

| Risk | Phase 1 handling |
| --- | --- |
| Browser cannot dial the sidecar over the chosen libp2p transport. | Record red/yellow and try the gateway fallback with the same frames. |
| HTTP token becomes a fake security model. | Keep token short-lived, local, explicit, and documented as non-production. |
| Node PTY dependency or native build fails. | Fall back to Python stdio bridge or a minimal command subprocess. |
| Node-owned PTY is not representative of GripLab local-client behavior. | Treat it as transport pressure only; add Python stdio bridge as a second checkpoint if needed. |
| Python-to-Node fd handoff becomes the project. | Keep fd passing as optional Phase 1B/1C; do not block the libp2p pressure test on it. |
| OrbitDB append slows live output. | Measure append-before-send and append-parallel modes. |
| Per-session DB topology does not scale. | Keep per-session DB vs shared DB as a Phase 1 knob and feed both into the simulator. |
| Sidecar becomes hidden authority. | Document it as the throw-away terminal provider; do not reuse it as final Glade authority. |
| Refresh cannot recover understandable history. | Emit explicit cursor diagnostics and classify replay as yellow/red. |

## First Checkpoints

1. HTTP bootstrap returns usable multiaddrs and a token.
2. Browser dials the sidecar and sends an authenticated `hello` frame.
3. Browser sends `open_terminal`; sidecar spawns a disposable PTY.
4. Browser sends input over libp2p; sidecar writes to the PTY; output returns
   over libp2p.
5. Sidecar appends coalesced output bytes to OrbitDB and replays entries after
   the last-seen entry CID after refresh.
6. Optional: replace Node PTY with a local Python bridge while keeping the same
   browser-facing Node sidecar.
7. Burst output test records p50/p95 input echo latency and replay behavior.
8. The plan records green/yellow/red plus the fallback route.

## Fallback Shape

If browser libp2p is too fragile, keep the same HTTP bootstrap endpoint and
frame envelope, but carry `/glade-spike/exchange/0.1.0` and
`/glade-spike/live/0.1.0` over a normal websocket gateway.

That fallback is still useful if it preserves:

- the same terminal frame contract
- the same replay cursor
- the same OrbitDB append-log experiment
- the same diagnostics
- the same separation between bootstrap authority and hot-path transport
