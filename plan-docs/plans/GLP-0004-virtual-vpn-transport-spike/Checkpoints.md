# Checkpoints — GLP-0004

Date: 2026-06-05
Status: proposed — no checkpoints recorded yet.

Roll-build tags will use the phase ids from `Plan.md`. Record per phase:
completed goal, tag, verification result, measurements, rollback note.

| Phase | Tag | Status | Verification | Measurements | Rollback |
| --- | --- | --- | --- | --- | --- |
| `P01` raw-pipe | `glp0004-p01` (planned) | not started | WSS LCS session runs unchanged over the pipe | — | revert agent; WSS direct |
| `P02` traversal+relay | `glp0004-p02` (planned) | not started | connect through NAT; relay fallback | connect time direct vs relay; relay throughput | — |
| `P03` browser path | `glp0004-p03` (planned) | not started | terminal/diff UI unchanged; 0 iroh in browser bundle | bundle delta | — |
| `P04` test-port forward | `glp0004-p04` (planned) | not started | hit remote dev-server via localhost port | — | — |
| `P05` resume protocol (opt) | `glp0004-p05` (planned) | not started | resume from offset N; no dup | — | — |

Commit trailer convention for normal commits:

```text
Plan: GLP-0004
Phase: P01
Checkpoint: raw-pipe
```
