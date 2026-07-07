# Workstreams

Plan: `GLP-0002`

Multi-checkout model: write ownership allocated by module/folder. Agents MUST NOT
edit another workstream's scope. The coordinating agent owns shared root
navigation files (`Registry.md`, `ActiveWork.md`, this plan folder).

| Workstream | Owner | Write Scope | Must Not Touch | Output | Merge Risk |
| --- | --- | --- | --- | --- | --- |
| `WS-A` Substrate core + sim | TBD | `glade/` Rust core crate + simulated-substrate harness + core tests | `grip-lab/`, provider, transport adapter | append-log/segment/anchor core; sim substrate; property+scenario suites | Core API churn ripples to B/C/E — freeze API at `P00` |
| `WS-B` Glade surface + Python module | TBD | `glade/` binding layer (PyO3), append-log tap / live-channel / exchange / presence API | core internals (WS-A), provider logic | Python wheel; tap + channel + exchange + presence surface | Async-bridge model; ABI; depends on WS-A API |
| `WS-C` Provider (PTY host) | TBD | provider server (`glial-py`/`glial-server`) terminal path | `glade/` internals, `grip-lab/` | PTY spawn; OpenTerminal exchange; **lease arbitration**; presence aggregation | Lease-handoff semantics; embeds WS-B module |
| `WS-D` grip-lab frontend | TBD | `grip-lab/src/` (terminal producer, multi-participant UI, avatar UI) | substrate, provider, transport | real producer behind `terminalController` seam; multi-participant + avatar UI | Browser transport constraints (WS-E); seam contract with WS-B |
| `WS-E` Transport + hub | TBD | libp2p adapter + hub (introduction / presence relay) | core internals, app logic | raw-stream / req-resp / relay+DCUtR / rendezvous / browser WS-WebRTC; hub | Gate C browser transport; relay capacity; DCUtR success rate |

## Boundaries & dependencies

- WS-A is the upstream dependency; its core API is frozen at the `P00` gate.
- WS-B sits between WS-A (core) and WS-C/WS-D (consumers); the read/write split
  is enforced at the WS-B API boundary.
- WS-E is swappable behind the Substrate adapter; the simulated substrate
  (WS-A) substitutes for WS-E through `P03`.
- Shared contracts (entry/anchor format, sim-substrate interface) change only via
  explicit plan updates recorded in `Decisions.md`.
