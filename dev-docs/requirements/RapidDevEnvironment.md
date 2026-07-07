# Rapid Development Environment

Status: working draft

Purpose: define the architectural requirements needed to preserve rapid product
development across the G* stack.

## Framing

Rapid development is a hard requirement, not a convenience feature.

Grip already enables fast UI iteration. The wider G* stack MUST preserve that
speed when adding sharing, source-backed collaboration, and agent participation.

The architecture MUST allow a useful app mock to exist before the full
distributed or source-backed system is complete.

## Development Workflow Target

The intended workflow is:

1. build a useful local UI or agent-facing interaction quickly
2. model local state and behavior in Grip
3. run the share layer locally with no mandatory remote server or mesh
4. add mapping for the subset of state that becomes shared
5. later connect real external sources and multi-session behavior

This workflow reduces the risk that product progress stalls on distributed
plumbing too early.

## Requirement Set

## Local-First Execution

- `RDE-001` The stack MUST support single-process, in-memory execution of the
  share substrate.
- `RDE-002` A developer MUST be able to run a useful app locally without first
  deploying a mesh or hosted server.
- `RDE-003` Local development mode MUST preserve the same core share metaphor
  used by the larger distributed system.

## Mock-First Promotion Path

- `RDE-010` A useful UI mock MUST be able to exist before real source-backed
  sharing is complete.
- `RDE-011` The path from mock to shared behavior SHOULD prefer adapter changes
  over UI rewrites.
- `RDE-012` Fake or stubbed source bindings MUST be usable in local development
  mode.
- `RDE-013` Promotion from local mock to multi-session behavior MUST preserve
  stable share identity and observable app semantics as much as possible.

## Grip Integration Expectations

- `RDE-020` Grip MUST be usable as a fast local runtime without forcing early
  commitment to full distributed mechanics.
- `RDE-021` Grip integration with the share substrate MUST be adapter-based.
- `RDE-022` Grip-specific persistence or projection optimizations MUST remain
  optional relative to the stable share substrate contract.

## AI-Assisted Development

- `RDE-030` The architecture SHOULD support rapid AI-assisted creation of
  high-value mocks that become real by adding mapping and source integrations.
- `RDE-031` An AI-generated mock SHOULD NOT require the architecture to settle
  the full communications topology before the mock is useful.
- `RDE-032` Stable references such as anchors, ids, and delegated handles
  SHOULD be available early so that AI-assisted workflows can promote from mock
  to source-backed behavior cleanly.

## Operational Simplicity

- `RDE-040` Developers SHOULD be able to inspect effective interest, current
  ownership, and current projection state locally.
- `RDE-041` The local development environment SHOULD make it obvious which
  state is private, shared, delegated, or derived from an external source.
- `RDE-042` Development-time sharing MUST NOT require the developer to solve the
  entire production hosting topology first.

## Architectural Implication

The stack should therefore evolve in this order:

1. stable share metaphor in `Glade`
2. fast local mapping in `Grip Share`
3. thin app/session orchestration in `Glial`
4. broader distributed hosting and routing only after the simpler model is
   mechanically sound
