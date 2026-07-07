# Glade Developer Golden Path

Status: working draft

Purpose: define the first product-grade developer journey that Glade, Glial,
Grip Share, and GripLab must make real before the architecture spends more
time on broad language refinement.

## Core Claim

GripLab is the golden path.

The first implementation should prove that a developer can start with a fast
Grip UI mock, declare real Glade-backed surfaces, generate client and provider
bindings, run a local provider, inspect provisioning, and later move the
provider without rewriting the UI.

This is the V0 acceptance path for the whole stack.

## Product Story

The developer story is:

```text
mock in browser -> declare surfaces -> generate bindings -> run local provider
-> swap mock tap to Glade-backed tap -> inspect in console -> move provider
```

If this path is not smooth, the larger deployment and composition story does
not matter yet.

## Golden Path Scenario

GripLab SHOULD exercise:
- browser UI with Grip taps
- mock-first tap behavior
- provider-backed workspace tree
- provider-backed file windows
- provider-backed command runs
- interactive terminal live channel
- append logs and replay cursors
- sparse file windows and invalidation
- dynamic resource creation such as terminal sessions
- chat or presence as a lower-risk stream/log surface
- control-plane console visibility
- AI agent attachment as a headless session or backend provider

GripLab is deliberately not a toy. It is small enough to build, but broad
enough to pressure the real boundaries.

## Required Developer Loop

The golden path MUST support this loop:

1. Build a working Grip UI using local mock taps.
2. Add a `.glade` package that declares one surface.
3. Generate TypeScript client bindings and Python or JS provider stubs.
4. Implement the provider handler using generated definitions.
5. Run a local Glade dev server with in-memory or file-backed records.
6. Switch one Grip tap from mock mode to Glade-backed mode.
7. See the provider claim, route, request/log/live state, and diagnostics in
   the control-plane console.
8. Move the provider process without rewriting the UI.

The first surface SHOULD be narrow enough to finish, but real enough to prove
the model.

## First Vertical Slice

The first useful vertical slice SHOULD be terminal-oriented:

```text
package -> application -> facet terminal -> provider -> service
exchange OpenTerminal -> live_channel TerminalPty -> log TerminalOutput
Grip terminal tap -> generated TS client -> generated provider stub
control-plane console -> provider claim, route, live channel, output log
```

This slice exercises:
- declaration parsing
- generated bindings
- provider registration
- dynamic resource creation
- live stream transport
- append log sidecar
- Grip tap mapping
- console projection

It avoids solving every file/cache problem first.

## Second Vertical Slice

The next slice SHOULD be file-oriented:

```text
workspace tree -> file window -> stale cached window -> invalidation
-> refresh -> authoritative patch
```

This slice exercises:
- sparse content
- cache policy
- invalidation
- stale display
- patch submission
- source authority

It should start with `authoritative-patch`, not CRDT.

## AI Integration In The Golden Path

AI SHOULD be tested through GripLab, not through a separate demo.

Two integration tracks are useful:

### Backend Agent Provider

An AI provider can claim exchanges such as:
- `ExplainTerminalError`
- `SummarizeRunLog`
- `SuggestFilePatch`
- `EditSelectedText`

This tests provider claims, exchanges, logs, file references, delegated
capabilities, and diagnostics.

### Local Agent Participant

A local agent, including a tool like Codex, can attach to a local Glade
instance as a headless session.

It receives delegated capabilities to:
- read selected file anchors
- read terminal/log output
- propose file patches
- run explicit exchanges

This tests the deeper claim that AI works inside the live environment instead
of through copied prompt context.

## Acceptance Criteria

The golden path is credible when:
- a mock GripLab terminal works before a provider exists
- a `.glade` declaration generates usable client and provider code
- the same UI can use the generated client without structural rewrite
- a provider can run locally and publish a leased claim
- a terminal session can be created on demand
- terminal output is visible live and replayable from a log cursor
- the control-plane console shows the relevant records and routes
- a provider restart produces understandable lease/claim/route behavior
- an AI agent can read a delegated reference and propose or publish an action
  through declared Glade surfaces

## Non-Goals For The Golden Path

The golden path does not need:
- full multi-tenant hosted admission
- production-grade scheduler partitioning
- CRDT collaborative editing
- app marketplace distribution
- cross-application enterprise identity mapping
- global package namespace validation
- perfect p2p relay behavior under every network condition

Those need plausible boundaries now, but they should not block the first
developer loop.

## Boundary Discipline

Every implementation decision in the golden path should answer:
- Is this Glade, Glial, Grip Share, or Grip/Grok?
- Is this desired state, observed state, decision state, or projection?
- Is this source of truth or derived cache?
- Is this local-dev convenience or production contract?
- Is this app semantics, environment policy, provisioning, routing, transport,
  or UI mapping?

If the answer is unclear, the feature is probably crossing a boundary.

## Open Issues

This document does not yet define:
- exact generated binding API
- exact Grip Share tap adapter shape
- local dev server process layout
- first console projection schema
- first AI delegated reference envelope
- provider restart behavior in the first implementation
