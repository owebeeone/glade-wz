# Risks — GLP-0006

- **Stage-2 arrives mid-plan (P2).** Share/users suppliers force WD-1 +
  AZ-1/2/3 from parked to blocking. Mitigation: only the enforcement/root
  semantics checkpoint is gated; P3.S1 (`swmr` adapter + file-window projection)
  and `PlanGladeUsers.md` Phases 0–4 can proceed in parallel.
- **Catalogue recognition mistaken for runtime support.** Portable `swmr` and
  `text_crdt` engines are released, but Glade does not yet expose their binding
  paths. Mitigation: GSC-07 requires exact adapter pins and node/client/Glial
  positive and fail-closed gates before declarations are accepted.
- **`window` reintroduced as a shape by old D8 wording.** That would recreate
  the category error the catalogue closed. Mitigation: preserve D8's viewport,
  backfill, and generation guarantees as an application view over explicit
  `swmr`; keep range/retention policy separate from delivery identity.
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
