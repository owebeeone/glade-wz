# Workstreams

Plan: `GLP-0001`

| Workstream | Owner | Write Scope | Must Not Touch | Output | Merge Risk |
| --- | --- | --- | --- | --- | --- |
| Spike coordination | current agent | `plan-docs/plans/GLP-0001-griplab-terminal-transport-spike/`, `plan-docs/Registry.md`, `plan-docs/ActiveWork.md` | unrelated docs, submodule code | plan, state, checkpoints, handoff | low |
| Terminal contract | current agent initially | `Support/TerminalSliceContract.md` | stable module `dev-docs` until contract settles | frame/lifecycle/replay shape | medium if over-scoped |
| Node sidecar implementation | current agent | `grip-lab/services/phase1-orbit/`, `grip-lab/scripts/phase1-orbit-*.test.mjs`, `grip-lab/package.json`, `grip-lab/package-lock.json` | current GripLab backend replacement, UI migration | libp2p + OrbitDB p2p friction harness | medium due dependency graph |
| Transport implementation | unassigned | future spike code path TBD | root navigation docs unless assigned | libp2p terminal harness | medium |
| Replay/log harness | current agent initially | `grip-lab/services/phase1-orbit/` | production storage architecture | OrbitDB events replay/visibility test | medium |
| Decision/handoff | coordinating agent | `Handoff.md`, `Checkpoints.md`, `Decisions.md` | implementation internals unless assigned | green/yellow/red decision | low |

## Agent Boundary

Only one agent should edit `Support/TerminalSliceContract.md` at a time.

Implementation agents should treat the contract as a disposable fixture, not as
final Glade architecture.
