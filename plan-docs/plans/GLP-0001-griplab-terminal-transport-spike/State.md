# State

Plan: `GLP-0001`
Status: active
Current phase: `P03`
Branch/checkouts: root checkout on `main`; `grip-lab` branch
`codex/phase1-node-orbitdb-spike`

## Current Position

The terminal/libp2p spike is approved as a disposable investigation. The current
implementation track has moved to a Node.js libp2p + OrbitDB sidecar in
`grip-lab`; the existing GripLab backend stays in place.

Current local evidence:

- `grip-lab` now has a Phase 1 sidecar that starts js-libp2p over localhost
  WebSocket, creates Helia + OrbitDB, opens an OrbitDB events database, appends a
  Glade-shaped terminal output frame, reads it back, and exits in `--once` mode.
- A second peer can dial the first peer and open its OrbitDB address.
- OrbitDB default write access rejects the second peer, so Phase 1 added an
  explicit `--allow-any-writer` test setting. This MUST NOT be treated as
  production authorization.
- With `--allow-any-writer`, the second peer can append. Immediate readback in
  the current script can wait for an expected record count. A local two-peer
  run reached `record_count: 2` within a 10-second wait window.
- Reusing an OrbitDB directory with Helia's default in-memory blockstore caused
  missing-block failures on restart. The sidecar now uses a persistent
  `blockstore-level` directory beside the OrbitDB directory.

## Next Checkpoint

Move from p2p/OrbitDB substrate proof toward live terminal transport over:

```text
current GripLab backend remains in place
Node sidecar/libp2p transport -> terminal-like input/output harness
```

Acceptance: terminal-like input/output frames move over libp2p without replacing
the current GripLab backend, and output frames continue to append to the
OrbitDB-backed replay log.

## Blockers

- Full `npm run build` in `grip-lab` is currently blocked before the Phase 1
  sidecar path because sibling file dependency `../grip-react` has no `dist/`
  output, so TypeScript cannot resolve `@owebeeone/grip-react`.

## Stop Conditions

Stop or redirect if:

- the contract starts solving full collaboration semantics
- the implementation path requires production Glade records before testing
  terminal latency
- Node/libp2p/OrbitDB setup dominates the terminal test
- the same answer can be obtained faster through a gateway fallback experiment
