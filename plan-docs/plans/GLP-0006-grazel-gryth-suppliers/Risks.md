# Risks — GLP-0006

- **Stage-2 arrives mid-plan (P2).** Share/users suppliers force WD-1 +
  AZ-1/2/3 from parked to blocking. Mitigation: P2-gate is explicit; P3.S1
  (window contract) can proceed in parallel if rulings stall.
- **Blob handling defaulting into op-chains.** A 2GB file must never become
  chain ops. Mitigation: P3-gate ruling BEFORE glade-files; the supplier kit
  gives no convenient wrong path.
- **Scope gravity from grip-lab.** The prototype had many behaviors; porting
  instead of re-deriving from traces would smuggle imperative design back in.
  Mitigation: grip-lab is reference-only; every supplier starts at its trace.
- **Channel semantics are new wire behavior** (frames exist but echo).
  Terminal is the first consumer; keep channels generic (bytes + control),
  WINCH strictly a control message, or terminal-isms leak into the wire.
- **Vocabulary drift**: "supplier" vs the code's legacy `provider` naming.
  Renames ride real changes (no cosmetic sweeps), but new code never says
  provider.
- **taut `--legacy-codec` deadline** (dies at taut v0.10) intersects any
  sysdata regen this plan needs — migrate before it bites (rider).
- **Cross-workspace coupling**: gryth-wz (UI) and glade-wz move together
  from P1.S4; pins must travel in both locks or fresh clones skew.
