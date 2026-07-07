# Client Bootstrap Sidecar Design

Status: Phase 1 design sketch
Plan: `GLP-0001`

Purpose: define a coherent bootstrap shape for running the current GripLab
Python local client plus an adjacent Node.js libp2p/OrbitDB sidecar on each
collaborator host.

This is still throw-away Phase 1 infrastructure. It MUST make the test reliable
enough to collect libp2p and OrbitDB evidence, but it MUST NOT harden into the
final Glade node manager.

## Current Fact Base

The current GripLab hub bootstrap already has these facts:

- The hub owns collaborator bootstrap over SSH.
- `remoteHubPort` is a remote loopback reverse forward to the hub websocket.
- `remoteClientPort` is the remote Python local-client listen port.
- `localPeerPort` is the hub-local forward to that remote Python client.
- The hub currently prepares a copied client payload and writes remote
  `client.json` / `forward.json`.
- The current prepare path can stop the remote client while updating payload.

The sidecar adds a second co-resident service. Therefore bootstrap can no longer
think in terms of "the client port"; it MUST assign a named port plan.

## Recommendation

Phase 1 SHOULD split bootstrap into three layers:

1. **Tunnel session**: long-lived SSH process that owns all forwards and does
   not run the client payload directly.
2. **Prepare/sync**: short-lived SSH/scp/rsync work that stages exact client and
   sidecar code plus config.
3. **Runtime supervisor**: remote wrapper that starts/stops the Python client
   and Node sidecar as sibling children.

This split keeps the control path alive while code is refreshed or child
processes restart. The current "SSH command owns both forwards and child
process" shape is workable for one Python client, but becomes brittle once the
sidecar also needs ports and restart coordination.

## Port Plan

Bootstrap MUST allocate stable named ports per collaborator. Numeric fields
SHOULD be derived once per peer and reused across retries.

```ts
interface BootstrapPortPlan {
  peerId: string;

  // Existing Python local-client path.
  remoteHubPort: number;       // remote -> hub websocket, SSH -R
  remoteClientPort: number;    // remote Python client listen port
  localClientPort: number;     // hub -> remote Python client, SSH -L

  // New Node sidecar path.
  remoteSidecarPort: number;       // remote Node sidecar libp2p WebSocket port
  localSidecarPort: number;        // hub/browser -> remote sidecar libp2p, SSH -L
  remoteSidecarBootstrapPort: number; // remote Node sidecar HTTP bootstrap port
  localSidecarBootstrapPort: number;  // hub/browser -> remote sidecar HTTP bootstrap, SSH -L
  remoteHubSidecarPort?: number;   // remote Node sidecar -> hub Node sidecar, SSH -R
}
```

Minimum Phase 1 forwards:

```text
-R 127.0.0.1:<remoteHubPort>:127.0.0.1:<hubWsPort>
-L 127.0.0.1:<localClientPort>:127.0.0.1:<remoteClientPort>
-L 127.0.0.1:<localSidecarPort>:127.0.0.1:<remoteSidecarPort>
-L 127.0.0.1:<localSidecarBootstrapPort>:127.0.0.1:<remoteSidecarBootstrapPort>
```

If the sidecar mesh needs hub-side sidecar bootstrapping, add:

```text
-R 127.0.0.1:<remoteHubSidecarPort>:127.0.0.1:<hubSidecarPort>
```

That fourth forward is the likely "bidirectional" piece. It lets the remote
Node sidecar dial the hub Node sidecar through remote loopback while the hub or
browser can dial the remote sidecar through hub loopback.

The Python client and Node sidecar on the same collaborator host SHOULD talk to
each other over `127.0.0.1` or local IPC. They SHOULD NOT need a tunnel for
co-resident communication.

## Sidecar Config

Bootstrap MUST write a sidecar config next to the existing client config:

```text
<workspace>/.griplab/client.json
<workspace>/.griplab/sidecar.json
<workspace>/.griplab/forward.json
```

`sidecar.json` SHOULD include:

```json
{
  "selfPeerId": "weftpi",
  "listen": {
    "host": "127.0.0.1",
    "port": 43180
  },
  "bootstrap": {
    "listen": {
      "host": "127.0.0.1",
      "port": 43280
    }
  },
  "advertise": {
    "browserHttpUrl": "http://127.0.0.1:42181",
    "browserWsMultiaddr": "/ip4/127.0.0.1/tcp/42180/ws"
  },
  "pythonClient": {
    "baseUrl": "http://127.0.0.1:3141",
    "wsUrl": "ws://127.0.0.1:3141/ws"
  },
  "hub": {
    "wsUrl": "ws://127.0.0.1:43140/ws"
  },
  "mesh": {
    "seedMultiaddrs": [
      "/ip4/127.0.0.1/tcp/43190/ws"
    ]
  },
  "orbitdb": {
    "directory": "<workspace>/.griplab/phase1-orbit"
  }
}
```

`advertise.browserHttpUrl` and `advertise.browserWsMultiaddr` are intentionally
hub-local values for a browser running on the hub machine. Without this, a
remote sidecar would advertise `127.0.0.1:<remoteSidecarPort>`, which is only
valid on the collaborator host and not dialable by the browser through the SSH
forward.

The sidecar uses two ports because the libp2p WebSocket listener does not serve
ordinary HTTP routes. `browserHttpUrl` points at the HTTP bootstrap port;
`browserWsMultiaddr` points at the libp2p WebSocket port.

`forward.json` SHOULD become the human/debug manifest of every assigned port,
not only the Python client forwards.

## Sync Strategy

Bootstrap MUST prove that the remote payload matches the hub payload hash before
declaring prepare successful.

Rsync SHOULD be used when all of the following are true:

- local `rsync` exists;
- remote `rsync` exists;
- the target OS/path shape is compatible with the command being issued;
- the sync target is a staged generation directory, not the active runtime
  directory.

If rsync is unavailable, bootstrap MUST fall back to the existing Python/scp
payload-copy flow.

The sync target SHOULD be generation-based:

```text
<workspace>/.griplab/releases/<payload-hash>/
<workspace>/.griplab/current.json
```

Bootstrap SHOULD NOT delete or rewrite the active `client_payload` directory
while a running client may still be executing from it. It should stage a new
release, write config that points at that release, start the new generation,
then retire the old generation after health passes or after an explicit
best-effort cleanup window.

For the fastest Phase 1 implementation, `current.json` MAY replace symlinks so
Windows and locked-down filesystems do not block the design.

## Runtime Supervisor

The remote runtime SHOULD be a small Python supervisor launched by bootstrap.
It owns exactly two child roles:

- `python-client`
- `node-sidecar`

The supervisor MUST write a pid/status file:

```json
{
  "generation": "sha256:...",
  "pythonClient": {
    "pid": 1234,
    "port": 3141,
    "status": "running"
  },
  "nodeSidecar": {
    "pid": 1235,
    "port": 43180,
    "status": "running"
  }
}
```

The supervisor MAY be crude for Phase 1. It only needs:

- start both children from the staged generation;
- stop an older generation by pid file;
- report status;
- redirect child stdout/stderr to `.griplab/logs`.

The tunnel SSH process MUST NOT depend on the supervisor or child process
lifetime. If a child crashes, the forwards SHOULD remain up long enough for the
hub to fetch logs and issue a restart.

## Bootstrap Flow

### Cold Start

```text
hub allocates BootstrapPortPlan
hub starts/refreshes tunnel-only SSH session with all forwards
hub diagnoses remote tools
hub syncs/stages payload generation
hub writes client.json, sidecar.json, forward.json
hub starts supervisor generation
supervisor starts Python client and Node sidecar
hub waits for:
  - Python client health through localClientPort
  - Node sidecar health/bootstrap through localSidecarBootstrapPort
  - Node sidecar libp2p dialability through localSidecarPort
  - peer.hello/heartbeat through remoteHubPort
hub marks peer starting/online
```

### Online Refresh

```text
hub keeps existing tunnel session alive
hub syncs/stages new payload generation
hub writes next config files
hub asks supervisor to start next generation
hub waits for next Python client and sidecar health
hub updates registered tunnel/sidecar endpoints
hub stops old generation
```

Online refresh SHOULD NOT stop the current Python client before the new payload
is staged. It MAY briefly drop terminal sessions during child replacement, but
the hub should keep enough tunnel/control state to recover.

### Failed Refresh

If the new generation fails health:

- existing generation SHOULD remain running when possible;
- bootstrap state MUST report the failed generation and log paths;
- the tunnel session SHOULD remain up;
- retry SHOULD reuse the same named port plan unless the failure is port
  binding itself.

## Browser Bootstrap

The browser SHOULD continue to load GripLab through the current UI path. For
Phase 1 terminal mesh bootstrap it asks the hub/Python client for sidecar
bootstrap metadata and receives a browser-dialable sidecar endpoint:

```json
{
  "peerId": "weftpi",
  "sidecar": {
    "httpUrl": "http://127.0.0.1:42180",
    "bootstrapUrl": "http://127.0.0.1:42180/phase1/bootstrap",
    "wsMultiaddr": "/ip4/127.0.0.1/tcp/42180/ws"
  }
}
```

The browser then calls the sidecar bootstrap endpoint for the short-lived mesh
token and libp2p/OrbitDB terminal metadata defined in
`HttpBootstrapTerminalShape.md`.

## Implementation Slice

The next implementation pass SHOULD be test-first and small:

1. Extend the Python `ForwardPlan` into a named port plan with sidecar fields.
2. Add tests that SSH tunnel command builders emit the extra `-L` sidecar
   forward and optional `-R` hub-sidecar forward.
3. Add tests that `sidecar.json` contains Python client URL, hub URL,
   browser-advertised forwarded URL, and OrbitDB directory.
4. Change prepare so online refresh does not stop the active client before
   payload staging.
5. Add an rsync-capability probe and command builder, with existing scp copy as
   fallback.
6. Split tunnel-only SSH lifetime from remote child-process lifetime.
7. Add a minimal remote supervisor only after the command/config tests pass.

## Open Decisions

- Does the hub run its own Node sidecar in Phase 1, or is each collaborator
  sidecar only dialed by the browser through `localSidecarPort`?
- Should `peer.health.get` include sidecar status directly, or should it expose
  only bootstrap-level checks until the sidecar endpoint proves useful?
