# Glade Control Plane Console

Status: working draft

Purpose: define the rudimentary service and UI application used to inspect and
operate Glade provisioning, placement, routing, and health state during
development and later production deployments.

## Core Claim

The control-plane console SHOULD be a first-class Glade application.

It is not the control plane itself. It is a visibility and operations
application over control-plane records.

The console backend SHOULD provide projections over Glade records. The console
UI SHOULD render those projections and publish authorized intent records for
operator actions.

The console MUST NOT become a hidden source of truth.

## Why This Exists

The distributed control plane is too abstract without visibility.

Before serious implementation work, the development environment SHOULD expose:
- what declarations were loaded
- which controllers are running
- which nodes are eligible
- which services are desired
- which allocations were made
- which provider claims are live
- which routes are published
- which leases are close to expiry
- which health checks are failing
- which decisions were made and why

This makes provisioning behavior debuggable while the architecture is still
small.

## Application Shape

The console has two parts:

| Part | Responsibility |
| --- | --- |
| `console service` | Watches Glade records, builds projections, exposes query surfaces, and validates operator intents. |
| `console UI` | Displays desired, observed, decision, route, health, and diagnostic views. |

In local development, both MAY run inside one dev server process.

In production, the console service SHOULD be deployable as an ordinary Glade
provider with explicit read and operation capabilities.

## Source Of Truth

The source of truth remains Glade records.

The console service MAY build derived views:
- node inventory
- deployment status
- allocation graph
- service instance table
- provider claim table
- route table
- lease timeline
- health timeline
- quota summary
- event and decision timeline

Derived views MUST be explainable from source records.

Derived views MUST expose enough source references for diagnostics, including:
- record id
- record kind
- source package id or system space id
- controller id, where relevant
- owner term, where relevant
- lease expiry, where relevant
- source record hash, where available

## Read Surfaces

The first console service SHOULD expose read surfaces for:

| Surface | Meaning |
| --- | --- |
| `DeclarationInventory` | Loaded packages, environment overlays, definitions, schemas, and feature versions. |
| `ControllerStatus` | Active controllers, partitions, owner terms, leases, and diagnostics. |
| `NodeInventory` | Node claims, node capacity, node health, eligibility, and resource usage. |
| `DeploymentStatus` | Desired deployment plans compared to running service instances. |
| `AllocationGraph` | Service allocations, deallocations, assignments, and their source decisions. |
| `ProviderClaims` | Live and expired provider claims with readiness and served surfaces. |
| `RouteTable` | Resource routes, session assignments, routing publications, and fallbacks. |
| `LeaseTimeline` | Records ordered by expiry, renewal, and stale state. |
| `HealthTimeline` | Health samples, failed checks, recovery events, and diagnostics. |
| `DecisionTimeline` | Placement, routing, rollout, drain, preemption, and rollback decisions. |
| `QuotaSummary` | Tenant, user, package, service, and class-of-service consumption. |

These surfaces MAY initially be simple JSON snapshots or streams.

## Operation Surfaces

Operator actions SHOULD publish intent records.

The console UI MUST NOT directly rewrite service instances, provider claims, or
routes.

Initial operation surfaces MAY include:

| Operation | Published record |
| --- | --- |
| `RequestReconcile` | Ask a controller partition to re-evaluate desired versus observed state. |
| `CreateDrainIntent` | Ask controllers to stop assigning new work and drain existing work. |
| `CreateRollbackIntent` | Ask rollout controllers to move back to a previous good version. |
| `CreateScaleIntent` | Ask controllers to adjust desired min or max instance counts. |
| `CreateRouteRefreshIntent` | Ask route controllers to rebuild or republish derived routes. |
| `AcknowledgeDiagnostic` | Mark an operator-visible diagnostic as acknowledged. |

Each operation MUST be capability checked.

Each operation SHOULD produce an auditable record that includes:
- operator principal
- target id
- reason
- requested change
- timestamp
- policy version
- optional expiry

## Minimal Dev Target

The first executable console target SHOULD be intentionally small.

It SHOULD show:
- loaded genesis id
- loaded package ids
- active local node id
- active controller ids
- desired services
- service allocations
- service instances
- provider claims
- published routes
- lease expiry countdowns
- recent diagnostics

It SHOULD support:
- refresh projections
- request reconcile for a local controller partition
- create a drain intent for a dev service instance
- inspect raw source records for any visible projection row

It MAY use:
- one local dev server
- in-memory record storage
- file-backed snapshots
- fake or loopback p2p
- mocked load samples

It MUST still use the same desired/observed/decision record concepts as larger
deployments.

## UI Requirements

The UI SHOULD make distributed state legible.

It SHOULD include:
- desired versus observed status
- record provenance links
- stale and expired lease indicators
- controller owner term display
- per-partition controller status
- route target and fallback display
- provider readiness versus liveness distinction
- diagnostic severity and acknowledgement state

It SHOULD NOT hide degraded state behind a single green/red status.

The UI SHOULD show enough intermediate states to debug provisioning:
- desired but not allocated
- allocated but not started
- started but not ready
- ready but not routed
- routed but unhealthy
- draining
- expired
- superseded

## Security And Capability Boundary

The console is powerful and MUST be capability scoped.

Read capabilities SHOULD be separated from operation capabilities.

Suggested capability groups:
- `control.read.declarations`
- `control.read.nodes`
- `control.read.deployments`
- `control.read.routes`
- `control.read.health`
- `control.operate.reconcile`
- `control.operate.drain`
- `control.operate.rollback`
- `control.operate.scale`
- `control.operate.routes`

Relay-only or low-trust peers MAY see reduced metadata projections.

The console MUST respect metadata exposure policy.

## Relationship To Grip

The console UI is a good Grip application candidate.

Grip can rapidly build the UI over projection taps while Glade remains the
source of truth for records, claims, leases, routes, and intents.

The Grip mapping SHOULD remain an adapter concern. The console declaration
should name the share surfaces and capabilities, not individual Grip tap keys.

## Relationship To Production

The development console SHOULD not be thrown away when moving to production.

The production version MAY replace:
- local in-memory projections with indexed projection services
- loopback p2p with real p2p routing
- mock load samples with real telemetry
- permissive local permissions with strict operator capabilities
- dev drain/reconcile operations with audited change-control workflows

The semantic surfaces SHOULD remain stable.

## Provisional Declaration Sketch

This sketch is illustrative only.

```glade
application glade_console {
  id "app:glade-console"
  facet declarations
  facet controllers
  facet nodes
  facet deployments
  facet allocations
  facet providers
  facet routes
  facet health
  facet diagnostics
}

provider glade_console_service {
  id "provider-def:glade-console-service"

  serves stream DeclarationInventory
  serves stream ControllerStatus
  serves stream NodeInventory
  serves stream DeploymentStatus
  serves stream AllocationGraph
  serves stream ProviderClaims
  serves stream RouteTable
  serves stream HealthTimeline
  serves stream DecisionTimeline

  serves exchange RequestReconcile
  serves exchange CreateDrainIntent
  serves exchange CreateRollbackIntent
}
```

## Open Issues

This document does not yet define:
- exact projection schema
- raw record inspection permissions
- metadata redaction policy
- UI routing structure
- development server package layout
- persisted event-log format
- whether projection surfaces are ordinary streams or a specialized query model
