# Harsh Reality Triage

Status: working draft

Purpose: capture the architecture review gaps without turning every concern
into an immediate spec project.

## Core Claim

The architecture is directionally strong, but the product loop is not yet
tight enough.

The immediate risk is spending too much time refining declaration language
surface area before proving the developer golden path.

The right move is to classify gaps:
- critical now
- plausible story now, expand later
- explicitly deferred

## Critical Now

These must be addressed before serious implementation work, because they shape
the first code slice.

### 1. Developer Golden Path

Current state: implied across several docs.

Risk: the stack becomes architecturally interesting but not productively usable.

Required next step: define and build the GripLab golden path.

Owner doc:
`/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/GladeDeveloperGoldenPath.md`

### 2. Generated Endpoint Contract

Current state: generated bindings are mentioned, but the developer-facing
contract is not concrete.

Risk: `.glade` declarations become documentation instead of a tool that creates
real client/provider seams.

Near-term requirement: one declared surface MUST generate:
- TypeScript client helper
- Python or JS provider stub
- shared ids and schema hashes
- provider claim helper
- local test harness hook

Expansion trigger: before implementing the first GripLab Glade-backed tap.

### 3. Mock-To-Real Tap Promotion

Current state: stated as a principle in rapid development and Grip Share docs.

Risk: UI rewrites creep back in and destroy the core value proposition.

Near-term requirement: define how one Grip mock tap switches to a Glade-backed
surface with minimal UI change.

Expansion trigger: first GripLab terminal or file-window tap migration.

### 4. Control-Plane Visibility

Current state: now documented as a console, but not schema-bound.

Risk: provisioning, claims, routes, and leases become invisible magic.

Near-term requirement: the first dev server must expose enough projection state
to debug provider registration and routing.

Expansion trigger: first local provider publishes a leased claim.

### 5. AI Agent Action Model

Current state: agents are modeled as sessions with delegated capabilities.

Risk: AI integration becomes a side channel that bypasses the architecture.

Near-term requirement: define one action loop:

```text
delegated reference -> agent reads context -> agent proposes action
-> optional user approval -> Glade exchange or patch -> audit record
```

Expansion trigger: first GripLab AI exchange such as `ExplainTerminalError` or
`SuggestFilePatch`.

## Plausible Story Now, Expand Later

These need a credible boundary now, not detailed implementation.

### 1. Frankenapps

Plausible story: Glial environments mount multiple workspaces and facets;
external connectors are providers; cross-app links are shared records; agent
actions are delegated and audited.

Owner doc:
`/Users/owebeeone/limbo/glade-wz/dev-docs/requirements/GlialFrankenappSilverPath.md`

Expand when GripLab proves one real provider surface and one agent flow.

### 2. Hosted Multi-Tenant Account Model

Plausible story: package namespace, organization, project, environment,
deployment, tenant, and quota live above core Glade declarations in hosted
provider policy and environment overlays.

Expand when hosted admission or package registry work begins.

### 3. Cache Semantics

Plausible story: each content model declares freshness, stale-read behavior,
invalidation, replay, retention, and local persistence policy.

Expand when implementing file windows, terminal logs, or offline rejoin.

### 4. Storage Provider Boundary

Plausible story: declaration records, append logs, blobs/chunks, encrypted
content, and local caches are separate stores behind Glade abstractions.

Expand when the first durable log or sparse file cache needs persistence.

### 5. Transport Capability Matrix

Plausible story: p2p is the first architecture-valid target; each surface type
declares transport needs such as live stream, append log, chunk fetch, pubsub,
relay, or fallback.

Expand before selecting or implementing libp2p protocols for the first live
terminal slice.

### 6. External Connector Registry

Plausible story: connector packages are application definition collections
with provider definitions, external authority policy, schemas, and capability
requirements.

Expand when the silver path moves from sketch to implementation.

## Explicitly Deferred

These are important, but should not block the first implementation.

- production-grade distributed scheduling
- global hosted namespace validation
- marketplace/package registry
- CRDT collaborative editing
- full enterprise identity provider integration
- complex billing and cost estimation
- multi-region route index invalidation
- formal connector certification

They should remain visible in decision logs and architecture docs, but coding
should not wait for them.

## Sequencing Recommendation

Next work should proceed in this order:

1. Define the exact first golden-path vertical slice.
2. Define one generated endpoint contract for that slice.
3. Define one Grip mock-to-real adapter seam.
4. Build or stub the local dev server and console projection for that slice.
5. Add one AI agent exchange only after the first provider-backed surface works.
6. Return to broader DSL/schema refinement with implementation pressure from
   the slice.

## Review Rule

When a new feature idea appears, classify it before designing it:
- golden-path blocker
- silver-path pressure test
- plausible story only
- deferred

If it is not a golden-path blocker, do not let it slow the first slice.
