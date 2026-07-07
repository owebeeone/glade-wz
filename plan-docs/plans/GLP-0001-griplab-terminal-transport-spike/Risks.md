# Risks

Plan: `GLP-0001`

| Risk | Why It Matters | Test | Fallback |
| --- | --- | --- | --- |
| Browser/provider libp2p interop is awkward. | GripLab needs browser participation. | Try shortest browser-to-provider path. | Use a local gateway while keeping Glade-shaped frames. |
| OrbitDB default write ACL blocks peer append. | A p2p terminal log needs explicit writer admission. | Open A's DB from B and attempt append. | Use `--allow-any-writer` only for Phase 1, then design Glade writer admission. |
| OrbitDB convergence is not immediate. | Reattach/replay must know when history is complete enough. | Measure A-to-B and B-to-A visibility on localhost. | Treat OrbitDB as replay log only and keep hot terminal bytes on direct streams. |
| OrbitDB metadata and Helia blocks can diverge. | Reusing a persisted OrbitDB directory with missing blocks breaks restart. | Run one-shot sidecar twice against the same directory. | Persist Helia blocks with `blockstore-level` or use explicitly fresh scratch dirs. |
| The p2p stack has a large dependency graph. | Supply-chain and maintenance risk are real for a massive infra project. | Keep pinned versions, run `npm audit`, record package count. | Move the p2p sidecar behind a narrow process boundary. |
| Output bursts starve input. | Terminal feels broken even if bytes move. | Burst output while typing. | Split input/control and output data streams. |
| Replay cursor is misleading. | Reattach must be understandable. | Refresh mid-stream and resume. | Show stale/unavailable diagnostic and continue live. |
| PTY resize is noisy or racy. | WINCH matters but should not dominate. | Send resize frames and observe behavior. | Accept last-writer-wins resize in V0. |
| GripLab app build is blocked by sibling package outputs. | App-level verification can fail for reasons outside Phase 1 p2p. | Run `npm run build`. | Build or package sibling GRIP libraries in a separate cleanup step. |
| Spike expands into architecture work. | We lose the quick answer. | Compare work to non-goals. | Stop and write handoff. |
