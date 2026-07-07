# G* Development Plan

Status: proposed plan

Purpose: define a practical step-by-step order for turning the current
Glade/Glial/Grip architecture into working software.

This is not a full implementation spec. It is the proposed build sequence that
keeps the first implementation small while preserving the larger architecture
boundaries.

## Core Claim

The first implementation should prove the GripLab golden path, not the whole
platform.

The build should move from:

```text
transport spike -> declaration review -> generated shapes -> canonical exchange
-> Grip/Glial/Glade mapping -> PTY terminal slice -> console visibility
```

Every shortcut is acceptable if it preserves the future boundary. No shortcut
is acceptable if it creates hidden state that cannot later be represented as a
Glade declaration, claim, lease, route, log, or projection.

## Phase 0: Checkpoint And Reduce Scope

Goal: freeze the current architecture direction and choose the first
implementation slice.

Steps:
1. Treat `/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/GladeDeveloperGoldenPath.md`
   as the first acceptance path.
2. Treat GripLab terminal as the first vertical slice.
3. Keep frankenapps, marketplace, production scheduling, and CRDT editing out
   of the first implementation.
4. Keep the implementation honest by preserving desired, observed, decision,
   and projection state boundaries.

Deliverable:
- a short terminal-slice implementation checklist before coding starts

Acceptance criteria:
- first slice is small enough to finish
- every planned shortcut is explicitly local-dev or prototype-only

## Phase 1: Small P2P Surface Demonstration

Goal: test the transport surfaces before binding them to the full Glade model.

Detailed specification:
`/Users/owebeeone/limbo/glade-wz/dev-docs/Phase1Libp2pTest.md`

Build a tiny libp2p demonstration app that proves:
- peer identity
- peer discovery or explicit peer dialing
- request-like exchange traffic
- append-log-like streaming
- live-channel-like bidirectional stream
- reconnect or simple failure diagnostics

This demo should not become the Glade implementation. It is a risk-reduction
spike for transport behavior.

Deliverables:
- minimal JS/TS p2p demo
- notes on what works cleanly
- notes on latency, stream behavior, reconnect, and browser constraints

Acceptance criteria:
- one peer can create a live bidirectional stream to another
- one peer can publish append-like output to another
- failure is observable rather than silent

## Phase 2: Review The `.glade` Example Files

Goal: make sure the current GripLab declaration sketch describes the real app
surface clearly enough to guide implementation.

Review:
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/GripLab.glade`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/griplab/workspace.glade`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/griplab/application.glade`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/griplab/provider.glade`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/griplab/scale.mode1.glade`
- `/Users/owebeeone/limbo/glade-wz/dev-docs/examples/griplab/env.local-dev.glade`

Questions to answer:
- Does each file have one clear role?
- Are aliases and canonical ids scoped clearly?
- Are terminal, log, file, exchange, and stream surfaces complete enough?
- Are local-dev environment assumptions separated from app semantics?
- Is legacy websocket mapping useful enough for migration?
- Which declarations are needed for the first terminal slice?

Deliverables:
- updated `.glade` examples if needed
- a first-slice declaration subset list

Acceptance criteria:
- the terminal slice can be derived from the declarations without guessing
- missing declaration concepts are named before implementation

## Phase 3: Define Declaration Outputs

Goal: define what `.glade` files compile into for Python and JavaScript or
TypeScript.

Do not build a full compiler first. Hand-shape the output that a compiler
should eventually generate.

Define:
- canonical declaration IR shape
- package hash placeholder
- definition ids and schema hashes
- TypeScript constants and client helpers
- Python provider definitions and stubs
- runtime validation hooks
- provider claim helpers

Required outputs for the first slice:
- `OpenTerminal` exchange definition
- `TerminalPty` live channel definition
- `TerminalOutput` append log definition
- `griplab_backend` provider definition
- terminal facet references

Deliverables:
- a proposed generated TypeScript module shape
- a proposed generated Python module shape
- one hand-written generated-output fixture

Acceptance criteria:
- TS and Python refer to the same ids and hashes
- provider code cannot accidentally serve an undeclared surface
- client code can call a generated helper without knowing low-level record
  details

## Phase 4: Canonical Endpoint / Bug Report Function

Goal: prove the bounded-work exchange API with something simpler than a PTY.

Create a canonical endpoint-like function, such as:

```text
SubmitBugReport
```

or:

```text
BugsQuery
```

This function should demonstrate the Glade exchange shape:
- request intent
- provider claim
- execution progress
- final publication
- requester observation
- diagnostics
- retention

This is the bridge for people who still think in request/response terms.

Deliverables:
- TypeScript caller example
- Python or JS provider example
- record shapes for request, attempt, progress, publication, observation, and
  diagnostics
- local dev runtime test path

Acceptance criteria:
- a developer can understand how an endpoint maps onto Glade records
- diagnostics are separate from provider result status
- retry uses a new attempt identity

## Phase 5: Flesh Out Grip -> Glial -> Glade Mapping

Goal: define the seam that lets a Grip mock tap become Glade-backed without a
UI rewrite.

Map:
- Grip tap identity to Glial facet/share interest
- Grip local state to Glade projection state
- mock tap data shape to generated Glade client shape
- delegated selections or anchors to Glial capability references
- side-effectful tap execution to Glade provider or owner role

This phase should produce one practical adapter seam, not a general theory of
all possible taps.

Target first seam:

```text
Grip terminal tap -> Glial terminal facet -> Glade OpenTerminal/TerminalPty
```

Deliverables:
- mock tap contract
- Glade-backed tap contract
- adapter boundary notes
- minimal migration checklist for one GripLab tap

Acceptance criteria:
- the UI consumes the same or nearly same tap shape before and after backing
  it with Glade
- provider/client details do not leak into UI components
- local mock mode remains possible

## Phase 6: Glade / Grip / PTY Terminal Interface

Goal: build the GripLab terminal golden-path slice.

Implement the first real terminal flow:

```text
OpenTerminal exchange -> TerminalPty live channel -> TerminalOutput append log
```

The flow should support:
- create terminal session
- attach UI session
- send input frames
- send resize frames
- receive output frames
- append output to replay log
- reconnect or reattach from log cursor
- publish provider claim and lease
- show diagnostics in the console

Initial implementation may use:
- local dev server
- one provider process
- in-memory records
- simple explicit peer connection
- hand-written generated bindings

It should not require:
- full distributed scheduler
- production p2p discovery
- CRDT editing
- hosted namespace admission

Deliverables:
- terminal provider stub
- TS client helper
- GripLab tap integration
- append log sidecar
- dev console projection

Acceptance criteria:
- terminal works through the Glade-shaped path
- output is visible live
- output can be replayed from a cursor
- provider restart creates visible lease/claim behavior
- control-plane console shows provider, route, live channel, log, and
  diagnostics

## Phase 7: Control-Plane Console V0

Goal: make provisioning and routing visible from the first real slice.

Console V0 should show:
- loaded package id
- active local node id
- active provider claim
- live terminal instance
- output log id
- route or local assignment
- lease expiry
- recent diagnostics

Operator actions can be minimal:
- refresh projections
- request reconcile
- inspect raw record

Deliverables:
- local projection service
- simple UI or JSON endpoint
- raw record inspection path

Acceptance criteria:
- no hidden provisioning state is required to understand the terminal slice
- every visible row can link back to a Glade-shaped record

## Phase 8: File Window And Cache Slice

Goal: prove declarative cache and sparse content behavior.

Build:
- workspace tree
- file window
- stale cached window
- invalidation
- refresh
- authoritative patch

This should follow the terminal slice, not precede it.

Deliverables:
- file content declaration subset
- TS file-window helper
- provider-side file-window implementation
- GripLab file tap integration

Acceptance criteria:
- UI can open a sparse file window without loading the whole file
- stale cached content can be shown intentionally
- invalidation and refresh are deterministic
- patch submission is explicitly authoritative-patch, not CRDT

## Phase 9: First AI Agent Flow

Goal: prove that AI participates through Glial/Glade rather than side-channel
prompt copying.

Start with one simple agent exchange:

```text
ExplainTerminalError
```

Then move to:

```text
SuggestFilePatch
```

The agent should receive delegated capabilities to:
- read a terminal/log cursor
- read a selected file anchor or file window
- publish a proposed explanation or patch

Deliverables:
- delegated reference shape
- agent exchange declaration
- provider or local-agent implementation
- audit/provenance record

Acceptance criteria:
- agent action is attributable
- agent access is narrower than the whole session
- user-visible output comes through declared Glade surfaces

## Phase 10: Replace Hand-Shaped Generation

Goal: turn the hand-written generated shapes into real generation.

Build only what the earlier slices prove is necessary.

Generate:
- TypeScript constants and helpers
- Python provider stubs or loaders
- schema references
- declaration hashes
- drift checks

Deliverables:
- minimal `.glade` parser or intermediate source parser
- generated TS fixture
- generated Python fixture
- conformance check against hand-shaped outputs

Acceptance criteria:
- generated outputs match the first-slice runtime expectations
- drift is detectable in CI
- humans can still review declaration source

## Phase 11: Promote From Dev Server To Small Mesh

Goal: move from local dev loop to first architecture-valid Mode 1 mesh.

Add:
- two app manager partitions or equivalent local partitions
- at least two peers
- explicit provider claim lease expiry
- route publication expiry
- simple reconnect behavior
- basic p2p transport for live channel

Deliverables:
- Mode 1 run script
- p2p terminal demo using the real Glade-shaped records
- console visibility across peers

Acceptance criteria:
- the terminal slice does not depend on one hidden HTTP server as authority
- records remain inspectable
- local-dev behavior and Mode 1 behavior share the same semantics

## Sequencing Rules

1. Do not refine the DSL ahead of implementation pressure.
2. Do not implement the distributed control plane before one provider-backed
   GripLab surface works.
3. Do not add CRDT until authoritative-patch file editing is insufficient.
4. Do not build a marketplace or hosted namespace registry before the golden
   path works.
5. Do not let AI integration bypass delegated capability and audit semantics.
6. Do not hide provider, route, lease, or diagnostic state from the console.

## Immediate Next Step

Create the terminal-slice implementation checklist.

It should name:
- exact current GripLab files to inspect
- first `.glade` declarations used
- generated TS shape
- generated Python or JS provider shape
- local dev runtime shape
- console projection rows
- tests or manual verification steps

That checklist should be short enough to execute.
