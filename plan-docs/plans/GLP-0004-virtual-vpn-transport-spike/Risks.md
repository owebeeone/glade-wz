# Risks — GLP-0004

Date: 2026-06-05

| # | Risk | Likelihood | Impact | Fallback / mitigation |
| --- | --- | --- | --- | --- |
| R1 | **Relay-fallback bandwidth.** When hole-punch fails (symmetric NATs), all traffic routes through the hub-relay. Heavy payloads (repo/artifact/A-V) could saturate it. | medium | medium | Size the hub-relay to the heaviest forwarded payload (Open Question #1). Keep big transfers off the tunnel, or require direct-path before allowing them. |
| R2 | **Laptop agent install.** Each participant runs a small native agent. Dents the "zero-install, just a browser" ideal. | high | low | Agent is small + native (no wasm); it also delivers forwarded test ports, so it earns its place. Far lighter than shipping iroh into the browser. |
| R3 | **Bootstrap direction is wrong for home NAT.** Current `ssh_bootstrap` has the hub SSH *into* the box (needs inbound). Home laptops deny inbound. | ~~high~~ resolved-in-design | ~~high~~ low | **Resolved by the v2pn (D-07):** iroh dials *outbound* to the relay, so the per-peer SSH `-R`/`-L` tunnels and the inbound assumption are **gone** entirely. Any node↔any node via direct/relay. Residual: SSH may still do initial *provisioning* (separate from connectivity). |
| R4 | **Network-change resilience.** iroh re-homes on relay automatically, but the forwarded TCP/WSS session may still drop on wifi→cellular. | medium | medium | App-level reconnect + the resumable log protocol (P05) so a re-dial resumes from offset, not from zero. |
| R5 | **TCP-over-QUIC assumptions.** Premise is that forwarding the WSS connection over an iroh bi-stream is transparent. | low | high | P01 is the explicit gate — if the existing protocol does not survive forwarding unchanged, stop and reassess before any further phase. |
| R6 | **Scope creep into a real VPN.** Temptation to add tun device / virtual IPs / netstack. | medium | medium | Hard non-goal (D-03). Per-connection forwarding only. If a real overlay is ever needed, that is a separate plan. |
| R7 | **Editing data layer.** Concurrent text editing may still need a CRDT (GLP-0003), which this transport does not provide. | medium | low | Out of scope here; flagged for maintainer to reconcile. Terminal/log/diff/presence do not need a CRDT. |

## Premise-breakers (stop and reassess if true)

- The existing WSS protocol cannot be forwarded transparently (R5).
- Relay-fallback cannot carry the required payload and hole-punch is unreliable
  for the target NATs (R1).
- The native agent is unacceptable for the audience (R2) — would force browser
  transport reconsideration (WSS-direct-to-hub instead of via local agent).
