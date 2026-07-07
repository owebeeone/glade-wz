# State

Plan: `GLP-0004`
Status: proposed
Current phase: `P01` (not started)
Branch/checkouts: root checkout on `main`; future branch
`codex/glp-0004-virtual-vpn-spike` TBD
Date: 2026-06-05

## Next Checkpoint

`P01` raw-pipe: forward one existing WSS LCS session over an iroh bi-stream,
unchanged, on one machine. This is the premise gate.

## Blockers

None to start. Decision pending from maintainer:
- Open Question #1 (heaviest forwarded payload) — sizes relay + decides whether
  direct paths matter.
- Whether to proceed to `accepted`/`active`, or hold as `proposed`.

## Notes

Born from the GLP-0001 / iroh-as-data-layer debugging arc. See `Decisions.md`.
The grip-core `delayedUpdates` re-key fix and the wasm seq-order assembler landed
during that arc are independent and remain valid regardless of which transport
direction wins.

### Transport evidence (measured — `grip-lab/scratch/iroh-tcp-test/`)

A throwaway iroh TCP-over-QUIC harness was run across mac / weftpi (Linux) /
magenta (Windows) on a real LAN. Verified measurements:

- **Relay connect:** n0 public relay ~4.5 s → local relay **~0.2 s**.
- **Direct UDP + LAN-local internal routing:** reliable across all pairs
  (cross-subnet *and* same-subnet), cross-platform, once the path settles. (A
  one-shot snapshot at +1.5 s mis-reported a hairpin; a path-watch over 20 s
  showed LAN-local selected 6/6.)
- **Latency:** iroh adds no tax — tracks the link; **p50 ~3.6 ms, p99 ~7 ms** on
  a clean 5 GHz LAN.
- **Throughput is link/radio-bound, not iroh:** iroh ≈ pure TCP in every test and
  direction (often slightly faster). Up to **28.6 MB/s** on a good link. The low
  1.9–3 MB/s figures were a **misconfigured 2.4 GHz band + the Pi's weak WiFi RX
  (16 MB/s TX vs 3 MB/s RX)** — not the transport.

Net: the v2pn premise (P01/P02 — forward a stream over iroh, direct/relay,
LAN-local) holds; iroh is a no-overhead substrate. See the scratch `RESULTS.md`.
