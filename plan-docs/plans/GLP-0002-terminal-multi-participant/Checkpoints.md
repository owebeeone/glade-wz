# Checkpoints

Plan: `GLP-0002`

Use checkpoint tags only at the integration gates below. For normal commits use
trailers:

```text
Plan: GLP-0002
Phase: P03
Checkpoint: multi-participant-green
```

| Checkpoint | Phase | Gate | Verification | Status |
| --- | --- | --- | --- | --- |
| `contract-lock` | `P00` | core API + sim-substrate interface + identity model agreed | strategy doc accepted; interfaces reviewed | pending |
| `sim-core-green` | `P01` | single-writer convergence + lazy window + reattach in sim | property + scenario suites pass | pending |
| `single-participant` | `P02` | real shell driven from browser via the seam | manual + scripted drive; mock→real swap | pending |
| `multi-participant-green` | `P03` | **current-functionality gate** — fan-out + reattach + single-driver lease under adversarial sim | full sim scenario suite at `N=2..1000` + churn/partition | pending |
| `over-the-wire` | `P04` | `P03` reproduced over libp2p; Gate C passes | conformance re-run; `Phase1Libp2pTest` Gate C | pending |
| `typing-avatar` | `P05` | who's-here / who's-typing / who's-driving accurate under churn | sim then real-transport presence tests | pending |

## Rollback notes

- Each checkpoint is an integration tag; revert is to the prior tag.
- `P04` regressions fall back to the sim substrate (WS-A) to localise whether a
  failure is core logic vs transport.
- If `P03` lease semantics prove unstable, fall back to watch-only multi-
  participant (no remote driving) to keep current-functionality shippable while
  arbitration is reworked.
