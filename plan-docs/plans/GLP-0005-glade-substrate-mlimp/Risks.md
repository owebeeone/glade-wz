# Risks — GLP-0005

| # | Risk | Likelihood | Impact | Mitigation / fallback |
| --- | --- | --- | --- | --- |
| R1 | taut TS codegen less mature than Rust gen | med | med | corpus-first: hand-written TS codec acceptable if pinned by corpus (precedent: `trial/`) |
| R2 | cross-language fold divergence (JS number/map-order vs Rust) — breaks browser-folds premise | med | high (red) | fold conformance vectors in P0.S5 before any client work; reuse taut `crdt/engine.py`; deterministic CBOR only in payloads |
| R3 | `taut.gen.rust` output does not compile/link in a real cargo target | med | med | retire early in P0.S4 with a minimal cargo target; fall back to hand-maintained Rust codec pinned by corpus |
| R4 | GQ-9 `prev-hash` chain not expressible in taut op envelope as-is | med | med | P0.S2 explicit step to extend the envelope shape; small, contained to the IR |
| R5 | IndexedDB flakiness in the TS local destination | med | low | memory destination first (own step); IndexedDB is a separate step with a recorded yellow path |
| R6 | scope creep into security / keys / reassembler | med | med | non-goals list; security owned by `GladeGrythSecurityModelAnalysisPrompt.md`; keyed bindings + reassembler explicitly post-LIMP |
| R7 | contract drift after P0 lock | low | high | wire changes only via `Decisions.md` entries; corpus is the arbiter |
| R8 | multi-repo submodule pin/merge friction (root + glade + grip-core) | med | med | root pins submodule commits at each phase tag; WS-E owns nav files; branch submodules only when written |
| R9 | roll-build forces a misleadingly partial checkpoint | low | med | per-method: tag only when phase goal met + verification passes; pause and report instead of forcing |

## Red lines (stop and report)

- fold parity cannot be pinned cross-language (R2 realized) → browser-folds
  premise fails; escalate before continuing P2.
- cycling on the same bug family → stop, report the cycle, ask for direction.
