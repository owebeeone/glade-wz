# GLP-0001: GripLab Terminal Transport Spike

Status: active
Owner: current agent
Affected modules: root planning docs, `grip-lab` Phase 1 Node sidecar, future
Glade transport

## Goal

Decide whether a disposable libp2p-based terminal path is viable enough to
justify deeper Glade/GripLab integration.

The immediate Phase 1 track is p2p communications only: a Node.js libp2p +
Helia + OrbitDB sidecar in `grip-lab`. The current GripLab backend MUST remain
in place while this substrate risk is tested.

This plan tests two hard bits only:

- low-latency live terminal input/output over libp2p streams
- terminal output replay through an OrbitDB-backed append-log-shaped history

This is a throw-away spike. Its output is a decision and a few contract
fixtures, not production architecture.

## Success Threshold

The threshold is intentionally low.

Green means:

- a Node sidecar can boot libp2p, Helia, and OrbitDB with pinned dependencies
- two local sidecar peers can connect and make terminal output records visible
- browser can send input frames to a terminal provider over libp2p
- terminal output returns with acceptable interactive latency on localhost/LAN
- burst output does not permanently starve input
- browser refresh can resume from a cursor or show an honest replay diagnostic
- failure is visible, not silent

Yellow means:

- libp2p works but requires a gateway, bridge, or priority-stream change
- terminal replay works only with explicit limitations
- latency is acceptable locally but not proven across harder topologies

Red means:

- browser/provider libp2p path is too fragile for the GripLab terminal
- terminal input latency is unacceptable under ordinary output load
- replay/cursor behavior cannot be made understandable

## Non-Goals

- production Glade records
- real capability enforcement
- package signing
- generated `.glade` compiler output
- full GripLab UI migration
- PTY migration/failover
- CRDT editing
- multi-writer terminal collaboration policy
- production p2p mesh operations
- dynamic provider placement

## Phases

| Phase | Goal | Acceptance |
| --- | --- | --- |
| `P01` | Define disposable terminal frames and replay cursor. | `Support/TerminalSliceContract.md` has frame shapes, lifecycle, diagnostics, and V0 non-goals. |
| `P02` | Pick the shortest implementation track. | Decision recorded: JS/browser-to-node first, browser-to-Rust first, or gateway fallback first. |
| `P03` | Prove Node libp2p + OrbitDB substrate boot and append/read. | Sidecar starts libp2p, Helia, and OrbitDB, appends a Glade-shaped terminal output frame, reads it back, and records dependency/audit friction. |
| `P04` | Prove two-peer append-log visibility. | Two local sidecars connect, append to the same OrbitDB events database, and record convergence timing and ACL requirements. |
| `P05` | Prove live terminal IO over libp2p. | Input, output, resize, close, and diagnostics frames move through libp2p in a local test without replacing the current GripLab backend. |
| `P06` | Run burst/slow-reader check. | Input latency remains acceptable or the plan records the need for separate priority streams. |
| `P07` | Decide green/yellow/red. | `Handoff.md` states what worked, what failed, and what the next plan should do. |

## First Implementation Bias

Start with the smallest path that can prove the terminal mechanics.

Chosen immediate path:

```text
current GripLab backend remains in place
  + Node.js libp2p/Helia/OrbitDB sidecar
  + Glade-shaped terminal output records
  + two-peer append/read/visibility measurement
```

Reason:
Node.js OrbitDB gives faster evidence on the p2p communications backbone,
OrbitDB write access, dependency footprint, and replay-log behavior. Rust
provider work remains important but is no longer the first checkpoint.

Later tracks:

- browser js-libp2p to Node.js sidecar
- browser js-libp2p to Rust provider
- browser to gateway fallback using Glade-shaped frames

If Node/OrbitDB setup becomes the blocker, record that as a spike result rather
than expanding the spike indefinitely.

## Verification

Minimum checks:

- one-shot Node sidecar append/read
- two-peer sidecar connect/open/append/visibility
- OrbitDB writer access behavior
- keystroke echo latency under normal output
- keystroke echo latency during burst output
- resize event delivery
- browser refresh and reattach
- provider crash diagnostic
- slow-reader behavior

No formal test suite is required before the spike starts. If code is written,
add only lightweight harness checks that directly support the green/yellow/red
decision.
