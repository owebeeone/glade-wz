# Workstreams — GLP-0004

Date: 2026-06-05
Status: proposed (single coordinating agent until `accepted`)

| Workstream | Owner | Write scope | Outputs | Merge risk |
| --- | --- | --- | --- | --- |
| v2pn module / iroh tunnel | TBD | `griplab_core` (loadable pyo3 module: iroh endpoint + bi-stream forwarder) | dumbpipe-style per-connection forward; outbound dial to hub-rendezvous; loaded by the LCS | low (repurposes existing module) |
| Hub rendezvous + relay | TBD | hub service (roster → node-ids/tickets; host iroh relay) | ticket handout; relay endpoint | medium (touches hub) |
| LCS protocol (forwarded) | TBD | `grip-lab` LCS WSS — **read-mostly**; no protocol change in P01–P04 | proof the protocol survives forwarding | low (unchanged) |
| Logstream resume protocol (P05) | TBD | `grip-lab` LCS protocol + agent | offset-resume; substrate-agnostic semantics test | medium (protocol change) |
| Coordination / nav | maintainer | root `plan-docs` (`Registry.md`, `ActiveWork.md`, this plan) | index + board | low |

## Ownership boundaries

- Single coordinating agent while `proposed`. Split into the workstreams above
  only on `accepted`/`active`.
- Do not edit `ssh_bootstrap.py` under this plan — the outbound-bootstrap
  redesign (Risk R3) is a separate plan; this spike assumes an agent can be
  started on each box by some means.
- Shared root nav files (`Registry.md`, `ActiveWork.md`) edited by the
  coordinating agent only.

## Dependencies

- Relay perf prototype (env-gated `GRIPLAB_IROH_RELAY` + local `iroh-relay`)
  already exists from the GLP-0001 arc and is reusable for P02.
- Temp debug instrumentation + `.env.development.local` from that arc should be
  cleaned up before this plan goes `active` (not part of this transport change).
