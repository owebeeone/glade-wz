# Checkpoints

Plan: `GLP-0001`

| Checkpoint | Phase | Status | Acceptance |
| --- | --- | --- | --- |
| `terminal-contract` | `P01` | complete | Disposable frame, lifecycle, replay, and diagnostic contract exists. |
| `implementation-track` | `P02` | superseded | Rust provider was selected first, then superseded by Node.js libp2p + OrbitDB sidecar per user redirect. |
| `node-orbitdb-sidecar` | `P03` | complete | Node sidecar boots libp2p, Helia, OrbitDB, appends a Glade-shaped terminal output frame, reads it back, and exits. |
| `two-peer-orbitdb-visibility` | `P04` | complete | Two sidecars connect over localhost WebSocket; B opens A's OrbitDB address, appends under an open Phase 1 ACL, and reaches `record_count: 2` within a 10-second wait. |
| `live-io` | `P05` | pending | Input/output/resize/close move over libp2p without replacing the current GripLab backend. |
| `replay-cursor` | `P04` | pending | Browser refresh can resume or show clear replay diagnostic. |
| `burst-slow-reader` | `P06` | pending | Burst output and slow reader behavior are measured. |
| `decision` | `P07` | pending | Handoff records green/yellow/red and next plan. |

## Commit Trailer

Use this trailer for spike commits:

```text
Plan: GLP-0001
Phase: P##
Checkpoint: <checkpoint-name>
```
