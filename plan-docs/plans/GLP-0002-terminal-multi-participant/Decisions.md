# Decisions

Plan: `GLP-0002`

Plan-local decisions. Promote stable ones to the root `DecisionLog.md`.

| ID | Decision | Rationale | Status |
| --- | --- | --- | --- |
| `D1` | Input arbitration = **single-driver lease** (owner_term/claim), not free-interleave, not concurrent multi-writer | Keeps the single-writer regime the lazy log depends on; gives clean convergence; seeds the avatar's "who is driving" | proposed |
| `D2` | Typing indicators + presence = **ephemeral live-stream**, never the append-log | Read/write asymmetry; low latency; loss-tolerant; replicating presence would add lag and abuse the substrate | proposed |
| `D3` | **Sim-substrate-first**: correctness/stress/scale proven in-process before real libp2p | Deterministic, fault-injectable, fast; real transport becomes a conformance re-run | accepted (inherited from prompt edit) |
| `D4` | **Identity threaded from `P02`**, before the avatar needs it | Avatar (`P05`) and lease (`P03`) both need attribution; retrofitting identity late is costly | proposed |
| `D5` | `P03` (multi-participant) == the **current-functionality gate**; avatar is the next phase, not part of this gate | Maintainer directive: stress point is multi-participant first, then push the avatar | accepted |
| `D6` | Multi-participant fan-out via **log/tap**, input via **1:1 live channel** | Input 1:1, output via the shared log → `N` watchers = `N` subscriptions, not `N` channels | accepted (inherited from proposal) |

## Open (resolve during execution)

- Presence channel realization: hub-aggregated vs gossipsub vs per-peer live-
  streams (lean hub-aggregated for known small membership).
- Reattach cutover exact semantics (replay cursor → live tail dedup/order).
- Lease handoff race resolution and expiry/timeout policy.
- Retention of live-channel logs / segments — ties to root `DecisionLog` GDL-012.

## Revision history

- (init) plan drafted.
