# Risks

Plan: `GLP-0002`

| ID | Risk | Severity | Mitigation / Fallback |
| --- | --- | --- | --- |
| `R1` | **Input arbitration** lease handoff has races/edge cases (double-driver, lost lease, stuck driver) | High | Model-check lease states in sim first; explicit expiry/heartbeat; fallback to watch-only multi-participant if unstable |
| `R2` | **Reattach cutover** (replay cursor → live tail) drops or duplicates output at the boundary | High | Dedicated cutover scenario tests in sim; define cursor/dedup contract before `P02`; single-writer keeps it tractable |
| `R3` | **Browser transport** (Gate C): WS/WebRTC + wasm + relay/DCUtR maturity for browser↔Rust | High | Prove on the `GLP-0001` spike; keep relay path as fallback if DCUtR direct fails; sim substrate isolates non-transport bugs |
| `R4` | **Dependency slip**: `GladeRustOrbitStrategy.md` late or divergent | Medium | `P00` gate explicit; WS-A may start with a thin stubbed core API to unblock WS-B/D against the seam |
| `R5` | **Sim/real fidelity gap**: sim hides real-network behavior (timing, backpressure, partial failure) | Medium | Sim models reorder/delay/drop/partition; `P04` conformance re-run is mandatory, not optional |
| `R6` | **Presence accuracy under churn** (`P05`): stale "typing"/"present" flags | Medium | Heartbeat + TTL on ephemeral signals; loss-tolerant by design; test under join/leave churn |
| `R7` | **Identity plumbing retrofitted late** breaks lease + avatar | Medium | `D4`: thread identity from `P02` |
| `R8` | **Core API churn** ripples across WS-B/C/D/E | Medium | Freeze core API at `P00`; shared-contract changes only via `Decisions.md` |
| `R9` | **Scope creep** into multi-writer / concurrent drivers | Low | Non-goal; `D1` single-driver lease; multi-writer explicitly deferred |
