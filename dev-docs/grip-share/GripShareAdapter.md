# Grip Share Adapter

Status: working draft

Purpose: define the layer that maps Grip local execution and persistence
mechanics onto Glade share mechanics.

## Core Claim

Grip integration is an adapter, not the definition of the distributed model.

The adapter exists so that:
- Grip can provide a rapid local development experience
- Grip structures can participate in shared state
- Glade can remain stable even if Grip internals evolve

## Adapter Responsibilities

The Grip Share layer owns:
- mapping Grip-local value identity onto Glade projection identity
- mapping Grip-local persistence/share events onto Glade share mutations
- mapping Grip execution roles onto Glade ownership expectations
- advertising which Grip surfaces are safe to share, control, observe, or keep
  private
- supplying Grip-friendly local-first behavior for mocks and single-process dev
- preserving enough metadata for efficient Grip follower materialization

The Grip Share layer does not own:
- the semantic meaning of shares
- the semantic meaning of ownership terms
- app/session orchestration policy
- transport topology

## Mapping Boundaries

### Local Grip concepts

Grip local execution remains responsible for:
- context resolution
- grip lookup
- tap execution
- drip propagation
- local reactivity

### Glade concepts

Glade remains responsible for:
- share identity
- interest declaration
- projection envelope
- ownership term semantics
- takeover and rejoin invariants

### Adapter seam

The adapter is the seam where these are translated.

Current working mapping directions:

| Grip-side concept | Glade-side concept |
| --- | --- |
| context `path` plus grip key | projection address within a share |
| local shared-source intent | `Interest Spec` mutation |
| primary-oriented tap execution | current Glade ownership role |
| persisted local change | share mutation candidate |
| stable selection or highlight anchor | delegated capability reference |

The advertisement contract for these mappings is defined in
`/Users/owebeeone/limbo/glade-wz/dev-docs/grip-share/GripShareAdvertisement.md`.

## Adapter Rules

- The adapter MUST treat Glade as the source of distributed semantics.
- The adapter MUST NOT require Glade to expose raw Grip tap internals as part
  of the stable kernel contract.
- The adapter MAY use Grip-specific metadata as an optimization for local
  follower materialization.
- Grip-specific optimization metadata MUST remain optional from Glade's point of
  view.
- The adapter SHOULD allow Grip to evolve internal tap and projection mechanics
  without forcing a Glade redesign.

## Rapid Development Behavior

The adapter is a key part of the rapid development story.

It SHOULD support:
- local-only execution with no mandatory hosted server
- in-memory single-process share behavior
- fake or stubbed source bindings
- later promotion to real shared or source-backed behavior without rewriting
  the UI structure

This is important because Grip is already good at rapidly producing useful UI
mock behavior. The adapter must preserve that speed rather than forcing the app
author to solve the full distributed problem before the UI is useful.

## Report Editor Example

For a report editor:

- Grip local runtime may own local selection, hover state, and panel state.
- The adapter exports the report document binding into a Glade `Interest Spec`.
- The adapter exports selected stable anchors only when they are intentionally
  shared or delegated.
- The resulting Glade projection is mapped back into Grip-observable values.

This means:
- a working local mock can exist before source-backed sharing exists
- source-backed sharing can be added by teaching the adapter how to map the
  relevant source binding and projection
- Glade does not need to understand Grip widgets or UI composition

## Current Risk

The major architectural risk is letting the adapter become the kernel by
accident.

That would happen if:
- Glade's public contract becomes whatever Grip currently serializes
- app architecture starts reasoning in tap-shaped distributed concepts
- other runtimes can only integrate by pretending to be Grip

The adapter should remain fluid. Glade should remain stable.
