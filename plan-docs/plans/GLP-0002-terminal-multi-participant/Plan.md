# GLP-0002: Multi-Participant Terminal + Typing Avatar

Status: draft
Owner: maintainer (coordinating); multi-agent by workstream
Affected modules: `glade` (substrate / Rust core / Python module), `grip-lab`
(frontend), provider server (PTY host — `glial-py` / `glial-server`, confirm),
transport (`rust-libp2p` adapter + hub), root `plan-docs`

## Goal

Deliver the terminal slice at **current target functionality — a single live
terminal shared by multiple participants** — and then add the **typing-indicator
avatar ("who is typing")** as the immediate follow-on.

The **explicit stress point for this phase is multiple terminal participants.**
Everything is designed and tested against that first; the avatar is the next
push, gated on current functionality being green.

This plan executes the design in `glade/dev-docs/GladeTerminalSliceProposal.md`
and consumes the Rust-core design from `glade/dev-docs/GladeRustOrbitStrategy.md`
(authored separately). It follows the transport spike `GLP-0001`.

## Core Claim

Multi-participant is **not** mainly a fan-out problem (that falls out of the
append-log → tap design). The hard nucleus is **input arbitration** (who holds
the keyboard) plus **late-join reattach** and **fan-out convergence**. All three
are validated on an **in-process simulated substrate** (same core API as the
libp2p adapter) before any real networking. The typing avatar then reuses the
arbitration lease ("who is driving") and a second ephemeral live-stream
("who is typing").

## Non-Goals

- Multi-writer *concurrent* PTY input (true simultaneous writers). v1 uses a
  **single-driver lease**, not concurrent multi-writer — stays in the single-
  writer regime the proposal depends on.
- Multi-writer/concurrent-document CRDT semantics.
- Full Frankenapp composition; AI-agent participant; non-terminal surfaces.
- Final transport-language choice beyond Rust-core + Python-binding (+ wasm).

## Architectural invariants (inherited, normative)

- **Read/write asymmetry.** Terminal output → append-log → tap (replicated,
  lazy). Keystrokes and typing indicators → live-stream (directed, ephemeral,
  NOT replicated).
- **Core decoupled from libp2p.** The oplog/sync core MUST sit behind a narrow
  Substrate/Transport adapter. A pure simulated substrate MUST drive the same
  core API for correctness, stress, replay, and scale testing.
- **Session == single-writer segmented log**, hub-introduced, directly synced.
  Cost O(participants), not O(total sessions).
- **Identity is threaded from P02**, before the avatar needs it.

## Phases

| Phase | Goal | Acceptance gate |
| --- | --- | --- |
| `P00` | Lock dependencies & core API contract | `GladeRustOrbitStrategy.md` accepted; append-log / live-channel / exchange / Substrate-adapter / simulated-substrate interfaces agreed and written down; identity model in the contract; `GLP-0001` spike learnings folded in. |
| `P01` | Substrate core + simulated substrate | Single-writer append-log (segments + signed anchors) behind the Substrate adapter; in-process sim substrate driving the same API; property/convergence tests green; lazy window + reattach unit-tested in sim under adversarial schedules. |
| `P02` | Single-participant terminal over the real seam (loopback/sim transport) | Provider spawns a PTY; output → append-log (+ live channel); keystrokes → live channel; `grip-lab` `terminalController` real producer (tap for output, channel handle for input); **mock→real swap with no consumer rewrite**; session carries participant identity. |
| `P03` | **Multi-participant (the stress point)** | (1) output fan-out 1→N via log/tap at sim `N=2..1000`; (2) late join / **reattach** (replay cursor → live tail) correct; (3) **input arbitration**: single-driver lease (owner_term/claim) — exactly one driver, clean request/grant/handoff/expiry, others watch; (4) all green under sim adversarial schedule incl. join/leave churn + partition/heal + reorder. **This gate == "current level of functionality."** |
| `P04` | Real transport (libp2p) | Wire the libp2p adapter: raw-stream live channel, request-response catch-up, relay + DCUtR reach, rendezvous introduction, browser WS/WebRTC. Re-run the full `P03` acceptance over real libp2p, browser ↔ provider ↔ hub. `Phase1Libp2pTest.md` **Gate C** (browser→Rust PTY) passes; latency/throughput within target. |
| `P05` | **Typing avatar ("who is typing") — NEXT** | Presence (identity + join/leave, hub-aggregated); ephemeral per-participant **typing signal** as a live-stream (debounced heartbeat, loss-tolerant, **not** in the log); driving state surfaced ("who is driving" from the P03 lease); `grip-lab` avatar UI. Accurate, low-latency who's-here / who's-typing / who's-driving with identity attribution; correct under churn; verified in sim then over real transport. **Gated on `P04` green.** |

## Sequencing

- `P00 → P01 → P02 → P03` is strictly ordered. `P03` is the current-functionality
  gate.
- `P04` re-proves `P03` over the wire. `P05` (avatar) starts only after `P04`.
- Sim-substrate-first: no phase depends on real networking for its correctness
  proof; real transport is a conformance re-run, not the primary gate.

## Verification

- **Primary gate: the simulated-substrate harness.** Deterministic, fault-
  injecting (reorder, delay, drop, partition/heal, churn). Scenario suites:
  fan-out, reattach cutover, lease request/grant/handoff/expiry races,
  join/leave churn, partition/heal.
- **Property tests**: single-writer convergence + deterministic ordering;
  window/segment/anchor integrity; multi-writer is **detected/refused**, not
  silently wrong.
- **Conformance**: scenarios that pass in sim MUST reproduce over the libp2p
  adapter (`P04`).
- **Gate C** from `Phase1Libp2pTest.md` for the browser→Rust PTY path.
- Each phase carries its own acceptance gate above; record at the matching
  checkpoint in `Checkpoints.md`.

## Inputs / dependencies

- `glade/dev-docs/GladeRustOrbitStrategy.md` (separate agent) — blocks `P01`.
- `GLP-0001` terminal transport spike — informs `P02`/`P04`.
- `glade/dev-docs/GladeTerminalSliceProposal.md` — the design this executes.
- `grip-lab/src/lab/terminalController.ts` — the consumer seam.

## Outputs

- Rust substrate core + simulated substrate + test suites (`glade`).
- Glade surface + Python module (append-log tap, live-channel handle, exchange,
  presence).
- Provider terminal path (PTY, exchange, lease arbitration, presence).
- `grip-lab` real terminal producer + multi-participant UI + avatar UI.
- libp2p transport adapter + hub introduction/presence relay.
- Stable design promoted to `glade/dev-docs/`; decisions to root `DecisionLog`.
