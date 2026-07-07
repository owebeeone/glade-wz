# State

Plan: `GLP-0002`
Status: draft
Current phase: `P00` (contract lock — pending `GladeRustOrbitStrategy.md`)
Branch/checkouts: root checkout on `main`; per-workstream branches TBD

## Next Checkpoint

`P00` contract lock: core API surface (append-log / live-channel / exchange /
Substrate adapter / simulated substrate) + identity model agreed and written,
once the Rust-Orbit strategy doc is accepted.

**Draft contract exists:** `Support/CoreApiContract.md` — Substrate adapter +
transport-assumptions table, the three surfaces with the read/write asymmetry
enforced, Presence sketched for `P05`, and simulated-substrate requirements.
Ready to lock once `GladeRustOrbitStrategy.md` lands; promotes to
`glade/dev-docs/` at lock. Open items listed in its §10.

## Blockers

- `P01` blocked on `glade/dev-docs/GladeRustOrbitStrategy.md` (separate agent).
- Provider host module to be confirmed (`glial-py` vs `glial-server`).

## Notes

- Stress driver for this plan is **multiple terminal participants**; `P03` is the
  current-functionality gate.
- Typing avatar (`P05`) is the next push, gated on `P04`.
