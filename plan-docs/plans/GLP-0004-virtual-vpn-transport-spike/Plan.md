# GLP-0004: Virtual-VPN Transport Spike (iroh-as-tunnel)

Status: proposed
Owner: maintainer (coordinating)
Affected modules: `griplab_core` (native agent/forwarder), `grip-lab` (LCS WSS
protocol — forwarded unchanged), hub (rendezvous + relay), root `plan-docs`
Date: 2026-06-05

Supersedes the transport *direction* explored in
`dev-docs/Phase1Libp2pTest.md` and `GLP-0001` (iroh/libp2p as a **data layer**).
This spike uses iroh as a **connectivity layer only**. See `Decisions.md` for the
full reasoning arc.

## Core Claim

This spike should answer one question:

```text
Can iroh act as a pure connectivity tunnel — NAT traversal + relay + identity —
with GripLab's existing WSS protocol forwarded on top, unchanged, so that
distributed work-from-home nodes collaborate without iroh in the browser and
without iroh-docs?
```

It should NOT answer every production topology question (provisioning, A/V
huddle, member/identity model). Those are separate.

## Why (one paragraph)

We spent significant effort treating iroh-docs as the data layer for terminal
output and kept bolting on adapters (seq-keys, an out-of-order assembler, an
ephemeral no-cache store, blob-per-chunk). Separately, the browser kept being
the binding constraint (no UDP/QUIC, no threads, no fs, ~1.6 MB wasm). The
synthesis: use iroh **only** for what it is genuinely best at — getting bytes
between two boxes behind home NATs — and run the **existing** WSS LCS protocol as
plain bytes over that pipe. This is the Tailscale shape (overlay for
connectivity; apps run plain on top), at the smaller "`ssh -L` over iroh"
granularity — not a full tun/netstack VPN.

## Target Persona (driver)

**The Builder** — SWE, but also PM / program manager / UX who must *run* a branch,
not just read it. Defining constraint: **does not own the compute** — no beefy
workstation, no mac/linux/windows fleet. Works on a **provisioned/borrowed box**,
often a **home laptop behind residential NAT** (no inbound reachability).
Jobs: (1) get build-ready fast, (2) **reach what they run** (forwarded test
ports), (3) test where they can't reach themselves (a peer's box), (4) **huddle**.

Two persona facts pin the design:
- Distributed home → every node behind NAT, **no inbound**. Connectivity must be
  **outbound-initiated** to a publicly-reachable hub. (This breaks the current
  "hub SSH *into* collaborator" bootstrap — see `Decisions.md` / `Risks.md`.)
- Bounded huddle → discovery is a **roster** (the hub), not open P2P. mDNS is
  LAN-only (out); the DHT is the known dead end. iroh is used for **traversal**,
  not discovery.

## Architecture Under Test

| Layer | Tech | Job |
| --- | --- | --- |
| browser ↔ local LCS | `ws://localhost` | trivial; **no P2P, no wasm** |
| LCS ↔ LCS / LCS ↔ hub | **iroh bi-streams** via the **v2pn loadable module** (pyo3) | NAT traversal, relay, identity — *connectivity only* |
| on top of the stream | **existing WSS LCS protocol** | terminal, diff, presence, test ports — **unchanged** |
| hub | rendezvous + relay + roster | discovery (roster) + NAT fallback + HA |

The v2pn is a **loadable native module** the LCS loads (reusing the
`griplab_core_py` pyo3/artifact machinery), repurposed from iroh-docs producer to
tunnel — **not** a standalone daemon (see `Decisions.md` D-06). Each participant
runs an LCS (client/build/hub roles already exist); the browser hits its *local*
LCS.

Key property: tunneling TCP/WSS over a **QUIC** stream avoids the
TCP-over-TCP meltdown that SSH tunnels suffer, and works outbound through home
NATs where inbound SSH cannot.

## Non-Goals

- Full overlay VPN: tun device, virtual IPs, userspace netstack. We do
  **per-connection forwarding** only (`ssh -L` over an iroh stream / dumbpipe).
- **Standalone daemon / separate process** for the v2pn. Phase 1 keeps it a
  **loadable module** in the LCS (D-06). Daemonization + LCS-in-Rust are deferred
  productionization choices, not spike scope.
- iroh-docs / any CRDT data layer. The data layer is the existing WSS protocol.
- iroh (or any P2P stack) **in the browser**.
- Provisioning / bootstrap redesign (separate; this spike assumes an agent can
  be started on each box).
- A/V huddle (WebRTC) — a different layer, deferred.
- Production identity/authz model (Member vs Node vs Session) — design separately.

## Phases

| Phase | Goal | Acceptance |
| --- | --- | --- |
| `P01` | **Raw pipe.** Forward one TCP connection between two native agents over an iroh bi-stream (dumbpipe-equivalent), same machine, default relay. | An existing WSS LCS session runs **unchanged** over the pipe (terminal streams, ordered, no app changes). |
| `P02` | **Traversal + relay.** Two nodes on different networks (or simulated NAT); hub as rendezvous + relay; hole-punch with relay fallback. | Connection establishes through NAT; measure connect time + throughput on direct vs relay path; relay fallback works when hole-punch fails. |
| `P03` | **Browser path.** browser → `ws://localhost` (agent) → tunnel → remote LCS. | The existing terminal/diff UI works unchanged; **zero** iroh in the browser bundle. |
| `P04` | **Test-port forwarding.** Forward an arbitrary remote dev-server port to the laptop over the same mechanism. | Persona job: hit a remote test server from the local browser via a localhost port. |
| `P05` (opt) | **Resumable log protocol** over the stream (cache/resume as an *app-protocol* concern, not iroh-docs). | Client resumes from offset N on reconnect; no re-pull, no dup. May split to its own plan. |

## Verification

- P01 is the gate: if the existing WSS protocol does **not** survive forwarding
  unchanged, the premise fails — stop and reassess.
- Measure and record (in `Checkpoints.md`): connect time (direct vs relay),
  relay-path throughput for the heaviest intended payload, behaviour under
  network change (laptop wifi→cellular).
- The substrate-agnostic logstream semantics test (ordered assembly,
  resume-from-offset, no-dup-on-reconnect) is the durable contract; it should
  pass over this transport without transport-specific code.

## Open Questions (decide before/within the spike)

1. Heaviest forwarded payload — thin collab signal, or whole-repo/build-artifact
   /A-V? Sizes the hub-relay and decides whether **direct** paths matter at all.
2. In-browser persistent store feasibility is now **moot** for the logstream
   (cache lives in the LCS, served over WSS) — confirm.
3. Where does the agent run for a remote build box — on the laptop (forwards to
   remote LCS) and/or co-resident with the LCS? (Affects the P03 topology.)
